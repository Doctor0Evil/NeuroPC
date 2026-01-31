use serde::Deserialize;
use organic_cpu_core::BioLimits;

#[derive(Clone, Debug, Deserialize)]
struct IdentitySection {
    id: String,
    label: String,
}

#[derive(Clone, Debug, Deserialize)]
struct EnvelopeSection {
    max_fatigue: f32,
    max_duty_cycle: f32,
    max_cognitive_load: f32,
}

#[derive(Clone, Debug, Deserialize)]
struct AssistiveSection {
    preferred_modalities: Vec<String>,
    rest_prompt_minutes: u32,
    max_daily_device_hours: f32,
}

#[derive(Clone, Debug, Deserialize)]
struct EcoSection {
    target_eco_impact_score: f32,
}

#[derive(Clone, Debug, Deserialize)]
struct OrganicCpuProfileFile {
    identity: IdentitySection,
    envelope: EnvelopeSection,
    assistive: AssistiveSection,
    eco: EcoSection,
}

#[derive(Clone, Debug)]
pub struct UserEnvelope {
    pub id: String,
    pub label: String,
    pub limits: BioLimits,
    pub rest_prompt_minutes: u32,
    pub max_daily_device_hours: f32,
    pub target_eco_impact_score: f32,
}

impl From<EnvelopeSection> for BioLimits {
    fn from(e: EnvelopeSection) -> Self {
        BioLimits {
            max_fatigue: e.max_fatigue,
            max_duty_cycle: e.max_duty_cycle,
            max_cognitive_load: e.max_cognitive_load,
        }
    }
}

pub fn load_profile_from_file(path: &str) -> anyhow::Result<UserEnvelope> {
    let text = std::fs::read_to_string(path)?;
    let parsed: OrganicCpuProfileFile = toml::from_str(&text)?;

    Ok(UserEnvelope {
        id: parsed.identity.id,
        label: parsed.identity.label,
        limits: parsed.envelope.into(),
        rest_prompt_minutes: parsed.assistive.rest_prompt_minutes,
        max_daily_device_hours: parsed.assistive.max_daily_device_hours,
        target_eco_impact_score: parsed.eco.target_eco_impact_score,
    })
}
