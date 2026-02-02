use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonutloopEntry {
    pub entry_id: String,
    pub subject_id: String,
    pub proposal_id: String,
    pub change_type: String,
    pub tsafe_mode: String,
    pub roh_before: f32,
    pub roh_after: f32,
    pub knowledge_factor: f32,
    pub cybostate_factor: f32,
    pub policy_refs: String,
    pub hexstamp: String,
    pub timestamp_utc: String,
    pub prev_hexstamp: String,
}
