use serde::{Deserialize, Serialize};

use crate::sovereignty_core::{
    AuditEntry, SovereigntyCore, UpdateEffectBounds, UpdateKind, UpdateProposal,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleProfile {
    pub id: String,
    pub formality: f32,
    pub directness: f32,
}

pub struct LanguageCowriter<'a, S> {
    pub module_name: String,
    pub style: StyleProfile,
    pub sovereignty: &'a mut SovereigntyCore<S>,
}

impl<'a, S> LanguageCowriter<'a, S> {
    /// Present diffs externally; here we only manage persistent style changes.
    pub fn propose_style_tweak(
        &mut self,
        new_formality: f32,
        new_directness: f32,
        evolve_token_id: Option<&str>,
    ) -> AuditEntry {
        let delta_formality = (new_formality - self.style.formality).abs();
        let delta_directness = (new_directness - self.style.directness).abs();
        let l2_delta = ((delta_formality.powi(2) + delta_directness.powi(2)) as f32).sqrt();

        let proposal = UpdateProposal {
            id: format!("style-update-{}", self.style.id),
            module: self.module_name.clone(),
            kind: UpdateKind::ParamNudge,
            scope: vec!["language_tuning".to_string()],
            description: "Adjust language style parameters".to_string(),
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
            self.style.formality = new_formality;
            self.style.directness = new_directness;
        }

        audit
    }
}
