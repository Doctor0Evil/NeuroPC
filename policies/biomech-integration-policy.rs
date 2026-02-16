use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomechModuleSpec {
    pub id: String,
    pub scope: String,
    pub riskclass: String,
    pub integrationrole: String,
    pub maxeffectsize: f32,
    pub maxupdatesperday: i32,
    pub requireevolvetoken: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskClassR0 {
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskClassR1 {
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskClassR2 {
    pub description: String,
    pub maxautochangesperday: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomechRiskClasses {
    pub R0_observer: RiskClassR0,
    pub R1_advisor: RiskClassR1,
    pub R2_bounded_auto: RiskClassR2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomechIntegrationPolicy {
    pub subjectid: String,
    pub version: String,
    pub modules: Vec<BiomechModuleSpec>,
    pub riskclasses: BiomechRiskClasses,
}

impl BiomechIntegrationPolicy {
    pub fn find_module(&self, id: &str) -> Option<&BiomechModuleSpec> {
        self.modules.iter().find(|m| m.id == id)
    }
}
