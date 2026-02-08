use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeforceAxis {
    pub min: f32,
    pub critical: f32,
    pub max_daily_depletion: f32,
    pub max_hourly_depletion: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeforceMode {
    pub Lmin: f32,
    pub Lcrit: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeforceEnvelopeSpec {
    pub version: u32,
    pub axes: std::collections::HashMap<String, LifeforceAxis>,
    pub modes: std::collections::HashMap<String, LifeforceMode>,
}

#[derive(Debug, Clone)]
pub struct LifeforceEnvelope {
    axis: LifeforceAxis,
    modes: std::collections::HashMap<String, LifeforceMode>,
}

#[derive(thiserror::Error, Debug)]
pub enum LifeforceError {
    #[error("error loading lifeforce.aln: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error in lifeforce.aln: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("invariant violation: {0}")]
    Invariant(String),
    #[error("unknown mode: {0}")]
    UnknownMode(String),
    #[error("lifeforce envelope violation: {0}")]
    Violation(String),
}

impl LifeforceEnvelope {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, LifeforceError> {
        let raw = fs::read_to_string(path)?;
        let spec: LifeforceEnvelopeSpec = serde_json::from_str(&raw)?;
        let axis = spec
            .axes
            .get("lifeforce")
            .cloned()
            .ok_or_else(|| LifeforceError::Invariant("missing lifeforce axis".into()))?;
        if !(0.0..=1.0).contains(&axis.min)
            || !(0.0..=1.0).contains(&axis.critical)
            || axis.critical <= 0.0
            || axis.min <= axis.critical
        {
            return Err(LifeforceError::Invariant(
                "invalid lifeforce bounds; require 0<critical<min<=1.0".into(),
            ));
        }
        Ok(Self {
            axis,
            modes: spec.modes,
        })
    }

    pub fn mode_bounds(&self, mode: &str) -> Result<LifeforceMode, LifeforceError> {
        self.modes
            .get(mode)
            .cloned()
            .ok_or_else(|| LifeforceError::UnknownMode(mode.to_string()))
    }

    /// Core guard: given current lifeforce, projected depletion, and mode,
    /// decide whether the action is admissible.
    pub fn check_action(
        &self,
        mode: &str,
        current_lifeforce: f32,
        projected_delta: f32, // negative for depletion
    ) -> Result<(), LifeforceError> {
        if !(0.0..=1.0).contains(&current_lifeforce) {
            return Err(LifeforceError::Violation(format!(
                "current lifeforce {} out of [0,1]",
                current_lifeforce
            )));
        }
        let m = self.mode_bounds(mode)?;
        let projected = (current_lifeforce + projected_delta).clamp(0.0, 1.0);
        if projected < m.Lcrit {
            return Err(LifeforceError::Violation(format!(
                "projected lifeforce {} would fall below critical {} in mode {}",
                projected, m.Lcrit, mode
            )));
        }
        if projected < m.Lmin {
            return Err(LifeforceError::Violation(format!(
                "projected lifeforce {} would fall below Lmin {} in mode {}",
                projected, m.Lmin, mode
            )));
        }
        Ok(())
    }
}
