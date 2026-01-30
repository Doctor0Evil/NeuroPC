#![forbid(unsafe_code)]

use crate::sovereignty::ota_io::{Caller, OtaAction, SovereignOtaIo};
use crate::sovereignty::policy::NrmlPolicy;

/// High-level OTA controller entrypoints.
pub struct OtaController<'a> {
    policy: &'a NrmlPolicy,
}

impl<'a> OtaController<'a> {
    pub fn new(policy: &'a NrmlPolicy) -> Self {
        Self { policy }
    }

    /// Commit a specific OTA package; must have explicit consent.
    pub fn commit_package(
        &self,
        io: &mut SovereignOtaIo,
        caller: &Caller,
        package_hash: &str,
    ) -> Result<(), String> {
        let action = OtaAction::Commit {
            package_hash: package_hash.to_string(),
        };

        let decision = io.request(caller, &action);
        if !decision.allowed {
            return Err(format!(
                "OTA Commit denied: {} (invariants: {:?})",
                decision.reason, decision.invariants
            ));
        }

        // At this point, SovereignOtaIo has:
        // - enforced NrmlPolicy evaluate_ota
        // - enforced has_valid_ota_consent
        // - logged the request and invariant status
        //
        // You can now call lower-level install logic knowing neurorights constraints were honored.
        // install_package(package_hash)?;

        Ok(())
    }

    /// Rollback to a specific version; must have explicit consent.
    pub fn rollback_to(
        &self,
        io: &mut SovereignOtaIo,
        caller: &Caller,
        target_version: &str,
    ) -> Result<(), String> {
        let action = OtaAction::Rollback {
            target_version: target_version.to_string(),
        };

        let decision = io.request(caller, &action);
        if !decision.allowed {
            return Err(format!(
                "OTA Rollback denied: {} (invariants: {:?})",
                decision.reason, decision.invariants
            ));
        }

        // Now safe to call rollback logic.
        // perform_rollback(target_version)?;

        Ok(())
    }
}
