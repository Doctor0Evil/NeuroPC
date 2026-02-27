//! neuro_print: governed, non-actuating "println! + neurorights + bioscale".
//!
//! This crate exposes:
//! - `neuro.print!` declarative macro.
//! - `NPF_NEURO_PRINT_BCR` for biocompatibility rating.
//! - `NeuroPrintContext` to pass BioState / route / domain.
//! - `npf_neuro_print` as the canonical NPF entrypoint.
//!
//! All outputs are non-actuating and must flow through sovereigntycore
//! guards (answer-quality, neurorights, RoH ≤ 0.3) before display/logging.

use serde::{Deserialize, Serialize};
use sovereigntycore::answerquality::{
    AnswerRoute, ChatAnswerEnvelope,
};
use organiccpucore::BioState;

/// Biocompatibility rating for neuro.print!
///
/// Non-actuating, suggest-only output. No direct control channels.
/// This constant is used by npf registries and policy logic.
pub const NPF_NEURO_PRINT_BCR: f32 = 0.31;

/// Minimal context for envelope-aware printing.
///
/// This is a copy-style context; it must never expose live handles
/// to actuation or raw neural channels.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuroPrintContext {
    pub biostate: BioState,
    pub route: AnswerRoute,
    /// Free-form domain tag: "BCI", "Biomech", "HIT", "MCI", "Legal", etc.
    pub domain: String,
    /// Optional session identifier for audit/log correlation.
    pub session_id: Option<String>,
}

/// Canonical NPF wrapper for neuro.print!
///
/// This function never writes to TTY or actuators. It only constructs
/// a ChatAnswerEnvelope<String> and delegates all gating/logging to
/// sovereigntycore.
pub fn npf_neuro_print(
    route: AnswerRoute,
    domain: &str,
    ctx: Option<&NeuroPrintContext>,
    body: String,
) -> ChatAnswerEnvelope<String> {
    sovereigntycore::neuro_print_envelope::neuro_print_envelope(
        route,
        domain,
        ctx,
        body,
    )
}

/// Declarative macro for governed, non-actuating output.
///
/// Usage example:
///
/// ```ignore
/// neuro.print!(
///     route = AnswerRoute::Info,
///     domain = "BCI",
///     ctx = &ctx_opt,
///     "kernel clipped high-intensity actions; training mode recommended today (duty: {:.2})",
///     duty_value,
/// );
/// ```
///
/// The macro expands to a call into `npf_neuro_print`, which returns a
/// ChatAnswerEnvelope<String>. Downstream UI/editor code decides how to
/// display/log the result.
#[macro_export]
macro_rules! neuro_print {
    (
        route = $route:expr,
        domain = $domain:expr,
        ctx = $ctx:expr,
        $fmt:literal $(, $args:expr )*
    ) => {{
        use $crate::NeuroPrintContext;
        use sovereigntycore::answerquality::AnswerRoute;

        // Format the body text. No side effects here.
        let body = format!($fmt $(, $args)*);

        // Call the NPF wrapper. This returns a ChatAnswerEnvelope<String>.
        let envelope = $crate::npf_neuro_print(
            $route,
            $domain,
            $ctx,
            body,
        );

        // Return the envelope to caller; they may pass it into a governed
        // emitter / UI layer. No direct printing or actuation.
        envelope
    }};
}
