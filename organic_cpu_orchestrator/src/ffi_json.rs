use serde::{Deserialize, Serialize};
use std::io::{self, Read};
use crate::OrganicCpuOrchestrator;
use organic_cpu_profile::UserEnvelope;
use organic_cpu_core::{BioState, EcoMetrics, SafeEnvelopeDecision};

/// ---- JSON types for FFI boundary ----

#[derive(Clone, Debug, Deserialize)]
pub struct BioSummaryJson {
    pub fatigue_index: f32,
    pub duty_cycle: f32,
    pub cognitive_load_index: f32,
    pub intent_confidence: f32,
    pub eco_impact_score: f32,
    pub device_hours: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CopilotInputJson {
    pub profile_id: String,      // e.g. "bostrom_primary"
    pub bio_summary: BioSummaryJson,
}

#[derive(Clone, Debug, Serialize)]
pub struct EcoJson {
    pub eco_impact_score: f32,
    pub device_hours: f32,
}

#[derive(Clone, Debug, Serialize)]
pub struct SovereignMetadata {
    /// Optional tag from caller (e.g. editor / host).
    pub host_id: String,
    /// Whether this run was in strict-safe mode (no experimental flags).
    pub safe_mode: bool,
    /// Hex-tag for session / traceability.
    pub session_tag: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct CopilotOutputJson {
    pub decision: String,
    pub eco: EcoJson,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SovereignMetadata>,
}

/// ---- Conversion from JSON to internal BioState ----

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

/// ---- Sovereign, chat-friendly entrypoint ----

pub fn run_ffi_once(
    load_profile: &dyn Fn(&str) -> anyhow::Result<UserEnvelope>,
    host_id: &str,
    safe_mode: bool,
    session_tag: &str,
) -> anyhow::Result<()> {
    // Read a single JSON object from stdin.
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    let input: CopilotInputJson = serde_json::from_str(&buf)?;

    // Sovereign: you control which profile file is allowed for this host.
    let profile = load_profile(&input.profile_id)?;
    let orchestrator = OrganicCpuOrchestrator::from_profile(&profile);

    let bio_state: BioState = (&input.bio_summary).into();
    let decision = orchestrator.policy.decide(&bio_state);

    let output = CopilotOutputJson {
        decision: String::from(decision),
        eco: EcoJson {
            eco_impact_score: input.bio_summary.eco_impact_score,
            device_hours: input.bio_summary.device_hours,
        },
        metadata: Some(SovereignMetadata {
            host_id: host_id.to_string(),
            safe_mode,
            session_tag: session_tag.to_string(),
        }),
    };

    let json = serde_json::to_string(&output)?;
    println!("{json}");
    Ok(())
}
