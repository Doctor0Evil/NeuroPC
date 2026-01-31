use serde::{Deserialize, Serialize};
use crate::{EnvelopePolicy, OrganicCpuOrchestrator};
use organic_cpu_core::{BioState, EcoMetrics, SafeEnvelopeDecision};

#[derive(Clone, Debug, Deserialize)]
pub struct CopilotInputJson {
    pub bio_summary: BioSummaryJson,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BioSummaryJson {
    pub fatigue_index: f32,
    pub duty_cycle: f32,
    pub cognitive_load_index: f32,
    pub intent_confidence: f32,
    pub eco_impact_score: f32,
    pub device_hours: f32,
}

#[derive(Clone, Debug, Serialize)]
pub struct CopilotOutputJson {
    pub decision: String,
    pub eco: EcoJson,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize)]
pub struct EcoJson {
    pub eco_impact_score: f32,
    pub device_hours: f32,
}

impl From<&BioSummaryJson> for BioState {
    fn from(b: &BioSummaryJson) -> Self {
        BioState {
            fatigue_index: b.fatigue_index,
            duty_cycle: b.duty_cycle,
            cognitive_load_index: b.cognitive_load_index,
            intent_confidence: b.intent_confidence,
            eco: EcoMetrics {
                eco_impact_score: b.eco_impact_score,
                device_hours: b.device_hours,
            },
        }
    }
}

impl From<SafeEnvelopeDecision> for String {
    fn from(d: SafeEnvelopeDecision) -> Self {
        match d {
            SafeEnvelopeDecision::AllowFullAction => "AllowFullAction".to_string(),
            SafeEnvelopeDecision::DegradePrecision => "DegradePrecision".to_string(),
            SafeEnvelopeDecision::PauseAndRest => "PauseAndRest".to_string(),
        }
    }
}

impl OrganicCpuOrchestrator {
    /// Core entrypoint for AI-chat tools: JSON in, JSON out.
    pub fn process_json(
        &self,
        input: CopilotInputJson,
    ) -> CopilotOutputJson {
        let bio_state: BioState = (&input.bio_summary).into();
        let decision = self.policy.decide(&bio_state);
        CopilotOutputJson {
            decision: String::from(decision),
            eco: EcoJson {
                eco_impact_score: input.bio_summary.eco_impact_score,
                device_hours: input.bio_summary.device_hours,
            },
            metadata: None,
        }
    }
}
