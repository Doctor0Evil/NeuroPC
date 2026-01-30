use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LatentNeuralState struct: Inferred states with biophysical mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatentNeuralState {
    pub uncertainty: f32,
    pub confidence: f32,
    pub affect: f32,          // -1.0 negative to 1.0 positive
    pub cognitive_load: f32,
    pub dominant_band: NeuralWaveBand,
    pub calibration_data: HashMap<String, f32>,  // Subjective labels
}

impl LatentNeuralState {
    pub fn new() -> Self {
        Self {
            uncertainty: 0.5,
            confidence: 0.5,
            affect: 0.0,
            cognitive_load: 0.5,
            dominant_band: NeuralWaveBand::Theta,
            calibration_data: HashMap::new(),
        }
    }

    /// Infer from text heuristics (simulated; real: NLP features).
    pub fn infer_from_text(&mut self, text: &str) {
        let mut rng = rand::thread_rng();
        self.uncertainty = if text.contains("?") { rng.gen_range(0.6..0.9) } else { rng.gen_range(0.1..0.4) };
        self.confidence = 1.0 - self.uncertainty;
        self.cognitive_load = (text.len() as f32 / 1000.0).clamp(0.0, 1.0);
        self.dominant_band = if self.cognitive_load > 0.7 { NeuralWaveBand::Alpha } else { NeuralWaveBand::Gamma };
    }

    /// Calibrate: Add subjective label, adjust latents (KL-like).
    pub fn calibrate(&mut self, key: String, value: f32) {
        self.calibration_data.insert(key, value);
        let avg = self.calibration_data.values().sum::<f32>() / self.calibration_data.len() as f32;
        self.uncertainty = (self.uncertainty + (1.0 - avg)) / 2.0;
    }

    /// Map to biophysical: Theta-gamma coupling simulation.
    pub fn biophysical_map(&self) -> f32 {
        match self.dominant_band {
            NeuralWaveBand::Theta => self.uncertainty * 0.5 + self.cognitive_load * 0.5,  // Encoding coupling
            NeuralWaveBand::Gamma => self.confidence * 0.7,  // Binding strength
            _ => 0.5,
        }
    }
}
