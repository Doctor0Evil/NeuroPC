use crate::policies::biomech_integration::BiomechIntegrationPolicy;
use crate::sovereigntycore::{Guard, GuardDecision, GuardOutcome, UpdateProposal};

pub struct BiomechGuard {
    pub policy: BiomechIntegrationPolicy,
}

impl BiomechGuard {
    pub fn new(policy: BiomechIntegrationPolicy) -> Self {
        Self { policy }
    }
}

impl Guard for BiomechGuard {
    fn name(&self) -> &str {
        "biomech_guard"
    }

    fn evaluate(&self, proposal: &UpdateProposal) -> GuardDecision {
        if !proposal.scope.contains(&"biomech".to_string()) {
            return GuardDecision::allow("non-biomech scope");
        }

        let module_id = &proposal.module;

        let module = match self.policy.find_module(module_id) {
            Some(m) => m,
            None => {
                return GuardDecision::deny(
                    GuardOutcome::PolicyViolation,
                    format!("Module {} not declared in biomech policy", module_id),
                )
            }
        };

        if module.integrationrole == "forbidden" {
            return GuardDecision::deny(
                GuardOutcome::PolicyViolation,
                format!("Module {} is forbidden by biomech policy", module_id),
            );
        }

        if proposal.effectbounds.l2deltanorm > module.maxeffectsize {
            return GuardDecision::deny(
                GuardOutcome::PolicyViolation,
                format!(
                    "Proposed effect {} exceeds maxeffectsize {} for module {}",
                    proposal.effectbounds.l2deltanorm, module.maxeffectsize, module_id
                ),
            );
        }

        GuardDecision::allow("biomech policy satisfied")
    }
}
