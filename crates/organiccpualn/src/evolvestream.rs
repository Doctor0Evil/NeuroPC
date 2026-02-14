use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateEffectBounds {
    pub l2_delta_norm: f32,
    pub irreversible: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolutionProposal {
    pub proposal_id: String,
    pub subject_id: String,
    pub species_id: String,
    pub kind: String,
    pub scope_id: String,
    pub module: String,
    pub effect_bounds: UpdateEffectBounds,
    pub roh_before: f32,
    pub roh_after: f32,
    pub decay_before: f32,
    pub decay_after: f32,
    pub computebioload_before: f32,
    pub computebioload_after: f32,
    pub decision: String,
    pub justice_flags: Vec<String>,
    pub tsafe_mode: String,
    pub domain_tags: Vec<String>,
    pub signer_dids: Vec<String>,
    pub hexstamp: String,
    pub timestamp_utc: String,
}
