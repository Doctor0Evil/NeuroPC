use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeRow {
    pub role_id: String,
    pub subject_id: String,
    pub bostrom_address: String,
    pub role_kind: String,
    pub can_veto: bool,
    pub can_init_evolve: bool,
    pub required_for_lifeforce: bool,
    pub required_for_arch_change: bool,
}

#[derive(Debug, Clone)]
pub struct StakeTable {
    pub rows: Vec<StakeRow>,
}

impl StakeTable {
    pub fn required_roles_for_scope(&self, scope: &[String]) -> Vec<String> {
        // Simple rule: lifeforce → Host + OrganicCPU, archchange → Host + OrganicCPU + ResearchAgent
        let mut out = Vec::new();
        if scope.iter().any(|s| s == "lifeforce") {
            out.push("Host".to_string());
            out.push("OrganicCPU".to_string());
        }
        if scope.iter().any(|s| s == "archchange") {
            out.push("Host".to_string());
            out.push("ResearchAgent".to_string());
        }
        out
    }
}
