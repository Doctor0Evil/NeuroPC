// Refined Co-Governor Decision Flow
impl EcoLifeforceCoGovernor {
    pub fn decide_dw_spend(
        &self,
        tokens: &HostTokens, // Use immutable borrow first
        eco_profile: &NightlyEcoProfile,
        biophysical_status: &BiophysicalStatus,
        request: &DwSpendRequest, // Bundled request with signature
    ) -> CoGovernorDecision {

        // 1. IMMUTABLE VALIDATION PHASE
        let eco_check = self.eco_governor.validate(request, eco_profile);
        let lifeforce_check = self.lifeforce_governor.validate(request, biophysical_status);

        if !eco_check.allowed || !lifeforce_check.allowed {
            return CoGovernorDecision::rejected(eco_check, lifeforce_check);
        }

        // 2. MUTABLE EXECUTION PHASE (Only if validation passes)
        let mut token_mut = tokens.clone(); // Operate on a mutable copy
        let eco_result = self.eco_governor.execute(&mut token_mut, request);
        let lifeforce_result = self.lifeforce_governor.execute(&mut token_mut, request);

        // 3. ATOMIC COMMIT OR ROLLBACK
        if eco_result.final_ok && lifeforce_result.final_ok {
            commit_tokens(tokens, token_mut); // Atomic update
            CoGovernorDecision::approved(eco_result, lifeforce_result)
        } else {
            // One subsystem failed at execution phase; full rollback
            CoGovernorDecision::rolled_back(eco_result, lifeforce_result)
        }
    }
}