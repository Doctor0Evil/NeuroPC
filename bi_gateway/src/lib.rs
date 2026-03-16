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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GatewayError {
    NotLiveSession,
    HostDidMismatch { snapshot: String, signer: String },
    BiEpochRegression { snapshot: u32, min_allowed: u32 },
    RoHNotSafe { roh_score: f32 },
    EnvelopesNotSafe,
    NeurorightsBlocked,
    BiophysicalDenied { reason: Option<String> },
    SignerError { message: String },
}

/// High-level BI gateway: validate snapshot + verdict, bind BI into tx,
/// and sign on behalf of the live brain-identity.
///
/// `min_bi_epoch` is the lowest acceptable epoch (e.g. from last signed tx),
/// preventing replay with stale BI state.
pub fn sign_organichain_tx<S: BiSigner>(
    signer: &S,
    mut tx: OrganichainTx,
    snapshot: &BiSessionSnapshot,
    verdict: &BiophysicalVerdict,
    min_bi_epoch: u32,
) -> Result<OrganichainTx, GatewayError> {
    // 1. Ensure this BI session is live, not replayed or stale.
    if !snapshot.live_session {
        return Err(GatewayError::NotLiveSession);
    }

    // 2. Enforce host DID agreement between snapshot and signer.
    if snapshot.host_did != signer.host_did() {
        return Err(GatewayError::HostDidMismatch {
            snapshot: snapshot.host_did.clone(),
            signer: signer.host_did().to_string(),
        });
    }

    // 3. Enforce monotone BI epoch (prevents credential replay).
    if snapshot.bi_epoch < min_bi_epoch {
        return Err(GatewayError::BiEpochRegression {
            snapshot: snapshot.bi_epoch,
            min_allowed: min_bi_epoch,
        });
    }

    // 4. Enforce RoH ceiling and monotone improvement (already computed).
    if !snapshot.roh_safe || snapshot.roh_score > 0.30 + 1e-6 {
        return Err(GatewayError::RoHNotSafe {
            roh_score: snapshot.roh_score,
        });
    }

    // 5. Enforce envelope safety (BiophysicalEnvelopeSpec, QuantumphysicalReceding, etc.).
    if !snapshot.envelopes_safe {
        return Err(GatewayError::EnvelopesNotSafe);
    }

    // 6. Enforce neurorights + consent boundary.
    if !snapshot.neurorights_ok {
        return Err(GatewayError::NeurorightsBlocked);
    }

    // 7. Enforce full biophysical-consensus verdict.
    if !verdict.allowed {
        return Err(GatewayError::BiophysicalDenied {
            reason: verdict.reason.clone(),
        });
    }

    // 8. Bind BI commitment + epoch into the tx's BI and host state fields.
    tx.bi_binding.bi_commitment = snapshot.bi_commitment.clone();
    tx.bi_binding.bi_epoch = snapshot.bi_epoch;

    // Note: host_state.roh_after should already reflect snapshot.roh_score,
    // but we clamp to be safe.
    tx.host_state.roh_after = snapshot.roh_score.min(0.30);

    // 9. Ask the BI enclave signer to sign this tx.
    let sig = signer
        .sign_tx(&tx)
        .map_err(|e| GatewayError::SignerError { message: e })?;

    // 10. Fill auth fields (host BI sig + OrganicCPU sig may be layered later).
    tx.auth.host_bi_signature = sig;
    // Caller or another module will add OrganicCPU + stake multisig; we leave as-is here.

    Ok(tx)
}
