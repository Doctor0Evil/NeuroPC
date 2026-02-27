use serde::{Deserialize, Serialize};

use crate::answerquality::Cybostate;
use crate::organiccpu_bridge::BioStateSnapshot;

/// Single answer‑quality record, emitted as NDJSON line (.answer.ndjson).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnswerNdjsonRecord {
    pub answer_id: String,
    pub subject_id: String,
    pub kernel_id: String,
    pub route: String,
    pub knowledge_factor: f32,
    pub roh: f32,
    pub cybostate: Cybostate,
    pub bio: BioStateSnapshot,
    pub actuation_forbidden: bool,
    pub non_commercial: bool,
    pub roh_domain: Option<String>,
    pub hexstamp: String,
    pub prev_hexstamp: String,
    pub timestamp_utc: u64,
    pub artifact_kind: String,
    pub contract_type: String,
}

/// Internal writer abstraction used by TextNeuroPrintBackend.
pub struct AnswerLedgerWriter {
    path: std::path::PathBuf,
    last_hex: std::cell::RefCell<String>,
}

impl AnswerLedgerWriter {
    pub fn new<P: Into<std::path::PathBuf>>(path: P) -> Self {
        Self {
            path: path.into(),
            last_hex: std::cell::RefCell::new(String::from("0xGENESIS")),
        }
    }

    pub fn last_hexstamp(&self) -> Result<String, String> {
        Ok(self.last_hex.borrow().clone())
    }

    pub fn compute_hexstamp(
        &self,
        subject_id: &str,
        kernel_id: &str,
        ts: u64,
        kf: f32,
        roh: f32,
    ) -> Result<String, String> {
        // Simple non‑cryptographic hexstamp to avoid blacklisted hashes.
        let seed = format!("{}:{}:{}:{:.4}:{:.4}", subject_id, kernel_id, ts, kf, roh);
        let mut acc: u64 = 0xcbf29ce484222325;
        for b in seed.as_bytes() {
            acc = acc ^ (*b as u64);
            acc = acc.wrapping_mul(0x100000001b3);
        }
        Ok(format!("0xNPANS{:016x}", acc))
    }

    pub fn append_entry(&self, entry: AnswerLedgerEntry) -> Result<(), String> {
        let record = AnswerNdjsonRecord {
            answer_id: entry.answer_id,
            subject_id: entry.subject_id,
            kernel_id: entry.kernel_id,
            route: format!("{:?}", entry.route),
            knowledge_factor: entry.knowledge_factor,
            roh: entry.roh,
            cybostate: entry.cybostate,
            bio: entry.bio,
            actuation_forbidden: true,
            non_commercial: true,
            roh_domain: None,
            hexstamp: entry.hexstamp.clone(),
            prev_hexstamp: entry.prev_hexstamp.clone(),
            timestamp_utc: entry.timestamp_utc,
            artifact_kind: entry.artifact_kind,
            contract_type: entry.contract_type,
        };

        let line = serde_json::to_string(&record).map_err(|e| e.to_string())?;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| e.to_string())?;
        use std::io::Write;
        writeln!(file, "{}", line).map_err(|e| e.to_string())?;

        *self.last_hex.borrow_mut() = record.hexstamp;
        Ok(())
    }
}

/// Internal helper used by backend.
#[derive(Clone, Debug)]
pub struct AnswerLedgerEntry {
    pub answer_id: String,
    pub subject_id: String,
    pub kernel_id: String,
    pub route: crate::answerquality::AnswerRoute,
    pub knowledge_factor: f32,
    pub roh: f32,
    pub cybostate: Cybostate,
    pub bio: BioStateSnapshot,
    pub timestamp_utc: u64,
    pub prev_hexstamp: String,
    pub hexstamp: String,
    pub artifact_kind: String,
    pub contract_type: String,
}
