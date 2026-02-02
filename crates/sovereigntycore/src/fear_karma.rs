//! Fear contribution and KARMA gain accounting for NeuroPC / OrganicCPU.
//!
//! This module is *numerical only*: it never models souls, afterlife,
//! or metaphysical states. It just turns bounded "fear load" into
//! evolution-scored credits under RoH ≤ 0.3, neurorights, and stake rules.

use serde::{Deserialize, Serialize};

/// Bounded, normalized fear metrics for a single session.
/// All values are in [0,1] and must be validated before use.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FearEnvelope {
    /// Instantaneous subjective fear intensity [0,1].
    pub fear_level: f32,
    /// Rate of change of fear per second, normalized [0,1].
    pub fear_rate: f32,
    /// Session duration in seconds, clamped by SANITY policy.
    pub duration_secs: f32,
    /// Psychophysiological load index (e.g., HRV + sensor fusion) [0,1].
    pub psych_load: f32,
}

/// Output of one evaluation step: how much fear was "spent" and
/// what KARMA credits are earned for governance/evolution records.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FearKarmaOutcome {
    /// Effective fear spent in this step (dimensionless 0–1).
    pub fear_spent: f32,
    /// KARMA credits earned (can be logged to .evolve.jsonl / donutloop).
    pub karma_earned: f32,
}

/// Static parameters that map fear→KARMA under safety constraints.
/// These should be stored as data (e.g., ALN/JSON) and loaded via sovereigntycore,
/// similar to RoH and neurorights.[file:10][file:14]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FearKarmaPolicy {
    /// Maximum allowed instantaneous fear level.
    pub max_fear_level: f32,
    /// Maximum allowed psych load.
    pub max_psych_load: f32,
    /// Hard cap on per-session fear_spent.
    pub max_fear_spent_per_session: f32,
    /// Scaling factor from fear_spent to KARMA.
    pub karma_per_fear_unit: f32,
}

impl FearEnvelope {
    /// Validate simple invariants and clamp to [0,1].
    pub fn validated(self) -> FearEnvelope {
        fn clamp01(x: f32) -> f32 {
            if x < 0.0 {
                0.0
            } else if x > 1.0 {
                1.0
            } else {
                x
            }
        }

        FearEnvelope {
            fear_level: clamp01(self.fear_level),
            fear_rate: clamp01(self.fear_rate),
            duration_secs: if self.duration_secs.is_finite() && self.duration_secs > 0.0 {
                self.duration_secs
            } else {
                0.0
            },
            psych_load: clamp01(self.psych_load),
        }
    }
}

impl FearKarmaPolicy {
    /// Simple invariant check; more complex guards live in sovereigntycore.
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.max_fear_level) {
            return Err("max_fear_level must be in [0,1]".into());
        }
        if !(0.0..=1.0).contains(&self.max_psych_load) {
            return Err("max_psych_load must be in [0,1]".into());
        }
        if self.max_fear_spent_per_session <= 0.0 {
            return Err("max_fear_spent_per_session must be > 0".into());
        }
        if self.karma_per_fear_unit < 0.0 {
            return Err("karma_per_fear_unit must be ≥ 0".into());
        }
        Ok(())
    }

    /// Core mapping from a single FearEnvelope to fear_spent + KARMA.
    ///
    /// This should be called *after* RoH ≤ 0.3, neurorights, and stake multisig
    /// checks in sovereigntycore; it does not override those guards.[file:10][file:14]
    pub fn evaluate_step(&self, env: FearEnvelope) -> FearKarmaOutcome {
        let env = env.validated();

        // Safety clipping: if any component exceeds policy, no KARMA is earned.
        if env.fear_level > self.max_fear_level || env.psych_load > self.max_psych_load {
            return FearKarmaOutcome {
                fear_spent: 0.0,
                karma_earned: 0.0,
            };
        }

        // Effective fear "pressure": intensity × rate × normalized duration.
        // Duration contribution is damped by 1 / (1 + duration) to prevent
        // long low-level sessions from dominating.
        let dur_factor = 1.0 / (1.0 + env.duration_secs / 60.0);
        let raw_fear_spent = env.fear_level * env.fear_rate * dur_factor;

        // Clip by policy per-session cap.
        let fear_spent = raw_fear_spent.min(self.max_fear_spent_per_session);

        // KARMA is linear in fear_spent and always ≥ 0.
        let karma_earned = fear_spent * self.karma_per_fear_unit;

        FearKarmaOutcome {
            fear_spent,
            karma_earned,
        }
    }
}
