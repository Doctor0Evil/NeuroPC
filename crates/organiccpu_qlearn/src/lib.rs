#[derive(Clone, Debug)]
pub struct QLearnOutputs {
    pub intent_confidence: f32,      // 0–1
    pub exploration_temperature: f32,// 0–1
    pub dream_sensitivity: f32,      // how much to trust dream metrics this step
    pub roh_estimate: f32            // estimated incremental RoH for proposed update
}

pub trait QLearnModel {
    fn infer(&self, state: &BioState) -> QLearnOutputs;
}
