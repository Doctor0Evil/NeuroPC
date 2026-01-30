use serde::Serialize;

use crate::{BrainPrintCore, NeurorightsMode};

/// Minimal, numeric-only export representation (identity- and finance-free).
#[derive(Debug, Clone, Serialize)]
pub struct BrainPrintExport {
    // Sovereignty + safety layer
    pub lifeforce_index:            f32,
    pub pain_envelope_proximity:    f32,
    pub neurorights_mode:           u8,
    pub evolve_mode_flag:           u8,
    pub sovereignty_violation_risk: f32,

    // Neurorights / EVOLVE flags (all host-local, non-financial)
    pub transhuman_evolution_rights: u8,
    pub self_evolution_active:       u8,
    pub continuous_sovereign_ops:    u8,
    pub self_identity_guard_level:   u8,
    pub sovereign_channel_state:     u8,

    // Longitudinal evolution metrics
    pub roh_index_current:        f32,
    pub roh_delta_week:           f32,
    pub eco_energy_band:          u8,
    pub eco_deviation_ratio:      f32,
    pub control_param_drift_norm: f32,

    // Environment / assistive load
    pub env_stim_load_quantile:  f32,
    pub env_volatility_index:    f32,
    pub assistive_energy_factor: f32,

    // Privacy / provenance (still non-identifying)
    pub k_anonymity_k:         u16,
    pub provenance_epoch_hash: u128,
}

impl From<&BrainPrintCore> for BrainPrintExport {
    fn from(core: &BrainPrintCore) -> Self {
        use NeurorightsMode::*;

        let mode_code = match core.sovereignty.neurorights_mode {
            Conservative => 0,
            Copilot      => 1,
            Autoevolve   => 2,
        };

        BrainPrintExport {
            // Sovereignty + safety
            lifeforce_index:            core.sovereignty.lifeforce_index,
            pain_envelope_proximity:    core.sovereignty.pain_envelope_proximity,
            neurorights_mode:           mode_code,
            evolve_mode_flag:           core.sovereignty.evolve_mode_flag as u8,
            sovereignty_violation_risk: core.sovereignty.sovereignty_violation_risk,

            // Neurorights / EVOLVE flags
            transhuman_evolution_rights: core.sovereignty.transhuman_evolution_rights,
            self_evolution_active:       core.sovereignty.self_evolution_active,
            continuous_sovereign_ops:    core.sovereignty.continuous_sovereign_ops,
            self_identity_guard_level:   core.sovereignty.self_identity_guard_level,
            sovereign_channel_state:     core.sovereignty.sovereign_channel_state,

            // Evolution metrics
            roh_index_current:        core.evolution.roh_index_current,
            roh_delta_week:           core.evolution.roh_delta_week,
            eco_energy_band:          core.evolution.eco_energy_band,
            eco_deviation_ratio:      core.evolution.eco_deviation_ratio,
            control_param_drift_norm: core.evolution.control_param_drift_norm,

            // Environment metrics
            env_stim_load_quantile:  core.environment.env_stim_load_quantile,
            env_volatility_index:    core.environment.env_volatility_index,
            assistive_energy_factor: core.environment.assistive_energy_factor,

            // Privacy / provenance
            k_anonymity_k:         core.environment.k_anonymity_k,
            provenance_epoch_hash: core.environment.provenance_epoch_hash,
        }
    }
}
