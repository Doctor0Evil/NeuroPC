use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuiMeta {
    pub version: String,
    pub kind: String,
    pub description: String,
    pub subjectid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuiState {
    pub id: String,
    pub view_id: String,
    pub prompt: String,
    pub on_event: String,
    pub transition_to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuiGuards {
    pub tsafe_required_mode: String,
    pub max_roh_delta: f32,
    pub allow_evolve_paths: bool,
    pub hardware_actuation: String,
    pub neurorights_policy_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuiInvariants {
    pub roh_ceiling: f32,
    pub noncommercialneuraldata: bool,
    pub forbiddecisionuse_domains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuiFlow {
    pub meta: NuiMeta,
    pub states: Vec<NuiState>,
    pub guards: NuiGuards,
    pub invariants: NuiInvariants,
}

impl NuiFlow {
    pub fn validate(&self) -> anyhow::Result<()> {
        use anyhow::bail;

        if (self.invariants.roh_ceiling - 0.30).abs() > f32::EPSILON {
            bail!("NuiFlow invariant failed: roh_ceiling must be 0.30");
        }
        if !self.invariants.noncommercialneuraldata {
            bail!("NuiFlow invariant failed: noncommercialneuraldata must be true");
        }
        if self
            .invariants
            .forbiddecisionuse_domains
            .iter()
            .any(|d| d.is_empty())
        {
            bail!("NuiFlow invariant failed: empty domain in forbiddecisionuse_domains");
        }
        if self.guards.hardware_actuation.to_lowercase() != "forbidden" {
            bail!("NuiFlow guard failed: hardware_actuation must be 'forbidden'");
        }
        if self.guards.max_roh_delta > 0.0 {
            bail!("NuiFlow guard failed: max_roh_delta must be <= 0.0");
        }
        Ok(())
    }
}
