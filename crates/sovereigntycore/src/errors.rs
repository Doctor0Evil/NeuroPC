use serde::{Deserialize, Serialize};

pub const NEUROMORPH_ACTUATION_FORBIDDEN: &str = crate::kernel_spec::NEUROMORPH_ACTUATION_FORBIDDEN;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SovereigntyErrorCode {
    RoHLimitExceeded,
    RoHNonMonotone,
    NeurorightsForbidden,
    StakeMissing,
    TokenInvalid,
    /// Hard non-actuation invariant for neuromorph scopes.
    NeuromorphActuationForbidden,
}

#[derive(thiserror::Error, Debug)]
pub enum SovereigntyError {
    #[error("RoH invariant violated: {0}")]
    RoHInvariant(String),
    #[error("Neurorights violation: {0}")]
    Neurorights(String),
    #[error("Actuation forbidden in neuromorph scope: {0}")]
    NeuromorphActuation(String),
}

impl SovereigntyError {
    pub fn reason_code(&self) -> SovereigntyErrorCode {
        match self {
            SovereigntyError::RoHInvariant(_) => SovereigntyErrorCode::RoHNonMonotone,
            SovereigntyError::Neurorights(_) => SovereigntyErrorCode::NeurorightsForbidden,
            SovereigntyError::NeuromorphActuation(_) => SovereigntyErrorCode::NeuromorphActuationForbidden,
        }
    }
}
