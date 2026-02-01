use serde::{Deserialize, Serialize};
use organiccpu_core::{BioState, SafeEnvelopeDecision, SafeEnvelopePolicy};
use sovereignty_core::{SovereigntyCore, UpdateEffectBounds, UpdateKind, UpdateProposal};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CyberNanoMode {
    Observe,
    SafeFilterOnly,
    SafeFilterPlusEvolution,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CyberNanoBootRequest {
    pub module_name: String,
    pub requested_mode: CyberNanoMode,
    pub requested_kernel_id: Option<String>,
    pub evolve_token_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrchestratorBioSnapshot {
    pub fatigue_index: f32,
    pub duty_cycle: f32,
    pub cognitive_load_index: f32,
    pub intent_confidence: f32,
    pub eco_impact_score: f32,
    pub device_hours: f32,
}

impl From<&BioState> for OrchestratorBioSnapshot {
    fn from(s: &BioState) -> Self {
        Self {
            fatigue_index: s.fatigue_index,
            duty_cycle: s.duty_cycle,
            cognitive_load_index: s.cognitive_load_index,
            intent_confidence: s.intent_confidence,
            eco_impact_score: s.eco.eco_impact_score,
            device_hours: s.eco.device_hours,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CyberNanoBootDecision {
    pub allowed: bool,
    pub granted_mode: CyberNanoMode,
    pub granted_kernel_id: Option<String>,
    pub envelope_decision: SafeEnvelopeDecision,
    pub bio_snapshot: OrchestratorBioSnapshot,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CyberNanoBootError {
    NeurorightsRejected(String),
    EnvelopeRejected(String),
    MissingEvolveToken(String),
    UnknownEvolveToken(String),
    IntegrationDepthForbidden(String),
    InternalError(String),
}

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
    let snapshot = OrchestratorBioSnapshot::from(bio_state);
    let envelope_decision = envelope_policy.decide(bio_state.clone());

    if matches!(envelope_decision, SafeEnvelopeDecision::PauseAndRest) {
        return Err(CyberNanoBootError::EnvelopeRejected(
            "OrganicCPU envelope requires PauseAndRest; cannot boot CyberNano now.".into(),
        ));
    }

    let proposal = UpdateProposal {
        id: "cybernano-boot".to_string(),
        module: request.module_name.clone(),
        kind: UpdateKind::ParamNudge,
        scope: vec!["cyberswarm.session".into()],
        description: "Request to start CyberNano session under OrganicCPU shell".into(),
        effect_bounds: UpdateEffectBounds {
            l2_delta_norm: 0.01,
            irreversible: false,
        },
        requires_evolve: matches!(request.requested_mode, CyberNanoMode::SafeFilterPlusEvolution),
    };

    let audit = sovereignty.evaluate_update(
        &proposal,
        request.evolve_token_id.as_deref(),
        /*auto*/ matches!(request.requested_mode, CyberNanoMode::SafeFilterPlusEvolution),
    );

    if !matches!(audit.decision, sovereignty_core::DecisionOutcome::Allowed) {
        if audit.reason.contains("Missing EVOLVE token") {
            return Err(CyberNanoBootError::MissingEvolveToken(audit.reason));
        }
        if audit.reason.contains("Unknown EVOLVE token") {
            return Err(CyberNanoBootError::UnknownEvolveToken(audit.reason));
        }
        if audit.reason.contains("Pain envelope exceeded") {
            return Err(CyberNanoBootError::NeurorightsRejected(audit.reason));
        }
        return Err(CyberNanoBootError::NeurorightsRejected(audit.reason));
    }

    let granted_mode = match request.requested_mode {
        CyberNanoMode::SafeFilterPlusEvolution => CyberNanoMode::SafeFilterPlusEvolution,
        other => other,
    };

    Ok(CyberNanoBootDecision {
        allowed: true,
        granted_mode,
        granted_kernel_id: request.requested_kernel_id.clone(),
        envelope_decision,
        bio_snapshot: snapshot,
        reason: "CyberNano session allowed under current neurorights and envelopes.".into(),
    })
}
