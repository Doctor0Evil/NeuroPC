use crate::stake::{ScopeKind, StakeTable};
use crate::update::UpdateProposal; // your existing proposal type

#[derive(Debug, Clone)]
pub struct StakeGuard {
    stake: StakeTable,
}

impl StakeGuard {
    /// Initialize from canonical stake file.
    /// Recommended path: "policies/bostrom-stake-v1.stake.aln"
    pub fn new_from_default_path() -> anyhow::Result<Self> {
        let stake = StakeTable::load_from_file(
            "policies/bostrom-stake-v1.stake.aln",
        )?;
        Ok(Self { stake })
    }

    pub fn new(stake: StakeTable) -> Self {
        Self { stake }
    }

    /// Enforce multisig for proposal scopes.
    /// If any ScopeKind requires roles that are not covered by proposal.signers,
    /// returns Err with a descriptive message.
    pub fn enforce(
        &self,
        proposal: &UpdateProposal,
    ) -> anyhow::Result<()> {
        let subjectid = &proposal.subjectid;

        // Map text scopes into ScopeKind values and collapse them.
        let mut scopes: Vec<ScopeKind> = Vec::new();
        for s in &proposal.scope {
            scopes.push(ScopeKind::from_str(s));
        }
        if scopes.is_empty() {
            scopes.push(ScopeKind::Other);
        }

        // Deduplicate scopes to avoid repeated checks.
        scopes.sort_by_key(|s| *s as u8);
        scopes.dedup();

        for scope in scopes {
            self.stake.check_multisig(
                subjectid,
                scope,
                &proposal.signers,
            )?;
        }

        Ok(())
    }
}
