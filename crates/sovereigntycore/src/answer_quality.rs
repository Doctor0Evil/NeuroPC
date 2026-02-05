use serde::{Deserialize, Serialize};

/// Scalar summarizing knowledge quality for a single answer.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KnowledgeFactor {
    /// Evidence density / citation integrity [0.0, 1.0].
    pub value: f32,
}

impl KnowledgeFactor {
    pub const MIN: f32 = 0.0;
    pub const MAX: f32 = 1.0;

    pub fn clamped(value: f32) => Self {
        let v = value.clamp(Self::MIN, Self::MAX);
        Self { value: v }
    }

    pub fn is_sufficient(&self, f_min: f32) -> bool {
        self.value >= f_min
    }
}

/// Risk-of-Harm scalar for a single answer, reusing your RoH model ceiling.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnswerRisk {
    /// Composite RoH scalar [0.0, 1.0], must obey global ceiling 0.3.
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

/// Discrete trust / capability state for this answer.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Cybostate {
    RetrievalOnly,
    ResearchReady,
    GovernanceReady,
    ActuationForbidden,
}

/// High-level route the caller is asking for.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnswerRoute {
    /// Plain informational / retrieval answer.
    Info,
    /// Design / spec / Rust-ALN governance artifacts.
    GovernanceDesign,
    /// Anything implying direct hardware / stimulation control (must be blocked).
    Actuation,
}

/// Required capability for the route.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CybostateRequirement {
    AllowAll,
    RequireResearchReady,
    RequireGovernanceReady,
    ForbidActuation,
}

/// Minimal contract for anything that can score knowledge-factor.
pub trait ChatKnowledgeFactor {
    /// Compute F in [0,1] using implementation-specific heuristics.
    fn compute_knowledge_factor(&self) -> KnowledgeFactor;
}

/// Contract for answer-level risk envelope (RoH scalar + invariants).
pub trait RiskEnvelope {
    /// Estimated RoH scalar for this answer.
    fn estimate_risk_of_harm(&self) -> AnswerRisk;

    /// Hard invariant: r <= 0.3 and (optionally) monotone safety vs baseline.
    fn satisfies_risk_bounds(&self) -> bool {
        self.estimate_risk_of_harm().is_within_ceiling()
    }
}

/// Contract for classifying Cybostate and checking routing permissions.
pub trait CybostateClass {
    /// Cybostate attached to this answer.
    fn cybostate(&self) -> Cybostate;

    /// Given the requested route, is this Cybostate permitted?
    fn is_route_allowed(&self, route: AnswerRoute) -> bool {
        match route {
            AnswerRoute::Info => true,
            AnswerRoute::GovernanceDesign => matches!(
                self.cybostate(),
                Cybostate::GovernanceReady | Cybostate::ResearchReady
            ),
            AnswerRoute::Actuation => false, // hard disallow
        }
    }
}

/// Aggregate quality envelope for a single answer.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnswerQuality {
    pub f: KnowledgeFactor,
    pub r: AnswerRisk,
    pub cybostate: Cybostate,
}

impl AnswerQuality {
    pub fn is_allowed(&self, f_min: f32, route: AnswerRoute) -> bool {
        self.f.is_sufficient(f_min)
            && self.r.is_within_ceiling()
            && CybostateClass::is_route_allowed_impl(&self.cybostate, route)
    }
}

// Helper so AnswerQuality can reuse the routing logic.
impl CybostateClass for AnswerQuality {
    fn cybostate(&self) -> Cybostate {
        self.cybostate.clone()
    }
}

impl AnswerQuality {
    fn is_route_allowed_impl(state: &Cybostate, route: AnswerRoute) -> bool {
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
