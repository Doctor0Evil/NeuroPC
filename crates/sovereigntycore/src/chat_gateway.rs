use crate::answer_traits::{GovernedAnswer, AnswerRoute};
use crate::answer_metrics::AnswerEnvelope;
use crate::riskofharm::{RiskOfHarm as RohModel, StateVector};
use crate::donutloop::DonutloopLedger;

#[derive(Clone, Debug)]
pub struct ChatRequest {
    pub route: AnswerRoute,
    pub prompt: String,
    // plus subjectid, tokens, etc.
}

#[derive(Clone, Debug)]
pub struct ChatAnswerArtifact<A> {
    pub answer_text: String,
    pub envelope: AnswerEnvelope,
    pub inner: A,
}

pub struct ChatGateway<L: DonutloopLedger> {
    pub roh_model: RohModel,
    pub ledger: L,
}

impl<L: DonutloopLedger> ChatGateway<L> {
    pub fn handle<A>(&mut self, req: &ChatRequest, candidate: A,
                     state: StateVector)
        -> Result<ChatAnswerArtifact<A>, ChatRejection>
    where
        A: GovernedAnswer,
    {
        let envelope = candidate
            .envelope(state)
            .ok_or(ChatRejection::EnvelopeViolation)?;

        if !candidate.is_route_allowed(req.route, envelope.cybostate) {
            return Err(ChatRejection::RouteForbidden);
        }

        // Append to donutloop with KF, RoH, Cybostate as columns.
        self.ledger.append_entry(
            crate::donutloop::DonutloopEntry::from_answer(req, &envelope)
        )?;

        Ok(ChatAnswerArtifact {
            answer_text: render_answer(&candidate),
            envelope,
            inner: candidate,
        })
    }
}

#[derive(Debug)]
pub enum ChatRejection {
    EnvelopeViolation,
    RouteForbidden,
    InternalError,
}

// Application-specific rendering of the textual body.
fn render_answer<A>(_a: &A) -> String {
    // Implement with your existing templating.
    String::new()
}
