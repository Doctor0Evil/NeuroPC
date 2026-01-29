use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsentGranularity {
    RawNeural,
    FeatureVector,
    DecodedIntent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroRightsHeader {
    pub schema_version: String, // "neuro_rights/v1.0"
    pub rights_doc_uri: String,
    pub subject_did: String,
    pub created_at: String,
    pub valid_until: Option<String>,
    pub signature_hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroRightsConsent {
    pub consent_granularity: ConsentGranularity,
    pub allow_online_modulation: bool,
    pub allow_offline_replay: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroRightsConstraints {
    pub max_daily_risk_class: u8, // 0–5
    pub require_reversible: bool,
    pub max_nonlogged_events: u32,
    pub privacy_level: PrivacyLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroRightsIntegrity {
    pub max_cumulative_irr_events_per_year: u32,
    pub allowed_data_retention_days: u16, // 1–365
    pub audit_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuroRightsFile {
    pub header: NeuroRightsHeader,
    pub consent: NeuroRightsConsent,
    pub constraints: NeuroRightsConstraints,
    pub integrity: NeuroRightsIntegrity,
}

impl NeuroRightsFile {
    pub fn validate(&self) -> Result<(), String> {
        if self.constraints.max_daily_risk_class > 5 {
            return Err("max_daily_risk_class must be <= 5".into());
        }
        if self.integrity.allowed_data_retention_days == 0
            || self.integrity.allowed_data_retention_days > 365
        {
            return Err("allowed_data_retention_days must be in [1,365]".into());
        }
        Ok(())
    }
}
