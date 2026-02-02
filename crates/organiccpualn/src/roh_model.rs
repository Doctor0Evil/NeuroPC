use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RohModel {
    pub model_id: String,
    pub version: String,
    pub active: bool,
    pub roh_ceiling: f32,
    pub axes: Vec<String>,
    pub weights: Vec<f32>,
    pub bias: f32,
}

#[derive(Debug, Clone)]
pub struct StateVector {
    pub components: Vec<f32>,
}

impl RohModel {
    pub fn compute_roh(&self, state: &StateVector) -> f32 {
        let mut acc = self.bias;
        for (w, x) in self.weights.iter().zip(state.components.iter()) {
            acc += w * x;
        }
        acc.max(0.0).min(self.roh_ceiling)
    }

    pub fn roh_delta(&self, before: &StateVector, after: &StateVector) -> f32 {
        self.compute_roh(after) - self.compute_roh(before)
    }

    pub fn validate_invariants(&self) -> bool {
        self.roh_ceiling <= 0.3 + f32::EPSILON
            && self.weights.iter().all(|w| *w >= 0.0)
            && self.weights.iter().sum::<f32>() <= 1.0 + 1e-6
    }
}
