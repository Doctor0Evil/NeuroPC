#[derive(Debug, Clone)]
pub struct BioCompatibleOrganicCpu {
    pub did: String,
    pub roh: f32,                    // 0.0–0.3 only
    pub automation_level: u8,        // 0–5, capped by .ocpuenv
    pub verbosity_band: u8,          // 0–3
    pub accessibility_envelope: OrganicAccessibilityEnvelope,
}

impl BioCompatibleOrganicCpu {
    pub fn new(did: String,
               roh: f32,
               automation_level: u8,
               verbosity_band: u8,
               env: OrganicAccessibilityEnvelope) -> Result<Self, Error> {
        if roh > 0.30 {
            return Err(Error::RohTooHigh);
        }
        if !env.is_monotone_tightening() {
            return Err(Error::EnvelopeLoosened);
        }
        Ok(Self {
            did,
            roh,
            automation_level: automation_level.min(env.max_automation_level),
            verbosity_band: verbosity_band.min(env.max_verbosity_band),
            accessibility_envelope: env,
        })
    }
}
