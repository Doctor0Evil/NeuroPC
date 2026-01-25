use neuro_automagic_core::{
    NeuroIntent,
    NeuroIntentKind,
    NeuroContext,
    heapless_string::HeaplessString,
};
use crate::event::NeuroAutomationEvent;
use crate::NeuroAutomagicEngine;

/// Canonical representation of a cargo instruction.
#[derive(Clone, Debug)]
pub struct CargoInstruction {
    pub command: String,
    pub args: Vec<String>,
}

pub struct CargoIntegrator<'a> {
    engine: &'a NeuroAutomagicEngine,
}

impl<'a> CargoIntegrator<'a> {
    pub fn new(engine: &'a NeuroAutomagicEngine) -> Self {
        CargoIntegrator { engine }
    }

    /// Convert a NeuroIntent into a canonical cargo instruction (if applicable).
    pub fn intent_to_cargo(&self, intent: &NeuroIntent) -> Option<CargoInstruction> {
        match &intent.kind {
            NeuroIntentKind::ExecuteCommand { command } => {
                // Very conservative: only map when the command starts with "cargo ".
                let cmd_str = command.to_string();
                if let Some(rest) = cmd_str.strip_prefix("cargo ") {
                    let mut parts = rest.split_whitespace();
                    let subcommand = parts.next().unwrap_or("build").to_string();
                    let args: Vec<String> = parts.map(|s| s.to_string()).collect();

                    let mut all_args = Vec::new();
                    all_args.push(subcommand);
                    all_args.extend(args);

                    Some(CargoInstruction {
                        command: "cargo".to_string(),
                        args: all_args,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Example: build an event for OnComplexSequenceDetected when a cargo
    /// command appears long or nested.
    pub fn build_complexity_event(&self, intent: NeuroIntent) -> NeuroAutomationEvent {
        let token_count = match &intent.kind {
            NeuroIntentKind::ExecuteCommand { command } => {
                let s = command.to_string();
                s.split_whitespace().count() as u32
            }
            _ => 0,
        };

        NeuroAutomationEvent {
            trigger: neuro_automagic_core::NeuroAutomationTrigger::OnComplexSequenceDetected,
            intent,
            repetition: None,
            complexity: Some(crate::ComplexityStats {
                token_count,
                nested_level: 0,
            }),
        }
    }
}
