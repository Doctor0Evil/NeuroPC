use serde::{Deserialize, Serialize};

/// NEUROMORPH_ACTUATION_FORBIDDEN reason code is a concrete string, not a label.
pub const NEUROMORPH_ACTUATION_FORBIDDEN: &str = "NEUROMORPH_ACTUATION_FORBIDDEN";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohModelSpec {
    pub id: String,
    pub global_ceiling: f32,
}

impl RohModelSpec {
    pub fn enforce_invariant(&self, roh_before: f32, roh_after: f32) -> Result<(), String> {
        if roh_after > roh_before {
            return Err(format!(
                "RoH monotonicity violated: roh_after {} > roh_before {}",
                roh_after, roh_before
            ));
        }
        if roh_after > self.global_ceiling {
            return Err(format!(
                "RoH ceiling violated: roh_after {} > ceiling {}",
                roh_after, self.global_ceiling
            ));
        }
        Ok(())
    }
}

/// Marker trait for envelopes that can never actuate.
pub trait NoActuation {}

/// Neuromorph envelopes must implement NoActuation at the type level.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuromorphEnvelope {
    pub subject_id: String,
    pub tsafe_mode: String,
    pub roh_before: f32,
    pub roh_after: f32,
}

impl NoActuation for NeuromorphEnvelope {}

impl NeuromorphEnvelope {
    pub fn validate_monotone(&self, roh_spec: &RohModelSpec) -> Result<(), String> {
        roh_spec.enforce_invariant(self.roh_before, self.roh_after)
    }
}
