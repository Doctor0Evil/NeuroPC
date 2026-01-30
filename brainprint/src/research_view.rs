use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum NeurorightsMode {
    Conservative,
    Copilot,
    AutoEvolve,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum EcoBand {
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SovereigntySafetyLayer {
    // Lifeforce / pain envelope proximity (0–1).
    pub lifeforce_index: f32,
    pub muscular_pain: u8,
    pub cognitive_load: u8,
    pub emotional_stress: u8,
    // Distances to rollback thresholds from your evolution policy.[file:10]
    pub pain_margin_muscular: f32,
    pub pain_margin_cognitive: f32,
    pub pain_margin_emotional: f32,
    // Neurorights + EVOLVE state.[file:10]
    pub neurorights_mode: NeurorightsMode,
    pub evolve_active: bool,
    pub evolve_scope_motormacros: bool,
    pub evolve_scope_languagetuning: bool,
    pub max_effect_size: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LongitudinalEvolutionLayer {
    // Risk of Harm index (0–1), standardized.[file:10]
    pub roh_instant: f32,
    pub roh_trend_24h: f32,
    // Eco usage.
    pub eco_band: EcoBand,
    pub eco_energy_nj_24h: f64,
    // Control-parameter drift (L2 norms) per day/week.[file:10]
    pub ctrl_drift_daily: f32,
    pub ctrl_drift_weekly: f32,
    // Knowledge factor for assistive tuning (0–1).[file:8]
    pub knowledge_factor: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrossHostAggregateLayer {
    // Strictly de-identified, lake-only metrics.[file:10]
    pub cohort_label: String,          // e.g. "organiccpu-motorassist-v1"
    pub host_anonymized_id: String,    // salted hash, no raw DID
    pub roh_percentile_in_cohort: f32,
    pub eco_percentile_in_cohort: f32,
    pub policy_violation_rate_7d: f32, // fraction of proposed updates rejected by sovereignty core
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrainPrintResearchView {
    pub schema_version: u16,

    // (a) Sovereignty and safety first.
    pub sovereignty: SovereigntySafetyLayer,

    // (b) Longitudinal evolution metrics.
    pub evolution: LongitudinalEvolutionLayer,

    // (c) Cross-host environmental aggregates (lake-only).
    pub aggregates: Option<CrossHostAggregateLayer>,

    // Provenance and policy flags.
    pub plane_label: String,           // "bioscale","biophysics","bci-hci-eeg",…[file:8]
    pub neurorights_policy_hash: String,
    pub evolution_policy_hash: String,
    pub state_hash_hex: String,

    // Host-local extension namespace; never interpreted cross-host.[file:10]
    pub host_extensions: serde_json::Value,
}
