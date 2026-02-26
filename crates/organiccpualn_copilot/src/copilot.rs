#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotInput {
    pub request_id: String,
    pub user_did: String,
    pub ocpuenv_shard_path: String,
    pub source_tool: String, // must be "copilot" or similar
    pub intent: String,      // must be "advisory" / "refactor" / "doc"
    pub roh_target: f32,     // 0.0 .. 0.3
    pub language: String,
    pub context_hash: String,
    pub neuroprofile_shard_path: String,
    pub evolution_envelope_id: String,
    pub timestamp_iso8601: String,
}

impl CopilotInput {
    pub fn new_advisory(
        request_id: String,
        user_did: String,
        ocpuenv_shard_path: String,
        language: String,
        context_hash: String,
        neuroprofile_shard_path: String,
        evolution_envelope_id: String,
        timestamp_iso8601: String,
    ) -> Result<Self, String> {
        let roh_target = 0.3_f32;
        Ok(Self {
            request_id,
            user_did,
            ocpuenv_shard_path,
            source_tool: "copilot".to_owned(),
            intent: "advisory".to_owned(),
            roh_target,
            language,
            context_hash,
            neuroprofile_shard_path,
            evolution_envelope_id,
            timestamp_iso8601,
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.intent != "advisory" {
            return Err("CopilotInput.intent must be \"advisory\"".into());
        }
        if !(0.0..=0.3).contains(&self.roh_target) {
            return Err("CopilotInput.roh_target must be ≤ 0.3".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopilotOutput {
    pub request_id: String,
    pub proposal_id: String,
    pub intent: String,          // always "advisory"
    pub roh_mode: String,        // e.g. "measured"
    pub roh_compliance: bool,    // RoH ≤ 0.3 for this interaction
    pub envelope_hash: String,   // hash of EvolutionProposalRecord (NDJSON)
    pub diff_format: String,     // "unified" | "aln_patch"
    pub diff_blob_base64: String,
    pub npf_policy_id: String,
    pub timestamp_iso8601: String,
}

impl CopilotOutput {
    pub fn validate(&self) -> Result<(), String> {
        if self.intent != "advisory" {
            return Err("CopilotOutput.intent must be \"advisory\"".into());
        }
        if !self.roh_compliance {
            return Err("CopilotOutput.roh_compliance must be true".into());
        }
        Ok(())
    }
}

/// Minimal EvolutionProposalRecord envelope seen by sovereign adapters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionProposalRecord {
    pub proposal_id: String,
    pub intent: String,        // must be "advisory"
    pub author_did: String,
    pub envelope_hash: String,
    pub roh_compliance: bool,
    pub kernel_compat: String, // e.g., "bostrom-sovereign-kernel-v2.ndjson"
}

impl EvolutionProposalRecord {
    pub fn is_advisory_only(&self) -> bool {
        self.intent == "advisory" && self.roh_compliance
    }
}
