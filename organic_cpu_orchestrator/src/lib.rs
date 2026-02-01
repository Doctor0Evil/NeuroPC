//! OrganicCPU orchestrator: safe outer shell.
//!
//! Exposes a minimal CyberNano boot API that is:
//! - Sovereignty-first (neurorights + EVOLVE gates),
//! - Biophysical-state aware (BioState summary only),
//! - Explicitly bounded in what CyberNano may request.

pub mod types;
pub mod cybernano_boot;

pub use types::{
    OrchestratorBioSnapshot,
    CyberNanoMode,
    CyberNanoBootRequest,
    CyberNanoBootDecision,
    CyberNanoBootError,
};

pub use cybernano_boot::cybernano_boot;
