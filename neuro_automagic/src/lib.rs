//! neuro_automagic
//!
//! Automagic assistance event model and integration helpers
//! for Cargo and editor tooling, built on neuro_automagic_core.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod event;
pub mod engine;
pub mod integration_cargo;
pub mod integration_editor;

pub use event::{NeuroAutomationEvent, RepetitionStats, ComplexityStats};
pub use engine::{NeuroAutomagicEngine, EngineConfig};
pub use integration_cargo::{CargoInstruction, CargoIntegrator};
pub use integration_editor::{EditorAction, EditorIntegrator};
