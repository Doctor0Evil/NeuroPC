use core::time::Duration;
use serde::{Serialize, Deserialize};

use crate::model::CerebralSample;
use neuroautomagiccore::{NeuroCitizen, NeuroContext};

/// Declares what a given adapter is allowed to see / emit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterCapabilities {
    pub can_read_surface_signals: bool,
    pub can_emit_neurovascular_metrics: bool,
    pub suggest_only: bool, // never actuate
}

/// Host abstraction for BCI/HCI streams.
pub trait CerebralAdapter {
    /// Static description for capability guards and neurorights checks.
    fn capabilities(&self) -> AdapterCapabilities;

    /// Attach to device / stream; host decides actual sampling period.
    fn start(
        &mut self,
        citizen: &NeuroCitizen,
        context: &NeuroContext,
        sample_period: Duration,
    ) -> Result<(), AdapterError>;

    /// Pull the next normalized sample: virtual & neurovascular objects + intent.
    fn next_sample(&mut self) -> Result<Option<CerebralSample>, AdapterError>;
}

#[derive(thiserror::Error, Debug)]
pub enum AdapterError {
    #[error("device unavailable")]
    DeviceUnavailable,
    #[error("permissions / neurorights violation: {0}")]
    RightsViolation(&'static str),
    #[error("transport error")]
    Transport,
}
