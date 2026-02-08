#![cfg_attr(not(feature = "std"), no_std)]

pub mod model;
pub mod adapter;

pub use model::{
    VirtualObject,
    VirtualObjectKind,
    NeurovascularObject,
    NeurovascularSignalKind,
    CerebralSample,
};
pub use adapter::{CerebralAdapter, AdapterCapabilities};
