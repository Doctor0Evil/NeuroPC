use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// NeuralWaveBand from neural_awareness.rs (import as needed)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuralWaveBand {
    Theta, Gamma, // Simplified for addiction
}

// RetentionProfile simplified for sim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionProfile {
    pub band: NeuralWaveBand,
    pub coupling: f32,
}

// MetaCognition for awareness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaCognition {
    pub uncertainty: f32,
    pub confidence: f32,
}

// AwarenessToken for syntax
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwarenessToken {
    Query, Commit, Insight,
}

// ConsentObject for governance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentObject {
    pub token: AwarenessToken,
    pub meta: MetaCognition,
}

// BrainFunction for circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainFunction {
    pub retention: RetentionProfile,
    pub meta: MetaCognition,
}

// NicotineCravingSimulator: SNN for quitting-data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NicotineCravingSimulator {
    pub da_level: f32, // Dopamine 0-1
    pub q_smoke: f32, // Q-value smoke
    pub q_abstain: f32, // Q-value abstain
    pub toxin_load: f32, // Cumulative toxins
    pub brain_fn: Arc<BrainFunction>, // Linked function
    pub consent: ConsentObject, // Governance
}

impl NicotineCravingSimulator {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            da_level: rng.gen_range(0.1..0.3),
            q_smoke: 0.8,
            q_abstain: 0.2,
            toxin_load: 0.0,
            brain_fn: Arc::new(BrainFunction {
                retention: RetentionProfile { band: NeuralWaveBand::Theta, coupling: 0.5 },
                meta: MetaCognition { uncertainty: 0.5, confidence: 0.5 },
            }),
            consent: ConsentObject {
                token: AwarenessToken::Query,
                meta: MetaCognition { uncertainty: 0.5, confidence: 0.5 },
            },
        }
    }

    /// Step sim: Math-grounded update
    pub fn step(&mut self, nic_conc: f32, dt: f32) -> f32 {
        // DA modulation: δ_nic = s * [Nic]
        let s = 0.7; // Sensitivity
        self.da_level += s * nic_conc * dt - 0.1 * dt; // Decay
        self.da_level = self.da_level.clamp(0.0, 1.0);

        // Q-update: L = -y log(p) - (1-y)log(1-p)
        let action = if self.q_smoke > self.q_abstain { 0 } else { 1 };
        let reward = if action == 0 { self.da_level } else { 0.1 };
        let prediction = if action == 0 { self.q_smoke } else { self.q_abstain };
        let delta = reward - prediction;
        let alpha = 0.01; // Learning-rate
        if action == 0 {
            self.q_smoke += alpha * delta;
        } else {
            self.q_abstain += alpha * delta;
        }

        // Toxin accumulation
        self.toxin_load += nic_conc * 0.05 * dt;

        // Consent check: Uncertainty <0.5 for Commit
        if self.consent.meta.uncertainty < 0.5 {
            self.consent.token = AwarenessToken::Commit;
        }

        // Return craving: Proof-convergent
        self.q_smoke - self.q_abstain
    }

    /// Simulate quitting: Extinction if low-uncertainty
    pub fn simulate_quit(&mut self, steps: usize) -> Vec<f32> {
        let mut cravings = Vec::new();
        for _ in 0..steps {
            let craving = self.step(0.0, 1.0); // No nic
            cravings.push(craving);
            self.consent.meta.uncertainty *= 0.95; // Reduce on sim
        }
        cravings
    }
}
