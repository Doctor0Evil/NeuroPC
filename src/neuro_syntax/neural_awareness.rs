use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// NeuralWaveBand enum grounding retention in biophysical oscillations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuralWaveBand {
    SlowWave,    // 0.5-4 Hz: Consolidation
    Theta,       // 4-8 Hz: Encoding/Retrieval
    Alpha,       // 8-12 Hz: Retention Gating
    Beta,        // 13-30 Hz: Control
    Gamma,       // >30 Hz: Binding
    HFO,         // 80-200+ Hz: Engrams
}

/// RetentionProfile struct for cognitive-retention length in bands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionProfile {
    pub primary_band: NeuralWaveBand,
    pub coupling_strength: f32,  // 0.0-1.0 for CFC (e.g., theta-gamma)
    pub min_retention_sec: u64,  // Minimum allowed retention
    pub max_retention_sec: u64,  // Maximum allowed retention
    pub priority: f32,           // 0.0-1.0 for consolidation scheduling
}

impl RetentionProfile {
    pub fn new(band: NeuralWaveBand, coupling: f32, min_sec: u64, max_sec: u64, priority: f32) -> Self {
        Self {
            primary_band: band,
            coupling_strength: coupling.clamp(0.0, 1.0),
            min_retention_sec: min_sec,
            max_retention_sec: max_sec,
            priority: priority.clamp(0.0, 1.0),
        }
    }

    /// Simulate CFC: Adjust retention based on coupling (biophysical-inspired).
    pub fn effective_retention(&self) -> u64 {
        let base = (self.min_retention_sec + self.max_retention_sec) / 2;
        (base as f32 * (1.0 + self.coupling_strength * self.priority)) as u64
    }
}

/// MetaCognition struct for awareness/uncertainty, tied to bands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaCognition {
    pub uncertainty: f32,        // 0.0-1.0
    pub confidence: f32,         // 0.0-1.0
    pub in_awareness: bool,
    pub last_access: DateTime<Utc>,
    pub assoc_band: NeuralWaveBand,
}

impl MetaCognition {
    pub fn new(uncertainty: f32, confidence: f32, in_awareness: bool, band: NeuralWaveBand) -> Self {
        Self {
            uncertainty: uncertainty.clamp(0.0, 1.0),
            confidence: confidence.clamp(0.0, 1.0),
            in_awareness,
            last_access: Utc::now(),
            assoc_band: band,
        }
    }

    /// Update on access: Biophysical drift simulation.
    pub fn access(&mut self) {
        self.last_access = Utc::now();
        self.in_awareness = true;
        self.uncertainty *= 0.9;  // Reduce uncertainty on replay
    }
}

/// ConsentState enum for governance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentState {
    Allowed,
    Denied,
    NeedsReview,
    Conditional,
}

/// GovernanceTags struct for SMART/EVOLVE integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceTags {
    pub owner_id: String,
    pub smart_token: Option<String>,
    pub consent_state: ConsentState,
    pub issued_at: DateTime<Utc>,
    pub evolve_threshold: f32,  // e.g., 0.5 for auto-issuance
}

impl GovernanceTags {
    pub fn new(owner: String, token: Option<String>, state: ConsentState, threshold: f32) -> Self {
        Self {
            owner_id: owner,
            smart_token: token,
            consent_state: state,
            issued_at: Utc::now(),
            evolve_threshold: threshold.clamp(0.0, 1.0),
        }
    }

    /// Adaptive issuance: Auto if meta meets threshold, else review.
    pub fn issue_smart(&mut self, meta: &MetaCognition) -> bool {
        if meta.uncertainty < self.evolve_threshold && meta.confidence > (1.0 - self.evolve_threshold) {
            self.smart_token = Some(format!("SMART-{}", Utc::now().timestamp()));
            self.consent_state = ConsentState::Allowed;
            true
        } else {
            self.consent_state = ConsentState::NeedsReview;
            false
        }
    }
}

/// AwarenessToken enum for `!`, `?`, `💡`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwarenessToken {
    Query,     // `?`: Uncertainty check, object-discovery
    Commit,    // `!`: Action/consent, motor-behavior
    Insight,   // `💡`: New struct, SMART proposal
}

/// ConsentObject struct binding token to meta/governance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentObject {
    pub token: AwarenessToken,
    pub target_id: String,
    pub meta: MetaCognition,
    pub governance: GovernanceTags,
}

impl ConsentObject {
    pub fn new(token: AwarenessToken, target: String, meta: MetaCognition, governance: GovernanceTags) -> Self {
        Self { token, target_id: target, meta, governance }
    }

    /// Evaluate execution: Band-dependent thresholds.
    pub fn allow_execution(&mut self) -> bool {
        self.meta.access();
        match self.token {
            AwarenessToken::Query => {  // Discovery: Low threshold
                if self.meta.uncertainty > 0.3 { self.governance.consent_state = ConsentState::NeedsReview; false } else { true }
            }
            AwarenessToken::Commit => {  // Motor: Strict
                if self.meta.confidence < 0.7 || self.meta.uncertainty > 0.2 { false } else { self.governance.issue_smart(&self.meta) }
            }
            AwarenessToken::Insight => {  // Struct creation: Adaptive
                self.governance.issue_smart(&self.meta)
            }
        }
    }
}

/// BrainFunction struct: Core unit with retention/awareness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainFunction {
    pub id: String,
    pub retention: RetentionProfile,
    pub meta: MetaCognition,
    pub governance: GovernanceTags,
    pub linked_objects: Vec<ConsentObject>,  // For chaining
}

impl BrainFunction {
    pub fn new(id: String, retention: RetentionProfile, meta: MetaCognition, governance: GovernanceTags) -> Self {
        Self { id, retention, meta, governance, linked_objects: Vec::new() }
    }

    /// Add consent-object: For discovery/motor/SMART.
    pub fn attach_consent(&mut self, obj: ConsentObject) {
        self.linked_objects.push(obj);
    }
}

/// MotorProgram struct for behavior, governed by consent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorProgram {
    pub id: String,
    pub sequence: Vec<String>,  // e.g., ["move_arm", "grasp"]
    pub controller_fn: Arc<Mutex<BrainFunction>>,
}

impl MotorProgram {
    pub fn new(id: String, sequence: Vec<String>, controller: BrainFunction) -> Self {
        Self { id, sequence, controller_fn: Arc::new(Mutex::new(controller)) }
    }

    /// Execute with governance: Only if consent allows.
    pub fn execute(&self, token: AwarenessToken) -> Result<String, String> {
        let mut ctrl = self.controller_fn.lock().unwrap();
        let mut consent = ConsentObject::new(token, self.id.clone(), ctrl.meta.clone(), ctrl.governance.clone());
        if consent.allow_execution() {
            // Simulate execution: In real, dispatch to organic_CPU.
            Ok(format!("Executed {:?} for program {}", token, self.id))
        } else {
            Err(format!("Denied: Uncertainty {} > threshold", consent.meta.uncertainty))
        }
    }
}
