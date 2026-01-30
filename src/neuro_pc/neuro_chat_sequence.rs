use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::neuro_pc::brain_function::NeuralWaveBand;

/// Role of each message in the dialogue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatRole {
    User,
    Assistant,
    System,
}

/// Latent "neural" proxies inferred from language (deviceless).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatentNeuralState {
    pub estimated_uncertainty: f32, // 0.0–1.0
    pub estimated_confidence: f32,  // 0.0–1.0
    pub affect_valence: f32,        // -1.0 (negative) to +1.0 (positive)
    pub cognitive_load: f32,        // 0.0–1.0
    pub dominant_band: NeuralWaveBand,
}

/// Single chat turn with latent state and links.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTurn {
    pub index: u32,
    pub role: ChatRole,
    pub timestamp: DateTime<Utc>,
    pub text: String,
    pub latent: LatentNeuralState,
    /// Optional references to BrainFunctions, ConsentObjects, etc.
    pub linked_ids: Vec<String>,
}

/// Entire chat sequence, suitable for training/analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroChatSequence {
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub turns: Vec<ChatTurn>,
}

impl NeuroChatSequence {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            created_at: Utc::now(),
            turns: Vec::new(),
        }
    }

    pub fn add_turn(&mut self, turn: ChatTurn) {
        self.turns.push(turn);
    }
}
