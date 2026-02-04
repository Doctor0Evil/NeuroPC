use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct KnowledgeFactor {
    /// F ∈ [0,1]: evidence density, citation integrity, cross-source agreement, freshness.
    pub value: f32,
}

impl KnowledgeFactor {
    pub fn new_clamped(value: f32) -> Self {
        Self { value: value.clamp(0.0, 1.0) }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RiskOfHarm {
    /// r ∈ [0,1], global invariant r ≤ 0.3 enforced by sovereigntycore.
    pub value: f32,
}

impl RiskOfHarm {
    pub fn new_checked(value: f32) -> Option<Self> {
        if (0.0..=0.3).contains(&value) {
            Some(Self { value })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Cybostate {
    RetrievalOnly,
    ResearchReady,
    GovernanceReady,
    ActuationForbidden,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnswerEnvelope {
    pub knowledge: KnowledgeFactor,
    pub roh: RiskOfHarm,
    pub cybostate: Cybostate,
}
