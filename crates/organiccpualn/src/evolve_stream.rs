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
    pub proposalid: String,
    pub subjectid: String,
    pub kind: String,
    pub module: String,
    pub updatekind: String,
    pub effectbounds: EffectBounds,
    pub rohbefore: f32,
    pub rohafter: f32,
    pub tsafemode: String,
    pub domaintags: Vec<String>,
    pub decision: String,
    pub hexstamp: String,
    pub timestamputc: String,

    /// NEW: reference to the prompt envelope that triggered this proposal.
    pub prompt_envelope_id: Option<String>,

    /// NEW: neurorights profile and token that governed this proposal.
    pub neurorights_profile_id: Option<String>,
    pub token_id: Option<String>,
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
