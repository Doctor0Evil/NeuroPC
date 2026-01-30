use crate::BrainPrint;

/// Logical planes for environment classification.
#[derive(Clone, Copy, Debug)]
pub enum BrainPrintPlane {
    Bioscale,
    Biophysics,
    BciHciEeg,
    Cybernetic,
    SoftwareOnly,
}

/// Levels of export freedom. Sovereignty remains host-local.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrainPrintUnleashLevel {
    /// Minimal: only hashed, coarse metrics.
    TelemetryMinimal,
    /// Adds richer, still non-identity metrics (eco, RoH, risk).
    TelemetryResearch,
    /// Internal only (must NEVER be serialized off-host).
    InnerDiagnostics,
}

/// Shape of an exportable, machine-readable record for researchers.
#[derive(Clone, Debug)]
pub struct BrainPrintResearchView {
    pub schema_version: u16,
    pub plane: BrainPrintPlane,
    pub unleash_level: BrainPrintUnleashLevel,
    pub timestamp_ms: u64,
    // Aggregated, non-financial metrics.
    pub brain: f64,
    pub wave: f64,
    pub nano: f64,
    pub smart: f64,
    pub lifeforce_index: f32,
    pub eco_band: u8,
    // Optional safety/eco meta (no identity).
    pub roh_index: Option<f32>,
    pub eco_energy_nj: Option<f64>,
    pub risk_of_harm: Option<f32>,
    pub knowledge_factor: Option<f32>,
    // Attestation hashes: host-local, non-identifying off-host.
    pub state_hash_hex: String,
    pub profile_hex: Option<String>,
}

/// Trait implemented via the `brainPrint!` macro to control export.
pub trait BrainPrintUnleash {
    /// Which level this crate wants to expose by default.
    fn unleash_level() -> BrainPrintUnleashLevel;

    /// Build a research-safe view – must NEVER include host_id or raw balances.
    fn to_research_view(bp: &BrainPrint) -> BrainPrintResearchView;
}
