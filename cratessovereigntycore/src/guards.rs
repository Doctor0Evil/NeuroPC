// 
pub struct EvolutionProposalRecord { /* parsed from .evolve.jsonl */ }

pub enum GuardDecision {
    Allowed,
    Rejected(String),
}

pub struct SovereigntyGuards<'a> {
    pub stake: &'a StakeTable,           // from .stake.aln
    pub roh_ceiling: f32,               // from .rohmodel.aln
    pub ledger: &'a mut DonutloopLedger // .donutloop.aln
}

impl<'a> SovereigntyGuards<'a> {
    pub fn evaluate_and_record(
        &mut self,
        proposal: EvolutionProposalRecord,
    ) -> GuardDecision {
        // 1. RoH guard
        if proposal.roh_after > self.roh_ceiling + 1e-6 { /* reject */ }
        if proposal.roh_after > proposal.roh_before + 1e-6 { /* reject */ }

        // 2. Stake guard
        if let Err(e) = self.stake.check_signers_for_scope(
            &proposal.scope,
            proposal.signer_roles.clone(),
        ) {
            return GuardDecision::Rejected(format!("Stake guard failed: {e}"));
        }

        // 3. Token guard: EVOLVE required for high‑impact scopes
        if (proposal.scope == "lifeforcealteration"
            || proposal.scope == "archchange")
            && proposal.token_kind != "EVOLVE"
        {
            return GuardDecision::Rejected(
                "Scope requires EVOLVE token; SMART not permitted".to_string()
            );
        }

        // 4. Donutloop logging (append‑only, RoH‑monotone + hash‑link)
        let entry = DonutloopEntry::from_proposal(&proposal, &self.ledger);
        if let Err(e) = self.ledger.append(entry) {
            return GuardDecision::Rejected(format!("Ledger append failed: {e}"));
        }

        GuardDecision::Allowed
    }
}
