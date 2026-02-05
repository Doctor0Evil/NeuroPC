use thiserror::Error;

use organiccpualn::prompt_envelope::NeurorightsBoundPromptEnvelope;
use neurorights_core::{guard_prompt_tool, NeurorightsPolicyDocument, ToolCapability, NeurorightsViolation};

/// Backend-specific error wrapper.
#[derive(Debug, Error)]
pub enum FirewallError {
    #[error("neurorights violation: {0}")]
    Neurorights(#[from] NeurorightsViolation),

    #[error("backend error: {0}")]
    Backend(String),
}

/// Trait that any backend (LLM, tool router) must implement.
/// Critically: it never sees a raw String, only a vetted envelope.
pub trait NeurorightsSafeBackend {
    type Response;

    fn handle_envelope(
        &self,
        envelope: &NeurorightsBoundPromptEnvelope,
    ) -> Result<Self::Response, String>;
}

/// Main firewall entrypoint:
/// - loads neurorights policy (caller supplies doc)
/// - runs guard_prompt_tool
/// - forwards envelope to backend only if allowed.
pub fn process_prompt<B: NeurorightsSafeBackend>(
    backend: &B,
    env: &NeurorightsBoundPromptEnvelope,
    policy: &NeurorightsPolicyDocument,
    tool: &ToolCapability,
) -> Result<B::Response, FirewallError> {
    // Enforce neurorights at type level.
    guard_prompt_tool(env, policy, tool)?;

    // If we reach here, the request is neurorights-clean for this tool.
    backend
        .handle_envelope(env)
        .map_err(FirewallError::Backend)
}

#[derive(Debug)]
pub enum NeurorightsViolation {
    MissingPolicy,
    MentalPrivacyViolation,
    ForbiddenDecisionUse(String),
    InnerStateScoringForbidden,
    CoercionForbidden,
}

pub struct NeurorightsFirewall {
    /// Loaded neurorights policy for this subject/session.
    pub policy: NeurorightsPolicyDocument,
}

impl NeurorightsFirewall {
    pub fn from_policy(policy: NeurorightsPolicyDocument) -> Self {
        Self { policy }
    }

    /// Hard gate; any violation returns Err and must abort the pipeline.
    pub fn validate_envelope(
        &self,
        env: &NeurorightsBoundPromptEnvelope,
    ) -> Result<(), NeurorightsViolation> {
        // 1. Mental privacy: forbid inner-state scoring if policy says so.
        if self.policy.dreamstate.dreamsensitive
            && env.domain_tags.iter().any(|t| t == "dreamstate")
            && self.policy.dreamstate.forbid_inner_state_scoring
        {
            return Err(NeurorightsViolation::InnerStateScoringForbidden);
        }

        // 2. Coercion / forbidden domains: employment, credit, etc.
        for tag in &env.domain_tags {
            if self.policy
                .dreamstate
                .forbid_decision_use
                .iter()
                .any(|forbidden| forbidden == tag)
            {
                return Err(NeurorightsViolation::ForbiddenDecisionUse(tag.clone()));
            }
        }

        // 3. Cybostate / actuation: ActuationForbidden never yields control outputs.
        if matches!(env.cybostate, CybostateClass::ActuationForbidden)
            && env.allowed_tools.iter().any(|t| t == "actuator")
        {
            return Err(NeurorightsViolation::CoercionForbidden);
        }

        Ok(())
    }
}
