use serde::{Deserialize, Serialize};

use crate::sovereignty_core::NeurorightsPolicyDocument;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SovereignMode {
    CONSERVATIVE,
    CO_PILOT,
    AUTO_EVOLVE,
}

pub struct SovereignControls<'a> {
    policy: &'a mut NeurorightsPolicyDocument,
}

impl<'a> SovereignControls<'a> {
    pub fn new(policy: &'a mut NeurorightsPolicyDocument) -> Self {
        Self { policy }
    }

    pub fn set_mode(&mut self, mode: SovereignMode) {
        self.policy.active_mode = match mode {
            SovereignMode::CONSERVATIVE => "CONSERVATIVE".to_string(),
            SovereignMode::CO_PILOT => "CO_PILOT".to_string(),
            SovereignMode::AUTO_EVOLVE => "AUTO_EVOLVE".to_string(),
        };
    }

    pub fn active_mode(&self) -> &str {
        &self.policy.active_mode
    }

    pub fn describe_active_mode(&self) -> String {
        let mode = &self.policy.active_mode;
        match mode.as_str() {
            "CONSERVATIVE" => "CONSERVATIVE: AI acts only when explicitly asked; no automatic evolution."
                .to_string(),
            "AUTO_EVOLVE" => "AUTO_EVOLVE: AI may adapt within hard EVOLVE-gated bounds; major changes still require explicit approval."
                .to_string(),
            _ => "CO_PILOT: AI suggests but does not apply changes without your consent.".to_string(),
        }
    }
}
