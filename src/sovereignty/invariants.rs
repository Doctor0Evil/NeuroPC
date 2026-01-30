#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Enumerated invariants (stable names for specs, logs, and checkers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SovereigntyInvariant {
    /// Sovereignty core module is always enabled.
    InvSovCoreEnabled,
    /// At least one rollback path is always reachable.
    InvRollbackReachable,
    /// User control channels (stop/undo/lock OTA) are always available.
    InvUserControlChannelAvailable,
    /// No OTA commit without explicit, valid ConsentObject.
    InvNoOtaWithoutExplicitConsent,
}

/// Current invariant status, for runtime checks and audit logs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantStatus {
    pub invariant: SovereigntyInvariant,
    pub satisfied: bool,
    pub details: Option<String>,
}

impl InvariantStatus {
    pub fn ok(invariant: SovereigntyInvariant) -> Self {
        Self { invariant, satisfied: true, details: None }
    }

    pub fn fail(invariant: SovereigntyInvariant, msg: impl Into<String>) -> Self {
        Self { invariant, satisfied: false, details: Some(msg.into()) }
    }
}
