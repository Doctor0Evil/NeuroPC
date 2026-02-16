use serde::{Deserialize, Serialize};
use crate::sovereigntycore::{
    AuditEntry,
    BiophysicalStateReader,
    DecisionOutcome,
    EvolveToken,
    SovereigntyCore,
    StateVector,
    UpdateEffectBounds,
    UpdateKind,
    UpdateProposal,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomechProfile {
    pub id: String,
    pub gain: f32,          // 0.0–1.0, abstract assist “strength”
    pub smoothing: f32,     // 0.0–1.0, how quickly it reacts
    pub max_duration_ms: u32
}

pub struct BiomechController<'a, S: BiophysicalStateReader> {
    pub modulename: String,
    pub profile: BiomechProfile,
    pub sovereignty: &'a mut SovereigntyCore<S>,
}

impl<'a, S: BiophysicalStateReader> BiomechController<'a, S> {
    pub fn propose_tune(
        &mut self,
        new_gain: f32,
        new_smoothing: f32,
        evolvetoken_id: Option<&str>,
    ) -> AuditEntry {
        let delta_gain = (new_gain - self.profile.gain).abs();
        let delta_smoothing = (new_smoothing - self.profile.smoothing).abs();
        let l2delta = (delta_gain.powi(2) + delta_smoothing.powi(2)).sqrt();

        let proposal = UpdateProposal {
            id: format!("biomech-update-{}", self.profile.id),
            module: self.modulename.clone(),
            kind: UpdateKind::ParamNudge,
            scope: vec!["biomech".to_string(), "motormacros".to_string()],
            description: "Adjust biomechanical assist gain/smoothing".to_string(),
            effectbounds: UpdateEffectBounds {
                l2deltanorm: l2delta,
                irreversible: false,
            },
            requiresevolve: true,
        };

        let audit = self
            .sovereignty
            .evaluateupdate(proposal, evolvetoken_id);

        if matches!(audit.decision, DecisionOutcome::Allowed) {
            self.profile.gain = new_gain;
            self.profile.smoothing = new_smoothing;
        }

        audit
    }

    /// Read current biophysical state (fatigue, pain, etc.) through the abstract interface.
    pub fn current_state(&self) -> StateVector {
        self.sovereignty.statereader.readstate()
    }
}
