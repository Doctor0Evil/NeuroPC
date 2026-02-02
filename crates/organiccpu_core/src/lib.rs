#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcoMetrics {
    pub eco_impact_score: f32, // 0.0–1.0
    pub device_hours: f32,     // hours/day
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BioState {
    pub fatigue_index: f32,        // 0–1
    pub duty_cycle: f32,           // 0–1
    pub cognitive_load_index: f32, // 0–1
    pub intent_confidence: f32,    // 0–1
    pub eco: EcoMetrics,
}
