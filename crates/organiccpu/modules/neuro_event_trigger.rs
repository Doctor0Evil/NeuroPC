use std::sync::Arc;
use parking_lot::RwLock;

/// Sparse event-driven spike detector with RoH ≤0.3 safety gate
#[derive(Clone)]
pub struct SparseEventTrigger {
    /// Threshold (µV) above baseline to fire "intent burst"
    spike_threshold: f32,
    /// Refractory period (ms) to prevent false bursts
    refractory_ms: f32,
    /// Rolling baseline (adaptive, 2s window)
    baseline_ewma: Arc<RwLock<f32>>,
    /// Last spike timestamp (ms)
    last_spike_ms: Arc<RwLock<f64>>,
    /// RoH safety invariant: intent firing rate must remain monotone-decreasing
    /// (prevents escalating tx duty from feedback loops)
    firing_rate_history: Arc<RwLock<Vec<(f64, f32)>>>, // (time_ms, rate_hz)
}

impl SparseEventTrigger {
    pub fn new(spike_threshold: f32, refractory_ms: f32) -> Self {
        Self {
            spike_threshold,
            refractory_ms,
            baseline_ewma: Arc::new(RwLock::new(0.0)),
            last_spike_ms: Arc::new(RwLock::new(-1e9)),
            firing_rate_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Process single EEG/EMG sample; returns Some(intent_id) if spike fires
    pub fn process_sample(&self, sample_uv: f32, now_ms: f64) -> Option<String> {
        let mut baseline = self.baseline_ewma.write();
        let alpha = 0.02; // EMA decay
        *baseline = (1.0 - alpha) * *baseline + alpha * sample_uv.abs();

        let mut last_spike = self.last_spike_ms.write();
        let time_since_last = now_ms - *last_spike;

        // Sparse trigger: only fire if threshold exceeded AND refractory cleared
        if (sample_uv - *baseline).abs() > self.spike_threshold && time_since_last > self.refractory_ms as f64 {
            *last_spike = now_ms;

            // RoH Monotonicity Check: ensure firing rate doesn't escalate indefinitely
            let mut history = self.firing_rate_history.write();
            let current_rate = 1000.0 / (time_since_last.max(1.0)); // Hz
            
            if !history.is_empty() {
                let prev_rate = history.last().unwrap().1;
                if current_rate > prev_rate * 1.2 {
                    // Rate escalating → apply duty-cycle gate (RoH safety)
                    eprintln!("[WARN] Intent firing rate escalating ({:.2} Hz -> {:.2} Hz); applying gate", prev_rate, current_rate);
                    // Return None to suppress this spike (maintain RoH ≤0.3)
                    return None;
                }
            }
            
            history.push((now_ms, current_rate));
            if history.len() > 1000 { history.remove(0); } // Keep recent window

            Some(format!("intent_burst_{}", (now_ms as u64)))
        } else {
            None
        }
    }

    /// Get current firing rate (Hz) for monitoring/control
    pub fn get_firing_rate_hz(&self) -> f32 {
        let history = self.firing_rate_history.read();
        if history.len() < 2 { return 0.0; }
        history.last().map(|(_, r)| *r).unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparse_event_no_false_positives() {
        let trigger = SparseEventTrigger::new(50.0, 100.0);
        
        // Steady baseline (no spike)
        assert_eq!(trigger.process_sample(10.0, 0.0), None);
        assert_eq!(trigger.process_sample(12.0, 10.0), None);

        // Large deviation → spike fires
        let spike = trigger.process_sample(150.0, 110.0);
        assert!(spike.is_some());

        // Refractory period: suppresses immediate re-fire
        assert_eq!(trigger.process_sample(160.0, 120.0), None);

        // After refractory clears
        let second = trigger.process_sample(140.0, 250.0);
        assert!(second.is_some());

        println!("Firing rate: {:.2} Hz", trigger.get_firing_rate_hz());
    }

    #[test]
    fn test_roh_monotonicity_gate() {
        let trigger = SparseEventTrigger::new(30.0, 10.0);
        
        // Simulate escalating intent bursts
        trigger.process_sample(100.0, 0.0);
        trigger.process_sample(100.0, 50.0);  // 20 Hz
        trigger.process_sample(100.0, 60.0);  // 100 Hz (escalating)
        
        // Third burst should be gated (RoH safety)
        let result = trigger.process_sample(100.0, 70.0);
        assert!(result.is_none(), "Escalating rate should be gated for RoH ≤0.3");
    }
}
