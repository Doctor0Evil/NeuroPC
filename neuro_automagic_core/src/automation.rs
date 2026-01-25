use crate::model::{NeuroIntent, NeuroCitizen};
use crate::right::{NeuroRight, NeuroRightSet, NeuroAccessDecision, check_access};

/// Triggers for automagic assistance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NeuroAutomationTrigger {
    OnFatigue,
    OnRepetition,
    OnComplexSequenceDetected,
}

/// Actions the system can perform automatically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NeuroAutomationAction {
    SuggestMacroExpansion,
    SuggestHigherLevelCommand,
    AutoFillCommandTemplate,
}

/// Automation rule binding trigger and action, with required rights.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeuroAutomationRule {
    pub name: crate::model::heapless_string::HeaplessString,
    pub trigger: NeuroAutomationTrigger,
    pub action: NeuroAutomationAction,
    pub required_right: NeuroRight,
}

impl NeuroAutomationRule {
    pub fn can_fire_for(
        &self,
        citizen: &NeuroCitizen,
        rights: &NeuroRightSet,
        _intent: &NeuroIntent,
    ) -> NeuroAccessDecision {
        check_access(citizen, rights, self.required_right)
    }
}
