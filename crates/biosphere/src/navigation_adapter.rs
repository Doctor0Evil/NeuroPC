use std::time::SystemTime;
use uuid::Uuid;

use crate::traits::{EvolutionToken, TraitId, TraitKind};

/// High-level environmental summary that the navigation adapter can use.
/// This is deliberately low-dimensional and task-specific (no raw neural data).
#[derive(Debug, Clone)]
pub struct NavigationContext {
    pub obstacle_density: f32,   // 0.0..=1.0
    pub ambient_noise: f32,      // 0.0..=1.0
    pub crowd_pressure: f32,     // 0.0..=1.0
    pub requested_heading_deg: f32, // user/host desired direction
}

/// Minimal view into neuromorphic navigation "organ" parameters.
/// These would be derived from local OECT/OECN hardware drivers.
#[derive(Debug, Clone)]
pub struct NavigationParams {
    pub spike_rate_hz: f32,
    pub sensitivity_band: f32, // 0.0..=1.0 sensitivity to obstacles
    pub suppression_band: f32, // 0.0..=1.0 suppression of non-critical stimuli
}

impl NavigationParams {
    pub fn clamp(&mut self) {
        self.spike_rate_hz = self.spike_rate_hz.clamp(0.0, 200.0);
        self.sensitivity_band = self.sensitivity_band.clamp(0.0, 1.0);
        self.suppression_band = self.suppression_band.clamp(0.0, 1.0);
    }
}

/// A navigation adapter is an "agent" that can (a) read state from
/// neuromorphic hardware drivers and (b) propose small evolution tokens
/// consistent with a given context.
pub trait NavigationAdapter {
    fn trait_id(&self) -> TraitId;

    /// Fetch current parameters from the local neuromorphic navigation organ.
    fn read_params(&self) -> NavigationParams;

    /// Given current params + context, propose at most N evolution tokens
    /// that adjust sensitivity/suppression/spike rate in a reversible way.
    fn propose_tokens(
        &self,
        now: SystemTime,
        context: &NavigationContext,
        max_tokens: u32,
    ) -> Vec<EvolutionToken>;

    /// Apply a checked evolution token to the underlying hardware driver.
    /// This is only called after the scheduler has validated the token against
    /// neurorights, consent, safety state, and lane budgets.
    fn apply_token(&mut self, token: &EvolutionToken) -> anyhow::Result<()>;
}

/// A simple reference implementation using local fields to represent
/// neuromorphic parameters; in a real system, this would bridge to hardware.
pub struct LocalNavigationAdapter {
    id: TraitId,
    params: NavigationParams,
}

impl LocalNavigationAdapter {
    pub fn new(version: u32) -> Self {
        Self {
            id: TraitId {
                kind: TraitKind::Navigation,
                version,
            },
            params: NavigationParams {
                spike_rate_hz: 20.0,
                sensitivity_band: 0.5,
                suppression_band: 0.2,
            },
        }
    }

    fn delta_token(&self, label: &str, effect_band: f32) -> EvolutionToken {
        EvolutionToken {
            id: Uuid::new_v4(),
            trait_id: self.id.clone(),
            delta_label: label.to_string(),
            cost_bands: crate::traits::BudgetBands::conservative(),
            expected_effect_band: effect_band.clamp(0.0, 1.0),
            reversible: true,
        }
    }
}

impl NavigationAdapter for LocalNavigationAdapter {
    fn trait_id(&self) -> TraitId {
        self.id.clone()
    }

    fn read_params(&self) -> NavigationParams {
        self.params.clone()
    }

    fn propose_tokens(
        &self,
        _now: SystemTime,
        context: &NavigationContext,
        max_tokens: u32,
    ) -> Vec<EvolutionToken> {
        let mut tokens = Vec::new();
        if max_tokens == 0 {
            return tokens;
        }

        // Example heuristics:
        // - If obstacle_density is high, gently increase sensitivity_band.
        // - If ambient_noise & crowd_pressure are high, increase suppression_band.
        let mut remaining = max_tokens;

        if context.obstacle_density > 0.6 && remaining > 0 {
            tokens.push(self.delta_token("nav.sensitivity+0.05", 0.15));
            remaining -= 1;
        }

        if (context.ambient_noise > 0.6 || context.crowd_pressure > 0.6) && remaining > 0 {
            tokens.push(self.delta_token("nav.suppression+0.05", 0.15));
            remaining -= 1;
        }

        tokens
    }

    fn apply_token(&mut self, token: &EvolutionToken) -> anyhow::Result<()> {
        // Interpret delta_label in a minimal, auditable grammar.
        match token.delta_label.as_str() {
            "nav.sensitivity+0.05" => {
                self.params.sensitivity_band += 0.05;
            }
            "nav.sensitivity-0.05" => {
                self.params.sensitivity_band -= 0.05;
            }
            "nav.suppression+0.05" => {
                self.params.suppression_band += 0.05;
            }
            "nav.suppression-0.05" => {
                self.params.suppression_band -= 0.05;
            }
            _ => {
                // Unknown delta; in a sovereign system, we refuse to apply it.
                anyhow::bail!("unsupported delta_label: {}", token.delta_label);
            }
        }

        self.params.clamp();
        Ok(())
    }
}
