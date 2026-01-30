// Requires in Cargo.toml:
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// chrono = { version = "0.4", features = ["serde"] }

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// High-level memory class / role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryClass {
    ExplicitDeclarative,
    ExplicitProcedural,
    ImplicitDeclarative,
    ImplicitProcedural,
}

/// Neuralwave-like band label (abstract, biophysically-informed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuralWaveBand {
    SlowWave, // 0.5–4 Hz (consolidation)
    Theta,    // 4–8 Hz (encoding/retrieval)
    Alpha,    // 8–12 Hz (retention/gating)
    Beta,     // 13–30 Hz (control)
    Gamma,    // >30 Hz (binding)
    HFO,      // 80–200+ Hz (engrams/ripples)
}

/// Retention profile for one BrainFunction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionProfile {
    pub primary_band: NeuralWaveBand,
    pub secondary_band: Option<NeuralWaveBand>,
    /// Phase–amplitude coupling strength (0.0–1.0).
    pub pac_strength: f32,
    /// Amplitude–amplitude coupling strength (0.0–1.0).
    pub aac_strength: f32,
    /// Min retention horizon in seconds.
    pub min_retention_sec: u64,
    /// Max retention horizon in seconds.
    pub max_retention_sec: u64,
    /// Relative consolidation priority (0.0–1.0).
    pub priority: f32,
}

impl RetentionProfile {
    pub fn effective_retention(&self) -> u64 {
        let base = (self.min_retention_sec + self.max_retention_sec) / 2;
        let coupling = 0.5 * (self.pac_strength + self.aac_strength);
        (base as f32 * (1.0 + coupling * self.priority)).round() as u64
    }
}

/// Metacognitive state tied to this BrainFunction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaCognition {
    pub uncertainty: f32,    // 0.0–1.0
    pub confidence: f32,     // 0.0–1.0
    pub in_awareness: bool,
    pub last_access: DateTime<Utc>,
}

impl MetaCognition {
    pub fn new(uncertainty: f32, confidence: f32, in_awareness: bool) -> Self {
        Self {
            uncertainty: uncertainty.clamp(0.0, 1.0),
            confidence: confidence.clamp(0.0, 1.0),
            in_awareness,
            last_access: Utc::now(),
        }
    }
}

/// Consent state (links to .cobj).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentState {
    Allowed,
    Denied,
    NeedsReview,
    Conditional,
}

/// Governance tags for neurorights / organichain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceTags {
    pub owner_id: String,           // e.g., your Bostrom address
    pub smart_token: Option<String>,
    pub consent_state: ConsentState,
    pub issued_at: DateTime<Utc>,
    /// 0.0–1.0 threshold for auto-evolution under EVOLVE semantics.
    pub evolve_threshold: f32,
}

impl GovernanceTags {
    pub fn new(owner_id: String) -> Self {
        Self {
            owner_id,
            smart_token: None,
            consent_state: ConsentState::NeedsReview,
            issued_at: Utc::now(),
            evolve_threshold: 0.5,
        }
    }
}

/// Main BrainFunction record (.brfn).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainFunction {
    pub id: String,
    pub label: String,
    pub class: MemoryClass,
    pub created_at: DateTime<Utc>,
    pub retention: RetentionProfile,
    pub meta: MetaCognition,
    pub governance: GovernanceTags,
    /// Optional references to related objects (.cobj, .nwbio, etc.).
    pub related_ids: Vec<String>,
}

impl BrainFunction {
    pub fn new(
        id: String,
        label: String,
        class: MemoryClass,
        retention: RetentionProfile,
        owner_id: String,
    ) -> Self {
        let meta = MetaCognition::new(uncertainty: 0.5, confidence: 0.5, in_awareness: false);
        let governance = GovernanceTags::new(owner_id);
        Self {
            id,
            label,
            class,
            created_at: Utc::now(),
            retention,
            meta,
            governance,
            related_ids: Vec::new(),
        }
    }
}
