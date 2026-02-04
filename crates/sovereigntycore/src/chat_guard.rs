use organiccpualn::rohmodel::{RohModelShard, RohInputs};
use neurorights_core::{NeurorightsBoundPromptEnvelope, CybostateClass};
use neurorights_firewall::NeurorightsFirewall;

pub struct ChatFitness {
    pub f_score: f32,     // fitness / fluency
    pub roh: f32,         // estimated incremental RoH
    pub cybostate: CybostateClass,
}

pub trait RightsBoundChatExecutor {
    type Answer;

    /// Core guarded entrypoint: envelopes in, rights-checked answers out.
    fn execute_guarded(
        &self,
        env: NeurorightsBoundPromptEnvelope,
    ) -> anyhow::Result<Self::Answer>;
}

pub struct GuardKernels<B> {
    pub backend: B,
    pub roh_model: RohModelShard,
    pub firewall: NeurorightsFirewall,
}

impl<B> RightsBoundChatExecutor for GuardKernels<B>
where
    B: Fn(&NeurorightsBoundPromptEnvelope) -> anyhow::Result<(String, ChatFitness)>,
{
    type Answer = String;

    fn execute_guarded(
        &self,
        env: NeurorightsBoundPromptEnvelope,
    ) -> anyhow::Result<Self::Answer> {
        // 1. Neurorights hard gate (no inner-state scoring, no forbidden domains, etc.).
        self.firewall.validate_envelope(&env)?;

        // 2. Ask backend to propose answer + provisional fitness/RoH/cybostate.
        let (raw_answer, fit) = (self.backend)(&env)?;

        // 3. Enforce RoH and cybostate thresholds.
        if fit.roh > self.roh_model.rohceiling()
            || matches!(fit.cybostate, CybostateClass::ActuationForbidden)
        {
            anyhow::bail!("Rejected by RoH/cybostate guard");
        }

        // 4. Optionally downgrade content based on F or domain.
        // e.g., lower F for high-risk domains, strip actuation suggestions, etc.

        Ok(raw_answer)
    }
}
