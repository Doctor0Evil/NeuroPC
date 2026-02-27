use serde::{Deserialize, Serialize};

/// Read‑only projection of OrganicCPU BioState into sovereigntycore.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BioStateSnapshot {
    pub fatigue_index: f32,        // 0..1
    pub duty_cycle: f32,           // 0..1
    pub cognitive_load_index: f32, // 0..1
    pub hrv_index: f32,            // 0..1, optional, may be 0.
    pub device_hours_today: f32,   // hours per day
}

/// Envelope‑level decision imported from OrganicCPU (SafeEnvelopePolicy).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SafeEnvelopeDecision {
    AllowFullAction,
    DegradePrecision,
    PauseAndRest,
}
