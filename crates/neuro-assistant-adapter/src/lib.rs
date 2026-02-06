//! neuro-assistant-adapter
//!
//! Thin, enforcement-centric adapter that:
//! - Accepts a NeurorightsBoundPromptEnvelope.
//! - Loads neurorights policy + tool capability.
//! - Calls neurorights-core guards via neurorights-firewall.
//! - Delegates to a backend that implements RightsBoundChatExecutor.
//! - Emits EvolutionProposalRecord and DonutloopEntry via sovereigntycore/organiccpualn.
//!
//! This is intended as the only high-trust entrypoint for Jupyter-style
//! assistant calls in NeuroPC.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod config;
pub mod error;
pub mod adapter;
pub mod jupyter;

pub use crate::config::AssistantAdapterConfig;
pub use crate::error::AssistantAdapterError;
pub use crate::adapter::AssistantAdapter;
pub use crate::jupyter::JupyterBridge;
