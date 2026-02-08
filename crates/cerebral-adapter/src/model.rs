use core::time::Duration;
use serde::{Serialize, Deserialize};

use neuroautomagiccore::{NeuroIntent, NeuroContext, NeuroCitizen};
use bioscaleneuropcmods::metrics::BioscaleMetric;

/// High-level UI / environment objects discoverable from BCI/HCI.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VirtualObject {
    pub id: String,
    pub kind: VirtualObjectKind,
    pub label: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VirtualObjectKind {
    UiElement,
    ToolHandle,
    WorkspaceRegion,
    TimelineMarker,
}

/// Biophysical / neurovascular projections into bioscale metrics.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurovascularObject {
    pub id: String,
    pub signal_kind: NeurovascularSignalKind,
    /// Normalized 0–1 metrics, compatible with BioState / BioscaleMetric.
    pub metrics: Vec<BioscaleMetric>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NeurovascularSignalKind {
    CognitiveLoad,
    Fatigue,
    Arousal,
    MotorReadiness,
}

/// One time-stamped sample from the adapter: device raw-ish plus normalized view.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CerebralSample {
    pub timestamp_ns: u64,
    /// Device-specific payload (already privacy-sanitized, no raw electrodes if forbidden).
    pub device_payload: Vec<u8>,
    /// Discovered virtual objects in this window.
    pub virtual_objects: Vec<VirtualObject>,
    /// Neurovascular objects / bioscale projections in this window.
    pub neurovascular_objects: Vec<NeurovascularObject>,
    /// Optional decoded intention, aligned with NeuroPC’s intent model.
    pub decoded_intent: Option<NeuroIntent>,
    /// Derived bioscale metrics for envelopes and RoH.
    pub bioscale_metrics: Vec<BioscaleMetric>,
}
