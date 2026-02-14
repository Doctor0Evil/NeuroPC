use crate::stake::StakeConfig;
use crate::neurorights_guard::NeurorightsPolicy;
use organiccpualn::evolvestream::EvolutionProposal;

/// Result of irreversibility check.
pub enum IrreversibilityDecision {
    Allowed,
    Rejected(String),
}

pub fn guard_irreversibility(
    proposal: &EvolutionProposal,
    stake: &StakeConfig,
    neurorights: &NeurorightsPolicy,
) -> IrreversibilityDecision {
    let irr = proposal.effect_bounds.irreversible;

    // If not irreversible, always allowed here.
    if !irr {
        return IrreversibilityDecision::Allowed;
    }

    // Cross-species irreversible always forbidden.
    if proposal.domain_tags.contains(&"cross-species".to_string())
        && neurorights
            .multi_species
            .as_ref()
            .map(|m| m.forbid_irreversible_cross_species)
            .unwrap_or(true)
    {
        return IrreversibilityDecision::Rejected(
            "Irreversible cross-species evolution forbidden".to_string(),
        );
    }

    // Require that this is Host self-arch evolution scope.
    if proposal.scope_id != "host_self_arch_evolution" {
        return IrreversibilityDecision::Rejected(format!(
            "Irreversible proposals only allowed for host_self_arch_evolution, got {}",
            proposal.scope_id
        ));
    }

    if !neurorights
        .reversibility
        .as_ref()
        .map(|r| r.host_may_choose_irreversible)
        .unwrap_or(false)
    {
        return IrreversibilityDecision::Rejected(
            "Neurorights policy does not allow Host-chosen irreversibility".to_string(),
        );
    }

    // Check that signer set satisfies Host-only self evolution.
    let ok = stake.validate_multisig(
        &proposal.scope_id,
        &proposal.signer_dids,
        "EVOLVE",
    );
    if !ok {
        return IrreversibilityDecision::Rejected(
            "Irreversible self-evolution requires valid Host EVOLVE signature".to_string(),
        );
    }

    IrreversibilityDecision::Allowed
}
