use alloc::string::String;
use thiserror::Error;
use neurorights_core::NeurorightsViolation;

#[derive(Debug, Error)]
pub enum AssistantAdapterError {
    #[error("neurorights violation: {0}")]
    Neurorights(String),

    #[error("backend error: {0}")]
    Backend(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("serialization error: {0}")]
    Serde(String),

    #[error("domain not allowed for this adapter: {0}")]
    DomainNotAllowed(String),

    #[error("internal config error: {0}")]
    Config(String),
}

impl From<NeurorightsViolation> for AssistantAdapterError {
    fn from(v: NeurorightsViolation) -> Self {
        AssistantAdapterError::Neurorights(v.to_string())
    }
}
