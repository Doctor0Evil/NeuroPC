use std::time::{Duration, SystemTime};
use uuid::Uuid;

use crate::traits::{
    ConsentState, EvolutionToken, EvolutionWindow, LaneProfile, NeurorightsGuard, SafetyState,
    TraitKind,
};

#[derive(Debug, Clone)]
pub enum ContextEventKind {
    NavigationSuggested,
    SafetyHighPriority,
    CommunicationAssist,
}

#[derive(Debug, Clone)]
pub struct ContextEvent {
    pub id: Uuid,
    pub kind: ContextEventKind,
    pub issued_by: String, // remote origin identifier
    pub signature_valid: bool,
    pub received_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct TurnState {
    pub window: EvolutionWindow,
    pub applied_tokens: u32,
}

impl TurnState {
    pub fn new(window: EvolutionWindow) -> Self {
        Self {
            window,
            applied_tokens: 0,
        }
    }
}

/// Host-side scheduler orchestrating 3-minute evolution turns.
pub struct TurnScheduler {
    pub current: TurnState,
    pub default_lane: LaneProfile,
}

impl TurnScheduler {
    pub fn new() -> Self {
        let lane = LaneProfile::navigation_default();
        let window = EvolutionWindow::open(lane.clone(), SafetyState::Green);
        Self {
            current: TurnState::new(window),
            default_lane: lane,
        }
    }

    pub fn maybe_rotate_turn(&mut self, now: SystemTime) {
        if !self.current.window.is_active(now) {
            // close old, open new with same lane for now
            let new_window =
                EvolutionWindow::open(self.default_lane.clone(), self.current.window.safety_state);
            self.current = TurnState::new(new_window);
        }
    }

    pub fn handle_context_event(&mut self, event: ContextEvent) {
        if !event.signature_valid {
            return;
        }

        // Environment can only suggest lanes; it cannot directly change parameters.
        let lane_profile = match event.kind {
            ContextEventKind::NavigationSuggested => LaneProfile::navigation_default(),
            ContextEventKind::SafetyHighPriority => LaneProfile {
                name: "safety".into(),
                active_traits: vec![TraitKind::SafetyAlert],
                max_tokens_per_turn: 6,
                budget_bands: self.default_lane.budget_bands,
            },
            ContextEventKind::CommunicationAssist => LaneProfile {
                name: "communication".into(),
                active_traits: vec![TraitKind::CommunicationAssist],
                max_tokens_per_turn: 3,
                budget_bands: self.default_lane.budget_bands,
            },
        };

        // Inner-ledger remains sovereign: we only update default_lane,
        // the actual turn rotation still happens under our control.
        self.default_lane = lane_profile;
    }

    pub fn try_apply_token(
        &mut self,
        now: SystemTime,
        consent: &ConsentState,
        token: &EvolutionToken,
    ) -> bool {
        if !self.current.window.can_accept_token(
            now,
            consent,
            self.current.applied_tokens,
            token,
        ) {
            return false;
        }

        // At this point, a lower-level adapter would be invoked to apply the
        // parameter change to neuromorphic hardware, but only within the
        // strictly defined delta_label semantics and local safety bounds.

        self.current.applied_tokens += 1;
        true
    }

    pub fn set_safety_state(&mut self, safety: SafetyState) {
        self.current.window.safety_state = safety;
    }
}
