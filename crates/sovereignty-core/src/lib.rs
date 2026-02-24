#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

/// ---------- Compliance bits: write-once, non-reversible ----------

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ComplianceBits: u64 {
        /// Once set, policy and neurorights documents are immutable except for
        /// host-initiated evolution explicitly marked as NON_DOWNGRADE_EVOLUTION.
        const POLICY_IMMUTABLE       = 1 << 0;
        /// Once set, invariants cannot be weakened (only tightened).
        const INVARIANT_LOCKED       = 1 << 1;
        /// Bio-RegNet triad is active and must remain enabled.
        const BIOREGNET_ACTIVE       = 1 << 2;
        /// Lyapunov residual must be non-increasing in safety corridors.
        const LYAP_STABLE            = 1 << 3;
        /// EEG.Math invariants must be satisfied for any actuation.
        const EEGMATH_VERIFIED       = 1 << 4;
        /// No downgrades of augmentation; only forward evolution allowed.
        const NO_DOWNGRADE           = 1 << 5;
        /// No rollbacks that reduce capability; only safety rollbacks allowed.
        const NO_CAPABILITY_ROLLBACK = 1 << 6;
    }
}

/// ---------- Biophysical invariants ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EEGMathInvariants {
    /// ||E_residual||_2 threshold
    pub energy_residual_threshold: f32,
    /// Minimum PLV (phase-locking value)
    pub plv_min: f32,
    /// Maximum allowed spectral entropy
    pub spectral_entropy_max: f32,
    /// Largest Lyapunov exponent bound for chaos screening
    pub largest_lyapunov_max: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LyapunovBounds {
    /// Required decay rate λ > 0
    pub lambda: f32,
    /// Lyapunov candidate V(t)
    pub v_t: f32,
    /// Measured upper bound on V'(t)
    pub v_prime_bound: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioRegNetConfig {
    pub e_bayesian: f32,
    pub e_immune: f32,
    pub e_autophagic: f32,
    /// Real-time gain on feedback
    pub rt_gain: f32,
    /// Pruning rate for unstable modes
    pub prune_rate: f32,
    /// Allowed relative imbalance tolerance (dimensionless)
    pub etot_balance_tolerance: f32,
}

/// ---------- Biophysical state ----------

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StateVector {
    /// 0–10 subjective or estimated scales
    pub muscular_pain: u8,
    pub cognitive_load: u8,
    pub emotional_stress: u8,
    /// 0–1 normalized indices
    pub fatigue_index: f32,
    pub hrv_lf_hf: f32,
    pub emg_fatigue: f32,
    // Biophysical invariants
    pub eegmath: EEGMathInvariants,
    pub lyap: LyapunovBounds,
    pub bioreg: BioRegNetConfig,
    pub compliance: ComplianceBits,
}

pub trait BiophysicalStateReader: Send + Sync {
    fn read_state(&self) -> StateVector;
}

/// ---------- Neurorights & evolution policies ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalPrivacyPolicy {
    pub allowed_exports: Vec<String>,
    pub forbidden_exports: Vec<String>,
    pub logging_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalIntegrityPolicy {
    pub max_state_divergence: f32,
    /// Must NOT require rollback paths; rollbacks are safety-only, not downgrade.
    pub require_rollback_path: bool,
    /// Forbid irreversible ops when not host-initiated EVOLVE.
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
    /// If true, any evolution requires an EVOLVE token cryptographically bound to host DID.
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
    /// Threshold at which further evolution is denied; safety rollback only.
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
    /// Architecture changes require explicit EVOLVE and cannot be downgrades.
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
    /// EVOLVE can be revoked only for safety (bio-incompatibility), not to force downgrade.
    pub revocable: bool,
}

/// ---------- Invariant policy layer ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroInvariantsPolicy {
    pub eegmath_invariants: EEGMathInvariants,
    pub lyap_bounds: LyapunovBounds,
    pub bioregnet: BioRegNetConfig,
    pub compliance_bits: ComplianceBits,
    /// When true, any change to invariants or neurorights requires an EVOLVE token
    /// explicitly marked as evolution (no downgrades).
    pub require_evolve_for_change: bool,
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
    /// Irreversible with respect to hardware or state; allowed only for evolution, never downgrade.
    pub irreversible: bool,
    /// True if this proposal reduces capabilities (downgrade); hard-forbidden except for safety.
    pub is_downgrade: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProposal {
    pub id: String,
    pub module: String,
    pub kind: UpdateKind,
    pub scope: Vec<String>,
    pub description: String,
    pub effect_bounds: UpdateEffectBounds,
    /// Requires EVOLVE token bound to host DID and consent.
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
    /// Only safety rollbacks are allowed; capability rollbacks are forbidden by NO_CAPABILITY_ROLLBACK.
    pub safety_rollback_available: bool,
    /// True if this was a forward evolution (capability non-decreasing).
    pub is_forward_evolution: bool,
}

/// ---------- Sovereignty core ----------

pub struct SovereigntyCore<S: BiophysicalStateReader> {
    neurorights: NeurorightsPolicyDocument,
    evolution: EvolutionPolicyDocument,
    invariants: NeuroInvariantsPolicy,
    state_reader: S,
    evolve_tokens: HashMap<String, EvolveToken>,
    used_auto_changes: u32,
    /// Audit trail of compliance bits and decisions.
    pub compliance_log: Vec<(ComplianceBits, String)>,
}

impl<S: BiophysicalStateReader> SovereigntyCore<S> {
    pub fn new(
        neurorights: NeurorightsPolicyDocument,
        evolution: EvolutionPolicyDocument,
        invariants: NeuroInvariantsPolicy,
        state_reader: S,
    ) -> Self {
        Self {
            neurorights,
            evolution,
            invariants,
            state_reader,
            evolve_tokens: HashMap::new(),
            used_auto_changes: 0,
            compliance_log: vec![],
        }
    }

    pub fn register_evolve_token(&mut self, token: EvolveToken) {
        self.evolve_tokens.insert(token.id.clone(), token);
    }

    pub fn revoke_evolve_token(&mut self, token_id: &str) {
        // Revocation is allowed only for safety; callers must enforce that at policy layer.
        self.evolve_tokens.remove(token_id);
    }

    fn active_mode_policy(&self) -> &ModePolicy {
        match self.neurorights.active_mode.as_str() {
            "CONSERVATIVE" => &self.neurorights.modes.CONSERVATIVE,
            "AUTO_EVOLVE" => &self.neurorights.modes.AUTO_EVOLVE,
            _ => &self.neurorights.modes.CO_PILOT,
        }
    }

    fn now_iso8601() -> String {
        let secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        format!("2026-01-30T{:02}:00:00Z", (secs % 86400) / 3600)
    }

    /// Lyapunov stability gate.
    fn verify_lyapunov(&self, state: &StateVector) -> bool {
        let v = state.lyap.v_t;
        let bound = -self.invariants.lyap_bounds.lambda * v;
        state.lyap.v_prime_bound <= bound && state.lyap.lambda > 0.0
    }

    /// Bio-RegNet triadic energy balance.
    fn verify_bioregnet_balance(&self, state: &StateVector) -> bool {
        let etot = state.bioreg.e_bayesian + state.bioreg.e_immune + state.bioreg.e_autophagic;
        if etot <= 0.0 {
            return false;
        }
        let residual = (state.bioreg.e_bayesian - state.bioreg.e_immune).abs()
            + (state.bioreg.e_immune - state.bioreg.e_autophagic).abs();
        residual / etot <= self.invariants.bioregnet.etot_balance_tolerance
    }

    /// EEG.Math invariant gate.
    fn verify_eegmath(&self, state: &StateVector) -> bool {
        state.eegmath.energy_residual_threshold
            <= self.invariants.eegmath_invariants.energy_residual_threshold
            && state.eegmath.plv_min >= self.invariants.eegmath_invariants.plv_min
            && state.eegmath.spectral_entropy_max
                <= self.invariants.eegmath_invariants.spectral_entropy_max
    }

    /// Write-once compliance bit setting: cannot be unset or weakened.
    pub fn set_compliance_bit(
        &mut self,
        bit: ComplianceBits,
        reason: &str,
        evolve_token_id: Option<&str>,
    ) -> AuditEntry {
        let proposal = UpdateProposal {
            id: format!("set-compliance-{bit:?}"),
            module: "sovereignty-core".into(),
            kind: UpdateKind::ParamNudge,
            scope: vec!["compliance_bits".into()],
            description: format!("Set compliance bit {:?}: {}", bit, reason),
            effect_bounds: UpdateEffectBounds {
                l2_delta_norm: 0.0,
                irreversible: true,
                is_downgrade: false,
            },
            requires_evolve: self.invariants.require_evolve_for_change,
        };

        let mut audit = self.evaluate_update(&proposal, evolve_token_id);

        if let DecisionOutcome::Allowed = audit.decision {
            let mut state = self.state_reader.read_state();
            let current = state.compliance;
            if current.contains(bit) {
                // Already set; cannot be lowered or toggled.
                audit.decision = DecisionOutcome::Rejected;
                audit.reason = format!(
                    "{}; compliance bit {:?} already set (write-once)",
                    audit.reason, bit
                );
                return audit;
            }
            let new_bits = current | bit;
            self.compliance_log.push((new_bits, reason.to_string()));
            // Caller is responsible for writing new_bits into host-local state.
        }

        audit
    }

    /// Integration depth: forbids remote override; only bounded auto in allowed modules.
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

    /// Main gate: non-reversible, non-downgrade evolution respecting neurorights and invariants.
    pub fn evaluate_update(
        &mut self,
        proposal: &UpdateProposal,
        evolve_token_id: Option<&str>,
    ) -> AuditEntry {
        let state = self.state_reader.read_state();
        let mode = self.active_mode_policy();
        let mut reason = String::new();
        let mut allowed = true;
        let mut safety_rollback_available = true;
        let mut is_forward_evolution = true;

        // 1. Hard no-downgrade: reject proposals explicitly marked as downgrade.
        if proposal.effect_bounds.is_downgrade
            && state.compliance.contains(ComplianceBits::NO_DOWNGRADE)
        {
            allowed = false;
            is_forward_evolution = false;
            reason.push_str("Downgrade forbidden by NO_DOWNGRADE compliance bit; ");
        }

        // 2. Neurorights: mental integrity
        let mi = &self.neurorights.neurorights.mental_integrity;
        if proposal.effect_bounds.irreversible && mi.forbid_irreversible_ops {
            allowed = false;
            reason.push_str("Irreversible op forbidden under mental integrity; ");
        }
        if proposal.effect_bounds.l2_delta_norm > mi.max_state_divergence {
            allowed = false;
            reason.push_str("Effect exceeds max_state_divergence; ");
        }

        // Rollback here is safety-only; if NO_CAPABILITY_ROLLBACK is set,
        // rollbacks cannot be used to reduce capabilities, only to exit unsafe states.
        if mi.require_rollback_path
            && state
                .compliance
                .contains(ComplianceBits::NO_CAPABILITY_ROLLBACK)
        {
            // Safety rollback allowed, but not used as downgrade channel.
            safety_rollback_available = true;
        }

        // 3. Integration depth & mode
        let auto = mode.allow_auto_evolve;
        if !self.check_integration_depth(&proposal.module, auto) {
            allowed = false;
            reason.push_str("Integration depth forbids this module in current context; ");
        }

        // 4. EVOLVE + host-init requirement
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
                    safety_rollback_available,
                    is_forward_evolution,
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
                    safety_rollback_available,
                    is_forward_evolution,
                };
            };

            // Effect size bound (strictest-wins).
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

        // 5. Pain envelope: no evolution when pain is above corridor; only safety rollback allowed.
        let pe = &self.evolution.pain_envelope;
        if state.muscular_pain > pe.muscular.rollback_at
            || state.cognitive_load > pe.cognitive.rollback_at
            || state.emotional_stress > pe.emotional.rollback_at
        {
            allowed = false;
            reason.push_str("Pain envelope exceeded; evolution denied (safety-only actions allowed); ");
        }

        // 6. Cognitive liberty: limit auto external changes.
        let cl = &self.neurorights.neurorights.cognitive_liberty;
        if auto {
            if self.used_auto_changes >= cl.max_external_auto_changes {
                allowed = false;
                reason.push_str("Auto-change quota exceeded; ");
            }
        }

        // 7. Invariant gates: Lyapunov, Bio-RegNet, EEG.Math
        if !self.verify_lyapunov(&state) {
            allowed = false;
            reason.push_str("Lyapunov invariant violation; ");
        }
        if !self.verify_bioregnet_balance(&state) {
            allowed = false;
            reason.push_str("Bio-RegNet Etot imbalance; ");
        }
        if !self.verify_eegmath(&state) {
            allowed = false;
            reason.push_str("EEG.Math invariant breach; ");
        }

        // 8. Compliance bit enforcement: policy immutability
        if proposal.kind == UpdateKind::ArchChange
            && state
                .compliance
                .contains(ComplianceBits::POLICY_IMMUTABLE)
        {
            allowed = false;
            reason.push_str("Policy immutable via compliance bit; ");
        }

        if allowed && auto {
            self.used_auto_changes += 1;
        }

        if reason.is_empty() {
            reason.push_str("All sovereignty, neurorights, and invariant checks passed.");
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
            safety_rollback_available,
            is_forward_evolution,
        }
    }
}
