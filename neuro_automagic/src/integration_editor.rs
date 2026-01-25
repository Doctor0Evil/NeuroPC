use neuro_automagic_core::{
    NeuroIntent,
    NeuroIntentKind,
};
use crate::event::NeuroAutomationEvent;
use crate::NeuroAutomagicEngine;
use crate::RepetitionStats;

/// Editor-level actions the plugin can execute.
#[derive(Clone, Debug)]
pub enum EditorAction {
    InsertSnippet { snippet: String },
    ShowCommandPaletteSuggestion { label: String, command: String },
    ShowInlineHint { message: String },
}

/// Integrator between the automagic engine and an editor environment.
pub struct EditorIntegrator<'a> {
    engine: &'a NeuroAutomagicEngine,
}

impl<'a> EditorIntegrator<'a> {
    pub fn new(engine: &'a NeuroAutomagicEngine) -> Self {
        EditorIntegrator { engine }
    }

    /// Example: build a repetition event when the same pattern appears.
    pub fn build_repetition_event(
        &self,
        intent: NeuroIntent,
        recent_count: u32,
    ) -> NeuroAutomationEvent {
        NeuroAutomationEvent {
            trigger: neuro_automagic_core::NeuroAutomationTrigger::OnRepetition,
            intent,
            repetition: Some(RepetitionStats { recent_count }),
            complexity: None,
        }
    }

    /// Translate automagic actions into editor actions.
    pub fn actions_to_editor(
        &self,
        actions: &[neuro_automagic_core::NeuroAutomationAction],
        intent: &NeuroIntent,
    ) -> Vec<EditorAction> {
        let mut result = Vec::new();

        for action in actions {
            match action {
                neuro_automagic_core::NeuroAutomationAction::SuggestMacroExpansion => {
                    if let NeuroIntentKind::ExecuteCommand { command } = &intent.kind {
                        result.push(EditorAction::ShowInlineHint {
                            message: format!("Consider creating a macro for: {}", command.to_string()),
                        });
                    }
                }
                neuro_automagic_core::NeuroAutomationAction::SuggestHigherLevelCommand => {
                    result.push(EditorAction::ShowCommandPaletteSuggestion {
                        label: "Use higher-level NeuroPC task".to_string(),
                        command: "neuropc.runTask".to_string(),
                    });
                }
                neuro_automagic_core::NeuroAutomationAction::AutoFillCommandTemplate => {
                    if let NeuroIntentKind::ExecuteCommand { .. } = &intent.kind {
                        result.push(EditorAction::InsertSnippet {
                            snippet: "cargo run --package <pkg> --bin <bin>".to_string(),
                        });
                    }
                }
            }
        }

        result
    }
}
