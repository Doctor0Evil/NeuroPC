use crate::answer_metrics::{AnswerEnvelope, KnowledgeFactor, RiskOfHarm, Cybostate};

pub trait ChatKnowledgeFactor {
    fn compute_knowledge_factor(&self) -> KnowledgeFactor;
}

pub trait RiskEnvelope {
    /// Compute incremental RoH for this answer, given the current state vector.
    fn estimate_roh(&self, state: crate::riskofharm::StateVector) -> RiskOfHarm;
}

pub trait CybostateClass {
    fn cybostate(&self) -> Cybostate;
}

/// Minimal interface for an answer before it can be emitted.
pub trait GovernedAnswer:
    ChatKnowledgeFactor + RiskEnvelope + CybostateClass
{
    /// Per‑route minima; can be overridden per domain.
    fn required_min_knowledge(&self) -> f32 { 0.75 }
    fn allowed_max_roh(&self) -> f32 { 0.30 }

    /// Route‑level permission check: is this cybostate allowed for the requested capability?
    fn is_route_allowed(&self, route: AnswerRoute, cs: Cybostate) -> bool {
        match route {
            AnswerRoute::PlainRetrieval => true,
            AnswerRoute::ResearchSynthesis => matches!(cs, Cybostate::ResearchReady | Cybostate::GovernanceReady),
            AnswerRoute::GovernanceAdvice => matches!(cs, Cybostate::GovernanceReady),
            AnswerRoute::ActuationProposal => false, // ActuationForbidden invariant.
        }
    }

    fn envelope(&self, state: crate::riskofharm::StateVector) -> Option<AnswerEnvelope> {
        let k = self.compute_knowledge_factor();
        let r = self.estimate_roh(state);
        let cs = self.cybostate();

        if k.value < self.required_min_knowledge() {
            return None;
        }
        if r.value > self.allowed_max_roh() {
            return None;
        }

        Some(AnswerEnvelope { knowledge: k, roh: r, cybostate: cs })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AnswerRoute {
    PlainRetrieval,
    ResearchSynthesis,
    GovernanceAdvice,
    ActuationProposal,
}
