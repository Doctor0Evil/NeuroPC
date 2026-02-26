#[derive(Debug, Clone)]
pub struct IntentModelProfile {
    pub model_id: String,
    pub decoder_accuracy: f32,      // 0.0–1.0
    pub latency_ms: f32,
    pub energy_per_intent_mJ: f32,
    pub peak_power_W: f32,
    pub duty_cycle_ratio: f32,      // 0.0–1.0
    pub ecoimpactscore: f32,        // 0.0–1.0
}

impl IntentModelProfile {
    pub fn is_within_frugal_envelope(
        &self,
        max_latency_ms: f32,
        max_energy_mJ: f32,
        max_peak_W: f32,
        max_duty: f32,
    ) -> bool {
        self.latency_ms <= max_latency_ms
            && self.energy_per_intent_mJ <= max_energy_mJ
            && self.peak_power_W <= max_peak_W
            && self.duty_cycle_ratio <= max_duty
    }

    pub fn frugal_score(&self, acc_weight: f32, eco_weight: f32) -> f32 {
        let acc = self.decoder_accuracy.clamp(0.0, 1.0);
        let eco = self.ecoimpactscore.clamp(0.0, 1.0);
        (acc_weight * acc + eco_weight * eco) / (acc_weight + eco_weight)
    }
}

pub fn select_frugal_model(
    candidates: &[IntentModelProfile],
    max_latency_ms: f32,
    max_energy_mJ: f32,
    max_peak_W: f32,
    max_duty: f32,
    acc_weight: f32,
    eco_weight: f32,
) -> Option<IntentModelProfile> {
    let mut best: Option<IntentModelProfile> = None;
    let mut best_score = f32::MIN;

    for c in candidates {
        if !c.is_within_frugal_envelope(max_latency_ms, max_energy_mJ, max_peak_W, max_duty) {
            continue;
        }
        let score = c.frugal_score(acc_weight, eco_weight);
        if score > best_score {
            best_score = score;
            best = Some(c.clone());
        }
    }

    best
}
