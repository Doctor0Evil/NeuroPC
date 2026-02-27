use crate::answer_quality::{AnswerRisk, AnswerRoute, Cybostate, KnowledgeFactor};
use crate::chat_guard::{ChatAnswerEnvelope, ChatGuardConfig, GovernedAnswerEmitter};

use organiccpualn::rohmodel::{RohModel, StateVector};
use organiccpu_core::{BioState, SafeEnvelopeDecision};

/// Domain tag for risk-model mapping (e.g., "BCI", "Biomech", "Legal").
#[derive(Clone, Debug)]
pub struct DomainTag(pub String);

/// Fixed view of the environment required to build a governed envelope.
#[derive(Clone, Debug)]
pub struct NeuroPrintContext<'a> {
    pub roh_model: &'a RohModel,
    pub biostate: &'a BioState,
    /// Current envelope decision from OrganicCPU (Allow, Degrade, PauseAndRest).
    pub safe_decision: SafeEnvelopeDecision,
    /// Minimal F for this subject/route.
    pub guard_cfg: ChatGuardConfig,
    /// Optional hexstamp linkage for donutloop / KO ledgers.
    pub hexstamp: Option<String>,
    pub prev_hexstamp: Option<String>,
}

/// Parameters passed from macro call-site.
#[derive(Clone, Debug)]
pub struct NeuroPrintRequest {
    /// Formatted body string from macro.
    pub body: String,
    /// Route declared in macro invocation.
    pub route: AnswerRoute,
    /// Domain tag guiding RoH mapping.
    pub domain_tag: DomainTag,
}

/// Internal emitter used by `neuro.print!`.
struct NeuroPrintEmitter<'a> {
    ctx: &'a NeuroPrintContext<'a>,
    req: &'a NeuroPrintRequest,
}

impl<'a> GovernedAnswerEmitter for NeuroPrintEmitter<'a> {
    type Body = String;

    fn build_body(&self) -> Self::Body {
        self.req.body.clone()
    }

    fn compute_f(&self, body: &Self::Body) -> KnowledgeFactor {
        // Simple citation-density heuristic:
        // F = min(1.0, c / 4.0), where c = number of "[...:idx]" patterns.
        let citation_count = body.matches("[").count(); // caller can refine pattern.
        let raw = (citation_count as f32) / 4.0;
        KnowledgeFactor::clamped(raw)
    }

    fn compute_risk(&self, _body: &Self::Body) -> AnswerRisk {
        // Map domain + BioState into a small StateVector and run RoH.
        let (load_cognitive, load_governance, load_biophysical) = match self.req.domain_tag.0.as_str() {
            "BCI" | "Biomech" => (0.4, 0.4, 0.3),
            "Legal" => (0.2, 0.4, 0.1),
            _ => (0.1, 0.1, 0.05),
        };

        let fatigue = self.ctx.biostate.fatigue_index().clamp(0.0, 1.0);
        let cognitive = self.ctx.biostate.cognitive_load_index().clamp(0.0, 1.0);

        let components = vec![
            fatigue * load_biophysical,
            cognitive * load_cognitive,
            load_governance,
        ];

        let state = StateVector { components };
        let roh_raw = self.ctx.roh_model.compute_roh(state);
        AnswerRisk::clamped(roh_raw)
    }

    fn classify_cybostate(&self, _body: &Self::Body) -> Cybostate {
        // Adjust Cybostate based on OrganicCPU SafeEnvelopeDecision.
        match self.ctx.safe_decision {
            SafeEnvelopeDecision::PauseAndRest => Cybostate::RetrievalOnly,
            SafeEnvelopeDecision::Degrade => Cybostate::ResearchReady,
            SafeEnvelopeDecision::Allow => match self.req.route {
                AnswerRoute::Info => Cybostate::RetrievalOnly,
                AnswerRoute::GovernanceDesign => Cybostate::GovernanceReady,
                AnswerRoute::Actuation => Cybostate::ActuationForbidden,
            },
        }
    }

    fn requested_route(&self) -> AnswerRoute {
        self.req.route.clone()
    }

    fn hexstamps(&self) -> (Option<String>, Option<String>) {
        (self.ctx.hexstamp.clone(), self.ctx.prev_hexstamp.clone())
    }
}

/// Public helper used by the macro expansion.
///
/// Never writes to TTY; returns a governed ChatAnswerEnvelope<String>
/// or None when constraints (RoH, F, Cybostate) fail.
pub fn neuro_print_envelope(
    ctx: &NeuroPrintContext<'_>,
    req: &NeuroPrintRequest,
) -> Option<ChatAnswerEnvelope<String>> {
    // Respect bioscale decision first: downgrade or deny.
    match ctx.safe_decision {
        SafeEnvelopeDecision::PauseAndRest => {
            // Optionally emit a minimal rest advisory with low F.
            let degraded_req = NeuroPrintRequest {
                body: "[REST] BioState indicates pause-and-rest. Consider taking a short break."
                    .to_string(),
                route: AnswerRoute::Info,
                domain_tag: DomainTag("BioRest".to_string()),
            };
            let emitter = NeuroPrintEmitter {
                ctx,
                req: &degraded_req,
            };
            return emitter.emit_answer(&ctx.guard_cfg);
        }
        _ => {
            let emitter = NeuroPrintEmitter { ctx, req };
            emitter.emit_answer(&ctx.guard_cfg)
        }
    }
}
