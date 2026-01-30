#![forbid(unsafe_code)]

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::sovereignty::consent::{ConsentStore, RequiredToken};
use crate::sovereignty::ota_io::{Caller, OtaAction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NrmlPolicy {
    pub owner_id: String,
    pub trusted_ota_sources: Vec<String>,
    pub sovereignty_core_enabled_flag: bool,
    pub rollback_available_flag: bool,
    pub user_control_channel_flag: bool,
    /// Directory where .cobj files live.
    pub consent_dir: PathBuf,
}

impl NrmlPolicy {
    pub fn evaluate_ota(&self, _caller: &Caller, action: &OtaAction) -> PolicyDecision {
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

    /// OTA Commit/Rollback consent: requires a matching .cobj.
    pub fn has_valid_ota_consent(&self, caller: &Caller, action: &OtaAction) -> bool {
        let (required_token, target_id) = match action {
            OtaAction::Discover { .. } | OtaAction::Download { .. } => {
                (RequiredToken::Any, caller.module_id.as_str())
            }
            OtaAction::Verify { package_hash } => {
                (RequiredToken::Any, package_hash.as_str())
            }
            OtaAction::Stage { package_hash } => {
                (RequiredToken::Any, package_hash.as_str())
            }
            OtaAction::Commit { package_hash } => {
                (RequiredToken::Commit, package_hash.as_str())
            }
            OtaAction::Rollback { target_version } => {
                (RequiredToken::Commit, target_version.as_str())
            }
        };

        let store = ConsentStore::new(&self.consent_dir);
        let now = Utc::now();
        match store.find_valid_for(&self.owner_id, target_id, required_token, now) {
            Ok(Some(_)) => true,
            _ => false,
        }
    }

    /// Evolution consent: e.g., OS / model / BrainFunction evolution.
    /// evolve_target_id should be a stable identifier (e.g., "bf_ota_core").
    pub fn has_valid_evolve_consent(&self, evolve_target_id: &str) -> bool {
        let store = ConsentStore::new(&self.consent_dir);
        let now = Utc::now();
        match store.find_valid_for(
            &self.owner_id,
            evolve_target_id,
            RequiredToken::Evolve,
            now,
        ) {
            Ok(Some(_)) => true,
            _ => false,
        }
    }
}
