use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateVector {
    pub muscular_pain: i32,
    pub cognitive_load: i32,
    pub emotional_stress: i32,
    pub fatigue_index: f32,
    pub hrv_lf_hf: f32,
    pub emg_fatigue: f32,
}

pub trait BiophysicalStateReader {
    fn read_state(&self) -> StateVector;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolveToken {
    pub id: String,
    pub max_effect_size: f32,
    pub physio_guard: Option<PhysioGuard>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhysioGuard {
    pub hrv_lf_hf_min: Option<f32>,
    pub emg_fatigue_max: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UpdateKind {
    ParamNudge,
    ThresholdShift,
    RoutingChange,
    ArchChange,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateEffectBounds {
    pub l2_delta_norm: f32,
    pub irreversible: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateProposal {
    pub id: String,
    pub module: String,
    pub kind: UpdateKind,
    pub scope: Vec<String>,
    pub description: String,
    pub effect_bounds: UpdateEffectBounds,
    pub requires_evolve: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DecisionOutcome {
    Allowed,
    Rejected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEntry {
    pub proposal_id: String,
    pub evolve_token_id: Option<String>,
    pub decision: DecisionOutcome,
    pub reason: String,
    pub timestamp: String,
    pub state_snapshot: StateVector,
    pub rollback_available: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsPolicyDocument {
    pub max_state_divergence: f32,
    pub forbid_irreversible_ops: bool,
    pub require_rollback_path: bool,
    pub max_external_auto_changes: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolutionPolicyDocument {
    pub muscular_rollback_at: i32,
    pub cognitive_rollback_at: i32,
    pub emotional_rollback_at: i32,
}

pub struct SovereigntyCore<S: BiophysicalStateReader> {
    pub neurorights: NeurorightsPolicyDocument,
    pub evolution: EvolutionPolicyDocument,
    pub state_reader: S,
    pub evolve_tokens: HashMap<String, EvolveToken>,
    pub used_auto_changes: u32,
}

impl<S: BiophysicalStateReader> SovereigntyCore<S> {
    pub fn new(
        neurorights: NeurorightsPolicyDocument,
        evolution: EvolutionPolicyDocument,
        state_reader: S,
    ) -> Self {
        Self {
            neurorights,
            evolution,
            state_reader,
            evolve_tokens: HashMap::new(),
            used_auto_changes: 0,
        }
    }

    pub fn register_evolve_token(&mut self, token: EvolveToken) {
        self.evolve_tokens.insert(token.id.clone(), token);
    }

    pub fn evaluate_update(
        &mut self,
        proposal: &UpdateProposal,
        evolve_token_id: Option<&str>,
        auto: bool,
    ) -> AuditEntry {
        let state = self.state_reader.read_state();
        let mut reason = String::new();
        let mut allowed = true;
        let rollback_available = true;

        let nr = &self.neurorights;

        if proposal.effect_bounds.irreversible && nr.forbid_irreversible_ops {
            allowed = false;
            reason.push_str("Irreversible op forbidden; ");
        }
        if proposal.effect_bounds.l2_delta_norm > nr.max_state_divergence {
            allowed = false;
            reason.push_str("Effect exceeds max_state_divergence; ");
        }
        if nr.require_rollback_path && !rollback_available {
            allowed = false;
            reason.push_str("No rollback path; ");
        }

        if proposal.requires_evolve {
            let Some(token_id) = evolve_token_id else {
                allowed = false;
                reason.push_str("Missing EVOLVE token; ");
                return AuditEntry {
                    proposal_id: proposal.id.clone(),
                    evolve_token_id: None,
                    decision: DecisionOutcome::Rejected,
                    reason,
                    timestamp: Self::now_iso8601(),
                    state_snapshot: state,
                    rollback_available,
                };
            };
            let Some(token) = self.evolve_tokens.get(token_id) else {
                allowed = false;
                reason.push_str("Unknown EVOLVE token; ");
                return AuditEntry {
                    proposal_id: proposal.id.clone(),
                    evolve_token_id: Some(token_id.to_string()),
                    decision: DecisionOutcome::Rejected,
                    reason,
                    timestamp: Self::now_iso8601(),
                    state_snapshot: state,
                    rollback_available,
                };
            };
            if proposal.effect_bounds.l2_delta_norm > token.max_effect_size {
                allowed = false;
                reason.push_str("Effect exceeds EVOLVE max_effect_size; ");
            }
            if let Some(pg) = &token.physio_guard {
                if let Some(min) = pg.hrv_lf_hf_min {
                    if state.hrv_lf_hf < min {
                        allowed = false;
                        reason.push_str("HRV below EVOLVE guard; ");
                    }
                }
                if let Some(max) = pg.emg_fatigue_max {
                    if state.emg_fatigue > max {
                        allowed = false;
                        reason.push_str("EMG fatigue above EVOLVE guard; ");
                    }
                }
            }
        }

        let evo = &self.evolution;
        if state.muscular_pain > evo.muscular_rollback_at
            || state.cognitive_load > evo.cognitive_rollback_at
            || state.emotional_stress > evo.emotional_rollback_at
        {
            allowed = false;
            reason.push_str("Pain envelope exceeded; ");
        }

        if auto {
            if self.used_auto_changes >= nr.max_external_auto_changes {
                allowed = false;
                reason.push_str("Auto-change quota exceeded; ");
            }
        }
        if allowed && auto {
            self.used_auto_changes += 1;
        }

        if reason.is_empty() {
            reason.push_str("All checks passed.");
        }

        AuditEntry {
            proposal_id: proposal.id.clone(),
            evolve_token_id: evolve_token_id.map(|s| s.to_string()),
            decision: if allowed {
                DecisionOutcome::Allowed
            } else {
                DecisionOutcome::Rejected
            },
            reason,
            timestamp: Self::now_iso8601(),
            state_snapshot: state,
            rollback_available,
        }
    }

    fn now_iso8601() -> String {
        "2026-02-01T00:00:00Z".to_string()
    }
}
