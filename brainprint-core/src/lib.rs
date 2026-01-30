use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NeurorightsMode {
    Conservative,
    Copilot,
    Autoevolve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyLayer {
    pub lifeforce_index: f32,
    pub pain_envelope_proximity: f32,
    pub neurorights_mode: NeurorightsMode,
    pub evolve_mode_flag: bool,
    pub sovereignty_violation_risk: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionLayer {
    pub roh_index_current: f32,
    pub roh_delta_week: f32,
    pub eco_energy_band: u8,
    pub eco_deviation_ratio: f32,
    pub control_param_drift_norm: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentLayer {
    pub env_stim_load_quantile: f32,
    pub env_volatility_index: f32,
    pub assistive_energy_factor: f32,
    pub k_anonymity_k: u16,
    pub provenance_epoch_hash: u128, // numeric, de-identified
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainPrintCore {
    pub sovereignty: SovereigntyLayer,
    pub evolution:   EvolutionLayer,
    pub environment: EnvironmentLayer,
}

/// Marker trait implemented by all brainPrint! telemetry records.
pub trait BrainPrintRecord {
    fn core(&self) -> &BrainPrintCore;
}
