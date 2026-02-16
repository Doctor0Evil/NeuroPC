use serde::{Deserialize, Serialize};
use sovereigntycore::evaluate_update; // your existing evaluation entry point
use organiccpualn::evolvestream::EvolutionProposalRecord;
use organiccpualn::rohmodel::RohModel;

/// Minimal view of the TFC Run Task payload we care about.
#[derive(Debug, Deserialize)]
pub struct TfcRunTaskPayload {
    pub run_id: String,
    pub workspace_name: String,
    pub organization_name: String,
    pub plan_json_url: String, // URL to fetch redacted plan JSON.
}

/// Neuromorphic Terraform proposal, aligned with your schema.
#[derive(Debug, Serialize, Deserialize)]
pub struct NeuroTerraformProposal {
    pub proposal_id: String,
    pub subject_id: String,
    pub tfc_org: String,
    pub workspace_name: String,
    pub plan_summary: serde_json::Value,
    pub roh_before: f32,
    pub roh_after_estimate: f32,
    pub domaintags: Vec<String>,
}

/// Adapter result to send back to TFC and AI-Chat.
#[derive(Debug, Serialize, Deserialize)]
pub struct TerraformerDecision {
    pub proposal_id: String,
    pub decision: String,          // Allowed/Rejected/Deferred
    pub message: String,           // Human-readable, AI-Chat friendly
    pub roh_before: f32,
    pub roh_after: f32,
}

pub struct NeuromorphicTerraformer {
    pub roh_model: RohModel,
    pub subject_id: String,
}

impl NeuromorphicTerraformer {
    pub fn new(roh_model: RohModel, subject_id: String) -> Self {
        Self { roh_model, subject_id }
    }

    /// Core entry: given a parsed TFC payload + plan summary,
    /// build a proposal, run sovereigntycore, and produce a decision.
    pub fn handle_run(
        &self,
        tfc: TfcRunTaskPayload,
        plan_summary: serde_json::Value,
    ) -> TerraformerDecision {
        let proposal_id = format!("tf-{}", tfc.run_id);

        // Example: make a crude RoH estimate for infra changes.
        let roh_before = 0.18_f32;
        let roh_after_estimate = 0.19_f32; // refined later from real metrics

        let neuro = NeuroTerraformProposal {
            proposal_id: proposal_id.clone(),
            subject_id: self.subject_id.clone(),
            tfc_org: tfc.organization_name.clone(),
            workspace_name: tfc.workspace_name.clone(),
            plan_summary,
            roh_before,
            roh_after_estimate,
            domaintags: vec!["infra".to_string(), "terraform".to_string()],
        };

        let evo_record = EvolutionProposalRecord {
            proposal_id: neuro.proposal_id.clone(),
            subject_id: neuro.subject_id.clone(),
            kind: "TerraformPlan".to_string(),
            module: "neuromorphic_terraformer".to_string(),
            updatekind: "InfraChange".to_string(),
            effectbounds: organiccpualn::evolvestream::EffectBounds {
                l2deltanorm: 0.05,
                irreversible: false,
            },
            rohbefore: neuro.roh_before,
            rohafter: neuro.roh_after_estimate,
            tsafemode: "SafeFilterPlusEvolution".to_string(),
            domaintags: neuro.domaintags.clone(),
            decision: "Deferred".to_string(),
            hexstamp: "0xNP09".to_string(),
            timestamputc: chrono::Utc::now().to_rfc3339(),
        };

        let result = evaluate_update(evo_record);

        TerraformerDecision {
            proposal_id,
            decision: result.decision.clone(),
            message: result.reason.clone(),
            roh_before,
            roh_after: roh_after_estimate,
        }
    }
}
