use serde::{Serialize, Deserialize};
use brainprint_core::{BrainPrintCore, BrainPrintRecord};
use brainprint_macros::BrainPrint;

#[derive(Debug, Clone, Serialize, Deserialize, BrainPrint)]
pub struct HostBrainPrint {
    pub core: BrainPrintCore,

    // Host-local, sovereignly approved extensions (namespaced in crate/module)
    pub host_bandwidth_hint: f32,    // unitless, 0–1
    pub host_focus_score:    f32,    // 0–1, no raw signals
}
