//! Append-only logging for ChatAnswerEnvelope<String> into .answer.ndjson.
//!
//! This module:
//! - Defines the on-disk shape of answer log rows.
//! - Uses hex-linked stamps for tamper-evidence (prev_hexstamp → hexstamp).
//! - Is explicitly non-financial and non-commercial.

use std::fs::{OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::answerquality::{ChatAnswerEnvelope, AnswerRoute, Cybostate};
use crate::hashing::hexstamp_for_answer; // small helper reusing donutloop hash core.
use crate::state::AnswerLogState;        // in-memory prev_hexstamp tracker.

/// On-disk representation of one answer row in .answer.ndjson.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerLogRow {
    pub body: String,
    pub route: AnswerRoute,
    pub domain: String,
    pub knowledge_factor: f32,
    pub roh: f32,
    pub cybostate: Cybostate,
    pub rest_advisory: bool,
    pub session_id: Option<String>,

    pub timestamp_utc: DateTime<Utc>,

    /// Current hexstamp (hash over body + metadata).
    pub hexstamp: String,
    /// Previous hexstamp in this stream (or "GENESIS").
    pub prev_hexstamp: String,
}

/// Logging error type.
#[derive(Debug)]
pub enum AnswerLogError {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl From<std::io::Error> for AnswerLogError {
    fn from(e: std::io::Error) -> Self {
        AnswerLogError::Io(e)
    }
}

impl From<serde_json::Error> for AnswerLogError {
    fn from(e: serde_json::Error) -> Self {
        AnswerLogError::Serde(e)
    }
}

/// Append a ChatAnswerEnvelope<String> into the configured .answer.ndjson file.
///
/// - Non-blocking on failure: caller decides how to handle errors.
/// - Does NOT perform financial or commercial semantics; this is a pure
///   audit / neurorights / RoH trace.
pub fn log_answer_envelope(
    state: &mut AnswerLogState,
    envelope: &ChatAnswerEnvelope<String>,
) -> Result<(), AnswerLogError> {
    let prev_hex = state.prev_hexstamp.clone().unwrap_or_else(|| "GENESIS".to_string());

    let row = AnswerLogRow {
        body: envelope.body.clone(),
        route: envelope.quality.route,
        domain: envelope.quality.domain.clone(),
        knowledge_factor: envelope.quality.knowledge_factor.0,
        roh: envelope.quality.risk.0,
        cybostate: envelope.quality.cybostate,
        rest_advisory: envelope.quality.rest_advisory,
        session_id: envelope.session_id.clone(),
        timestamp_utc: envelope.quality.timestamp_utc,
        hexstamp: hexstamp_for_answer(envelope, &prev_hex),
        prev_hexstamp: prev_hex.clone(),
    };

    let path = Path::new(&state.log_path); // e.g., "logs/answers-2026v1.answer.ndjson"
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    let mut writer = BufWriter::new(file);

    let line = serde_json::to_string(&row)?;
    writer.write_all(line.as_bytes())?;
    writer.write_all(b"\n")?;
    writer.flush()?;

    state.prev_hexstamp = Some(row.hexstamp);
    Ok(())
}
