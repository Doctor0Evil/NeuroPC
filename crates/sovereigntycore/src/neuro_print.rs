use serde::{Deserialize, Serialize};
use std::fmt::Arguments;

use crate::answerquality::{
    AnswerQuality, AnswerRoute, ChatKnowledgeFactor, Cybostate, CybostateClass, RiskEnvelope,
};
use crate::organiccpu_bridge::{BioStateSnapshot, SafeEnvelopeDecision};
use crate::rohmodel::{RohModel, StateVector};
use crate::sovereign_kernel::AnswerQualitySpec;
use crate::donutloop::AnswerLedgerWriter;

/// Biocompatibility rating in [0,1] for this neuro.print! façade.
pub const NEURO_PRINT_BCR: f32 = 0.26;

/// High‑level context for a single neuro.print! emission.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuroPrintContext {
    /// Subject DID / Bostrom address.
    pub subject_id: String,
    /// Sovereign kernel id (ties into bostrom‑sovereign‑kernel‑v2.ndjson).
    pub kernel_id: String,
    /// Logical route for this answer (Info vs GovernanceDesign, never Actuation).
    pub route: AnswerRoute,
    /// Snapshot of bioscale / OrganicCPU state.
    pub bio: BioStateSnapshot,
    /// Safe envelope decision from OrganicCPU (Allow/Degrade/Pause).
    pub envelope: SafeEnvelopeDecision,
    /// Knowledge/risk spec from sovereign kernel.
    pub quality_spec: AnswerQualitySpec,
    /// RoH model to score this answer.
    pub roh_model: RohModel,
    /// Optional per‑domain RoH profile id (future extension).
    pub roh_domain_profile: Option<String>,
    /// Optional donutloop writer; if None, no ledger logging occurs.
    pub ledger_writer: Option<AnswerLedgerWriter>,
}

/// Serializable governed answer artifact for .answer.ndjson.
/// This is what neuro.print! emits – never raw TTY.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatAnswerEnvelope<T> {
    pub body: T,
    pub quality: AnswerQuality,
    pub subject_id: String,
    pub kernel_id: String,
    pub route: AnswerRoute,
    pub bio: BioStateSnapshot,
    pub envelope: SafeEnvelopeDecision,
    /// Actuation‑forbidden proof bit; must always be true for this macro.
    pub actuation_forbidden: bool,
    /// Non‑commercial, non‑financial flag (mirrors neurorights policy).
    pub non_commercial: bool,
    /// RoH domain tag (e.g., "neuromorph.general"); can be None.
    pub roh_domain: Option<String>,
}

/// Minimal trait describing what the macro backend must implement.
pub trait NeuroPrintBackend {
    type Body: Serialize + Clone;

    /// Produce the rendered body (e.g., Markdown) from arguments.
    fn build_body(&self, args: &Arguments<'_>) -> Self::Body;

    /// Compute KnowledgeFactor F in [0,1] for this tentative answer.
    fn compute_knowledge_factor(&self, body: &Self::Body) -> crate::answerquality::KnowledgeFactor;

    /// Map context + body into a RoH StateVector and compute AnswerRisk.
    fn compute_risk(
        &self,
        ctx: &NeuroPrintContext,
        body: &Self::Body,
    ) -> crate::answerquality::AnswerRisk;

    /// Classify Cybostate based on context and body.
    fn classify_cybostate(&self, ctx: &NeuroPrintContext, body: &Self::Body) -> Cybostate;

    /// Optionally log into .answer.ndjson + donutloop.
    fn log_answer(
        &self,
        ctx: &NeuroPrintContext,
        envelope: &ChatAnswerEnvelope<Self::Body>,
    ) -> Result<(), String>;
}

/// Thin façade: validates envelope, delegates to backend, returns governed artifact.
/// Never writes to stdout/stderr; returns None if constraints fail.
///
/// RoH invariant: r <= 0.3 always. Non‑commercial and actuation‑forbidden enforced.
pub fn neuro_print_internal<B: NeuroPrintBackend>(
    backend: &B,
    ctx: &NeuroPrintContext,
    fmt_args: &Arguments<'_>,
) -> Option<ChatAnswerEnvelope<B::Body>> {
    // 1. If OrganicCPU says Pause, we emit nothing.
    match ctx.envelope {
        SafeEnvelopeDecision::PauseAndRest => {
            return None;
        }
        SafeEnvelopeDecision::DegradePrecision | SafeEnvelopeDecision::AllowFullAction => {}
    }

    // 2. Render body through backend.
    let body = backend.build_body(fmt_args);

    // 3. Compute quality scalars.
    let f = backend.compute_knowledge_factor(&body);
    let r = backend.compute_risk(ctx, &body);
    let cybo = backend.classify_cybostate(ctx, &body);

    let quality = AnswerQuality { f, r, cybostate: cybo.clone() };

    // 4. Hard guards:
    //    - RoH ceiling (global <= 0.3).
    //    - Neurorights‑driven minimum F (from AnswerQualitySpec).
    //    - Route vs Cybostate compatibility.
    if !r.is_within_ceiling() {
        return None;
    }
    if !f.is_sufficient(ctx.quality_spec.min_knowledge_factor) {
        return None;
    }
    if !CybostateClass::is_route_allowed_impl(cybo.clone(), ctx.route.clone()) {
        return None;
    }

    // 5. Enforce actuation‑forbidden semantics: this macro is never allowed for Actuation route.
    if matches!(ctx.route, AnswerRoute::Actuation) {
        return None;
    }

    // 6. Build envelope.
    let envelope = ChatAnswerEnvelope {
        body: body.clone(),
        quality: quality.clone(),
        subject_id: ctx.subject_id.clone(),
        kernel_id: ctx.kernel_id.clone(),
        route: ctx.route.clone(),
        bio: ctx.bio.clone(),
        envelope: ctx.envelope.clone(),
        actuation_forbidden: true,
        non_commercial: true, // locked by design; matches neurorights policy shard.
        roh_domain: ctx.roh_domain_profile.clone(),
    };

    // 7. Optional ledger logging.
    if let Some(writer) = &ctx.ledger_writer {
        if let Err(err) = backend.log_answer(ctx, &envelope) {
            // On ledger failure, we can either drop or still return.
            // For strict donutloop compliance, drop the answer.
            eprintln!("neuro.print! ledger error (suppressed to non‑TTY caller): {}", err);
            return None;
        }
    }

    Some(envelope)
}

/// Public macro: never prints; returns an Option<ChatAnswerEnvelope<Body>> to the caller.
///
/// Usage:
///     let env = neuro.print!(backend, ctx, "Hello {}!", user);
///
/// The caller is responsible for serializing `env` into `.answer.ndjson`.
#[macro_export]
macro_rules! neuro_print {
    ($backend:expr, $ctx:expr, $($arg:tt)*) => {{
        use std::fmt::Arguments;
        let args: Arguments = format_args!($($arg)*);
        $crate::neuro_print::neuro_print_internal(&$backend, &$ctx, &args)
    }};
}
