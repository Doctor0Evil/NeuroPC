use alloc::string::String;
use alloc::vec::Vec;

/// Static configuration for the adapter.
/// In practice you can load this from ALN/JSON, but we keep it as Rust
/// so Jupyter stays enforcement-centric.
#[derive(Clone, Debug)]
pub struct AssistantAdapterConfig {
    /// Path to neurorights policy JSON, e.g.
    /// "policies/bostrom-neurorights-v1.neurorights.json".
    pub neurorights_policy_path: String,
    /// Logical tool ID for this adapter ("jupyter-assistant").
    pub tool_id: String,
    /// Domains this adapter is allowed to serve.
    pub allowed_domains: Vec<String>,
    /// Path to evolution proposals JSONL file.
    /// e.g. "qpudatashards/particles/evolution-proposals.evolve.jsonl".
    pub evolve_log_path: String,
    /// Path to donutloop ledger ALN/JSONL file.
    /// e.g. "logs/donutloopledger.aln".
    pub donutloop_path: String,
}
