use neuro_automagic_core::{
    NeuroCitizen,
    NeuroContext,
    NeuroPlatform,
    NeuroIntent,
    NeuroIntentKind,
    model::heapless_string::HeaplessString,
};
use crate::{NeuroAutomagicEngine, EngineConfig, CargoIntegrator, EditorIntegrator};
use crate::event::NeuroAutomationEvent;
use crate::RepetitionStats;

/// Represents a running NeuroPC + Cargo + editor session for the primary citizen.
pub struct NeuroPcCargoSession {
    pub citizen: NeuroCitizen,
    pub context: NeuroContext,
    pub engine: NeuroAutomagicEngine,
    pub rights: neuro_automagic_core::NeuroRightSet,
}

impl NeuroPcCargoSession {
    pub fn new_default(project_name: &str, project_root: &str) -> Self {
        let (config, rights, citizen) = EngineConfig::neuro_pc_cargo_default();

        let context = NeuroContext {
            platform: NeuroPlatform::NeuroPcUserland,
            project: Some(neuro_automagic_core::NeuroProjectScope {
                name: HeaplessString::from_str(project_name),
                root_path: HeaplessString::from_str(project_root),
            }),
            file: None,
        };

        let engine = NeuroAutomagicEngine::new(config);

        NeuroPcCargoSession {
            citizen,
            context,
            engine,
            rights,
        }
    }

    /// Short helper: create an intent from a raw cargo-like command.
    pub fn intent_from_command(&self, raw: &str) -> NeuroIntent {
        NeuroIntent {
            citizen: self.citizen.clone(),
            context: self.context.clone(),
            kind: NeuroIntentKind::ExecuteCommand {
                command: HeaplessString::from_str(raw),
            },
            raw_input: HeaplessString::from_str(raw),
        }
    }

    /// Process a repeated command pattern, returning editor actions if automagic triggers fire.
    pub fn handle_repetition(
        &self,
        raw_command: &str,
        recent_count: u32,
    ) -> Vec<crate::integration_editor::EditorAction> {
        let intent = self.intent_from_command(raw_command);

        let event = NeuroAutomationEvent {
            trigger: neuro_automagic_core::NeuroAutomationTrigger::OnRepetition,
            intent: intent.clone(),
            repetition: Some(RepetitionStats { recent_count }),
            complexity: None,
        };

        let actions = self.engine.process_event(&self.citizen, &self.rights, &event);
        let editor_integrator = crate::integration_editor::EditorIntegrator::new(&self.engine);

        editor_integrator.actions_to_editor(&actions, &intent)
    }

    /// Process a complex command sequence and return a CargoInstruction plus any editor actions.
    pub fn handle_complex_command(
        &self,
        raw_command: &str,
    ) -> (Option<crate::integration_cargo::CargoInstruction>, Vec<crate::integration_editor::EditorAction>) {
        let intent = self.intent_from_command(raw_command);

        let cargo_integrator = crate::integration_cargo::CargoIntegrator::new(&self.engine);
        let cargo_instruction = cargo_integrator.intent_to_cargo(&intent);

        let complexity_event = cargo_integrator.build_complexity_event(intent.clone());

        let actions = self.engine.process_event(&self.citizen, &self.rights, &complexity_event);
        let editor_integrator = crate::integration_editor::EditorIntegrator::new(&self.engine);

        let editor_actions = editor_integrator.actions_to_editor(&actions, &intent);

        (cargo_instruction, editor_actions)
    }
}
