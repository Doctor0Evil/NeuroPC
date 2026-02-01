use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcoMetrics {
    pub eco_impact_score: f32, // 0.0-1.0
    pub device_hours: f32,     // hours per day
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BioState {
    pub fatigue_index: f32,        // 0.0-1.0
    pub duty_cycle: f32,           // 0.0-1.0
    pub cognitive_load_index: f32, // 0.0-1.0
    pub intent_confidence: f32,    // 0.0-1.0
    pub eco: EcoMetrics,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SafeEnvelopeDecision {
    AllowFullAction,
    DegradePrecision,
    PauseAndRest,
}

pub trait SafeEnvelopePolicy {
    fn decide(&self, state: BioState) -> SafeEnvelopeDecision;
}

/// Simple conservative envelope for first-pass use.
pub struct BaselineEnvelope;

impl SafeEnvelopePolicy for BaselineEnvelope {
    fn decide(&self, state: BioState) -> SafeEnvelopeDecision {
        let overload =
            state.fatigue_index > 0.7 ||
            state.duty_cycle > 0.6 ||
            state.cognitive_load_index > 0.75;

        if overload {
            SafeEnvelopeDecision::PauseAndRest
        } else if state.intent_confidence < 0.6 {
            SafeEnvelopeDecision::DegradePrecision
        } else {
            SafeEnvelopeDecision::AllowFullAction
        }
    }
}
