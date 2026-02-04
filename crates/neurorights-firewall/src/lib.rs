use neurorights_core::{NeurorightsBoundPromptEnvelope, CybostateClass};
use organiccpualn::NeurorightsPolicyDocument; // your existing policy type

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
