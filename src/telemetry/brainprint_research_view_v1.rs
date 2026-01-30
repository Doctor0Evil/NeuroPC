use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Coarse plane classification derived from BrainPrint.plane_flags
/// and/or NeurorightsPolicyDocument + OS context.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectPlane {
    Bioscale,
    BciHci,
    Cybernetic,
    SoftwareOnly,
    AugmentedCitizen,
    Sandbox,
}

/// Neurorights / OS sovereignty mode, aligned with NeurorightsPolicyDocument.modes.activemode.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SovereigntyMode {
    CONSERVATIVE,
    COPILOT,
    AUTOEVOLVE,
}

/// High-level semantic type for a namespaced metric.
/// Keeps neuromorphic/organic_cpu feature vectors simple and numeric.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticType {
    Scalar01,
    ScalarUnbounded,
    Count,
    DurationMs,
    RatePerS,
    CategoricalIndex,
}

/// Sovereign, host-defined extension metric entry.
///
/// Computed via the `brainPrint!` unleash-macro and exported only after
/// sovereignty and neurorights checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespacedMetric {
    /// Logical namespace, e.g. "bci", "sw", "nano", "motor", "lang".
    pub namespace: String,

    /// Metric name within the namespace, e.g. "signal_quality", "mem_usage".
    pub name: String,

    /// Semantic interpretation, used by data-lake feature builders.
    pub semantic_type: SemanticType,

    /// Numeric value; interpretation depends on semantic_type.
    pub value: f64,

    /// Optional unit label, e.g. "ms", "Hz", "nj"; empty for normalized metrics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// Flattened snapshot of neurorights / evolution policy state relevant
/// to export and analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyFlags {
    pub mental_privacy_strict: bool,
    pub mental_integrity_strict: bool,
    pub cognitive_liberty_active: bool,
    pub evolve_required_for_arch_change: bool,

    /// From CognitiveLibertyPolicy.max_external_auto_changes.
    pub max_external_auto_changes: u32,

    /// Number of auto-changes consumed in the current session/window.
    pub auto_changes_used: u32,

    /// True if muscular, cognitive, or emotional pain channel exceeded rollback threshold.
    pub pain_envelope_exceeded: bool,

    /// True if this record was allowed to be exported under current policy.
    pub export_allowed: bool,
}

/// BrainPrintResearchView v1: de-identified, sovereignty-first telemetry view.
///
/// Each instance corresponds to one line in an NDJSON/JSONL stream,
/// conforming to brainprint_research_v1.schema.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainPrintResearchViewV1 {
    /// Research view schema version. For this struct: 1.
    pub schema_version: u16,

    /// Opaque per-record identifier (UUID, monotonic ID, etc.).
    pub record_id: String,

    /// schema_version of the source BrainPrint capsule.
    pub source_capsule_schema: u16,

    /// Timestamp in ms since Unix epoch, copied from BrainPrint header.
    pub timestamp_ms: u64,

    /// Plane classification derived from plane_flags and OS context.
    pub subject_plane: SubjectPlane,

    /// Active neurorights/OS mode at snapshot time.
    pub sovereignty_mode: SovereigntyMode,

    /// Optional cohort tag used by the data lake (study ID, deployment cohort).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_cohort: Option<String>,

    /// Optional hex fingerprint of the active neurorights/evolution profile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_hex: Option<String>,

    /// Normalized vitality index (0–1).
    pub lifeforce_index: f32,

    /// Normalized blood biophysics (0–1).
    pub blood_level: f32,

    /// Normalized oxygen biophysics (0–1).
    pub oxygen_level: f32,

    /// Normalized cognitive clarity index (0–1).
    pub clarity_index: f32,

    /// Coarse eco-energy band: 0 = low, 1 = medium, 2 = high.
    pub eco_band: u8,

    /// Estimated energy usage over the aggregation window in nanojoules.
    pub eco_energy_nj: f64,

    /// Standardized risk-of-harm / cognitive-load index.
    pub roh_index: f64,

    /// Change in roh_index over the last ~24 hours; positive = increasing risk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roh_trend_24h: Option<f64>,

    /// Change in roh_index over the last ~7 days.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roh_trend_7d: Option<f64>,

    /// Composite harm proximity metric in [0,1], combining pain envelope and biophysics.
    pub risk_of_harm: f32,

    /// Normalized fatigue estimate (0–1), derived from biophysical state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fatigue_index: Option<f32>,

    /// Optional normalized index of current assistive work being done by AI modules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assistive_load_index: Option<f32>,

    /// Snapshot of neurorights/evolution policy state relevant to this record.
    pub policy_flags: PolicyFlags,

    /// Sovereign, host-defined extra metrics, each namespaced and typed.
    #[serde(default)]
    pub namespaced_metrics: Vec<NamespacedMetric>,

    /// Optional freeform labels for grouping; must not contain PII.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,
}

impl BrainPrintResearchViewV1 {
    /// Fixed schema version for this struct.
    pub const SCHEMA_VERSION: u16 = 1;

    /// Helper constructor that pre-fills schema_version and leaves other
    /// fields to be populated by the caller or by a brainPrint! unleash implementation.
    pub fn new_empty(record_id: impl Into<String>) -> Self {
        BrainPrintResearchViewV1 {
            schema_version: Self::SCHEMA_VERSION,
            record_id: record_id.into(),
            source_capsule_schema: 1, // caller SHOULD override from BrainPrint.header.schema_version
            timestamp_ms: 0,
            subject_plane: SubjectPlane::SoftwareOnly,
            sovereignty_mode: SovereigntyMode::COPILOT,
            host_cohort: None,
            profile_hex: None,
            lifeforce_index: 0.0,
            blood_level: 0.0,
            oxygen_level: 0.0,
            clarity_index: 0.0,
            eco_band: 0,
            eco_energy_nj: 0.0,
            roh_index: 0.0,
            roh_trend_24h: None,
            roh_trend_7d: None,
            risk_of_harm: 0.0,
            fatigue_index: None,
            assistive_load_index: None,
            policy_flags: PolicyFlags {
                mental_privacy_strict: true,
                mental_integrity_strict: true,
                cognitive_liberty_active: true,
                evolve_required_for_arch_change: true,
                max_external_auto_changes: 0,
                auto_changes_used: 0,
                pain_envelope_exceeded: false,
                export_allowed: false,
            },
            namespaced_metrics: Vec::new(),
            labels: HashMap::new(),
        }
    }
}
