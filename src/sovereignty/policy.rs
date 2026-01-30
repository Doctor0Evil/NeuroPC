#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use crate::sovereignty::ota_io::{Caller, OtaAction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NrmlPolicy {
    pub owner_id: String,
    /// Trusted OTA signing keys, allowed modules, etc. (simplified for now).
    pub trusted_ota_sources: Vec<String>,
    /// Whether sovereignty core is reported enabled.
    pub sovereignty_core_enabled_flag: bool,
    /// Whether at least one rollback path is available.
    pub rollback_available_flag: bool,
    /// Whether user control channel is available.
    pub user_control_channel_flag: bool,
}

impl NrmlPolicy {
    pub fn evaluate_ota(&self, _caller: &Caller, action: &OtaAction) -> PolicyDecision {
        // Skeleton: only allow OTA actions from trusted sources.
        match action {
            OtaAction::Discover { source } | OtaAction::Download { source, .. } => {
                if self.trusted_ota_sources.contains(source) {
                    PolicyDecision {
                        allowed: true,
                        reason: format!("Source {} trusted by policy", source),
                    }
                } else {
                    PolicyDecision {
                        allowed: false,
                        reason: format!("Source {} not in trusted_ota_sources", source),
                    }
                }
            }
            _ => PolicyDecision {
                allowed: true,
                reason: "Non-network OTA action allowed by default".to_string(),
            },
        }
    }

    pub fn sovereignty_core_enabled(&self) -> bool {
        self.sovereignty_core_enabled_flag
    }

    pub fn rollback_path_available(&self) -> bool {
        self.rollback_available_flag
    }

    pub fn user_control_channel_available(&self) -> bool {
        self.user_control_channel_flag
    }

    /// For now, just return false; you will wire this to actual .cobj lookup.
    pub fn has_valid_ota_consent(&self, _caller: &Caller, _action: &OtaAction) -> bool {
        false
    }
}
