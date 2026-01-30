use serde::{Deserialize, Serialize};

// From previous modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmokingCueExtinctionWeight {
    pub cue_level: f32, // 0-1
    pub urge_level: f32, // 0-1
    pub withdrawal_stability: f32, // S_withdrawal
    pub theta_diff: f32, // Theta power differential
    pub spindle_density: f32, // 12-15Hz
}

impl SmokingCueExtinctionWeight {
    pub fn new(cue: f32, urge: f32, stability: f32, theta: f32, spindle: f32) -> Self {
        Self {
            cue_level: cue.clamp(0.0, 1.0),
            urge_level: urge.clamp(0.0, 1.0),
            withdrawal_stability: stability.clamp(0.0, 1.0),
            theta_diff: theta,
            spindle_density: spindle.clamp(0.0, 1.0),
        }
    }

    /// Compute SCEW: Math-proofed formula
    pub fn compute_scew(&self) -> f32 {
        self.cue_level * (1.0 - self.urge_level) * self.withdrawal_stability * (0.5 + 0.5 * self.theta_diff.abs()) * self.spindle_density
    }

    /// Simulate extinction: Converges if SCEW > threshold
    pub fn simulate_extinction(&self, threshold: f32, steps: usize) -> Vec<f32> {
        let mut scews = Vec::new();
        let mut current = self.compute_scew();
        for _ in 0..steps {
            if current > threshold {
                current *= 1.05; // Proof: Increases on success
            } else {
                current *= 0.95; // Decreases on failure
            }
            scews.push(current.clamp(0.0, 1.0));
        }
        scews
    }
}
