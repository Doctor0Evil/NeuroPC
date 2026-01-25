use crate::model::NeuroCitizen;
use core::fmt;

/// Fine-grained rights flags.
/// These are designed to *never* exclude the primary augmented-citizen.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NeuroRight {
    ReadProject,
    WriteProject,
    ExecuteCommands,
    ManageAutomationRules,
}

/// A set of rights for a given citizen.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeuroRightSet {
    pub rights: heapless::Vec<NeuroRight, 16>,
}

impl NeuroRightSet {
    pub fn new() -> Self {
        NeuroRightSet {
            rights: heapless::Vec::new(),
        }
    }

    pub fn grant(&mut self, right: NeuroRight) {
        if !self.rights.contains(&right) {
            let _ = self.rights.push(right);
        }
    }

    pub fn has(&self, right: NeuroRight) -> bool {
        self.rights.contains(&right)
    }
}

/// Result of an access-control check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NeuroAccessDecision {
    Allowed,
    Denied,
}

/// Access-control helper that ensures the primary augmented-citizen
/// is always included (cannot be locked out).
pub fn check_access(citizen: &NeuroCitizen, rights: &NeuroRightSet, required: NeuroRight) -> NeuroAccessDecision {
    if citizen.is_primary_augmented_citizen {
        return NeuroAccessDecision::Allowed;
    }

    if rights.has(required) {
        NeuroAccessDecision::Allowed
    } else {
        NeuroAccessDecision::Denied
    }
}

/// Minimal heapless dependency.
/// In a real project, this can be re-exported or replaced as needed.
mod heapless {
    pub use heapless::Vec;
}
