use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::neuro_pc::brain_function::{MetaCognition, GovernanceTags, ConsentState};

/// Awareness token matching ?, !, 💡 semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwarenessToken {
    Query,   // "?" – diagnostic / discovery
    Commit,  // "!" – motor/structural commit
    Insight, // "💡" – new pattern / schema crystallization
}

/// Consent object bound to a target BrainFunction or MotorProgram.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentObject {
    pub id: String,
    pub token: AwarenessToken,
    pub target_id: String,
    pub meta_snapshot: MetaCognition,
    pub governance: GovernanceTags,
    pub created_at: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
}

impl ConsentObject {
    pub fn new(
        id: String,
        token: AwarenessToken,
        target_id: String,
        meta_snapshot: MetaCognition,
        mut governance: GovernanceTags,
    ) -> Self {
        // Default: mark as NeedsReview until evaluated.
        governance.consent_state = ConsentState::NeedsReview;
        Self {
            id,
            token,
            target_id,
            meta_snapshot,
            governance,
            created_at: Utc::now(),
            valid_until: None,
        }
    }

    /// Evaluate if an operation is allowed, based on token + meta.
    pub fn evaluate(&mut self) -> bool {
        match self.token {
            AwarenessToken::Query => {
                // Queries are low-risk; allow unless explicitly denied.
                if self.governance.consent_state == ConsentState::Denied {
                    false
                } else {
                    self.governance.consent_state = ConsentState::Allowed;
                    true
                }
            }
            AwarenessToken::Commit => {
                // Strict thresholds for commits (motor or structural).
                if self.meta_snapshot.confidence >= 0.7 && self.meta_snapshot.uncertainty <= 0.2 {
                    self.governance.consent_state = ConsentState::Allowed;
                    true
                } else {
                    self.governance.consent_state = ConsentState::NeedsReview;
                    false
                }
            }
            AwarenessToken::Insight => {
                // Insight creates new structs; require moderate confidence.
                if self.meta_snapshot.confidence >= 0.5 {
                    self.governance.consent_state = ConsentState::Conditional;
                    true
                } else {
                    self.governance.consent_state = ConsentState::NeedsReview;
                    false
                }
            }
        }
    }
}
