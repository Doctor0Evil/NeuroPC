use crate::types::{
    OrchestratorBioSnapshot,
    CyberNanoMode,
    CyberNanoBootRequest,
    CyberNanoBootDecision,
    CyberNanoBootError,
};
use organiccpu_core::{BioState, SafeEnvelopeDecision, SafeEnvelopePolicy};
use sovereignty_core::{
    SovereigntyCore,
    UpdateProposal,
    UpdateKind,
    UpdateEffectBounds,
    DecisionOutcome,
};

/// Entry-point: CyberNano asks to start a session inside OrganicCPU.
///
/// - Reads biophysical state.
/// - Applies safe-envelope policy (OrganicCPU).
/// - Applies neurorights + EVOLVE consent (sovereignty core).
/// - Returns a bounded mode and kernel id or an error.
///
/// CyberNano MUST treat any non-allowed result as a hard stop.
pub fn cybernano_boot<S, P>(
    sovereignty: &mut SovereigntyCore<S>,
    envelope_policy: &P,
    bio_state: &BioState,
    request: &CyberNanoBootRequest,
) -> Result<CyberNanoBootDecision, CyberNanoBootError>
where
    S: sovereignty_core::BiophysicalStateReader,
    P: SafeEnvelopePolicy,
{
    // 1) Summarize biophysical state for logs and for CyberNano.
    let snapshot = OrchestratorBioSnapshot::from(bio_state);

    // 2) Safe-envelope decision from OrganicCPU.
    let envelope_decision = envelope_policy.decide(bio_state.clone());
    if matches!(envelope_decision, SafeEnvelopeDecision::PauseAndRest) {
        return Err(CyberNanoBootError::EnvelopeRejected(
            "OrganicCPU envelope requires PauseAndRest; cannot boot CyberNano now.".into(),
        ));
    }

    // 3) Build a sovereignty "update proposal" for starting CyberNano.
    // Effect bounds are conservative: small param nudge only.
    let proposal = UpdateProposal {
        id: "cybernano-boot".to_string(),
        module: request.module_name.clone(),
        kind: UpdateKind::ParamNudge,
        scope: vec!["cybernano.session".into()],
        description: "Request to start CyberNano session under OrganicCPU shell".into(),
        effect_bounds: UpdateEffectBounds {
            l2_delta_norm: 0.01,
            irreversible: false,
        },
        requires_evolve: matches!(request.requested_mode, CyberNanoMode::SafeFilterPlusEvolution),
    };

    // 4) Neurorights + EVOLVE evaluation.
    let audit = sovereignty.evaluate_update(
        &proposal,
        request.evolve_token_id.as_deref(),
    );

    if !matches!(audit.decision, DecisionOutcome::Allowed) {
        // Map common rejection reasons into structured errors.
        let reason = audit.reason.clone();

        if reason.contains("Missing EVOLVE token") {
            return Err(CyberNanoBootError::MissingEvolveToken(reason));
        }
        if reason.contains("Unknown EVOLVE token") {
            return Err(CyberNanoBootError::UnknownEvolveToken(reason));
        }
        if reason.contains("Integration depth forbids this module") {
            return Err(CyberNanoBootError::IntegrationDepthForbidden(reason));
        }

        return Err(CyberNanoBootError::NeurorightsRejected(reason));
    }

    // 5) Determine granted mode (sovereignty core may downgrade active mode).
    let granted_mode = match request.requested_mode {
        CyberNanoMode::SafeFilterPlusEvolution => {
            // If active mode doesn't allow auto-evolve, downgrade to SafeFilterOnly.
            let mode_policy = sovereignty.active_mode_policy();
            if mode_policy.allow_auto_evolve {
                CyberNanoMode::SafeFilterPlusEvolution
            } else {
                CyberNanoMode::SafeFilterOnly
            }
        }
        other => other,
    };

    // 6) Kernel profile is advisory: sovereignty/ALN policies can refine this later.
    let decision = CyberNanoBootDecision {
        allowed: true,
        granted_mode,
        granted_kernel_id: request.requested_kernel_id.clone(),
        envelope_decision,
        bio_snapshot: snapshot,
        reason: "CyberNano session allowed under current neurorights and envelopes.".into(),
    };

    Ok(decision)
}
