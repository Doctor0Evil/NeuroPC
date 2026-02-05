use serde::{Deserialize, Serialize};
use thiserror::Error;

use organiccpualn::prompt_envelope::NeurorightsBoundPromptEnvelope;

/// Minimal view of your neurorights policy JSON.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsPolicyDocument {
    pub policyid: String,
    pub subjectid: String,

    pub mentalprivacy: bool,
    pub mentalintegrity: bool,
    pub cognitiveliberty: bool,

    /// Dream / neural sensitive flags already in your stack.
    pub dreamstate: Option<DreamStateSlice>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DreamStateSlice {
    pub dreamsensitive: bool,
    pub forbiddecisionuse: Vec<String>, // ["employment","housing","credit","insurance"]
    pub forgetslahours: u32,
    pub noncommercial: bool,
    pub soulnontradeable: bool,
}

/// Tool capability description for checking allowed operations.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCapability {
    pub tool_id: String,
    pub can_read_inner_state: bool,
    pub can_score_user: bool,
    pub can_write_longterm_profile: bool,
    pub eco_cost_estimate: f32, // 0..1
}

#[derive(Debug, Error)]
pub enum NeurorightsViolation {
    #[error("tool {tool_id} not allowed by envelope")]
    ToolNotAllowed { tool_id: String },

    #[error("inner-state scoring is forbidden for domain {domain}")]
    InnerStateScoringForbidden { domain: String },

    #[error("coercive or non-revocable use detected for domain {domain}")]
    CoerciveUse { domain: String },

    #[error("noncommercial neurorights profile forbids this use")]
    NonCommercialViolation,

    #[error("eco-impact {eco} exceeds allowed limit {limit}")]
    EcoOverLimit { eco: f32, limit: f32 },
}

/// Core guard: check envelope + tool against neurorights policy.
pub fn guard_prompt_tool(
    env: &NeurorightsBoundPromptEnvelope,
    policy: &NeurorightsPolicyDocument,
    tool: &ToolCapability,
) -> Result<(), NeurorightsViolation> {
    // 1) Tool must be listed in allowed_tools.
    if !env.allowed_tools.iter().any(|t| t == &tool.tool_id) {
        return Err(NeurorightsViolation::ToolNotAllowed {
            tool_id: tool.tool_id.clone(),
        });
    }

    // 2) No inner-state scoring for protected domains.
    let protected_domains = policy
        .dreamstate
        .as_ref()
        .map(|d| d.forbiddecisionuse.clone())
        .unwrap_or_default();

    for tag in &env.domain_tags {
        if protected_domains.iter().any(|d| d == tag) && tool.can_score_user {
            return Err(NeurorightsViolation::InnerStateScoringForbidden {
                domain: tag.clone(),
            });
        }
    }

    // 3) Noncommercial neurorights: forbid tools that write long-term profiles
    // or are marked as "commercial".
    if let Some(dream) = &policy.dreamstate {
        if dream.noncommercial && tool.can_write_longterm_profile {
            return Err(NeurorightsViolation::NonCommercialViolation);
        }
    }

    // 4) Simple eco guard: bound eco cost by a global limit from policy or hard-coded.
    let eco_limit = 0.5_f32; // initial conservative bound, refine via policy if desired.
    if tool.eco_cost_estimate > eco_limit {
        return Err(NeurorightsViolation::EcoOverLimit {
            eco: tool.eco_cost_estimate,
            limit: eco_limit,
        });
    }

    Ok(())
}
