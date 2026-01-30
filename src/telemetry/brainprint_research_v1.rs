use serde::{Deserialize, Serialize};

/// Policy flags snapshot, matches `policy_flags` in brainprint_research_v1.schema.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyFlags {
    /// True if current policy forbids export beyond local vault unless explicitly consented.
    pub mental_privacy_strict: bool,

    /// True if irreversible ops are forbidden and rollback path is required.
    pub mental_integrity_strict: bool,

    /// True if self-chosen augmentation is allowed and explanations are required for external changes.
    pub cognitive_liberty_active: bool,

    /// True if evolution policy requires EVOLVE tokens for architectural changes.
    pub evolve_required_for_arch_change: bool,

    /// From CognitiveLibertyPolicy.max_external_auto_changes.
    pub max_external_auto_changes: i32,

    /// Number of auto-changes consumed in the current session/window.
    pub auto_changes_used: i32,

    /// True if muscular, cognitive, or emotional pain channel exceeded rollback threshold.
    pub pain_envelope_exceeded: bool,

    /// True if this research view passed sovereignty export checks.
    pub export_allowed: bool,
}

/// High-level semantic type for namespaced metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticType {
    /// Scalar in [0,1].
    Scalar0_1,
    /// Scalar, no fixed bounds.
    ScalarUnbounded,
    /// Non-negative count.
    Count,
    /// Duration in milliseconds.
    DurationMs,
    /// Rate per second.
    RatePerS,
    /// Integer index into a categorical domain.
    CategoricalIndex,
}

/// A single sovereign, namespaced extra metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespacedMetric {
    /// Logical namespace (e.g., "bci", "sw", "nano", "motor", "lang").
    pub namespace: String,

    /// Metric name within namespace (e.g., "signal_quality", "mem_usage").
    pub name: String,

    /// High-level semantic type, for data-lake tooling.
    pub semantic_type: SemanticType,

    /// Numeric value, interpretation depends on semantic_type.
    pub value: f64,

    /// Optional unit label (e.g., "ms", "Hz", "nj"); empty/None for normalized metrics.
    #[serde(default)]
    pub unit: Option<String>,
}

/// Sovereignty/OS mode at snapshot time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SovereigntyMode {
    Conservative,
    Copilot,
    Autoevolve,
}

/// Decoded plane label derived from plane_flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectPlane {
    Bioscale,
    BciHci,
    Cybernetic,
    SoftwareOnly,
    AugmentedCitizen,
    Sandbox,
}

/// BrainPrintResearchView v1, NDJSON-compatible, sovereignty-first research telemetry view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainPrintResearchViewV1 {
    /// Research view schema version. For this struct: 1.
    pub schema_version: u16,

    /// Opaque, per-record identifier (e.g., UUIDv4 or host-local monotonic ID). No raw host_id.
    pub record_id: String,

    /// schema_version of the source BrainPrint capsule this view was derived from.
    pub source_capsule_schema: u16,

    /// Unix epoch timestamp in milliseconds, copied from the source BrainPrint header.
    pub timestamp_ms: i64,

    /// Decoded plane label derived from plane_flags (no raw flags).
    pub subject_plane: SubjectPlane,

    /// Active neurorights/OS mode at snapshot time.
    pub sovereignty_mode: SovereigntyMode,

    /// Optional cohort tag used by the data lake (e.g., study ID, deployment cohort). No direct identity.
    #[serde(default)]
    pub host_cohort: Option<String>,

    /// Optional hex fingerprint of the active evolution/neurorights profile.
    #[serde(default)]
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
    pub eco_band: i32,

    /// Estimated energy usage over the aggregation window in nanojoules.
    pub eco_energy_nj: f64,

    /// Standardized risk-of-harm / cognitive-load index (e.g., capped wave/brain ratio).
    pub roh_index: f64,

    /// Change in roh_index over the last ~24h; positive = increasing risk.
    #[serde(default)]
    pub roh_trend_24h: Option<f64>,

    /// Change in roh_index over the last ~7 days.
    #[serde(default)]
    pub roh_trend_7d: Option<f64>,

    /// Composite harm proximity metric in [0,1].
    pub risk_of_harm: f32,

    /// Normalized fatigue estimate (0–1).
    #[serde(default)]
    pub fatigue_index: Option<f32>,

    /// Optional normalized index of current assistive work being done by AI modules.
    #[serde(default)]
    pub assistive_load_index: Option<f32>,

    /// Flattened snapshot of neurorights/evolution policy state relevant to telemetry export.
    pub policy_flags: PolicyFlags,

    /// Sovereign, host-defined extra metrics with explicit namespaces.
    pub namespaced_metrics: Vec<NamespacedMetric>,

    /// Optional freeform labels used by data-lake or dashboards for grouping. No PII.
    #[serde(default)]
    pub labels: std::collections::HashMap<String, String>,
}
