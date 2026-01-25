//! neuro_automagic_core
//!
//! Core data model for NeuroPC neuro-intent, neuro-context,
//! neuro-citizen, neuro-rights, and neuro-automation.
//!
//! This crate is designed to be reusable across:
//! - NeuroPC kernel/userland
//! - Reality.os
//! - Editor / AI-chat integrations

#![cfg_attr(not(feature = "std"), no_std)]

pub mod model;
pub mod right;
pub mod automation;
pub mod normalizer;

pub use model::{
    NeuroIntent,
    NeuroIntentKind,
    NeuroContext,
    NeuroPlatform,
    NeuroProjectScope,
    NeuroFileScope,
    NeuroCitizen,
    NeuroCitizenId,
};
pub use right::{NeuroRight, NeuroRightSet, NeuroAccessDecision};
pub use automation::{
    NeuroAutomationRule,
    NeuroAutomationTrigger,
    NeuroAutomationAction,
};
pub use normalizer::{NeuroIntentNormalizer, NormalizedIntent};
