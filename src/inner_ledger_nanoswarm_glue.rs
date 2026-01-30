use serde::{Deserialize, Serialize};

use crate::inner_ledger::{
    IdentityHeader,
    InnerLedger,
    InnerLedgerError,
    LedgerEvent,
};
use crate::personal_microspace::{
    HostId,
    MicrospaceScope,
    PersonalBiophysicalMicrospace,
    MicrospaceSafetyBands,
    SmartAutomationBudget,
};
use crate::nanoswarm_rights::{
    DemonstratedConsentShard,
    NanoswarmActionClass,
    NanoswarmGuardDecision,
    NanoswarmNeurorightsFloors,
    NanoswarmStateSnapshot,
    guard_nanoswarm_operation,
};
use crate::types::SystemAdjustment;
use crate::deepbrain_invariants::SovereigntyFlags;

/// Enumeration of nanoswarm-related runtime operations that can be
/// invoked via SystemAdjustment.payload or a side-channel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NanoswarmOperationKind {
    /// Low-risk maintenance (e.g., cleaning, mild sensing).
    Maintenance,
    /// Moderate interventions (e.g., tissue support, micro-repair).
    Support,
    /// High-risk modes (e.g., deep cleaning, microsurgery).
    HighRisk,
}

impl NanoswarmOperationKind {
    pub fn to_action_class(&self) -> NanoswarmActionClass {
        match self {
            NanoswarmOperationKind::Maintenance => NanoswarmActionClass::Maintenance,
            NanoswarmOperationKind::Support => NanoswarmActionClass::Support,
            NanoswarmOperationKind::HighRisk => NanoswarmActionClass::HighRisk,
        }
    }
}

/// Parameters for a proposed nanoswarm operation, derived from
/// higher-level planners or control policies.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NanoswarmOperationProposal {
    pub kind: NanoswarmOperationKind,
    /// SMART cost required for this operation.
    pub required_smart: f64,
    /// Proposed change in NANO fraction (relative to max).
    pub proposed_nano_delta_fraction: f64,
    /// Proposed eco-impact delta.
    pub proposed_eco_delta: f64,
}

/// Host-level configuration of nanoswarm neurorights floors.
/// This is typically static or updated only via governance events.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HostNanoswarmConfig {
    pub floors: NanoswarmNeurorightsFloors,
    pub microspace: PersonalBiophysicalMicrospace,
}

/// Extension trait for InnerLedger to apply nanoswarm operations
/// under sovereignty and neurorights guards.
pub trait NanoswarmLedgerExt {
    fn apply_nanoswarm_operation(
        &mut self,
        id_header: IdentityHeader,
        required_k: f32,
        sovereignty_flags: &SovereigntyFlags,
        host_nano_cfg: &mut HostNanoswarmConfig,
        op: NanoswarmOperationProposal,
        maybe_consent: Option<&DemonstratedConsentShard>,
        timestamp_utc: &str,
    ) -> Result<LedgerEvent, InnerLedgerError>;
}

impl NanoswarmLedgerExt for InnerLedger {
    fn apply_nanoswarm_operation(
        &mut self,
        id_header: IdentityHeader,
        required_k: f32,
        sovereignty_flags: &SovereigntyFlags,
        host_nano_cfg: &mut HostNanoswarmConfig,
        op: NanoswarmOperationProposal,
        maybe_consent: Option<&DemonstratedConsentShard>,
        timestamp_utc: &str,
    ) -> Result<LedgerEvent, InnerLedgerError> {
        // 1. Validate identity for inner ledger (existing auth path).
        crate::inner_ledger::validate_identity_for_inner_ledger(id_header, required_k)?;

        // 2. Assert sovereignty invariants (core must be enabled, rollback rules, etc.).
        sovereignty_flags
            .assert_invariants(&self.env)
            .map_err(|e| InnerLedgerError::SovereigntyViolation(e.to_string()))?;

        // 3. Construct a snapshot of current host state for nanoswarm guards.
        let state_snapshot = NanoswarmStateSnapshot {
            host_id: HostId(self.env.hostid.clone()),
            scope: host_nano_cfg.microspace.scope.clone(),
            current_blood: self.env.blood_min,   // or a richer lifeforce state if available
            current_oxygen: self.env.oxygen_min, // ditto
            current_nano_fraction: self.state.nano_fraction, // assuming such a field exists
            current_pain_level: self.state.pain_level,       // optional, if modeled
            current_eco_delta: self.state.eco_delta,         // optional, if modeled
        };

        let action_class = op.kind.to_action_class();

        // 4. Invoke nanoswarm rights guard.
        let decision = guard_nanoswarm_operation(
            &host_nano_cfg.floors,
            &host_nano_cfg.microspace,
            &state_snapshot,
            action_class,
            op.required_smart,
            op.proposed_nano_delta_fraction,
            op.proposed_eco_delta,
            maybe_consent,
        );

        match decision {
            NanoswarmGuardDecision::Rejected(reason) => {
                // Optionally: emit a dedicated audit event here.
                return Err(InnerLedgerError::NanoswarmGuardViolation(reason));
            }
            NanoswarmGuardDecision::Allowed => {
                // fall through
            }
        }

        // 5. At this point, nanoswarm operation is allowed under neurorights
        // and sovereignty floors. We now translate it into a SystemAdjustment
        // that will be processed by the existing lifeforce logic.
        let nanoswarm_adj = SystemAdjustment {
            delta_brain: 0.0,
            delta_wave: 0.0,
            delta_blood: 0.0,
            delta_oxygen: 0.0,
            delta_nano: op.proposed_nano_delta_fraction,
            delta_smart: -op.required_smart, // SMART debit
            eco_cost: op.proposed_eco_delta,
            kl_step: 0.0,        // no identity drift by default; can be set by higher-level logic
            risk_increment: 0.0, // or derived from op.kind if needed
        };

        // 6. Delegate to existing guarded adjustment path, which enforces
        // BRAIN/BLOOD/OXYGEN/NANO invariants and deep-brain identity drift.
        let event = self.system_apply_system_adjustment(
            nanoswarm_adj,
            timestamp_utc,
        )?;

        // 7. Update SMART accounting in the microspace.
        host_nano_cfg
            .microspace
            .apply_smart_debit(op.required_smart);

        Ok(event)
    }
}

/// Optional helper method on InnerLedger to keep the core adjustment
/// path encapsulated. This calls your existing logic that applies
/// SystemAdjustment to state + env, hashes, and returns a LedgerEvent.
impl InnerLedger {
    pub fn system_apply_system_adjustment(
        &mut self,
        adj: SystemAdjustment,
        timestamp_utc: &str,
    ) -> Result<LedgerEvent, InnerLedgerError> {
        // This function is a thin wrapper around whatever you already
        // use inside InnerLedger::system_apply for normal lifeforce
        // adjustments. The idea is to reuse the same invariants.

        // Example sketch; replace with your real implementation:
        crate::lifeforce_guards::apply_lifeforce_guarded_adjustment(
            &mut self.state,
            &self.env,
            adj,
        )?;

        // Construct LedgerEvent as usual (hashing, signatures, etc.).
        let event = LedgerEvent::new_from_state(
            &self.state,
            timestamp_utc,
        )?;

        Ok(event)
    }
}
