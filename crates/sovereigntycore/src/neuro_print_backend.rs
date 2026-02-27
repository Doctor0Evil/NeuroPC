use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::answerquality::{
    AnswerRisk, AnswerRoute, ChatKnowledgeFactor, Cybostate, CybostateClass,
    KnowledgeFactor, RiskEnvelope,
};
use crate::donutloop::{AnswerLedgerEntry, AnswerLedgerWriter};
use crate::neuro_print::{ChatAnswerEnvelope, NeuroPrintBackend, NeuroPrintContext};
use crate::organiccpu_bridge::BioStateSnapshot;
use crate::rohmodel::{RohModel, StateVector};

/// Simple text backend: treats the body as a UTF‑8 String.
#[derive(Clone, Debug)]
pub struct TextNeuroPrintBackend;

impl NeuroPrintBackend for TextNeuroPrintBackend {
    type Body = String;

    fn build_body(&self, args: &std::fmt::Arguments<'_>) -> Self::Body {
        // Render into a String; caller may post‑process Markdown if desired.
        args.to_string()
    }

    fn compute_knowledge_factor(&self, body: &Self::Body) -> KnowledgeFactor {
        // Extremely simple heuristic: length + presence of citations.
        let len = body.len() as f32;
        let has_citations = body.contains('[') && body.contains("].");

        let mut score = 0.0f32;

        if len > 128.0 {
            score += 0.3;
        }
        if len > 512.0 {
            score += 0.2;
        }
        if has_citations {
            score += 0.3;
        }

        // Clamp into [0,1].
        KnowledgeFactor::clamped(score.min(0.9))
    }

    fn compute_risk(&self, ctx: &NeuroPrintContext, _body: &Self::Body) -> AnswerRisk {
        // Build a tiny StateVector from BioState and envelope.
        // Axes are 0..1 normalized; RoH ceiling 0.3 enforced by RohModel + AnswerRisk.
        let mut components: Vec<f32> = Vec::with_capacity(4);

        let bio: &BioStateSnapshot = &ctx.bio;

        // Cognitive load
        components.push(bio.cognitive_load_index.clamp(0.0, 1.0));
        // Fatigue
        components.push(bio.fatigue_index.clamp(0.0, 1.0));
        // Duty cycle
        components.push(bio.duty_cycle.clamp(0.0, 1.0));
        // Envelope pressure: Pause -> high, Degrade -> medium, Allow -> low.
        let envelope_pressure = match ctx.envelope {
            crate::organiccpu_bridge::SafeEnvelopeDecision::AllowFullAction => 0.1,
            crate::organiccpu_bridge::SafeEnvelopeDecision::DegradePrecision => 0.25,
            crate::organiccpu_bridge::SafeEnvelopeDecision::PauseAndRest => 0.4,
        };
        components.push(envelope_pressure);

        let state = StateVector { components };
        let roh_raw = ctx.roh_model.compute_roh(state);

        AnswerRisk::clamped(roh_raw)
    }

    fn classify_cybostate(&self, ctx: &NeuroPrintContext, _body: &Self::Body) -> Cybostate {
        match ctx.route {
            AnswerRoute::Info => Cybostate::RetrievalOnly,
            AnswerRoute::GovernanceDesign => Cybostate::GovernanceReady,
            AnswerRoute::Actuation => Cybostate::ActuationForbidden,
        }
    }

    fn log_answer(
        &self,
        ctx: &NeuroPrintContext,
        envelope: &ChatAnswerEnvelope<Self::Body>,
    ) -> Result<(), String> {
        let writer = match &ctx.ledger_writer {
            Some(w) => w,
            None => return Ok(()),
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs();

        let entry = AnswerLedgerEntry {
            answer_id: format!("ans-{}", now),
            subject_id: ctx.subject_id.clone(),
            kernel_id: ctx.kernel_id.clone(),
            route: envelope.route.clone(),
            knowledge_factor: envelope.quality.f.value,
            roh: envelope.quality.r.roh,
            cybostate: envelope.quality.cybostate.clone(),
            bio: envelope.bio.clone(),
            timestamp_utc: now,
            prev_hexstamp: writer.last_hexstamp()?,
            hexstamp: writer.compute_hexstamp(
                &ctx.subject_id,
                &ctx.kernel_id,
                now,
                envelope.quality.f.value,
                envelope.quality.r.roh,
            )?,
            /// Non‑financial, non‑commercial, answer‑quality artifacts only.
            artifact_kind: "answerquality".to_string(),
            contract_type: "neuroassistive".to_string(),
        };

        writer.append_entry(entry)
    }
}
