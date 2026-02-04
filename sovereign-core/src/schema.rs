use serde::{Deserialize, Serialize};

/// .rohmodel.aln – viability kernel, RoH invariants, modes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohModePolytope {
    pub mode_name: String,          // e.g., "Baseline", "Training"
    pub description: String,
    /// Matrix A (rows of length 7 for 7D biophysical microspace).
    pub A: Vec<[f32; 7]>,
    /// Vector b, same length as A.
    pub b: Vec<f32>,
    /// Hard ceiling on modeled Risk-of-Harm for this mode.
    pub roh_ceiling: f32,           // must be <= 0.30 at runtime
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohModel {
    pub version: String,
    pub subject_id: String,         // DID, e.g. bostrom18…
    pub modes: Vec<RohModePolytope>,
}

/// .stake.aln – roles, scopes, multisig topology.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RoleKind {
    Host,
    OrganicCpu,
    ResearchAgent,
    OffDeviceSwarm,
    Auditor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleEntry {
    pub role: RoleKind,
    pub did: String,                // DID or address string
    pub scopes: Vec<String>,        // e.g., ["EVOLVE:*", "SMART:assist"]
}

/// Minimal EVOLVE / SMART semantics on chain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MultisigRule {
    /// e.g., "arch.change", "roh.relax", "ota.update".
    pub operation: String,
    /// Required roles for approval, e.g., ["Host", "OrganicCpu"].
    pub required_roles: Vec<RoleKind>,
    /// Minimum number of distinct signers from `required_roles`.
    pub threshold: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakeConfig {
    pub subject_id: String,
    pub version: String,
    pub roles: Vec<RoleEntry>,
    pub multisig_rules: Vec<MultisigRule>,
}

/// .neurorights.json – neurorights + SMARTEVOLVE semantics.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmartScopeGuard {
    /// Scope name, e.g. "SMART:assist.language", "SMART:mobility".
    pub scope: String,
    /// Can external actors ever fully freeze this scope? false for Host.
    pub allow_freeze: bool,
    /// Maximum allowed tightening factor per update (0–1, monotone only).
    pub max_tightening_per_update: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsDocument {
    pub version: String,
    pub subject_id: String,
    /// Ban discrimination by augmentation status.
    pub ban_augmentation_discrimination: bool,
    /// Ban punitive use of telemetry for employment/housing/credit.
    pub ban_punitive_telemetry_use: bool,
    /// "No new ceilings" flag: evolution cannot regress host vs non‑augmented.
    pub no_new_ceilings: bool,
    /// SMARTEVOLVE semantics for scopes.
    pub smart_scope_guards: Vec<SmartScopeGuard>,
}

/// .evolve.jsonl – one UpdateProposal per line.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UpdateKind {
    ParamNudge,
    ThresholdShift,
    RoutingChange,
    ArchChange,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateEffect {
    pub knowledge_factor_delta: f32, // + is more capability
    pub risk_of_harm_delta: f32,     // must not push RoH > 0.3
    pub cybostate_factor_delta: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateProposal {
    pub id: String,                  // UUIDv7 string
    pub module: String,
    pub kind: UpdateKind,
    pub scopes: Vec<String>,         // affected domains
    pub description: String,
    pub predicted_effect: UpdateEffect,
    /// If true, an EVOLVE token signed by Host is required.
    pub requires_evolve: bool,
}

/// donutloop.aln – append-only sovereign ledger.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventType {
    EvolutionProposalSubmitted,
    EvolutionDecisionApplied,
    NeurorightCheckFailed,
    NeurorightViolationLogged,
    ArchitectureChangeApproved,
    ArchitectureChangeRejected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEventPayload {
    pub roh_before: Option<f32>,
    pub roh_after: Option<f32>,
    pub knowledge_factor_before: Option<f32>,
    pub knowledge_factor_after: Option<f32>,
    pub cybostate_factor_before: Option<f32>,
    pub cybostate_factor_after: Option<f32>,
    pub reason: Option<String>,
    pub reference_id: Option<String>, // e.g. UpdateProposal.id
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEntry {
    pub event_id: String,            // UUIDv7
    pub timestamp: String,           // ISO-8601
    pub trace_id: String,            // link related events
    pub event_type: EventType,
    pub payload: AuditEventPayload,
    pub event_hash: String,          // hex
    pub prev_hash: String,           // hex ("" only for genesis)
    pub signature: String,           // Ed25519 hex signature
    pub signer_did: String,          // Host or OrganicCPU DID
}
