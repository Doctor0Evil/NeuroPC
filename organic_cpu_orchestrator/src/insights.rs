use serde::{Deserialize, Serialize};

/// High-level hints for AI chats: how much to lean in.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AutomagicHints {
    /// 0.0–1.0: how strongly to apply auto-completion / scaffolding.
    pub automagic_level: f32,
    /// Whether to suggest a rest / lighter topic.
    pub suggest_rest: bool,
    /// Optional human-readable hint for UI or prompt decoration.
    pub hint_text: String,
}

/// Map a decision + fatigue into chat-facing automagic hints.
pub fn derive_automagic_hints(
    decision: &str,
    fatigue_index: f32,
) -> AutomagicHints {
    match decision {
        "AllowFullAction" => AutomagicHints {
            automagic_level: 1.0,
            suggest_rest: fatigue_index > 0.8,
            hint_text: "High-capacity mode: dense help allowed.".to_string(),
        },
        "DegradePrecision" => AutomagicHints {
            automagic_level: 0.5,
            suggest_rest: fatigue_index > 0.7,
            hint_text: "Go shorter, slower, with more step-by-step scaffolding."
                .to_string(),
        },
        "PauseAndRest" => AutomagicHints {
            automagic_level: 0.2,
            suggest_rest: true,
            hint_text: "Suggest a break or low-strain, creative tasks."
                .to_string(),
        },
        _ => AutomagicHints {
            automagic_level: 0.7,
            suggest_rest: false,
            hint_text: "Unknown decision: use moderate assistance.".to_string(),
        },
    }
}
