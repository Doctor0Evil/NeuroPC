use organiccpualn::rohmodel::{RohModel, StateVector};

/// Biocompatibility rating in [0,1].
pub const NPF_ROH_SAFE_HINT_BCR: f32 = 0.31;

/// Minimal view of an AI-assisted action we want to score.
#[derive(Clone, Debug)]
pub struct AssistedActionContext {
    /// 0..1 subjective cognitive load (e.g., from bioscale metrics).
    pub cognitive_load: f32,
    /// 0..1 "complexity" of the requested change (size, cross-crate impact).
    pub change_complexity: f32,
    /// 0..1 "governance" sensitivity (touches .aln/.stake/.neurorights, etc.).
    pub governance_sensitivity: f32,
}

/// Result: scalar in [0, 0.3], plus a boolean "near-ceiling" hint.
#[derive(Clone, Debug)]
pub struct RohSafeHint {
    pub roh_estimate: f32,
    pub near_ceiling: bool,
}

/// Neuroprint-function:
/// - Builds a tiny StateVector from 3 axes and runs your RoH model.
/// - Clamps result to global ceiling.
/// - Marks near_ceiling when within epsilon of 0.3.
/// - Intended to gate **suggestions** (e.g., “maybe split this into smaller PRs”),
///   never as a direct block on user action.
pub fn npf_roh_safe_hint(model: &RohModel, ctx: &AssistedActionContext) -> RohSafeHint {
    // Map into a 3D state aligned with your existing axes conceptually.
    let components = vec![
        ctx.cognitive_load.max(0.0).min(1.0),
        ctx.change_complexity.max(0.0).min(1.0),
        ctx.governance_sensitivity.max(0.0).min(1.0),
    ];
    let state = StateVector { components };

    let mut roh = model.compute_roh(state);
    // Global ceiling enforced by RohModel, but we clamp defensively.
    if roh > RohModel::ROH_CEILING {
        roh = RohModel::ROH_CEILING;
    }

    let near_ceiling = (RohModel::ROH_CEILING - roh) < 0.03;

    RohSafeHint {
        roh_estimate: roh,
        near_ceiling,
    }
}
