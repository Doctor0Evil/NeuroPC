use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohAxis {
    pub name: String,
    pub range: (f32, f32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohModelMeta {
    pub subject_id: String,
    pub version: String,
    pub description: String,
    pub kind: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohWeights {
    pub energy_load: f32,
    pub thermal_load: f32,
    pub cognitive_load: f32,
    pub inflammation: f32,
    pub eco_impact: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohModelCore {
    pub id: String,
    pub weights: RohWeights,
    pub roh_ceiling: f32,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohModelShard {
    pub meta: RohModelMeta,
    pub axes: Vec<RohAxis>,
    pub model: RohModelCore,
}

#[derive(Clone, Debug)]
pub struct RohInputs {
    pub energy_load: f32,
    pub thermal_load: f32,
    pub cognitive_load: f32,
    pub inflammation: f32,
    pub eco_impact: f32,
}

impl RohModelShard {
    pub fn compute_roh(&self, inputs: &RohInputs) -> f32 {
        let w = &self.model.weights;
        let r = w.energy_load * inputs.energy_load
            + w.thermal_load * inputs.thermal_load
            + w.cognitive_load * inputs.cognitive_load
            + w.inflammation * inputs.inflammation
            + w.eco_impact * inputs.eco_impact;
        r.clamp(0.0, 1.0)
    }

    pub fn roh_ceiling(&self) -> f32 {
        self.model.roh_ceiling
    }
}
