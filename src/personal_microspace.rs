//! personal_microspace.rs
//! Sovereignty-first representation of a host-local biophysical microspace
//! and SMART-governed nanoswarm behaviors.

use serde::{Deserialize, Serialize};

/// Host-level identifier (e.g., ALN/Bostrom DID).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HostId(pub String);

/// Logical identifier for a nanoswarm controller operating in a host's microspace.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NanoswarmId(pub String);

/// Enum describing the scope of a nanoswarm operation.
/// Note: strictly host-local. No cross-host variants are allowed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MicrospaceScope {
    /// Operations constrained to within the physical body volume.
    IntraBody,
    /// Operations in a thin halo region around the body (e.g., skin-adjacent air).
    BodyHalo,
    /// Operations in a narrowly defined local environment (e.g., room-level),
    /// still under host sovereignty and eco constraints.
    LocalEnv,
}

/// Biophysical safety envelopes that must be respected by microspace operations.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MicrospaceSafetyBands {
    /// Minimum allowable BLOOD level for safe nanoswarm activity.
    pub blood_min: f64,
    /// Minimum allowable OXYGEN level for safe nanoswarm activity.
    pub oxygen_min: f64,
    /// NANO budget (fraction of max) that nanoswarm operations must not exceed.
    pub nano_max_fraction: f64,
    /// Eco impact corridor (delta must remain within [-eco_delta_max, +eco_delta_max]).
    pub eco_delta_max: f64,
    /// Pain envelope: max allowed normalized pain for nanoswarm operations.
    pub pain_envelope_max: f64,
}

/// SMART-budgeted automation allowance relevant to nanoswarm control.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmartAutomationBudget {
    /// Total SMART available for all automation in this epoch.
    pub smart_total: f64,
    /// SMART already consumed by nanoswarm operations in this epoch.
    pub smart_used_nano: f64,
    /// Maximum fraction of SMART that nanoswarm can consume.
    pub nano_smart_fraction_max: f64,
}

impl SmartAutomationBudget {
    pub fn remaining_for_nano(&self) -> f64 {
        let nano_cap = self.smart_total * self.nano_smart_fraction_max;
        (nano_cap - self.smart_used_nano).max(0.0)
    }
}

/// A host-local biophysical microspace under sovereign control.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersonalBiophysicalMicrospace {
    pub host_id: HostId,
    pub nanoswarm_id: NanoswarmId,
    pub scope: MicrospaceScope,
    pub safety_bands: MicrospaceSafetyBands,
    pub smart_budget: SmartAutomationBudget,
    /// Flag indicating whether autonomous nanoswarm operation is currently allowed.
    pub autonomous_enabled: bool,
}

impl PersonalBiophysicalMicrospace {
    /// Checks whether nanoswarm operations are allowed given current lifeforce
    /// and eco metrics plus an incremental cost proposal.
    pub fn can_apply_operation(
        &self,
        current_blood: f64,
        current_oxygen: f64,
        current_nano_fraction: f64,
        current_pain_level: f64,
        current_eco_delta: f64,
        proposed_nano_delta_fraction: f64,
        proposed_eco_delta: f64,
        required_smart: f64,
    ) -> Result<(), &'static str> {
        if !self.autonomous_enabled {
            return Err("Autonomous nanoswarm operation disabled");
        }

        // Lifeforce minima invariant.
        if current_blood < self.safety_bands.blood_min {
            return Err("BLOOD below nanoswarm safety minimum");
        }
        if current_oxygen < self.safety_bands.oxygen_min {
            return Err("OXYGEN below nanoswarm safety minimum");
        }

        // Pain envelope invariant.
        if current_pain_level > self.safety_bands.pain_envelope_max {
            return Err("Pain envelope exceeded for nanoswarm operation");
        }

        // NANO budget invariant.
        let new_nano_fraction = current_nano_fraction + proposed_nano_delta_fraction;
        if new_nano_fraction > self.safety_bands.nano_max_fraction {
            return Err("NANO fraction would exceed microspace budget");
        }

        // Eco corridor invariant (absolute delta bound).
        let new_eco_delta = current_eco_delta + proposed_eco_delta;
        if new_eco_delta.abs() > self.safety_bands.eco_delta_max {
            return Err("Eco impact delta would leave allowed corridor");
        }

        // SMART-governed automation invariant.
        let remaining_smart = self.smart_budget.remaining_for_nano();
        if required_smart > remaining_smart {
            return Err("Insufficient SMART budget for nanoswarm operation");
        }

        Ok(())
    }

    /// Applies SMART accounting after a nanoswarm operation has been accepted.
    pub fn apply_smart_debit(&mut self, smart_cost: f64) {
        self.smart_budget.smart_used_nano += smart_cost.max(0.0);
    }
}
