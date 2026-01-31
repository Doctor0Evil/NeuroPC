use crate::context::{AgentTurnContext, evaluate_ota_change};

pub struct GenerationParams {
    pub max_tokens: usize,
    pub step_size: usize,
    pub verbosity: f32,
}

pub fn params_from_hints(ctx: &AgentTurnContext) -> GenerationParams {
    let lvl = ctx.reality_hints.automagic_level.clamp(0.0, 1.0);

    // Map automagic_level to behavior:
    //  - 1.0: long, dense, multi-step completions
    //  - 0.5: medium answers, more scaffolding
    //  - 0.2: very short, gentle suggestions
    let max_tokens = (256.0 + 768.0 * lvl) as usize;
    let step_size = if lvl > 0.7 { 4 } else if lvl > 0.4 { 2 } else { 1 };
    let verbosity = lvl;

    GenerationParams {
        max_tokens,
        step_size,
        verbosity,
    }
}

/// Example of one NeuroPC "tick" using hints + OTA guard.
pub fn neuropc_tick(
    ctx: &AgentTurnContext,
    proposed_delta_risk: f32,
) {
    // 1) Evaluate any evolution step.
    let ota = evaluate_ota_change(ctx, proposed_delta_risk);
    if !ota.applied {
        // Log & quarantine the change; keep running with previous policy.
        log::warn!("OTA quarantined: {}", ota.reason);
    }

    // 2) Derive generation parameters from automagic hints.
    let gen = params_from_hints(ctx);

    // 3) Your agent uses `gen.max_tokens`, `gen.step_size`, `gen.verbosity`
    //    to shape how much it writes and how "heavy" it goes this turn.
    //    Pseudocode:
    // neuro_llm::respond_with_params(user_input, gen);
}
