use organiccpualn::rohmodel::{RohInputs, RohModelShard};

pub struct RiskOfHarm {
    model: RohModelShard,
}

impl RiskOfHarm {
    pub fn new(model: RohModelShard) -> Self {
        Self { model }
    }

    pub fn roh_ceiling(&self) -> f32 {
        self.model.roh_ceiling()
    }

    pub fn estimate(&self, inputs: &RohInputs) -> f32 {
        self.model.compute_roh(inputs)
    }

    pub fn is_within_ceiling(&self, inputs: &RohInputs) -> bool {
        self.estimate(inputs) <= self.roh_ceiling()
    }
}
