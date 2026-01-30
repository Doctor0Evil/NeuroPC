use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

/// ---------- Policy types ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalPrivacyPolicy {
    pub allowed_exports: Vec<String>,
    pub forbidden_exports: Vec<String>,
    pub logging_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalIntegrityPolicy {
    pub max_state_divergence: f32,
    pub require_rollback_path: bool,
    pub forbid_irreversible_ops: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveLibertyPolicy {
    pub allow_self_chosen_augmentation: bool,
    pub max_external_auto_changes: u32,
    pub require_explanation_for_all: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurorightsPolicy {
    pub mental_privacy: MentalPrivacyPolicy,
    pub mental_integrity: MentalIntegrityPolicy,
    pub cognitive_liberty: CognitiveLibertyPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiInitiative {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "suggest_only")]
    SuggestOnly,
    #[serde(rename = "bounded_autonomy")]
    BoundedAutonomy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModePolicy {
    pub ai_initiative: AiInitiative,
    pub allow_auto_evolve: bool,
    #[serde(default)]
    pub requires_evolve_token: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModesConfig {
    pub CONSERVATIVE: ModePolicy,
    pub CO_PILOT: ModePolicy,
    pub AUTO_EVOLVE: ModePolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurorightsPolicyDocument {
    pub subject_id: String,
    pub version: String,
    pub neurorights: NeurorightsPolicy,
    pub modes: ModesConfig,
    pub active_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainChannel {
    pub max: u8,
    pub rollback_at: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainEnvelope {
    pub muscular: PainChannel,
    pub cognitive: PainChannel,
    pub emotional: PainChannel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionBounds {
    pub max_param_change_per_day: f32,
    pub max_arch_change_per_month: f32,
    pub require_evolve_for_arch_change: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationDepth {
    pub observer_only: Vec<String>,
    pub advisor: Vec<String>,
    pub bounded_auto: Vec<String>,
    pub forbidden: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionPolicyDocument {
    pub subject_id: String,
    pub pain_envelope: PainEnvelope,
    pub evolution_bounds: EvolutionBounds,
    pub integration_depth: IntegrationDepth,
}

/// ---------- EVOLVE token ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysioGuard {
    pub hrv_lf_hf_min: Option<f32>,
    pub emg_fatigue_max: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolveToken {
    pub id: String,
    pub subject_id: String,
    pub scope: Vec<String>,
    pub max_effect_size: f32,
    pub valid_from: String,
    pub valid_until: String,
    pub physio_guard: Option<PhysioGuard>,
    pub revocable: bool,
}

/// ---------- Biophysical state (abstract) ----------

#[derive(Debug, Clone, Default)]
pub struct StateVector {
    /// 0–10 subjective or estimated scale
    pub muscular_pain: u8,
    pub cognitive_load: u8,
    pub emotional_stress: u8,
    /// 0–1 normalized indices
    pub fatigue_index: f32,
    pub hrv_lf_hf: f32,
    pub emg_fatigue: f32,
}

pub trait BiophysicalStateReader: Send + Sync {
    fn read_state(&self) -> StateVector;
}

/// ---------- Update proposal & audit ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateKind {
    ParamNudge,
    ThresholdShift,
    RoutingChange,
    ArchChange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEffectBounds {
    pub l2_delta_norm: f32,
    pub irreversible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProposal {
    pub id: String,
    pub module: String,
    pub kind: UpdateKind,
    pub scope: Vec<String>,
    pub description: String,
    pub effect_bounds: UpdateEffectBounds,
    pub requires_evolve: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionOutcome {
    Allowed,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub proposal_id: String,
    pub evolve_token_id: Option<String>,
    pub decision: DecisionOutcome,
    pub reason: String,
    pub timestamp: String,
    pub state_snapshot: StateVector,
    pub rollback_available: bool,
}

/// ---------- Consent engine ----------

pub struct SovereigntyCore<S: BiophysicalStateReader> {
    neurorights: NeurorightsPolicyDocument,
    evolution: EvolutionPolicyDocument,
    state_reader: S,
    evolve_tokens: HashMap<String, EvolveToken>,
    used_auto_changes: u32,
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

    pub fn revoke_evolve_token(&mut self, token_id: &str) {
        self.evolve_tokens.remove(token_id);
    }

    /// Check whether a module is allowed given integration depth roles.
    fn check_integration_depth(&self, module: &str, auto: bool) -> bool {
        let depth = &self.evolution.integration_depth;
        let m = module.to_string();
        if depth.forbidden.contains(&m) {
            return false;
        }
        if auto && depth.bounded_auto.contains(&m) {
            return true;
        }
        if !auto && (depth.advisor.contains(&m) || depth.observer_only.contains(&m)) {
            return true;
        }
        false
    }

    fn active_mode_policy(&self) -> &ModePolicy {
        match self.neurorights.active_mode.as_str() {
            "CONSERVATIVE" => &self.neurorights.modes.CONSERVATIVE,
            "AUTO_EVOLVE" => &self.neurorights.modes.AUTO_EVOLVE,
            _ => &self.neurorights.modes.CO_PILOT,
        }
    }

    fn now_iso8601() -> String {
        // Simple placeholder; in production wire a real clock / chrono.
        let secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        format!("2026-01-30T{:02}:00:00Z", (secs % 86400) / 3600)
    }

    /// Main entry: decide if an update may run under current policies + state.
    pub fn evaluate_update(
        &mut self,
        proposal: &UpdateProposal,
        evolve_token_id: Option<&str>,
    ) -> AuditEntry {
        let state = self.state_reader.read_state();
        let mode = self.active_mode_policy();
        let mut reason = String::new();
        let mut allowed = true;
        let mut rollback_available = true;

        // 1. Neurorights: mental integrity hard rules
        let mi = &self.neurorights.neurorights.mental_integrity;
        if proposal.effect_bounds.irreversible && mi.forbid_irreversible_ops {
            allowed = false;
            reason.push_str("Irreversible op forbidden; ");
        }
        if proposal.effect_bounds.l2_delta_norm > mi.max_state_divergence {
            allowed = false;
            reason.push_str("Effect exceeds max_state_divergence; ");
        }
        if mi.require_rollback_path && !rollback_available {
            allowed = false;
            reason.push_str("No rollback path; ");
        }

        // 2. Integration depth & role
        let auto = mode.allow_auto_evolve;
        if !self.check_integration_depth(&proposal.module, auto) {
            allowed = false;
            reason.push_str("Integration depth forbids this module in current context; ");
        }

        // 3. Mode & EVOLVE requirements
        if proposal.requires_evolve || mode.requires_evolve_token {
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

            // Effect size bound
            if proposal.effect_bounds.l2_delta_norm > token.max_effect_size {
                allowed = false;
                reason.push_str("Effect exceeds EVOLVE max_effect_size; ");
            }

            // Physiological guards
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

        // 4. Pain envelope thresholds (rollback triggers configured)
        let pe = &self.evolution.pain_envelope;
        if state.muscular_pain > pe.muscular.rollback_at
            || state.cognitive_load > pe.cognitive.rollback_at
            || state.emotional_stress > pe.emotional.rollback_at
        {
            allowed = false;
            reason.push_str("Pain envelope exceeded; ");
        }

        // 5. Cognitive liberty: limit external auto changes per session
        let cl = &self.neurorights.neurorights.cognitive_liberty;
        if auto {
            if self.used_auto_changes >= cl.max_external_auto_changes {
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
}
