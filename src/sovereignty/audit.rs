#![forbid(unsafe_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use crate::sovereignty::invariants::InvariantStatus;

/// Different event types; extend as needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventKind {
    OtaRequest,
    // Add: PolicyChange, ConsentUpdate, etc.
}

/// One event in the log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp: DateTime<Utc>,
    pub caller_module: String,
    pub caller_instance: Option<String>,
    pub kind: AuditEventKind,
    /// Serialized OtaAction or other payload (metadata only).
    pub action: serde_json::Value,
    /// Policy decision metadata.
    pub policy_decision: serde_json::Value,
    /// Invariant statuses at the time of the event.
    pub invariants: Vec<InvariantStatus>,
}

/// Stored record with hash chaining.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuditRecord {
    pub event: AuditEvent,
    pub prev_hash: String,
    pub hash: String,
}

/// Simple file-based append-only logger.
pub struct AuditLogger {
    path: PathBuf,
    last_hash: String,
}

impl AuditLogger {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            last_hash: String::from("GENESIS"),
        }
    }

    /// Append an event, compute hash chain, and write as one JSON line.
    pub fn append(&mut self, event: AuditEvent) -> std::io::Result<()> {
        let record = self.make_record(event);
        let json = serde_json::to_string(&record).expect("serialize AuditRecord");

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(json.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }

    fn make_record(&mut self, event: AuditEvent) -> AuditRecord {
        let mut hasher = Sha256::new();
        let payload = serde_json::to_vec(&event).expect("serialize AuditEvent");
        hasher.update(&payload);
        hasher.update(self.last_hash.as_bytes());
        let hash_bytes = hasher.finalize();
        let hash_hex = hex::encode(hash_bytes);

        let record = AuditRecord {
            event,
            prev_hash: self.last_hash.clone(),
            hash: hash_hex.clone(),
        };

        self.last_hash = hash_hex;
        record
    }
}
