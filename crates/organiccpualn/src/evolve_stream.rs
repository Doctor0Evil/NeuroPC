use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectBounds {
    pub l2_delta_norm: f32,
    pub irreversible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolutionProposalRecord {
    pub proposal_id: String,
    pub subject_id: String,
    pub kind: String,
    pub module: String,
    pub update_kind: String,
    pub effect_bounds: EffectBounds,
    pub roh_before: f32,
    pub roh_after: f32,
    pub tsafe_mode: String,
    pub domain_tags: Vec<String>,
    pub decision: String,
    pub hexstamp: String,
    pub timestamp_utc: String,
}

pub trait EvolutionLogReader {
    fn read_all<R: BufRead>(&self, reader: R) -> anyhow::Result<Vec<EvolutionProposalRecord>>;
}

pub trait EvolutionLogWriter {
    fn append<W: Write>(&self, writer: &mut W, rec: &EvolutionProposalRecord) -> anyhow::Result<()>;
}

pub struct JsonlEvolutionLog;

impl EvolutionLogReader for JsonlEvolutionLog {
    fn read_all<R: BufRead>(&self, reader: R) -> anyhow::Result<Vec<EvolutionProposalRecord>> {
        let mut out = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let rec: EvolutionProposalRecord = serde_json::from_str(&line)?;
            out.push(rec);
        }
        Ok(out)
    }
}

impl EvolutionLogWriter for JsonlEvolutionLog {
    fn append<W: Write>(&self, writer: &mut W, rec: &EvolutionProposalRecord) -> anyhow::Result<()> {
        let line = serde_json::to_string(rec)?;
        writeln!(writer, "{line}")?;
        Ok(())
    }
}
