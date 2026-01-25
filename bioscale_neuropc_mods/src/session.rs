use crate::metrics::BioscaleMetric;

/// Minimal description of a NeuroPC session for bioscale use.
#[derive(Clone, Debug)]
pub struct NeuroPcSessionInfo {
    pub project_name: String,
    pub device_hours_today: f64,
    pub recent_compile_fail_rate: f64,
    pub recent_command_repetition: f64,
}

/// Snapshot of bioscale-relevant state derived from PC activity.
#[derive(Clone, Debug)]
pub struct BioscaleSnapshot {
    pub session: NeuroPcSessionInfo,
    pub metrics: Vec<BioscaleMetric>,
}
