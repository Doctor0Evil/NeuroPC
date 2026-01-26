use crate::asset_eco_governor::*;
use crate::lifeforce_governor::*;

pub struct EcoLifeforceCoGovernor {
    eco_governor: AssetEcoGovernor,
    lifeforce_governor: LifeforceGovernor,
}

impl EcoLifeforceCoGovernor {
    pub fn new(
        eco_policy: UpgradePolicy,
        lifeforce_policy: LifeforcePolicy,
        stakeholder_did: &str,
    ) -> Self {
        Self {
            eco_governor: AssetEcoGovernor::new(eco_policy, stakeholder_did),
            lifeforce_governor: LifeforceGovernor::new(lifeforce_policy, stakeholder_did),
        }
    }

    pub fn decide_dw_spend(
        &self,
        tokens: &mut HostTokens,
        eco_profile: &NightlyEcoProfile,
        biophysical_status: &BiophysicalStatus,
        intensity_01: f32,
        signature: DIDSignature,
    ) -> (UpgradeDecision, DwSpendDecision) {
        // Step 1: Check eco-governor approval
        let eco_decision = self.eco_governor.decide_upgrade(
            tokens,
            eco_profile,
            IntegrationType::EcoOptimizer,
            0, // eco_cost for DW is handled by lifeforce
            0, // evolution_cost for DW is handled by lifeforce
            Some(signature.clone()),
        );

        if !eco_decision.allowed {
            return (eco_decision, DwSpendDecision {
                allowed: false,
                reason: format!("Eco-governor rejected: {}", eco_decision.reason),
                blood_spent: 0,
                protein_spent: 0,
                sugar_spent: 0,
                dw_spent: 0,
            });
        }

        // Step 2: Check lifeforce-governor approval
        let lifeforce_decision = self.lifeforce_governor.decide_dw_spend(
            tokens,
            biophysical_status,
            intensity_01,
            Some(signature.clone()),
        );

        if !lifeforce_decision.allowed {
            return (eco_decision, lifeforce_decision);
        }

        // Both approvals passed: return both decisions
        (eco_decision, lifeforce_decision)
    }
}
