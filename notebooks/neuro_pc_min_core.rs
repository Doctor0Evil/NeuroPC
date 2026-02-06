/// Minimal bioscale state used by NeuroPC.
#[derive(Clone, Debug)]
pub struct EcoMetrics {
    pub eco_impact_score: f32, // 0.0–1.0, lower is better
    pub device_hours: f32,     // hours per day
}

#[derive(Clone, Debug)]
pub struct BioState {
    pub fatigue_index: f32,        // 0.0–1.0
    pub duty_cycle: f32,           // 0.0–1.0
    pub cognitive_load_index: f32, // 0.0–1.0
    pub intent_confidence: f32,    // 0.0–1.0
    pub eco: EcoMetrics,
}

#[derive(Clone, Debug)]
pub struct BioLimits {
    pub max_fatigue: f32,
    pub max_duty_cycle: f32,
    pub max_cognitive_load: f32,
}

#[derive(Clone, Debug)]
pub enum SafeEnvelopeDecision {
    AllowFullAction,
    DegradePrecision,
    PauseAndRest,
}

#[derive(Clone, Debug)]
pub struct SafeEnvelopePolicy {
    pub limits: BioLimits,
    pub min_intent_confidence: f32,
}

impl SafeEnvelopePolicy {
    pub fn decide(&self, state: &BioState) -> SafeEnvelopeDecision {
        let overload = state.fatigue_index > self.limits.max_fatigue
            || state.duty_cycle > self.limits.max_duty_cycle
            || state.cognitive_load_index > self.limits.max_cognitive_load;

        if overload {
            SafeEnvelopeDecision::PauseAndRest
        } else if state.intent_confidence < self.min_intent_confidence {
            SafeEnvelopeDecision::DegradePrecision
        } else {
            SafeEnvelopeDecision::AllowFullAction
        }
    }
}
