use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DonutloopEntry {
    pub entry_id: String,
    pub proposal_id: String,
    pub policy_id: String,
    pub decision: String,
    pub roh_before: f32,
    pub roh_after: f32,
    pub hexstamp: String,
    pub prev_hash: String,
    pub entry_hash: String,
    pub evolve_stream_pointer: String,
    pub timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DonutloopShard {
    pub meta: crate::aln_meta::AlnMeta, // reuse your existing meta type
    pub entries: Vec<DonutloopEntry>,
}

impl DonutloopShard {
    pub fn append_entry(&mut self, entry: DonutloopEntry) {
        self.entries.push(entry);
    }
}
