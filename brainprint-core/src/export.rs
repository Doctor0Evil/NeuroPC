use serde::Serialize;
use crate::BrainPrintCore;

/// Minimal, numeric-only export representation.
#[derive(Debug, Clone, Serialize)]
pub struct BrainPrintExport {
    pub lifeforce_index: f32,
    pub pain_envelope_proximity: f32,
    pub neurorights_mode: u8,
    pub evolve_mode_flag: u8,
    pub roh_index_current: f32,
    pub roh_delta_week: f32,
    pub eco_energy_band: u8,
    pub eco_deviation_ratio: f32,
    pub control_param_drift_norm: f32,
    pub env_stim_load_quantile: f32,
    pub env_volatility_index: f32,
    pub assistive_energy_factor: f32,
    pub k_anonymity_k: u16,
    pub provenance_epoch_hash: u128,
}

impl From<&BrainPrintCore> for BrainPrintExport {
    fn from(core: &BrainPrintCore) -> Self {
        use crate::NeurorightsMode::*;
        let mode_code = match core.sovereignty.neurorights_mode {
            Conservative => 0,
            Copilot      => 1,
            Autoevolve   => 2,
        };
        BrainPrintExport {
            lifeforce_index:          core.sovereignty.lifeforce_index,
            pain_envelope_proximity:  core.sovereignty.pain_envelope_proximity,
            neurorights_mode:         mode_code,
            evolve_mode_flag:         core.sovereignty.evolve_mode_flag as u8,
            roh_index_current:        core.evolution.roh_index_current,
            roh_delta_week:           core.evolution.roh_delta_week,
            eco_energy_band:          core.evolution.eco_energy_band,
            eco_deviation_ratio:      core.evolution.eco_deviation_ratio,
            control_param_drift_norm: core.evolution.control_param_drift_norm,
            env_stim_load_quantile:   core.environment.env_stim_load_quantile,
            env_volatility_index:     core.environment.env_volatility_index,
            assistive_energy_factor:  core.environment.assistive_energy_factor,
            k_anonymity_k:            core.environment.k_anonymity_k,
            provenance_epoch_hash:    core.environment.provenance_epoch_hash,
        }
    }
}
