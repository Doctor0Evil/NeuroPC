#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DreamMetrics {
    pub dream_immersion: f32,        // D in [0,1]
    pub lucidity_level: f32,         // L in [0,1]
    pub affective_tone: f32,         // A in [-1,1] or mapped to [0,1]
    pub narrative_structure: f32,    // N in [0,1]
    pub control_level: f32,          // C in [0,1]
    pub integration_score: f32,      // I in [0,1]
    pub primary_consciousness: f32,  // P in [0,1]
    pub secondary_consciousness: f32,// S in [0,1]
    pub vigilance_score: f32,        // V in [0,1]
    pub risk_score: f32,             // R in [0,1]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BioState {
    // existing fields…
    pub fatigue_index: f32,
    pub duty_cycle: f32,
    pub cognitive_load_index: f32,
    pub intent_confidence: f32,
    pub eco: EcoMetrics,

    // new dream-state slice (always present; zeros if unknown)
    pub dream: DreamMetrics,
}
