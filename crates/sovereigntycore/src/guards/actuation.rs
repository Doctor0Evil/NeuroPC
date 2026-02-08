use crate::errors::{SovereigntyError, NEUROMORPH_ACTUATION_FORBIDDEN};

/// Every syscall or device-API hook must call this before actuating.
pub fn enforce_neuromorph_non_actuation(scope: &str, requested_action: &str) -> Result<(), SovereigntyError> {
    if scope == "neuromorph" {
        Err(SovereigntyError::NeuromorphActuation(format!(
            "{}: attempted {:?} in neuromorph scope",
            NEUROMORPH_ACTUATION_FORBIDDEN, requested_action
        )))
    } else {
        Ok(())
    }
}
