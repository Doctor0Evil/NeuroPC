use serde::{Deserialize, Serialize};

/// Minimal RoH view, consistent with existing RoH 0.3 governance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohSnapshot {
    /// Global Risk-of-Harm scalar, normalized, hard-clamped in [0.0, 0.3] for CapControlledHuman.
    pub roh: f32,
    /// Optional domain tags, e.g. ["CNS", "CARDIO", "ECO"].
    pub domains: Vec<String>,
}

/// Nicotine neuroprint diagnostics (normalized indices; all advisory).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NicotineNeuroprintDiag {
    pub p300_latency_index: f32,
    pub p300_amplitude_index: f32,
    pub hf_hrv_index: f32,
    pub pupil_constriction_velocity_index: f32,
    pub eda_reactivity_index: f32,
    pub keystroke_entropy_index: f32,
    pub rt_variance_index: f32,
    pub microsaccade_stability_index: f32,
}

/// Advisory-only classification derived from the nicotine_neuroprint.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NicotineNeuroprintBand {
    /// No reliable nicotine-related pattern present.
    Neutral,
    /// Pattern consistent with acute abstinence (higher variance, slower P300, higher HF-HRV index).
    AbstinenceLike,
    /// Pattern consistent with acute nicotine / stabilized attention window.
    NicotineLike,
}

/// Read-only advisory output; never used for actuation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NicotineNeuroprintView {
    pub band: NicotineNeuroprintBand,
    pub composite_index: f32,
    pub roh_snapshot: RohSnapshot,
}

/// Pure diagnostic guard: maps neuroprint + RoH into an advisory view.
/// Invariants:
/// - Does not mutate capability or consent.
/// - Does not change RoH; caller must enforce RoH_after <= RoH_before <= 0.3.
pub trait RohNeuroprintGuard {
    fn classify_neuroprint(
        &self,
        roh: &RohSnapshot,
        diag: &NicotineNeuroprintDiag,
    ) -> NicotineNeuroprintView;
}

/// Default, non-actuating implementation.
/// Thresholds are internal, config-derived, and may be updated via EVOLVE,
/// but never loosened in ways that would increase RoH for CapControlledHuman.
pub struct DefaultRohNeuroprintGuard;

impl DefaultRohNeuroprintGuard {
    pub fn new() -> Self {
        DefaultRohNeuroprintGuard
    }

    fn compute_composite(diag: &NicotineNeuroprintDiag) -> f32 {
        let central_speed = 1.0 - diag.p300_latency_index;
        let autonomic_shift = 0.5 * (1.0 - diag.hf_hrv_index)
            + 0.3 * diag.pupil_constriction_velocity_index
            + 0.2 * (1.0 - diag.eda_reactivity_index);
        let behavioral_stability = 0.4 * (1.0 - diag.keystroke_entropy_index)
            + 0.4 * (1.0 - diag.rt_variance_index)
            + 0.2 * diag.microsaccade_stability_index;
        let composite = (central_speed + autonomic_shift + behavioral_stability) / 3.0;
        composite.clamp(0.0, 1.0)
    }

    fn classify_band(composite: f32) -> NicotineNeuroprintBand {
        if composite >= 0.66 {
            NicotineNeuroprintBand::NicotineLike
        } else if composite <= 0.33 {
            NicotineNeuroprintBand::AbstinenceLike
        } else {
            NicotineNeuroprintBand::Neutral
        }
    }
}

impl RohNeuroprintGuard for DefaultRohNeuroprintGuard {
    fn classify_neuroprint(
        &self,
        roh: &RohSnapshot,
        diag: &NicotineNeuroprintDiag,
    ) -> NicotineNeuroprintView {
        // RoH invariant is enforced by the caller; this guard never changes it.
        let composite = Self::compute_composite(diag);
        let band = Self::classify_band(composite);

        NicotineNeuroprintView {
            band,
            composite_index: composite,
            roh_snapshot: RohSnapshot {
                roh: roh.roh.clamp(0.0, 0.3),
                domains: roh.domains.clone(),
            },
        }
    }
}
