use std::cmp::Ordering;
use crate::did_verification::{DIDSignature, DIDVerificationResult, verify_did_signature, did_to_public_key};

#[derive(Debug, Clone)]
pub struct LifeforceGovernor {
    policy: LifeforcePolicy,
    stakeholder_did: String,
}

impl LifeforceGovernor {
    pub fn new(policy: LifeforcePolicy, stakeholder_did: &str) -> Self {
        Self {
            policy,
            stakeholder_did: stakeholder_did.to_string(),
        }
    }

    // DID verification helper
    pub fn verify_did_signature(
        &self,
        signature: &DIDSignature,
        message: &str,
    ) -> Result<DIDVerificationResult, anyhow::Error> {
        let public_key = did_to_public_key(&self.stakeholder_did)?;
        verify_did_signature(signature, &public_key)
    }

    pub fn decide_dw_spend(
        &self,
        tokens: &mut HostTokens,
        status: &BiophysicalStatus,
        intensity_01: f32,
        signature: Option<DIDSignature>,
    ) -> DwSpendDecision {
        // Verify DID signature before processing
        if let Some(sig) = signature {
            let message = format!(
                "dw_event:intensity:{}",
                intensity_01
            );
            match self.verify_did_signature(&sig, &message) {
                Ok(DIDVerificationResult::Valid) => {}
                Ok(DIDVerificationResult::Invalid) => {
                    return DwSpendDecision {
                        allowed: false,
                        reason: "Invalid DID signature for DW event".to_string(),
                        blood_spent: 0,
                        protein_spent: 0,
                        sugar_spent: 0,
                        dw_spent: 0,
                    };
                }
                Ok(DIDVerificationResult::MissingSignature) => {
                    return DwSpendDecision {
                        allowed: false,
                        reason: "Missing DID signature for DW event".to_string(),
                        blood_spent: 0,
                        protein_spent: 0,
                        sugar_spent: 0,
                        dw_spent: 0,
                    };
                }
                Ok(DIDVerificationResult::InvalidFormat) => {
                    return DwSpendDecision {
                        allowed: false,
                        reason: "Invalid DID signature format".to_string(),
                        blood_spent: 0,
                        protein_spent: 0,
                        sugar_spent: 0,
                        dw_spent: 0,
                    };
                }
                Err(e) => {
                    return DwSpendDecision {
                        allowed: false,
                        reason: format!("DID verification error: {}", e),
                        blood_spent: 0,
                        protein_spent: 0,
                        sugar_spent: 0,
                        dw_spent: 0,
                    };
                }
            }
        } else {
            return DwSpendDecision {
                allowed: false,
                reason: "DID signature required for DW event".to_string(),
                blood_spent: 0,
                protein_spent: 0,
                sugar_spent: 0,
                dw_spent: 0,
            };
        }

        // [Existing decision logic remains unchanged]
    }
}
