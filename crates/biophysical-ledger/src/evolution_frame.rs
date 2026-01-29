use crate::lifeforce::{LifeforceBand, SafetyCurveWave};
use crate::eco::{EcoBandProfile, EcoImpactScore};
use crate::identity::HostId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvolutionPlane {
    SoftwareOnly,
    NeuromorphAdapter,
    OrganicCpuTile,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvolutionScope {
    Weights,
    Routing,
    IoAdapter,
    SchedulerHint,
}

#[derive(Clone, Copy, Debug)]
pub struct EvolutionCost {
    pub flop_budget: u64,
    pub nJ_budget: u64,
    pub eco_intent: EcoBandProfile,
}

#[derive(Clone, Copy, Debug)]
pub struct ExpectedEffect {
    pub latency_band: i8,
    pub error_band: i8,
    pub eco_impact_band: EcoImpactScore,
}

#[derive(Clone, Copy, Debug)]
pub struct GuardsSnapshot {
    pub lifeforce: LifeforceBand,
    pub safety_wave: SafetyCurveWave,
    pub daily_turn_seq: u32,
}

#[derive(Clone, Debug)]
pub struct EvolutionFrameId(pub [u8; 32]);

#[derive(Clone, Debug)]
pub struct EvolutionFrame {
    pub host: HostId,
    pub frame_id: EvolutionFrameId,
    pub plane: EvolutionPlane,
    pub scope: EvolutionScope,
    pub cost: EvolutionCost,
    pub expected_effect: ExpectedEffect,
    pub guards_snapshot: GuardsSnapshot,
}

#[derive(Clone, Copy, Debug)]
pub enum SystemAdjustmentDelta {
    ScaleDelta(f32),
    WaveBudgetShift(i32),
    NanoEnvelopeAdjust(i32),
}

#[derive(Clone, Debug)]
pub struct EvolutionDecision {
    pub frame_id: EvolutionFrameId,
    pub verdict: EvolutionVerdict,
    pub applied_deltas: Vec<SystemAdjustmentDelta>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvolutionVerdict {
    Safe,
    Defer,
    DenyHardStop,
}

pub trait EvolutionFrameValidator {
    fn validate_frame(&self, frame: &EvolutionFrame) -> EvolutionVerdict;
}

pub trait SystemAdjustment {
    fn apply_deltas(
        &mut self,
        frame: &EvolutionFrame,
        deltas: &[SystemAdjustmentDelta],
    ) -> EvolutionDecision;
}
