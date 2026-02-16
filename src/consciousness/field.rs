use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Global consciousness field vector 𝐂(t)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct ConsciousnessField {
    pub integration: f32,      // I(t) – effective connectivity
    pub differentiation: f32,  // D(t) – pattern diversity
    pub complexity: f32,       // Ξ(t) – synergistic information proxy
    pub ignition_freq: f32,    // G(t) – broadcast event rate
    pub energetic_load: f32,   // E(t) – average event energy proxy
}

/// Tracks recent history and computes drift magnitude for policy gating
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsciousnessDriftTracker {
    history: VecDeque<(f64, ConsciousnessField)>,
    max_history: usize,
    pub max_allowed_drift_rate: f32, // configured by your evolution profile
}

impl ConsciousnessDriftTracker {
    pub fn new(max_history: usize, max_allowed_drift_rate: f32) -> Self {
        Self {
            history: VecDeque::with_capacity(max_history + 1),
            max_history,
            max_allowed_drift_rate,
        }
    }

    pub fn update(&mut self, field: ConsciousnessField) {
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        self.history.push_back((t, field));
        if self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    /// L2 drift rate over most recent window (normalized per second)
    pub fn recent_drift_rate(&self) -> f32 {
        if self.history.len() < 2 {
            return 0.0;
        }
        let (t0, c0) = self.history[0];
        let (t1, c1) = *self.history.back().unwrap();
        let dt = (t1 - t0) as f32;
        if dt <= 0.0 { return 0.0; }

        let di = c1.integration - c0.integration;
        let dd = c1.differentiation - c0.differentiation;
        let dx = c1.complexity - c0.complexity;
        let dg = c1.ignition_freq - c0.ignition_freq;
        let de = c1.energetic_load - c0.energetic_load;

        ((di*di + dd*dd + dx*dx + dg*dg + de*de).sqrt()) / dt
    }

    pub fn drift_within_bounds(&self) -> bool {
        self.recent_drift_rate() <= self.max_allowed_drift_rate
    }
}
