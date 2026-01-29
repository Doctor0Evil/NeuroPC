use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiofluxHeader {
    pub schema_version: String,      // "bioflux/v1.0"
    pub experiment_id: String,
    pub micro_epoch_ref: Option<String>,
    pub created_at: String,          // ISO 8601
    pub temporal_resolution_ms: u16, // 1–100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiofluxSample {
    pub t_ms: u64,
    pub circadian_phase_rad: f32, // 0..2π
    pub dopamine_uM: f32,         // 0.1..10
    pub h2o2_uM: f32,             // 0.05..5
    pub fatigue_index: f32,       // 0..1
    pub focus_index: f32,         // 0..1
    pub stress_index: f32,        // 0..1
    pub clarity_index: f32,       // 0..1
    pub confidence: f32,          // 0..1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiofluxFile {
    pub header: BiofluxHeader,
    pub samples: Vec<BiofluxSample>,
}

impl BiofluxFile {
    pub fn validate(&self) -> Result<(), String> {
        let res = self.header.temporal_resolution_ms;
        if res < 1 || res > 100 {
            return Err("temporal_resolution_ms out of range [1,100]".into());
        }
        let mut last_t = None;
        for s in &self.samples {
            if !(0.0..=6.2831855).contains(&s.circadian_phase_rad) {
                return Err("circadian_phase_rad out of range [0,2π)".into());
            }
            if !(0.1..=10.0).contains(&s.dopamine_uM) {
                return Err("dopamine_uM out of range [0.1,10.0]".into());
            }
            if !(0.05..=5.0).contains(&s.h2o2_uM) {
                return Err("h2o2_uM out of range [0.05,5.0]".into());
            }
            for v in [
                s.fatigue_index,
                s.focus_index,
                s.stress_index,
                s.clarity_index,
                s.confidence,
            ] {
                if !(0.0..=1.0).contains(&v) {
                    return Err("index field out of range [0,1]".into());
                }
            }
            if let Some(prev) = last_t {
                if s.t_ms <= prev {
                    return Err("t_ms must be strictly increasing".into());
                }
            }
            last_t = Some(s.t_ms);
        }
        Ok(())
    }
}
