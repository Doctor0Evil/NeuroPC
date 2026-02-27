#[derive(Clone, Debug)]
pub struct AnswerLogState {
    /// Filesystem path for .answer.ndjson
    pub log_path: String,
    /// Last hexstamp written to this log (None → GENESIS).
    pub prev_hexstamp: Option<String>,
}
