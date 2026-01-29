use crate::evolution_frame::{
    EvolutionFrame, EvolutionFrameValidator, EvolutionVerdict, SystemAdjustmentDelta,
};
use crate::lifeforce::{LifeforceBand, SafetyCurveWave};
use crate::eco::{EcoBandProfile};
use crate::tokens::{BrainBudget, NanoBudget};
use crate::traits::{MetabolicConsent, TraitFlags};

pub struct GuardContext {
    pub brain: BrainBudget,
    pub nano: NanoBudget,
    pub eco: EcoBandProfile,
    pub lifeforce: LifeforceBand,
    pub safety_wave: SafetyCurveWave,
    pub traits: TraitFlags,
    pub metabolic_consent: MetabolicConsent,
}

pub struct NeuromorphGuard {
    pub ctx: GuardContext,
}

impl EvolutionFrameValidator for NeuromorphGuard {
    fn validate_frame(&self, frame: &EvolutionFrame) -> EvolutionVerdict {
        if !self.metabolic_consent_allows(frame) {
            return EvolutionVerdict::DenyHardStop;
        }

        if self.ctx.lifeforce.is_hard_stop() || self.ctx.safety_wave.is_hard_stop() {
            return EvolutionVerdict::DenyHardStop;
        }

        if !self.brain_nano_within_limits(frame) || !self.eco_within_limits(frame) {
            return EvolutionVerdict::DenyHardStop;
        }

        if self.ctx.lifeforce.is_soft_warn() || self.ctx.safety_wave.is_soft_warn() {
            return EvolutionVerdict::Defer;
        }

        EvolutionVerdict::Safe
    }
}

impl NeuromorphGuard {
    fn metabolic_consent_allows(&self, frame: &EvolutionFrame) -> bool {
        self.ctx
            .metabolic_consent
            .allows_plane_and_scope(frame.plane, frame.scope)
    }

    fn brain_nano_within_limits(&self, frame: &EvolutionFrame) -> bool {
        self.ctx.brain.allows(frame.cost.flop_budget)
            && self.ctx.nano.allows(frame.cost.nJ_budget)
    }

    fn eco_within_limits(&self, frame: &EvolutionFrame) -> bool {
        self.ctx.eco.allows(frame.cost.eco_intent)
    }

    pub fn downscale_for_soft_warn(
        &self,
        deltas: &[SystemAdjustmentDelta],
    ) -> Vec<SystemAdjustmentDelta> {
        deltas
            .iter()
            .map(|d| match d {
                SystemAdjustmentDelta::ScaleDelta(v) => {
                    SystemAdjustmentDelta::ScaleDelta((*v).clamp(-0.05, 0.05))
                }
                SystemAdjustmentDelta::WaveBudgetShift(s) => {
                    SystemAdjustmentDelta::WaveBudgetShift((*s).clamp(-1, 1))
                }
                SystemAdjustmentDelta::NanoEnvelopeAdjust(s) => {
                    SystemAdjustmentDelta::NanoEnvelopeAdjust((*s).clamp(-1, 1))
                }
            })
            .collect()
    }
}
