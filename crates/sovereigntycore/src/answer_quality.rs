use serde::{Deserialize, Serialize};

/// Biocompatibility rating for this answer-quality façade, in [0,1].
/// This is a metadata constant, not used in any logic.
pub const NPF_NEURO_PRINT_BCR: f32 = 0.31;

/// Scalar summarizing knowledge quality for a single answer, F ∈ [0,1].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KnowledgeFactor {
    pub value: f32,
}

impl KnowledgeFactor {
    pub const MIN: f32 = 0.0;
    pub const MAX: f32 = 1.0;

    pub fn clamped(value: f32) -> Self {
        let v = value.clamp(Self::MIN, Self::MAX);
        Self { value: v }
    }

    pub fn is_sufficient(&self, f_min: f32) -> bool {
        self.value >= f_min
    }
}

/// Risk-of-Harm scalar for a single answer, r ∈ [0, 0.3].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnswerRisk {
    pub roh: f32,
}

impl AnswerRisk {
    pub const GLOBAL_CEILING: f32 = 0.3;

    pub fn clamped(roh: f32) -> Self {
        let r = roh.clamp(0.0, Self::GLOBAL_CEILING);
        Self { roh: r }
    }

    pub fn is_within_ceiling(&self) -> bool {
        self.roh <= Self::GLOBAL_CEILING + f32::EPSILON
    }
}

/// Cybostate classification for an answer.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Cybostate {
    RetrievalOnly,
    ResearchReady,
    GovernanceReady,
    ActuationForbidden,
}

/// High-level route requested by the caller.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnswerRoute {
    /// Plain informational retrieval.
    Info,
    /// Design/governance artifacts (.aln, .stake, .rohmodel, kernel specs).
    GovernanceDesign,
    /// Anything implying direct actuation (must be blocked at this layer).
    Actuation,
}

/// Aggregate quality envelope for a single answer.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnswerQuality {
    pub f: KnowledgeFactor,
    pub r: AnswerRisk,
    pub cybostate: Cybostate,
    pub route: AnswerRoute,
}

impl AnswerQuality {
    /// Combined predicate: F ≥ F_min, r ≤ 0.3, route compatible with cybostate.
    pub fn is_allowed(&self, f_min: f32) -> bool {
        self.f.is_sufficient(f_min)
            && self.r.is_within_ceiling()
            && Self::is_route_allowed_impl(&self.cybostate, &self.route)
    }

    fn is_route_allowed_impl(state: &Cybostate, route: &AnswerRoute) -> bool {
        match route {
            AnswerRoute::Info => true,
            AnswerRoute::GovernanceDesign => matches!(
                state,
                Cybostate::GovernanceReady | Cybostate::ResearchReady
            ),
            AnswerRoute::Actuation => false,
        }
    }
}
