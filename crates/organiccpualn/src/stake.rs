use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakeAddresses {
    pub bostrom_primary: String,
    pub bostrom_alt: Option<String>,
    pub bostrom_safe_1: Option<String>,
    pub evm_erc20: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakeTokens {
    pub SMART: Option<TokenScope>,
    pub EVOLVE: Option<TokenScope>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenScope {
    pub scope: Vec<String>,
    pub veto_powers: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakeInvariants {
    pub must_match_host: bool,
    pub can_hard_stop: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakeRole {
    pub id: String,
    pub label: String,
    pub did: String,
    pub addresses: Option<StakeAddresses>,
    pub tokens: StakeTokens,
    pub invariants: Option<StakeInvariants>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakeMeta {
    pub version: String,
    pub description: String,
    pub kind: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakeShard {
    pub meta: StakeMeta,
    pub roles: Vec<StakeRole>,
}

impl StakeShard {
    pub fn find_host_role(&self, host_did: &str) -> Option<&StakeRole> {
        self.roles
            .iter()
            .find(|r| r.did == host_did && r.invariants.as_ref().map(|iv| iv.must_match_host).unwrap_or(false))
    }
}
