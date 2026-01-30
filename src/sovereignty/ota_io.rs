#![forbid(unsafe_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::sovereignty::invariants::{InvariantStatus, SovereigntyInvariant};
use crate::sovereignty::policy::NrmlPolicy;
use crate::sovereignty::audit::{AuditEvent, AuditEventKind, AuditLogger};

/// Who is asking (e.g., "bf_ota_core", "module_xyz").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Caller {
    pub module_id: String,
    pub instance_id: Option<String>,
}

/// OTA-specific actions that require policy evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OtaAction {
    Discover { source: String },
    Download { source: String, package_hash: String },
    Verify { package_hash: String },
    Stage { package_hash: String },
    Commit { package_hash: String },
    Rollback { target_version: String },
}

/// Result of a policy-enforced OTA I/O request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtaDecision {
    pub allowed: bool,
    pub reason: String,
    pub invariants: Vec<InvariantStatus>,
    pub timestamp: DateTime<Utc>,
}

/// Sovereign I/O façade.
pub struct SovereignOtaIo<'a> {
    policy: &'a NrmlPolicy,
    logger: &'a mut AuditLogger,
}

impl<'a> SovereignOtaIo<'a> {
    pub fn new(policy: &'a NrmlPolicy, logger: &'a mut AuditLogger) -> Self {
        Self { policy, logger }
    }

    /// Main entry point: evaluate + (if allowed) perform the action.
    /// In this short-term skeleton, we only decide + log; actual fs/net can be added behind
    /// additional, tightly-controlled adapters.
    pub fn request(&mut self, caller: &Caller, action: &OtaAction) -> OtaDecision {
        // 1. Evaluate policy.
        let policy_decision = self.policy.evaluate_ota(caller, action);

        // 2. Check invariants that MUST hold before any OTA side effect.
        let mut inv_status = Vec::new();

        let sov_ok = self.policy.sovereignty_core_enabled();
        inv_status.push(if sov_ok {
            InvariantStatus::ok(SovereigntyInvariant::InvSovCoreEnabled)
        } else {
            InvariantStatus::fail(
                SovereigntyInvariant::InvSovCoreEnabled,
                "Sovereignty core not reported enabled by policy",
            )
        });

        let rb_ok = self.policy.rollback_path_available();
        inv_status.push(if rb_ok {
            InvariantStatus::ok(SovereigntyInvariant::InvRollbackReachable)
        } else {
            InvariantStatus::fail(
                SovereigntyInvariant::InvRollbackReachable,
                "No rollback path reported available",
            )
        });

        let user_ch_ok = self.policy.user_control_channel_available();
        inv_status.push(if user_ch_ok {
            InvariantStatus::ok(SovereigntyInvariant::InvUserControlChannelAvailable)
        } else {
            InvariantStatus::fail(
                SovereigntyInvariant::InvUserControlChannelAvailable,
                "User control channel not available",
            )
        });

        // 3. If this is a Commit or Rollback, enforce explicit consent invariant.
        if matches!(action, OtaAction::Commit { .. } | OtaAction::Rollback { .. }) {
            let has_consent = self.policy.has_valid_ota_consent(caller, action);
            inv_status.push(if has_consent {
                InvariantStatus::ok(SovereigntyInvariant::InvNoOtaWithoutExplicitConsent)
            } else {
                InvariantStatus::fail(
                    SovereigntyInvariant::InvNoOtaWithoutExplicitConsent,
                    "Missing or invalid explicit ConsentObject for OTA Commit/Rollback",
                )
            });
        }

        let all_invariants_ok = inv_status.iter().all(|s| s.satisfied);
        let allowed = policy_decision.allowed && all_invariants_ok;

        // 4. Log the decision (no secrets, only metadata + hashes).
        let event = AuditEvent {
            timestamp: Utc::now(),
            caller_module: caller.module_id.clone(),
            caller_instance: caller.instance_id.clone(),
            kind: AuditEventKind::OtaRequest,
            action: serde_json::to_value(action).unwrap_or_default(),
            policy_decision: serde_json::to_value(&policy_decision).unwrap_or_default(),
            invariants: inv_status.clone(),
        };
        let _ = self.logger.append(event);

        OtaDecision {
            allowed,
            reason: if allowed {
                policy_decision.reason
            } else {
                format!("Denied: {}", policy_decision.reason)
            },
            invariants: inv_status,
            timestamp: Utc::now(),
        }
    }
}
