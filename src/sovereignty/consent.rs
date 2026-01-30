#![forbid(unsafe_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Awareness token semantics for consent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwarenessToken {
    Query,   // low-risk, discovery
    Commit,  // structural/motor commit
    Insight, // new struct / schema
}

/// Core ConsentObject as stored on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentObject {
    pub id: String,
    pub token: AwarenessToken,
    pub target_id: String,
    pub owner_id: String,
    pub created_at: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
    pub revoked: bool,
}

impl ConsentObject {
    /// Simple validity check against time and revocation.
    pub fn is_currently_valid(&self, now: DateTime<Utc>, owner: &str) -> bool {
        if self.revoked {
            return false;
        }
        if self.owner_id != owner {
            return false;
        }
        if let Some(expiry) = self.valid_until {
            if now > expiry {
                return false;
            }
        }
        true
    }
}

/// Associate OTA actions with required token kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiredToken {
    Any,    // e.g., for Discover/Download
    Commit, // Commit / Rollback
}

pub struct ConsentStore {
    root: PathBuf,
}

impl ConsentStore {
    /// root_dir is the directory where .cobj files live.
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        Self {
            root: root_dir.as_ref().to_path_buf(),
        }
    }

    /// Load all .cobj files from disk.
    fn load_all(&self) -> std::io::Result<Vec<ConsentObject>> {
        let mut result = Vec::new();
        if !self.root.exists() {
            return Ok(result);
        }
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("cobj") {
                let data = fs::read_to_string(&path)?;
                if let Ok(cobj) = serde_json::from_str::<ConsentObject>(&data) {
                    result.push(cobj);
                }
            }
        }
        Ok(result)
    }

    /// Find a consent object that matches owner, target and token requirement.
    pub fn find_valid_for(
        &self,
        owner_id: &str,
        target_id: &str,
        required: RequiredToken,
        now: DateTime<Utc>,
    ) -> std::io::Result<Option<ConsentObject>> {
        let all = self.load_all()?;
        let mut best: Option<ConsentObject> = None;

        for c in all {
            if !c.is_currently_valid(now, owner_id) {
                continue;
            }
            if c.target_id != target_id {
                continue;
            }
            match required {
                RequiredToken::Any => {
                    best = Some(c);
                    break;
                }
                RequiredToken::Commit => {
                    if c.token == AwarenessToken::Commit {
                        best = Some(c);
                        break;
                    }
                }
            }
        }

        Ok(best)
    }
}
