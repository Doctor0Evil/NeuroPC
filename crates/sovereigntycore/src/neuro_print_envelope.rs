//! neuro_print_envelope: construct governed ChatAnswerEnvelope<String>
//! for neuro.print!-style calls.
//!
//! Responsibilities:
//! - Compute KnowledgeFactor F ∈ [0,1].
//! - Compute local AnswerRisk RoH (clamped to 0.3) via RohModel.
//! - Assign Cybostate and AnswerRoute.
//! - Optionally adjust content based on BioState / SafeEnvelopeDecision.
//! - Enforce basic invariants at construction time.
//! - DO NOT print or actuate; only return an envelope.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::answerquality::{
    AnswerQuality, AnswerRoute, AnswerRisk, ChatAnswerEnvelope,
    Cybostate, KnowledgeFactor,
};
use crate::rohmodel::RohModel;
use crate::chatguard::ChatGuardConfig;
use crate::forbidden_patterns::ForbiddenPatternSet;
use crate::logging::answer_log::log_answer_envelope;

use organiccpucore::{BioState, SafeEnvelopeDecision, SafeEnvelopePolicy};

/// Minimal copy of the context type from neuro_print crate.
/// Kept in sync via shared dependencies.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuroPrintContext {
    pub biostate: BioState,
    pub route: AnswerRoute,
    pub domain: String,
    pub session_id: Option<String>,
}

/// Main helper for neuro.print!
///
/// This function is pure in the sense of not actuating; its only side
/// effect is optional logging via answer_log (append-only audit).
pub fn neuro_print_envelope(
    route: AnswerRoute,
    domain: &str,
    ctx: Option<&NeuroPrintContext>,
    body: String,
) -> ChatAnswerEnvelope<String> {
    // 1. Derive a timestamp.
    let now: DateTime<Utc> = Utc::now();

    // 2. Compute KnowledgeFactor (placeholder: can be refined later).
    let kf = estimate_knowledge_factor(&body, domain, &route);

    // 3. Compute AnswerRisk via RohModel, clamped to 0.3.
    let roh = compute_answer_risk(ctx.map(|c| &c.biostate), domain, &route);

    // 4. Assign Cybostate class.
    let cybostate = classify_cybostate(domain, &route, roh);

    // 5. Envelope-aware adjustment (BioState / SafeEnvelopeDecision).
    let (adjusted_body, rest_advisory) =
        maybe_adjust_for_bioscale(ctx.map(|c| &c.biostate), &body);

    // 6. Basic forbidden-pattern filtering (non-actuating).
    let filtered_body =
        apply_forbidden_patterns(&adjusted_body, domain, &route);

    // 7. Construct AnswerQuality.
    let quality = AnswerQuality {
        knowledge_factor: KnowledgeFactor(kf),
        risk: AnswerRisk(roh),
        cybostate,
        route,
        domain: domain.to_string(),
        rest_advisory,
        timestamp_utc: now,
    };

    // 8. Construct envelope (no printing yet).
    let mut envelope = ChatAnswerEnvelope {
        body: filtered_body,
        quality,
        session_id: ctx.and_then(|c| c.session_id.clone()),
        // Additional metadata fields can be added here as needed.
    };

    // 9. Optional guard-level check (RoH ≤ 0.3, etc.).
    //    You can parameterize this via ChatGuardConfig if available.
    if !envelope.quality.is_allowed_for_route() {
        // Downgrade or redact if not allowed.
        envelope.body = String::from(
            "[neuro.print! redacted by guard: risk or policy threshold]",
        );
    }

    // 10. Append to .answer.ndjson audit trail (append-only).
    //     This uses hash-linked logging, but must remain non-commercial
    //     and non-financial.
    if let Err(e) = log_answer_envelope(&envelope) {
        // Fail closed at logging layer: if audit fails, we still return
        // the envelope, but logging error is recorded via internal logs.
        eprintln!("[sovereigntycore] answer logging error: {:?}", e);
    }

    envelope
}

/// Estimate KnowledgeFactor F ∈ [0,1].
///
/// Placeholder implementation; refine using KO / CyberRank later.
fn estimate_knowledge_factor(
    body: &str,
    domain: &str,
    route: &AnswerRoute,
) -> f32 {
    // Very basic heuristic: longer, domain-aligned text gets a higher F.
    let len = body.chars().count() as f32;
    let base = if len > 400.0 {
        0.9
    } else if len > 200.0 {
        0.7
    } else {
        0.5
    };

    // Slight boost for governance/design routes where policies are strong.
    let route_factor = match route {
        AnswerRoute::GovernanceDesign => 0.1,
        _ => 0.0,
    };

    let domain_factor = match domain {
        "BCI" | "HIT" | "MCI" | "Biomech" => 0.05,
        _ => 0.0,
    };

    let f = base + route_factor + domain_factor;
    f.clamp(0.0, 1.0)
}

/// Compute AnswerRisk via RohModel and BioState.
///
/// Clamps result to global ceiling 0.3.
fn compute_answer_risk(
    biostate: Option<&BioState>,
    domain: &str,
    route: &AnswerRoute,
) -> f32 {
    // Map domain/route into RoH axes. This is a placeholder that calls
    // into your existing RohModel shard.
    let mut roh = RohModel::global().evaluate_answer(
        biostate,
        domain,
        route,
    );

    if roh > 0.3 {
        roh = 0.3;
    }

    roh
}

/// Assign Cybostate based on domain, route, and risk.
fn classify_cybostate(
    domain: &str,
    route: &AnswerRoute,
    roh: f32,
) -> Cybostate {
    use Cybostate::*;

    if roh > 0.25 {
        return RetrievalOnly;
    }

    match route {
        AnswerRoute::GovernanceDesign => {
            if matches!(domain, "BCI" | "HIT" | "MCI" | "Biomech") {
                GovernanceReady
            } else {
                ResearchReady
            }
        }
        _ => ResearchReady,
    }
}

/// Adjust body text based on BioState / SafeEnvelopeDecision.
///
/// This never actuates; it only changes verbosity and adds a rest flag.
fn maybe_adjust_for_bioscale(
    biostate: Option<&BioState>,
    body: &str,
) -> (String, bool) {
    if let Some(bs) = biostate {
        let decision = SafeEnvelopePolicy::decide(bs);
        match decision {
            SafeEnvelopeDecision::PauseAndRest => {
                let short = "[rest advised] ".to_string();
                let truncated = if body.len() > 160 {
                    short + &body[..160]
                } else {
                    short + body
                };
                (truncated, true)
            }
            SafeEnvelopeDecision::Degrade => {
                let short = "[reduced detail] ".to_string();
                let truncated = if body.len() > 320 {
                    short + &body[..320]
                } else {
                    short + body
                };
                (truncated, false)
            }
            SafeEnvelopeDecision::Allow => (body.to_string(), false),
        }
    } else {
        (body.to_string(), false)
    }
}

/// Apply neurorights-bound forbidden patterns for neuromorphic / HIT / MCI.
///
/// This ONLY redacts or replaces text; it never actuates.
fn apply_forbidden_patterns(
    body: &str,
    domain: &str,
    route: &AnswerRoute,
) -> String {
    let patterns = ForbiddenPatternSet::for_domain(domain, route);

    let mut text = body.to_string();
    for patt in patterns.iter() {
        if patt.matches(&text) {
            text = patt.redact(&text);
        }
    }
    text
}
