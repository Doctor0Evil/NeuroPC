use organic_cpu_orchestrator::{
    CyberNanoBootRequest,
    CyberNanoMode,
    cybernano_boot,
};
use sovereignty_core::SovereigntyCore;
use organiccpu_core::{BioState, SafeEnvelopePolicy};

pub fn start_cybernano_session<S, P>(
    sovereignty: &mut SovereigntyCore<S>,
    envelope_policy: &P,
    bio_state: &BioState,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: sovereignty_core::BiophysicalStateReader,
    P: SafeEnvelopePolicy,
{
    let request = CyberNanoBootRequest {
        module_name: "cybernano.viability_kernel".into(),
        requested_mode: CyberNanoMode::SafeFilterOnly,
        requested_kernel_id: Some("CN-VK-Baseline-2026v1".into()),
        evolve_token_id: None,
    };

    let decision = cybernano_boot(sovereignty, envelope_policy, bio_state, &request)?;

    if !decision.allowed {
        return Err("CyberNano boot denied".into());
    }

    // CyberNano must now honor:
    // - decision.granted_mode
    // - decision.granted_kernel_id
    // - decision.envelope_decision
    // and treat decision.bio_snapshot as read-only context.

    Ok(())
}
