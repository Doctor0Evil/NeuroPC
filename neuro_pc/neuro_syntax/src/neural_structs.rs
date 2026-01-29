#![forbid(unsafe_code)]

use std::time::{Duration, SystemTime};

/// Basic "neuralwave band" abstraction for cognitive-retention budgeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeuralWaveBand {
    /// Fast, fragile traces; working-memory like.
    GammaFast,
    /// Attention and encoding support.
    BetaEncode,
    /// Consolidation-friendly band (maps to NREM-like slow oscillations).
    SlowConsolidate,
    /// Integrative/abstractive band (maps to REM/theta-like).
    ThetaIntegrate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryClass {
    ExplicitDeclarative,
    ExplicitProcedural,
    ImplicitDeclarative,
    ImplicitProcedural,
}

/// Retention "budget" for a single struct.
#[derive(Debug, Clone)]
pub struct RetentionProfile {
    pub wave_band: NeuralWaveBand,
    /// Desired minimum retention horizon.
    pub min_retention: Duration,
    /// Desired maximum retention horizon.
    pub max_retention: Duration,
    /// Relative consolidation priority (0.0–1.0).
    pub priority: f32,
}

/// Metacognitive measures attached to a brain struct.
#[derive(Debug, Clone)]
pub struct MetaCognition {
    /// 0.0 = fully certain, 1.0 = maximum uncertainty.
    pub uncertainty: f32,
    /// 0.0 = no confidence, 1.0 = full confidence.
    pub confidence: f32,
    /// Whether this item is currently in awareness/foreground.
    pub in_awareness: bool,
    /// Last time it was consciously accessed.
    pub last_awareness: Option<SystemTime>,
}

impl MetaCognition {
    pub fn new(uncertainty: f32, confidence: f32, in_awareness: bool) -> Self {
        Self {
            uncertainty: uncertainty.clamp(0.0, 1.0),
            confidence: confidence.clamp(0.0, 1.0),
            in_awareness,
            last_awareness: if in_awareness {
                Some(SystemTime::now())
            } else {
                None
            },
        }
    }
}

/// Consent state for a given operation or struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsentState {
    /// Explicitly permitted under current conditions.
    Allowed,
    /// Explicitly denied; operation must not proceed.
    Denied,
    /// Needs re-evaluation (e.g., new context, new governance rules).
    NeedsReview,
    /// Time-limited or usage-limited allowance.
    Conditional,
}

/// Governance tags for SMART-like control over structs.
#[derive(Debug, Clone)]
pub struct GovernanceTags {
    pub owner: String,
    pub smart_token: Option<String>, // e.g., "SMART", "EVOLVE"
    pub consent_state: ConsentState,
    pub issued_at: SystemTime,
}

/// Base "brain-function" struct; everything hangs off this.
#[derive(Debug, Clone)]
pub struct BrainFunction {
    pub id: String,
    pub label: String,
    pub memory_class: MemoryClass,
    pub retention: RetentionProfile,
    pub meta: MetaCognition,
    pub governance: GovernanceTags,
}

/// Neural awareness tokens. These are the semantic counterparts of `?`, `!`, and `💡`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwarenessToken {
    /// Query / metacognitive check
    Question,
    /// Commit / action with consent
    Commit,
    /// Insight / new struct crystallization
    Insight,
}

/// A consent-object that bundles awareness, metacognition, and governance.
#[derive(Debug, Clone)]
pub struct ConsentObject {
    pub token: AwarenessToken,
    pub target_id: String,
    pub meta_snapshot: MetaCognition,
    pub governance: GovernanceTags,
}

impl ConsentObject {
    /// Basic constructor that binds to an existing BrainFunction.
    pub fn from_brain_function(token: AwarenessToken, bf: &BrainFunction) -> Self {
        Self {
            token,
            target_id: bf.id.clone(),
            meta_snapshot: bf.meta.clone(),
            governance: bf.governance.clone(),
        }
    }

    /// Evaluate whether the operation should execute, given current meta state.
    pub fn is_execution_allowed(&self) -> bool {
        match self.governance.consent_state {
            ConsentState::Denied => false,
            ConsentState::Allowed => true,
            ConsentState::Conditional => {
                // Example policy: require uncertainty below 0.5 for Commit,
                // allow Question or Insight regardless (they are non-destructive).
                match self.token {
                    AwarenessToken::Commit => self.meta_snapshot.uncertainty < 0.5,
                    AwarenessToken::Question | AwarenessToken::Insight => true,
                }
            }
            ConsentState::NeedsReview => false,
        }
    }
}

/// Simple motor "program" representation that can be coupled with consent-objects.
#[derive(Debug, Clone)]
pub struct MotorProgram {
    pub id: String,
    pub description: String,
    /// Link to the brain function that owns or triggers this motor program.
    pub controller_id: String,
}

/// Motor behavior executor with explicit consent checks.
pub struct MotorExecutor;

impl MotorExecutor {
    pub fn execute_with_consent(
        program: &MotorProgram,
        consent: &ConsentObject,
    ) -> Result<(), String> {
        if !consent.is_execution_allowed() {
            return Err(format!(
                "Execution denied for program {} by consent-object on {}",
                program.id, consent.target_id
            ));
        }
        // This is where, in an organic CPU / neuromorphic stack, you would
        // dispatch the motor program's low-level sequence. Here we just log.
        println!(
            "[MOTOR_EXEC] Executing program '{}' controlled by '{}'",
            program.id, program.controller_id
        );
        Ok(())
    }
}
