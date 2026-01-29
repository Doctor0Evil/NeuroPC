use crate::evolution_frame::{EvolutionDecision, EvolutionFrame, EvolutionVerdict};
use crate::guards::NeuromorphGuard;
use crate::system::SystemState;

pub struct LedgerRpc {
    pub guard: NeuromorphGuard,
    pub system_state: SystemState,
}

impl LedgerRpc {
    pub fn submit_evolution_frame(&mut self, frame: EvolutionFrame) -> EvolutionDecision {
        let verdict = self.guard.validate_frame(&frame);

        match verdict {
            EvolutionVerdict::DenyHardStop => EvolutionDecision {
                frame_id: frame.frame_id.clone(),
                verdict,
                applied_deltas: Vec::new(),
            },
            EvolutionVerdict::Defer => EvolutionDecision {
                frame_id: frame.frame_id.clone(),
                verdict,
                applied_deltas: Vec::new(),
            },
            EvolutionVerdict::Safe => {
                let candidate_deltas = self.system_state.propose_deltas(&frame);
                let applied = self
                    .guard
                    .downscale_for_soft_warn(&candidate_deltas);

                let decision = self.system_state.apply_deltas(&frame, &applied);

                EvolutionDecision {
                    frame_id: frame.frame_id,
                    verdict,
                    applied_deltas: decision.applied_deltas,
                }
            }
        }
    }
}
