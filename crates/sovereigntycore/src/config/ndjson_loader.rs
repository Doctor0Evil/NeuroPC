use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_ndjson<T: DeserializeOwned, P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut out = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let value: T = serde_json::from_str(&line)?;
        out.push(value);
    }
    Ok(out)
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EvolveRecord {
    pub proposal_id: String,
    pub kind: String,
    pub inputs: serde_json::Value,
    pub effect_bounds: serde_json::Value,
    pub roh_before: f32,
    pub roh_after: f32,
    pub decision: String,
    pub hexstamp: String,
    pub timestamp: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BchainProof {
    pub id: String,
    pub artifact_hash: String,
    pub ledger_pointer: String,
    pub consensus: serde_json::Value,
}
