#![allow(clippy::needless_doctest_main)]

/// Macro façade for governed prints.
///
/// Usage:
/// ```ignore
/// let envelope = neuro::print!(
///     ctx = &np_ctx,                // &NeuroPrintContext
///     route = Info,                 // or GovernanceDesign
///     domain = "BCI",               // domain tag
///     "Hello, {}", username         // format string + args
/// );
///
/// if let Some(env) = envelope {
///     // UI/editor decides how to render env.body based on env.quality.
/// }
/// ```
#[macro_export]
macro_rules! neuro_print {
    (ctx = $ctx:expr, route = $route:ident, domain = $domain:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {{
        use $crate::prelude_neuro_print::*;
        let body = format!($fmt $(, $arg)*);

        let req = NeuroPrintRequest {
            body,
            route: AnswerRoute::$route,
            domain_tag: DomainTag($domain.to_string()),
        };

        // This call is pure: it only returns an envelope, no TTY or I/O.
        neuro_print_envelope($ctx, &req)
    }};
}
