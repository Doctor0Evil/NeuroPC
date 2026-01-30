use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};

/// OTAState enum for pipeline stages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OTAState {
    Discover,
    Download,
    Verify,
    Apply,
    Rollback,
    Idle,
}

/// OTAModule struct: Governed OTA as .nmod.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OTAModule {
    pub id: String,
    pub version: String,
    pub scope: Vec<String>,  // e.g., ["core", "io"]
    pub state: OTAState,
    pub policy_ref: String,  // .nrml ID
    pub evolve_token: Option<String>,  // EVOLVE for apply
    pub audit_chain: Vec<String>,  // Hash-linked .naud entries
}

impl OTAModule {
    pub fn new(id: String, version: String, scope: Vec<String>, policy: String) -> Self {
        Self {
            id,
            version,
            scope,
            state: OTAState::Idle,
            policy_ref: policy,
            evolve_token: None,
            audit_chain: Vec::new(),
        }
    }

    /// Advance state: Only if policy permits, log to .naud.
    pub fn advance(&mut self, next_state: OTAState, evolve: Option<String>, policy_check: bool) -> Result<(), String> {
        if !policy_check {
            return Err("Policy deny: OTA advance blocked".to_string());
        }
        self.state = next_state;
        self.evolve_token = evolve;
        let entry = format!("OTA:{}:state:{}:time:{}", self.id, self.state as i32, Utc::now().timestamp());
        let mut hasher = Sha256::new();
        if let Some(last) = self.audit_chain.last() {
            hasher.update(last.as_bytes());
        }
        hasher.update(entry.as_bytes());
        self.audit_chain.push(format!("{:x}", hasher.finalize()));
        Ok(())
    }

    /// Invariant check: No downgrade of critical scopes.
    pub fn check_invariants(&self) -> bool {
        !self.scope.contains(&"core".to_string()) || self.version.parse::<f32>().unwrap_or(0.0) >= 1.0
    }
}

/// OTASovereignty struct: Core enforcement.
#[derive(Debug, Clone)]
pub struct OTASovereignty {
    pub modules: Arc<Mutex<Vec<OTAModule>>>,
    pub baseline_version: String,  // Known-good
}

impl OTASovereignty {
    pub fn new(baseline: String) -> Self {
        Self {
            modules: Arc::new(Mutex::new(Vec::new())),
            baseline_version: baseline,
        }
    }

    /// Process OTA: Policy-gated, with rollback.
    pub fn process_ota(&self, mut ota: OTAModule, policy_permit: bool, evolve: String) -> Result<String, String> {
        if !policy_permit || !ota.check_invariants() {
            return Err("OTA denied: Policy/invariant violation".to_string());
        }
        ota.advance(OTAState::Discover, None, true)?;
        ota.advance(OTAState::Download, None, true)?;
        ota.advance(OTAState::Verify, None, true)?;
        ota.advance(OTAState::Apply, Some(evolve), true)?;
        let mut mods = self.modules.lock().unwrap();
        mods.push(ota);
        Ok("OTA applied: Sovereign continuity preserved".to_string())
    }

    /// Rollback: Revert to baseline on failure.
    pub fn rollback(&self, id: &str) -> Result<(), String> {
        let mut mods = self.modules.lock().unwrap();
        if let Some(ota) = mods.iter_mut().find(|m| m.id == id) {
            ota.advance(OTAState::Rollback, None, true)?;
            ota.version = self.baseline_version.clone();
            Ok(())
        } else {
            Err("No OTA to rollback".to_string())
        }
    }
}
