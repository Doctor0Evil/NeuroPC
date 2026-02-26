use std::collections::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;

/// Computes 24-hour baseline spike statistics per electrode
pub struct SpikeBaselineCharacterizer {
    electrodes: HashMap<String, ElectrodeBaseline>,
    timestamp_start: f64,  // ms since epoch
    lock: Arc<RwLock<()>>,
}

#[derive(Clone)]
struct ElectrodeBaseline {
    electrode_id: String,
    spikes_per_second: Vec<f32>,  // Rolling window, 1Hz granularity
    burst_events: Vec<(f64, u32)>,  // (timestamp_ms, spike_count_in_10ms)
    neurorights_class: String,
}

impl SpikeBaselineCharacterizer {
    pub fn new() -> Self {
        Self {
            electrodes: HashMap::new(),
            timestamp_start: chrono::Utc::now().timestamp_millis() as f64,
            lock: Arc::new(RwLock::new(())),
        }
    }

    /// Register an electrode for tracking
    pub fn register_electrode(&mut self, id: &str, neurorights_class: &str) {
        self.electrodes.insert(id.to_string(), ElectrodeBaseline {
            electrode_id: id.to_string(),
            spikes_per_second: vec![0.0; 86400],  // 24 hours @ 1Hz resolution
            burst_events: Vec::new(),
            neurorights_class: neurorights_class.to_string(),
        });
    }

    /// Record a spike on an electrode
    pub fn record_spike(&mut self, electrode_id: &str, timestamp_ms: f64) {
        if let Some(baseline) = self.electrodes.get_mut(electrode_id) {
            let elapsed_ms = timestamp_ms - self.timestamp_start;
            let second_index = (elapsed_ms / 1000.0) as usize;
            
            if second_index < baseline.spikes_per_second.len() {
                baseline.spikes_per_second[second_index] += 1.0;
            }
        }
    }

    /// Detect bursting episodes (>50 Hz in 10ms window)
    pub fn detect_bursts(&mut self) {
        for baseline in self.electrodes.values_mut() {
            let mut i = 0;
            while i + 10 < baseline.spikes_per_second.len() {
                let burst_rate: f32 = baseline.spikes_per_second[i..i+10].iter().sum::<f32>() / 10.0;
                
                if burst_rate > 50.0 {
                    baseline.burst_events.push((
                        self.timestamp_start + (i as f64 * 1000.0),
                        burst_rate as u32
                    ));
                }
                i += 1;
            }
        }
    }

    /// Compute final statistics and generate baseline ALN
    pub fn finalize_24h_baseline(&self) -> HashMap<String, BaselineStats> {
        let mut results = HashMap::new();

        for (electrode_id, baseline) in &self.electrodes {
            let firing_rates = &baseline.spikes_per_second;
            let mean = firing_rates.iter().sum::<f32>() / firing_rates.len() as f32;
            let variance: f32 = firing_rates.iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f32>() / firing_rates.len() as f32;
            let std = variance.sqrt();

            let mut sorted = firing_rates.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let percentile_05 = sorted[(sorted.len() as f32 * 0.05) as usize];
            let percentile_95 = sorted[(sorted.len() as f32 * 0.95) as usize];

            results.insert(electrode_id.clone(), BaselineStats {
                mean_firing_rate: mean,
                std_firing_rate: std,
                percentile_05,
                percentile_95,
                burst_count: baseline.burst_events.len(),
                neurorights_class: baseline.neurorights_class.clone(),
            });
        }

        results
    }
}

#[derive(Clone)]
pub struct BaselineStats {
    pub mean_firing_rate: f32,
    pub std_firing_rate: f32,
    pub percentile_05: f32,
    pub percentile_95: f32,
    pub burst_count: usize,
    pub neurorights_class: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baseline_characterization() {
        let mut characterizer = SpikeBaselineCharacterizer::new();
        
        characterizer.register_electrode("thread_0_ch_42", "motor_intent");
        
        // Simulate 86400 spikes over 24 hours at ~15 Hz
        for second in 0..86400 {
            for _ in 0..15 {
                let timestamp_ms = (second as f64 * 1000.0);
                characterizer.record_spike("thread_0_ch_42", timestamp_ms);
            }
        }

        characterizer.detect_bursts();
        let stats = characterizer.finalize_24h_baseline();
        
        assert!(stats.contains_key("thread_0_ch_42"));
        let s = &stats["thread_0_ch_42"];
        println!("Mean rate: {:.1} Hz", s.mean_firing_rate);
        assert!(s.mean_firing_rate > 14.5 && s.mean_firing_rate < 15.5);
    }
}
