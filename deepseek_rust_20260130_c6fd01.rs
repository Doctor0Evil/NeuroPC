pub struct EvolutionRights {
    pub may_explore: bool,      // Right to try new cognitive architectures
    pub may_integrate: bool,    // Right to merge with safe systems
    pub may_transcend: bool,    // Right to move beyond human baselines
    pub may_refuse: bool,       // Right to reject all enhancements
}

impl EvolutionRights {
    fn enforce(&self, proposal: &EvolutionProposal) -> Result<(), RightsViolation> {
        // Check against biophysical limits
        // Verify no vendor lock-in can be created
        // Ensure no reduction in future choice space
        // Confirm consciousness continuity is preserved
    }
}