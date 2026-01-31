use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Command, Stdio};

/// JSON shape expected by organic_cpu_cli / ffi_json.
#[derive(Clone, Debug, Serialize)]
struct BioSummaryJson {
    fatigue_index: f32,
    duty_cycle: f32,
    cognitive_load_index: f32,
    intent_confidence: f32,
    eco_impact_score: f32,
    device_hours: f32,
}

#[derive(Clone, Debug, Serialize)]
struct CopilotInputJson {
    profile_id: String,
    bio_summary: BioSummaryJson,
}

#[derive(Clone, Debug, Deserialize)]
struct EcoJson {
    eco_impact_score: f32,
    device_hours: f32,
}

#[derive(Clone, Debug, Deserialize)]
struct SovereignMetadata {
    host_id: String,
    safe_mode: bool,
    session_tag: String,
}

#[derive(Clone, Debug, Deserialize)]
struct CopilotOutputJson {
    decision: String,
    eco: EcoJson,
    #[serde(default)]
    metadata: Option<SovereignMetadata>,
}

/// Minimal hint struct you can plug into Reality.os scheduling / UI.
#[derive(Clone, Debug)]
pub struct RealityHints {
    pub automagic_level: f32,
    pub suggest_rest: bool,
    pub note: String,
}

fn derive_reality_hints(decision: &str, fatigue_index: f32) -> RealityHints {
    match decision {
        "AllowFullAction" => RealityHints {
            automagic_level: 1.0,
            suggest_rest: fatigue_index > 0.8,
            note: "High-capacity mode: allow dense, fast interactions.".into(),
        },
        "DegradePrecision" => RealityHints {
            automagic_level: 0.5,
            suggest_rest: fatigue_index > 0.7,
            note: "Slow pacing, shorter bursts, more scaffolding.".into(),
        },
        "PauseAndRest" => RealityHints {
            automagic_level: 0.2,
            suggest_rest: true,
            note: "Recommend microbreak or low-strain mode.".into(),
        },
        _ => RealityHints {
            automagic_level: 0.7,
            suggest_rest: false,
            note: "Unknown decision: stay moderate.".into(),
        },
    }
}

/// Call organic_cpu_cli once and get hints for the current Reality.os tick.
pub fn query_bio_feedback(
    organic_cpu_cli_bin: &str,
    profiles_dir: &str,
    profile_id: &str,
    host_id: &str,      // e.g. "reality.os"
    safe_mode: bool,
    session_tag: &str,  // e.g. "0xNP0B"
    fatigue_index: f32,
    duty_cycle: f32,
    cognitive_load_index: f32,
    intent_confidence: f32,
    eco_impact_score: f32,
    device_hours: f32,
) -> anyhow::Result<RealityHints> {
    let input = CopilotInputJson {
        profile_id: profile_id.to_string(),
        bio_summary: BioSummaryJson {
            fatigue_index,
            duty_cycle,
            cognitive_load_index,
            intent_confidence,
            eco_impact_score,
            device_hours,
        },
    };

    let mut child = Command::new(organic_cpu_cli_bin)
        .arg(profiles_dir)
        .arg(host_id)
        .arg(if safe_mode { "true" } else { "false" })
        .arg(session_tag)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Write JSON to stdin
    {
        let stdin = child.stdin.as_mut().ok_or_else(|| {
            anyhow::anyhow!("failed to open stdin for organic_cpu_cli")
        })?;
        let json = serde_json::to_string(&input)?;
        stdin.write_all(json.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "organic_cpu_cli exited with status {:?}",
            output.status.code()
        ));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let copilot: CopilotOutputJson = serde_json::from_str(&stdout)?;

    Ok(derive_reality_hints(
        &copilot.decision,
        fatigue_index,
    ))
}
