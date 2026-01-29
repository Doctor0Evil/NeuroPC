//! inner_ledger/evolution_frame.rs
//! Defines EvolutionFrame, guard traits, and ledger logic
//! for secure, auditable neuromorphic evolution.

use std::time::{SystemTime, Duration};
use uuid::Uuid;

/// Define the hardware plane of the adaptation.
#[derive(Debug, Clone)]
pub enum EvolutionPlane {
    SoftwareOnly,
    NeuromorphAdapter,
    OrganicCpuTile,
}

/// Evolution scope: defines the subsystem affected.
#[derive(Debug, Clone)]
pub enum EvolutionScope {
    Weights,
    Routing,
    IoAdapter,
    SchedulerHint,
}

/// Computational and environmental cost specification.
#[derive(Debug, Clone)]
pub struct EvolutionCost {
    pub flop_budget: u64,
    pub nJ_budget: f64,
    pub eco_intent: String,
}

/// Expected outcome bounds.
#[derive(Debug, Clone)]
pub struct ExpectedEffect {
    pub latency_band: i8,
    pub error_band: i8,
    pub eco_impact_band: i8,
}

/// Safety snapshot at submission time.
#[derive(Debug, Clone)]
pub struct GuardsSnapshot {
    pub lifeforce_band: String,
    pub safety_wave: String,
    pub daily_turn_seq: u64,
}

/// The full proposed change record.
#[derive(Debug, Clone)]
pub struct EvolutionFrame {
    pub host: String,
    pub frame_id: Uuid,
    pub plane: EvolutionPlane,
    pub scope: EvolutionScope,
    pub cost: EvolutionCost,
    pub expected_effect: ExpectedEffect,
    pub guards_snapshot: GuardsSnapshot,
}

/// Result status codes for frame adjudication.
#[derive(Debug, Clone)]
pub enum EvolutionVerdict {
    Safe,
    Defer,
    DenyHardStop,
}

/// The result of ledger evaluation.
#[derive(Debug, Clone)]
pub struct EvolutionDecision {
    pub frame_id: Uuid,
    pub verdict: EvolutionVerdict,
    pub applied_deltas: Vec<String>,
    pub timestamp: SystemTime,
}

/// Core engine responsible for validating frames and applying safe deltas.
pub struct InnerLedger {
    pub lifeforce_state: String,
    pub eco_profile: String,
    pub turn_timer: SystemTime,
}

impl InnerLedger {
    pub fn new() -> Self {
        InnerLedger {
            lifeforce_state: "Stable".into(),
            eco_profile: "EcoBandLow".into(),
            turn_timer: SystemTime::now(),
        }
    }

    /// Validate frame before any application.
    pub fn validate_frame(&self, frame: &EvolutionFrame) -> EvolutionDecision {
        // Hard stop guard
        if self.lifeforce_state == "HardStop"
            || frame.guards_snapshot.lifeforce_band == "HardStop"
        {
            return EvolutionDecision {
                frame_id: frame.frame_id,
                verdict: EvolutionVerdict::DenyHardStop,
                applied_deltas: vec![],
                timestamp: SystemTime::now(),
            };
        }

        // Resource constraints
        if frame.cost.flop_budget > 10_000_000 || frame.cost.nJ_budget > 50.0 {
            return EvolutionDecision {
                frame_id: frame.frame_id,
                verdict: EvolutionVerdict::Defer,
                applied_deltas: vec![],
                timestamp: SystemTime::now(),
            };
        }

        // Safe to apply small-delta adjustments
        let deltas = self.apply_micro_adjustments(frame);

        EvolutionDecision {
            frame_id: frame.frame_id,
            verdict: EvolutionVerdict::Safe,
            applied_deltas: deltas,
            timestamp: SystemTime::now(),
        }
    }

    /// Dummy version of the micro-adjustment to illustrate reversible updates.
    fn apply_micro_adjustments(&self, frame: &EvolutionFrame) -> Vec<String> {
        match frame.scope {
            EvolutionScope::Weights => vec!["ScaleDelta(+0.01)".into()],
            EvolutionScope::Routing => vec!["WaveBudgetShift(+2Hz)".into()],
            EvolutionScope::IoAdapter => vec!["NanoEnvelopeAdjust(+5nJ)".into()],
            EvolutionScope::SchedulerHint => vec!["TurnPriorityShift(+1)".into()],
        }
    }

    /// 3-minute evolution turn gate.
    pub fn can_open_turn(&self) -> bool {
        self.turn_timer.elapsed()
            .map(|d| d >= Duration::from_secs(180))
            .unwrap_or(true)
    }
}
