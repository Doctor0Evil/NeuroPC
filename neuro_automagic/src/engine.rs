use neuro_automagic_core::{
    NeuroAutomationRule,
    NeuroAutomationAction,
    NeuroRightSet,
    NeuroCitizen,
    NeuroIntent,
    NeuroAccessDecision,
};
use crate::event::NeuroAutomationEvent;

/// Configuration for the engine, including globally enabled rules.
#[derive(Clone, Debug)]
pub struct EngineConfig {
    pub rules: alloc::vec::Vec<NeuroAutomationRule>,
}

/// Automagic engine: evaluates events and returns suggested actions.
pub struct NeuroAutomagicEngine {
    config: EngineConfig,
}

impl NeuroAutomagicEngine {
    pub fn new(config: EngineConfig) -> Self {
        NeuroAutomagicEngine { config }
    }

    /// Process an event and return a list of suggested actions.
    pub fn process_event(
        &self,
        citizen: &NeuroCitizen,
        rights: &NeuroRightSet,
        event: &NeuroAutomationEvent,
    ) -> alloc::vec::Vec<NeuroAutomationAction> {
        let mut actions = alloc::vec::Vec::new();

        for rule in &self.config.rules {
            if rule.trigger != event.trigger {
                continue;
            }

            let decision = rule.can_fire_for(citizen, rights, &event.intent);
            if decision == NeuroAccessDecision::Allowed {
                actions.push(rule.action.clone());
            }
        }

        actions
    }
}

// Minimal std/alloc glue for this crate.
extern crate alloc;
