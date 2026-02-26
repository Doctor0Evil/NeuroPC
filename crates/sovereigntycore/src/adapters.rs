pub fn register_advisory_proposal(
    out: CopilotOutput,
) -> Result<EvolutionProposalRecord, GuardrailError> {
    // 1. Check RoH monotone tightening: out.roh_delta <= 0
    // 2. Rebuild npf_* snippets and ensure no actuation vocabulary.
    // 3. Emit EvolutionProposalRecord to `bostrom-sovereign-kernel-v2.ndjson`.
}
