use crate::answer_traits::{AnswerRoute, GovernedAnswer};
use crate::answer_metrics::AnswerEnvelope;
use crate::donutloop::{DonutloopEntry, DonutloopLedger};
use crate::riskofharm::{RiskOfHarm as RohModel, StateVector};

use neurorights_core::{CybostateClass, NeurorightsBoundAnswer, NeurorightsBoundPromptEnvelope};
use neurorights_firewall::NeurorightsFirewall;
use sovereigntycore::riskofharm::RiskModule;

/// Neurorights-visible backend abstraction: only sees bound envelopes, never raw prompts.
pub trait NeurorightsBackend {
    fn handle_envelope(
        &self,
        env: NeurorightsBoundPromptEnvelope,
    ) -> anyhow::Result<NeurorightsBoundAnswer>;
}

/// Top-level gateway combining:
/// - neurorights firewall
/// - RoH / cybostate gating for governance
/// - donutloop logging for accepted answers
pub struct ChatGateway<B, L>
where
    B: NeurorightsBackend,
    L: DonutloopLedger,
{
    firewall: NeurorightsFirewall,
    risk: RiskModule,
    roh_model: RohModel,
    ledger: L,
    backend: B,
}

impl<B, L> ChatGateway<B, L>
where
    B: NeurorightsBackend,
    L: DonutloopLedger,
{
    pub fn new(
        firewall: NeurorightsFirewall,
        risk: RiskModule,
        roh_model: RohModel,
        ledger: L,
        backend: B,
    ) -> Self {
        Self {
            firewall,
            risk,
            roh_model,
            ledger,
            backend,
        }
    }

    /// High-level handle: envelope in, neurorights-safe answer out, with RoH + donutloop.
    pub fn handle(
        &mut self,
        env: NeurorightsBoundPromptEnvelope,
    ) -> anyhow::Result<NeurorightsBoundAnswer> {
        // 1. Neurorights gate.
        self.firewall.validate_envelope(&env)?;

        // 2. RoH + cybostate check (e.g., enforce RoH ≤ 0.3 for GovernanceReady).
        let roh = self.risk.estimate_for_prompt(&env)?;
        if roh > 0.3 && matches!(env.cybostate, CybostateClass::GovernanceReady) {
            anyhow::bail!("RoH ceiling exceeded for governance path");
        }

        // 3. Delegate to backend that only sees envelopes, never raw prompts.
        let answer = self.backend.handle_envelope(env.clone())?;

        // 4. Append governance-visible donutloop entry.
        self.ledger.append_entry(DonutloopEntry::from_envelope_and_answer(
            &env,
            &answer,
            roh,
        ))?;

        Ok(answer)
    }
}

#[derive(Clone, Debug)]
pub struct ChatRequest {
    pub route: AnswerRoute,
    pub prompt: String,
    // plus subject_id, tokens, etc.
}

#[derive(Clone, Debug)]
pub struct ChatAnswerArtifact<A> {
    pub answer_text: String,
    pub envelope: AnswerEnvelope,
    pub inner: A,
}

#[derive(Debug)]
pub enum ChatRejection {
    EnvelopeViolation,
    RouteForbidden,
    InternalError,
}

impl<B, L> ChatGateway<B, L>
where
    B: NeurorightsBackend,
    L: DonutloopLedger,
{
    /// Lower-level helper when you already have a candidate answer and state vector
    /// (e.g., post-decoding, pre-render), still respecting route/cybostate + donutloop.
    pub fn finalize_answer<A>(
        &mut self,
        req: &ChatRequest,
        candidate: A,
        state: StateVector,
    ) -> Result<ChatAnswerArtifact<A>, ChatRejection>
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
        self.ledger
            .append_entry(DonutloopEntry::from_answer(req, &envelope))
            .map_err(|_| ChatRejection::InternalError)?;

        Ok(ChatAnswerArtifact {
            answer_text: render_answer(&candidate),
            envelope,
            inner: candidate,
        })
    }
}

// Application-specific rendering of the textual body.
fn render_answer<A>(_a: &A) -> String {
    // Implement with your existing templating.
    String::new()
}
