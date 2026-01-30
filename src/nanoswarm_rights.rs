//! nanoswarm_rights.rs
//! Rights and guards for SMART-governed, host-local nanoswarm behavior,
//! aligned with neurorights, self-only doctrine, and eco constraints.

use serde::{Deserialize, Serialize};

use crate::personal_microspace::{HostId, MicrospaceScope, PersonalBiophysicalMicrospace};

/// High-level classification of nanoswarm actions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NanoswarmActionClass {
    /// Low-risk maintenance (cleaning, debris removal, mild sensing).
    Maintenance,
    /// Moderate interventions (tissue support, micro-repair within safe bands).
    Support,
    /// High-risk modes (deep cleaning, microsurgery, intensive neuromorphic sensing).
    HighRisk,
}

/// Consent token bound to the host DID and a specific nanoswarm action class.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DemonstratedConsentShard {
    pub host_id: HostId,
    pub action_class: NanoswarmActionClass,
    pub timestamp_utc: String,
    /// Optional expiry in UTC (ISO-8601); empty string means no explicit expiry.
    pub expires_utc: String,
}

/// Runtime state snapshot used for rights checks.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NanoswarmStateSnapshot {
    pub host_id: HostId,
    pub scope: MicrospaceScope,
    pub current_blood: f64,
    pub current_oxygen: f64,
    pub current_nano_fraction: f64,
    pub current_pain_level: f64,
    pub current_eco_delta: f64,
}

/// Rights-enforcing guard results.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NanoswarmGuardDecision {
    Allowed,
    Rejected(String),
}

/// Static configuration of neurorights floors for nanoswarm behaviors.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NanoswarmNeurorightsFloors {
    /// Prohibits cross-host lifeforce effects by doctrine.
    pub forbid_cross_host_effects: bool,
    /// Require explicit consent shards for high-risk actions.
    pub require_consent_high_risk: bool,
    /// Require SMART-governed automation (no platform override).
    pub require_smart_governance: bool,
}

/// Utility: check if a consent shard matches host and action class.
fn consent_matches(
    host_id: &HostId,
    action_class: &NanoswarmActionClass,
    shard: &DemonstratedConsentShard,
) -> bool {
    &shard.host_id == host_id && &shard.action_class == action_class
}

/// Central guard for nanoswarm rights.
/// This should be called from your inner-ledger system_apply path
/// before any nanoswarm-related state transitions.
pub fn guard_nanoswarm_operation(
    floors: &NanoswarmNeurorightsFloors,
    microspace: &PersonalBiophysicalMicrospace,
    state: &NanoswarmStateSnapshot,
    action_class: NanoswarmActionClass,
    required_smart: f64,
    proposed_nano_delta_fraction: f64,
    proposed_eco_delta: f64,
    maybe_consent: Option<&DemonstratedConsentShard>,
) -> NanoswarmGuardDecision {
    // Self-only doctrine: host IDs must match.
    if state.host_id != microspace.host_id {
        return NanoswarmGuardDecision::Rejected(
            "Cross-host nanoswarm operation prohibited by self-only doctrine",
        );
    }

    // Cross-host effects forbidden: microspace scope must be host-local.
    if floors.forbid_cross_host_effects {
        match state.scope {
            MicrospaceScope::IntraBody | MicrospaceScope::BodyHalo | MicrospaceScope::LocalEnv => {
                // all allowed scopes are host-local by construction
            }
        }
    }

    // High-risk actions require explicit consent shard if configured.
    if floors.require_consent_high_risk {
        if matches!(action_class, NanoswarmActionClass::HighRisk) {
            match maybe_consent {
                None => {
                    return NanoswarmGuardDecision::Rejected(
                        "Missing DemonstratedConsentShard for high-risk nanoswarm action",
                    )
                }
                Some(shard) => {
                    if !consent_matches(&microspace.host_id, &action_class, shard) {
                        return NanoswarmGuardDecision::Rejected(
                            "Consent shard does not match host or action class",
                        );
                    }
                }
            }
        }
    }

    // SMART governance invariant is enforced by using PersonalBiophysicalMicrospace
    // as the authority for SMART budgeting.
    if floors.require_smart_governance && !microspace.autonomous_enabled {
        return NanoswarmGuardDecision::Rejected(
            "SMART-governed autonomous nanoswarm operation is disabled",
        );
    }

    // Delegate lifeforce, pain, eco, and SMART checks to microspace.
    if let Err(err) = microspace.can_apply_operation(
        state.current_blood,
        state.current_oxygen,
        state.current_nano_fraction,
        state.current_pain_level,
        state.current_eco_delta,
        proposed_nano_delta_fraction,
        proposed_eco_delta,
        required_smart,
    ) {
        return NanoswarmGuardDecision::Rejected(err.to_string());
    }

    NanoswarmGuardDecision::Allowed
}
