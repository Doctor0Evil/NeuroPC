use serde::{Serialize, Deserialize};
use organichain::tx::OrganichainTx;

/// Minimal view of current BI + host state at the OrganicCPU boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiSessionSnapshot {
    /// Host DID, e.g. "didaln:bostrom18sd2u..."
    pub host_did: String,
    /// Commitment to brain-identity state (hex), from BI enclave.
    pub bi_commitment: String,
    /// Monotone epoch counter for this BI stream.
    pub bi_epoch: u32,
    /// True iff this session is currently live (not replay, not stale).
    pub live_session: bool,
    /// Local view of RoH scalar 0.0..0.3, recomputed by Sovereign Ledger.
    pub roh_score: f32,
    /// True iff RoH monotone + ceiling constraints already hold.
    pub roh_safe: bool,
    /// True iff all relevant BiophysicalEnvelopeSpec checks passed.
    pub envelopes_safe: bool,
    /// True iff neurorights + consent checks passed for this tx scope.
    pub neurorights_ok: bool,
}

/// Verdict from the biophysical-consensus validator for a candidate tx.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiophysicalVerdict {
    pub allowed: bool,
    pub reason: Option<String>,
}

/// Abstract signer that lives in the BI enclave / keystore.
/// Implementations never expose private keys.
pub trait BiSigner {
    /// Returns host DID bound to this signer.
    fn host_did(&self) -> &str;

    /// Sign an OrganichainTx digest, returning hex-encoded signature.
    fn sign_tx(&self, tx: &OrganichainTx) -> Result<String, String>;
}
