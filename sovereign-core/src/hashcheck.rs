use crate::schema::{AuditEntry, NeurorightsDocument, RohModel};
use serde::Serialize;
use sha2::{Digest, Sha256};

/// Canonical JSON: stable field order via Serde + no whitespace.
/// (You can later swap this for a strict RFC 8785 implementation.)
pub fn canonical_json<T: Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_string(value).map_err(|e| format!("canonical_json error: {e}"))
}

/// Compute SHA-256 hash as hex over canonical JSON.
pub fn hash_canonical<T: Serialize>(value: &T) -> Result<String, String> {
    let json = canonical_json(value)?;
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    Ok(format!("0x{}", hex::encode(hasher.finalize())))
}

/// Verify that all RoH ceilings in .rohmodel.aln respect the hard 0.3 invariant.
pub fn verify_roh_invariants(model: &RohModel) -> Result<(), String> {
    for mode in &model.modes {
        if mode.roh_ceiling > 0.30 {
            return Err(format!(
                "RoH ceiling {} exceeds 0.30 in mode {}",
                mode.roh_ceiling, mode.mode_name
            ));
        }
        if mode.A.len() != mode.b.len() {
            return Err(format!(
                "Ax≤b dimension mismatch in mode {}: {} rows, {} b entries",
                mode.mode_name,
                mode.A.len(),
                mode.b.len()
            ));
        }
    }
    Ok(())
}

/// Verify neurorights invariants that must hold at boot.
pub fn verify_neurorights(doc: &NeurorightsDocument) -> Result<(), String> {
    if !doc.ban_augmentation_discrimination {
        return Err("neurorights must ban augmentation-based discrimination".into());
    }
    if !doc.ban_punitive_telemetry_use {
        return Err("neurorights must ban punitive telemetry use".into());
    }
    if !doc.no_new_ceilings {
        return Err("\"no_new_ceilings\" must be true for sovereign host".into());
    }
    for guard in &doc.smart_scope_guards {
        if !guard.allow_freeze && guard.max_tightening_per_update > 1.0 {
            return Err(format!(
                "SMART scope {} has invalid tightening factor >1.0",
                guard.scope
            ));
        }
    }
    Ok(())
}

/// Compute chained hash for a new AuditEntry given previous hash.
pub fn compute_chained_hash(entry: &AuditEntry, prev_hash: &str) -> Result<String, String> {
    let mut clone = entry.clone();
    clone.prev_hash = prev_hash.to_string();
    hash_canonical(&clone)
}
