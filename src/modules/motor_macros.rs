use serde::{Deserialize, Serialize};

use crate::sovereignty_core::{
    AuditEntry, EvolveToken, SovereigntyCore, UpdateEffectBounds, UpdateKind, UpdateProposal,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorMacroProfile {
    pub id: String,
    pub sensitivity: f32,
    pub smoothing: f32,
    pub max_macro_duration_ms: u32,
}

pub struct MotorMacroController<'a, S> {
    pub module_name: String,
    pub profile: MotorMacroProfile,
    pub sovereignty: &'a mut SovereigntyCore<S>,
}

impl<'a, S> MotorMacroController<'a, S> {
    pub fn propose_adaptation(
        &mut self,
        new_sensitivity: f32,
        new_smoothing: f32,
        evolve_token_id: Option<&str>,
    ) -> AuditEntry {
        let delta_sens = (new_sensitivity - self.profile.sensitivity).abs();
        let delta_smooth = (new_smoothing - self.profile.smoothing).abs();
        let l2_delta = ((delta_sens.powi(2) + delta_smooth.powi(2)) as f32).sqrt();

        let proposal = UpdateProposal {
            id: format!("motor-macro-update-{}", self.profile.id),
            module: self.module_name.clone(),
            kind: UpdateKind::ParamNudge,
            scope: vec!["motor_macros".to_string()],
            description: "Adjust motor macro sensitivity and smoothing".to_string(),
            effect_bounds: UpdateEffectBounds {
                l2_delta_norm: l2_delta,
                irreversible: false,
            },
            requires_evolve: true,
        };

        let audit = self
            .sovereignty
            .evaluate_update(&proposal, evolve_token_id);

        if matches!(audit.decision, crate::sovereignty_core::DecisionOutcome::Allowed) {
            self.profile.sensitivity = new_sensitivity;
            self.profile.smoothing = new_smoothing;
        }

        audit
    }
}
