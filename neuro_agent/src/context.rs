use serde::{Deserialize, Serialize};

/// Imported from Reality.os module, duplicated here as JSON shape for agents.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealityHints {
    pub automagic_level: f32, // 0.0–1.0
    pub suggest_rest: bool,
    pub note: String,
}

/// Agent-facing context for each turn.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentTurnContext {
    pub session_tag: String,
    pub reality_hints: RealityHints,
    /// True = no experimental / OTA changes allowed this turn.
    pub safe_mode: bool,
}

/// Result of an attempted OTA evolution step.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OtaResult {
    pub applied: bool,
    pub reason: String,       // why applied or why quarantined
}

/// Simple OTA guard: only allow safer-only changes when safe_mode is true.
pub fn evaluate_ota_change(
    ctx: &AgentTurnContext,
    proposed_delta_risk: f32, // <0 safer, >0 riskier
) -> OtaResult {
    if ctx.safe_mode && proposed_delta_risk > 0.0 {
        OtaResult {
            applied: false,
            reason: "CHCIL: safer-only in safe_mode; risk-increasing OTA quarantined."
                .to_string(),
        }
    } else {
        OtaResult {
            applied: true,
            reason: "OTA accepted under current safety policy.".to_string(),
        }
    }
}
