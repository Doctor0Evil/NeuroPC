use organiccpu::neuro_event_trigger::SparseEventTrigger;
use organiccpu::eco_safe_rf_gating::{EcoSafeRfGate, verify_wildlife_safe};

pub struct RealityOsNeuromorphicLayer {
    event_trigger: SparseEventTrigger,
    rf_gating: EcoSafeRfGate,
}

impl RealityOsNeuromorphicLayer {
    pub fn process_neural_input(&self, eeg_sample: f32, ts_ms: f64) -> Option<String> {
        // Sparse triggering
        if let Some(intent) = self.event_trigger.process_sample(eeg_sample, ts_ms) {
            // Pre-tx wildlife check
            if verify_wildlife_safe(&self.rf_gating).is_err() {
                eprintln!("[BLOCK] Intent tx blocked: RF gate failed wildlife check");
                return None;
            }
            // Check RF allowance
            if let Ok(_) = self.rf_gating.check_tx_allowed(100.0, ts_ms) {
                return Some(intent);
            }
        }
        None
    }
}
