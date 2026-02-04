use crate::hashcheck::{compute_chained_hash, hash_canonical};
use crate::schema::{AuditEntry, EventType};
use chrono::Utc;
use ed25519_dalek::{Keypair, Signer};
use rand_core::OsRng;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use uuid::Uuid;

/// In-memory representation of the donutloop ledger.
#[derive(Clone, Debug)]
pub struct DonutLoop {
    pub entries: Vec<AuditEntry>,
}

impl DonutLoop {
    /// Load from an .aln file (one JSON AuditEntry per line).
    pub fn load(path: &str) -> Result<Self, String> {
        if !std::path::Path::new(path).exists() {
            return Ok(Self { entries: Vec::new() });
        }
        let file = fs::File::open(path).map_err(|e| format!("open donutloop: {e}"))?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();
        for line in reader.lines() {
            let line = line.map_err(|e| format!("read donutloop line: {e}"))?;
            if line.trim().is_empty() {
                continue;
            }
            let entry: AuditEntry =
                serde_json::from_str(&line).map_err(|e| format!("parse AuditEntry: {e}"))?;
            entries.push(entry);
        }
        Ok(Self { entries })
    }

    /// Append a new entry, computing prev_hash and signature.
    pub fn append_signed(
        &mut self,
        path: &str,
        event_type: EventType,
        payload: crate::schema::AuditEventPayload,
        signer_did: &str,
        keypair: &Keypair,
        trace_id: Option<String>,
    ) -> Result<AuditEntry, String> {
        let prev_hash = self
            .entries
            .last()
            .map(|e| e.event_hash.clone())
            .unwrap_or_else(|| "".into());

        let event_id = Uuid::now_v7().to_string();
        let timestamp = Utc::now().to_rfc3339();
        let trace_id = trace_id.unwrap_or_else(|| event_id.clone());

        let mut entry = AuditEntry {
            event_id,
            timestamp,
            trace_id,
            event_type,
            payload,
            event_hash: String::new(),
            prev_hash: prev_hash.clone(),
            signature: String::new(),
            signer_did: signer_did.to_string(),
        };

        // Hash over canonical JSON with prev_hash already set.
        let event_hash = compute_chained_hash(&entry, &prev_hash)?;
        entry.event_hash = event_hash.clone();

        // Sign the event_hash.
        let to_sign = event_hash.as_bytes();
        let sig = keypair.sign(to_sign);
        entry.signature = format!("0x{}", hex::encode(sig.to_bytes()));

        // Append to file.
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("open donutloop for append: {e}"))?;
        let line = serde_json::to_string(&entry)
            .map_err(|e| format!("serialize AuditEntry: {e}"))?;
        file.write_all(line.as_bytes())
            .map_err(|e| format!("write donutloop: {e}"))?;
        file.write_all(b"\n")
            .map_err(|e| format!("write newline: {e}"))?;

        self.entries.push(entry.clone());
        Ok(entry)
    }

    /// Verify internal hash chain integrity.
    pub fn verify_chain(&self) -> Result<(), String> {
        let mut prev = "".to_string();
        for entry in &self.entries {
            let expected = compute_chained_hash(entry, &prev)?;
            if entry.event_hash != expected {
                return Err(format!(
                    "hash chain broken at event {} (expected {}, got {})",
                    entry.event_id, expected, entry.event_hash
                ));
            }
            prev = entry.event_hash.clone();
        }
        Ok(())
    }
}

/// Helper to generate a fresh Ed25519 keypair (for testing / bootstrap only).
pub fn generate_keypair() -> Keypair {
    Keypair::generate(&mut OsRng)
}
