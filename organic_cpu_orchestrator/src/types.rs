use serde::{Deserialize, Serialize};
use organiccpu_core::{BioState, SafeEnvelopeDecision};

/// Minimal biophysical snapshot CyberNano is allowed to see.
/// This is a copy, never a live handle.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrchestratorBioSnapshot {
    pub fatigue_index: f32,         // 0.0–1.0
    pub duty_cycle: f32,            // 0.0–1.0
    pub cognitive_load_index: f32,  // 0.0–1.0
    pub intent_confidence: f32,     // 0.0–1.0
    pub eco_impact_score: f32,      // 0.0–1.0 (lower = better)
    pub device_hours: f32,          // hours per day
}

impl From<&BioState> for OrchestratorBioSnapshot {
    fn from(s: &BioState) -> Self {
        Self {
            fatigue_index: s.fatigue_index,
            duty_cycle: s.duty_cycle,
            cognitive_load_index: s.cognitive_load_index,
            intent_confidence: s.intent_confidence,
            eco_impact_score: s.eco.eco_impact_score,
            device_hours: s.eco.device_hours,
        }
    }
}

/// How CyberNano wishes to run inside your OrganicCPU shell.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CyberNanoMode {
    /// Read-only introspection, no control.
    Observe,
    /// Can propose safe-filtered control, must stay inside envelopes.
    SafeFilterOnly,
    /// Can also propose evolution of its own kernels, EVOLVE-gated.
    SafeFilterPlusEvolution,
}

/// What CyberNano is allowed to ask for at boot time.
/// No direct actuation, only safe-filter and kernel-selection intentions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CyberNanoBootRequest {
    /// Logical module name, used in integration-depth checks.
    pub module_name: String,

    /// Requested mode for this session (Observe / SafeFilterOnly / SafeFilterPlusEvolution).
    pub requested_mode: CyberNanoMode,

    /// Optional ID of a pre-defined CyberNano viability-kernel profile.
    /// Example: "CN-VK-Rehab-2026v1".
    pub requested_kernel_id: Option<String>,

    /// Optional EVOLVE token id when requesting evolution powers.
    pub evolve_token_id: Option<String>,
}

/// Decision returned to CyberNano after sovereignty + envelope checks.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CyberNanoBootDecision {
    /// Whether CyberNano may start, under the conditions below.
    pub allowed: bool,

    /// Final mode authorized by the sovereignty core (may be downgraded).
    pub granted_mode: CyberNanoMode,

    /// Which kernel profile CyberNano is permitted to load (if any).
    pub granted_kernel_id: Option<String>,

    /// Safe-envelope decision for initial load (Allow / Degrade / Pause).
    pub envelope_decision: SafeEnvelopeDecision,

    /// Snapshot of the host biophysical state at decision time.
    pub bio_snapshot: OrchestratorBioSnapshot,

    /// Human-readable reason string for logs and for you.
    pub reason: String,
}

/// Errors that CyberNano must handle without bypass.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CyberNanoBootError {
    NeurorightsRejected(String),
    EnvelopeRejected(String),
    MissingEvolveToken(String),
    UnknownEvolveToken(String),
    IntegrationDepthForbidden(String),
    InternalError(String),
}
