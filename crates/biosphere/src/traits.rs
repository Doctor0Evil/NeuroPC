use std::time::{Duration, SystemTime};
use uuid::Uuid;

/// Non-derogable neurorights invariants that every adapter and token must respect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeurorightsGuard {
    pub mental_privacy_enforced: bool,
    pub cognitive_liberty_enforced: bool,
    pub identity_integrity_enforced: bool,
    pub local_sovereignty_enforced: bool,
}

impl NeurorightsGuard {
    pub fn strict() -> Self {
        Self {
            mental_privacy_enforced: true,
            cognitive_liberty_enforced: true,
            identity_integrity_enforced: true,
            local_sovereignty_enforced: true,
        }
    }

    pub fn check_or_panic(&self) {
        assert!(self.mental_privacy_enforced, "Mental privacy violation");
        assert!(self.cognitive_liberty_enforced, "Cognitive liberty violation");
        assert!(self.identity_integrity_enforced, "Identity integrity violation");
        assert!(self.local_sovereignty_enforced, "Local sovereignty violation");
    }
}

/// High-level neuromorphic function class.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TraitKind {
    Navigation,
    SafetyAlert,
    CommunicationAssist,
    SensoryFilter,
    AttentionModulator,
}

/// Versioned trait identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitId {
    pub kind: TraitKind,
    pub version: u32,
}

/// Band-limited “budget” semantics for lifeforce/eco consumption.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BudgetBands {
    pub lifeforce_band: f32, // 0.0..=1.0 relative budget
    pub eco_band: f32,       // 0.0..=1.0 relative budget
}

impl BudgetBands {
    pub fn conservative() -> Self {
        Self {
            lifeforce_band: 0.25,
            eco_band: 0.25,
        }
    }
}

/// Consent scope for a class of updates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsentScope {
    None,
    ReadOnly,          // Only summaries/features may be computed
    ConservativeTuning, // Small, reversible parameter tweaks
    FullTuning,        // Within pre-defined safe ranges
}

/// Live consent state for one trait category.
#[derive(Debug, Clone)]
pub struct ConsentState {
    pub trait_kind: TraitKind,
    pub scope: ConsentScope,
    pub granted_at: SystemTime,
    pub expires_at: Option<SystemTime>,
    pub user_descriptor: String, // short human-readable description
}

impl ConsentState {
    pub fn is_active(&self, now: SystemTime) -> bool {
        if let Some(exp) = self.expires_at {
            now < exp && self.scope != ConsentScope::None
        } else {
            self.scope != ConsentScope::None
        }
    }

    pub fn allows_evolution(&self) -> bool {
        matches!(
            self.scope,
            ConsentScope::ConservativeTuning | ConsentScope::FullTuning
        )
    }
}

/// Safety state derived from subjective + physiological signals.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SafetyState {
    Green,
    Yellow,
    Red,
}

/// Per-turn evolution token describing a micro-change proposal.
#[derive(Debug, Clone)]
pub struct EvolutionToken {
    pub id: Uuid,
    pub trait_id: TraitId,
    pub delta_label: String, // e.g. "nav.sensitivity+0.02"
    pub cost_bands: BudgetBands,
    pub expected_effect_band: f32, // 0.0..=1.0, size of change
    pub reversible: bool,
}

impl EvolutionToken {
    pub fn navigation_delta(version: u32, label: &str, effect_band: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            trait_id: TraitId {
                kind: TraitKind::Navigation,
                version,
            },
            delta_label: label.to_string(),
            cost_bands: BudgetBands::conservative(),
            expected_effect_band: effect_band.min(1.0).max(0.0),
            reversible: true,
        }
    }
}

/// Per-lane profile (navigation, safety, communication) defining budgets.
#[derive(Debug, Clone)]
pub struct LaneProfile {
    pub name: String,
    pub active_traits: Vec<TraitKind>,
    pub max_tokens_per_turn: u32,
    pub budget_bands: BudgetBands,
}

impl LaneProfile {
    pub fn navigation_default() -> Self {
        Self {
            name: "navigation".into(),
            active_traits: vec![TraitKind::Navigation, TraitKind::SafetyAlert],
            max_tokens_per_turn: 4,
            budget_bands: BudgetBands::conservative(),
        }
    }
}

/// 3-minute evolution window configuration.
#[derive(Debug, Clone)]
pub struct EvolutionWindow {
    pub id: Uuid,
    pub opened_at: SystemTime,
    pub duration: Duration,
    pub lane_profile: LaneProfile,
    pub neurorights_guard: NeurorightsGuard,
    pub safety_state: SafetyState,
}

impl EvolutionWindow {
    pub fn open(lane_profile: LaneProfile, safety_state: SafetyState) -> Self {
        let guard = NeurorightsGuard::strict();
        guard.check_or_panic();
        Self {
            id: Uuid::new_v4(),
            opened_at: SystemTime::now(),
            duration: Duration::from_secs(180),
            lane_profile,
            neurorights_guard: guard,
            safety_state,
        }
    }

    pub fn is_active(&self, now: SystemTime) -> bool {
        now.duration_since(self.opened_at)
            .map(|d| d <= self.duration)
            .unwrap_or(false)
    }

    /// Evaluates whether a token may be applied under current safety + consent.
    pub fn can_accept_token(
        &self,
        now: SystemTime,
        consent: &ConsentState,
        applied_tokens: u32,
        token: &EvolutionToken,
    ) -> bool {
        if !self.is_active(now) {
            return false;
        }

        if !consent.is_active(now) || !consent.allows_evolution() {
            return false;
        }

        if !self
            .lane_profile
            .active_traits
            .contains(&token.trait_id.kind)
        {
            return false;
        }

        if applied_tokens >= self.lane_profile.max_tokens_per_turn {
            return false;
        }

        match self.safety_state {
            SafetyState::Red => false,
            SafetyState::Yellow => token.expected_effect_band <= 0.25,
            SafetyState::Green => token.expected_effect_band <= 0.5,
        }
    }
}
