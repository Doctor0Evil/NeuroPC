#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

pub type EpochNanos = i128;

/// Core biophysical state exported from BFC / Organic_CPU envelopes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BioState {
    /// 0–1, validated against your fatigue protocol (actigraphy/HRV/etc.).
    pub fatigue: f32,
    /// 0–1, fraction of recent window under active duty.
    pub duty: f32,
    /// 0–1, CognitiveLoadIndex from your NeuroPC pipeline.
    pub cognitive_load_index: f32,
}

/// Eco metrics linking infra state to your body state.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EcoMetrics {
    /// Dimensionless EcoImpactScore (explicitly documented formula).
    pub eco_impact_score: f32,
    /// Total DeviceHours for the controlling node.
    pub device_hours: f32,
    /// Thermal comfort index (e.g. WBGT‑based).
    pub thermal_comfort_index: f32,
}

/// Waste load for cyboquatic / waste‑processing control.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasteLoad {
    /// Per‑class normalized loads (e.g. kg/h or g/s).
    /// Index order MUST be fixed and documented in schema.
    pub per_class_load: Vec<f32>,
    /// Optional human‑readable class labels, not used in tight loops.
    pub class_labels: Vec<String>,
}

/// Actuation intent from cyboquatic / cybocindric controllers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ControlIntent {
    pub pump_speed_rps: f32,
    pub valve_position: f32,       // 0–1
    pub cooler_setpoint_c: f32,    // °C
}

/// Sovereign provenance for training / control records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingProvenance {
    /// Your DID / Bostrom address.
    pub subject_id: String,
    /// e.g. "NeuroPC/Organic_CPU".
    pub organic_host: String,
    /// BFC mode or protocol ID (task, posture, etc.).
    pub bfc_mode: String,
    /// Governance tags: "EVOLVE", "MUTATION", "TECH", etc.
    pub consent_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoBioState {
    pub timestamp: EpochNanos,
    pub bio_state: BioState,
    pub eco_metrics: EcoMetrics,
    pub waste_load: WasteLoad,
    pub control_intent: ControlIntent,
    pub training_provenance: TrainingProvenance,
}
