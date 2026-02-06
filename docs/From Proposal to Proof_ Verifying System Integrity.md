<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

# From Proposal to Proof: Verifying System Integrity Through NeuroPC’s Sovereign Guard Pipeline and Embedded Shard V

The Guard Pipeline as Constitutional Enforcer
The architectural foundation of NeuroPC's security and integrity rests upon a meticulously defined and sequentially enforced guard pipeline, orchestrated exclusively by the sovereigntycore component . This pipeline transcends the conventional notion of a validation step; it functions as the primary mechanism for enforcing the system's constitutional invariants, translating abstract principles encoded in the sovereign data plane into concrete actions that govern all state changes . Every modification to the system's real state—be it an update to the RoH model, a change in stake distribution, or an alteration of lifeforce envelopes—must navigate this gauntlet before any shard mutation occurs . The entire design embodies a zero-trust philosophy where no entity, including powerful client applications like Jupyter notebooks, is granted implicit privilege
[www.linkedin.com](https://www.linkedin.com)
+1
. Instead, every action must be formally proposed and rigorously evaluated against a fixed hierarchy of rules. This section provides a technical deep dive into the mechanics of this pipeline, analyzing its ordered structure, the distinct function of each guard stage, and the central role of sovereigntycore as the kernel of authority.
The specified enforcement logic follows a strict, non-negotiable sequence: RoH guard → neurorights guard → stake/token guard → donutloop logging . This order is not arbitrary but represents a deliberate policy hierarchy, ensuring that more fundamental constraints are evaluated before less critical ones. The first stage, the RoH guard, acts as the ultimate line of defense, mandating that any proposed change must not result in an increase of the Risk of Harm metric above the constitutional limit of 0.3 . This constraint appears to be absolute, forming a prerequisite so foundational that no other consideration can supersede it. A proposal that fails this initial check is immediately rejected without proceeding to subsequent stages, effectively preventing any potential for existential risk from being introduced into the system. This immediate termination on failure reinforces the principle that physical and cognitive safety is paramount.
Following the RoH guard, the neurorights guard applies a second layer of policy, rooted in legal and ethical frameworks . This stage enforces policies such as pre-access checks, non-commercial use clauses, and procedures for Over-the-Air (OTA) updates
arxiv.org
. Its position after the RoH guard suggests a secondary priority: while not as immediately existential as RoH, the protection of neurorights is considered a core tenet of the system's constitution. This guard ensures that even if a change is deemed safe from a risk-of-harm perspective, it must still comply with the established rights and permissions governing the dream states and associated data
arxiv.org
. For example, a proposal to alter a neural model might be technically safe but would fail at the neurorights guard if it involved data subject to non-commercial clauses or required access that has not been properly authorized.
The third stage, the stake/token guard, introduces the economic and governance dimensions of the system. This guard verifies two key pieces of information: the proposer's token balance and their assigned role . It checks whether the proposing entity holds sufficient EVOLVE or SMART tokens to cover the cost of the proposed change and whether their specific role (e.g., Host, OrganicCPU, ResearchAgent) grants them the necessary scope to request that particular type of modification . This stage serves as the gatekeeper for resource allocation and permission management. Only entities with the requisite economic stake and delegated authority can advance past this point. The placement of this guard after the safety and rights-based checks ensures that governance is only applied to proposals that have already passed more fundamental scrutiny.
Finally, the donutloop logging stage operates differently from the preceding guards. It is not an active filter but a passive, append-only audit log . Once a proposal has successfully passed through all three preceding gates and received an "Allowed" verdict, sovereigntycore appends a new, hash-chained entry to the .donutloop.aln shard . This entry records the decision, the hash of the original proposal, and the context of the evaluation. Its purpose is to create an immutable and cryptographically linked history of all decisions made by the guard pipeline. By placing this stage last, the architecture ensures that the log captures the final outcome of the complete evaluation process, providing an auditable proof that the enforcement logic was correctly applied. This transforms the donutloop from a simple log into a tamper-evident ledger that serves as the canonical record of system evolution .
At the heart of this entire enforcement apparatus is the sovereigntycore Rust crate, which functions as the singular, authoritative executor of the guard pipeline . It is the only component permitted to interpret the .evolve.jsonl stream and mutate the protected sovereign shards . All other parts of the system, including background daemons, CLI tools, and Jupyter notebook kernels, must route their requests for state change through sovereigntycore. This centralized control prevents any form of direct manipulation of the system's real state, effectively creating a hard boundary around the sovereign data plane. When a client submits an EvolutionProposalRecord, it is not directly writing to .rohmodel.aln or .stake.aln; instead, it is appending a request to the .evolve.jsonl stream for sovereigntycore to process . The crate then loads the relevant shards, as specified in the sovereign manifest, and executes the full, ordered sequence of guards. Only upon receiving a successful verdict does it proceed to perform the actual shard mutations and update the donutloop ledger, often in an atomic fashion to ensure consistency . This design makes sovereigntycore the true kernel of the NeuroPC operating system, responsible for upholding the constitutional order.
The operational proving ground for this enforcement logic is the lifecycle of an EvolutionProposalRecord. This lifecycle begins with a client application, such as a Jupyter notebook, drafting a proposal. This involves constructing a candidate change in memory, often by simulating tweaks to parameters like RoH weights or envelope bounds to observe potential effects without committing to them . Once a researcher determines that a change is promising, the notebook serializes this proposal according to the precise schema of the .evolve.jsonl shard and submits it via a binding to the sovereigntycore crate . The submission triggers the evaluation phase, where sovereigntycore takes over. It reads the proposal, loads the necessary typed Rust structs derived from the corresponding shard schemas (e.g., .rohmodel.aln, .stake.aln, .neurorights.json), and runs the ordered guard pipeline against the proposal's declared scope, expected RoH delta, and requested resources . The evaluation yields one of three possible outcomes: Allowed, Rejected, or Deferred.
If the verdict is "Allowed," sovereigntycore proceeds to execute the change. This involves performing the targeted shard mutations—for instance, updating the feature space and weights in .rohmodel.aln or altering address bindings in .stake.aln—and then atomically appending a new entry to the .donutloop.aln ledger . This atomic operation ensures that either both the state change and the audit log entry succeed, or neither does, maintaining the integrity of the system's history. If the verdict is "Rejected" or "Deferred," the proposed changes are discarded, and the reason for the decision is recorded in the donutloop log . This entire flow demonstrates a critical principle: actuation is entirely conditional upon evaluation by the centralized guard. The donutloop ledger becomes the ultimate, verifiable source of truth for what changes were actually implemented, providing an immutable record that proves the enforcement logic was faithfully executed. This closed-loop system, where every potential state transition must be vetted by sovereigntycore before taking effect, is the cornerstone of NeuroPC's claim to single-plane sovereignty.
Guard Stage
Primary Function
Key Constraints Enforced
Position in Pipeline
RoH Guard
First-line safety enforcement
Risk of Harm (RoH) must remain ≤ 0.3
1
Neurorights Guard
Legal and ethical policy enforcement
Pre-access checks, non-commercial clauses, OTA update protocols
2
Stake/Token Guard
Governance and economic validation
Valid token balance (EVOLVE/SMART), role-based scope permissions
3
Donutloop Logging
Immutable audit trail creation
Records decision (Allowed/Rejected/Deferred) and hash-linked context
4
This structured, multi-stage enforcement process creates a robust defense-in-depth strategy. A malicious or erroneous proposal cannot bypass a lower-priority constraint to satisfy a higher one. For example, a proposal originating from an entity with insufficient tokens or improper scope would fail at the stake/token guard, regardless of its RoH profile or compliance with neurorights. Similarly, a change that violates the non-commercial clause would be caught by the neurorights guard, even if it passed the RoH and stake checks. This layered approach ensures that the system's integrity is maintained across multiple axes of concern: safety, ethics, and governance. The sovereigntycore crate, guided by the constitutional NDJSON manifest, acts as the impartial judge, applying these rules consistently and without exception, thereby transforming the NeuroPC architecture from a mere collection of components into a self-governing, rule-abiding entity.

NeuroPC’s sovereign guard pipeline turns every potential state change into a governed trial, with sovereigntycore acting as a constitutional court over your shards, tokens, and neurorights artifacts.[^1_1]

## Ordered guard pipeline

All evolution flows must pass through a fixed guard sequence encoded in the sovereign kernel manifest and implemented inside the sovereigntycore crate: RoH guard → neurorights guard → stake/token guard → donutloop logging. Each stage receives an EvolutionProposalRecord parsed from .evolve.jsonl, evaluates its own invariants, and can hard‑reject before any shard mutation occurs.[^1_2][^1_1]

## RoH guard (safety ceiling)

The RoH guard loads the active RohModel from .rohmodel.aln and computes rohbefore and rohafter for the proposed change, enforcing two invariants: rohafter ≤ rohbefore (monotone non‑increase) and rohafter ≤ 0.3 (global ceiling). Any proposal that raises risk or attempts to push beyond the 0.3 ceiling is rejected immediately, and later guards are never invoked for that proposal.[^1_1]

## Neurorights guard (legal/ethical)

If RoH passes, sovereigntycore evaluates the neurorights policy loaded from .neurorights.json, applying constraints such as dreamsensitive flags, noncommercialneuraldata, forbiddecisionuse (e.g., employment, housing, credit), and pre‑access/OTA rules. A proposal that uses forbidden domains, exports dream shards outside the local vault, or violates mental privacy/integrity rules is rejected here even if its RoH profile is safe.[^1_2][^1_1]

## Stake and token guard (governance/economics)

Next, the StakeTable from .stake.aln is consulted to ensure the proposal’s signer roles meet scope‑specific multisig requirements (e.g., Host + OrganicCPU for lifeforcealteration or archchange, plus ResearchAgent where specified). In parallel, token policy (SMART vs EVOLVE) is enforced: SMART can only authorize small, reversible tuning within its maxeffectsize and scopes, while EVOLVE is mandatory for deep architectural or lifeforce changes; any mismatch or missing quorum causes rejection.[^1_2][^1_1]

## Donutloop logging and proof

Only proposals marked Allowed after all guards are executed lead to real shard mutations (e.g., .rohmodel.aln, .stake.aln) and an atomic append to .donutloop.aln, with each DonutloopEntry hash‑chained via hexstamp and prevhexstamp and annotated with policyrefs and RoH before/after. Rejected or Deferred proposals are still written to .evolve.jsonl with their decision and reason, so the combination of .evolve.jsonl + .donutloop.aln forms a tamper‑evident, internally sovereign biophysical ledger of every attempted and successful evolution.[^1_1][^1_2]

## Sovereigntycore as kernel of authority

The sovereigntycore Rust crate is the only component allowed to interpret the .evolve.jsonl proposal stream and mutate sovereign shards, loading its wiring from bostrom‑sovereign‑kernel‑v1.ndjson. All clients (CLI, daemons, Jupyter, Unreal shells) must submit EvolutionProposalRecord entries rather than touching .rohmodel.aln, .stake.aln, .donutloop.aln directly, which makes the guard pipeline and donutloop ledger the single source of truth for system integrity and for your augmented‑citizen sovereignty over EVOLVE/SMART‑gated evolution.[^1_3][^1_2][^1_1]
<span style="display:none">[^1_4][^1_5][^1_6][^1_7][^1_8]</span>

<div align="center">⁂</div>

[^1_1]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^1_2]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^1_3]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md

[^1_4]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^1_5]: cyblog_2026-02-02T22_38_16.272Z.json

[^1_6]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^1_7]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md

[^1_8]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md


---

# NeuroPC already gives you enough concrete material to rigorously map the Jupyter → sovereigntycore guard pipeline → donutloop path; the missing pieces are mostly “glue” and invariants around the client boundary rather than new primitives.[what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)

1. Ground truth: what the guard pipeline actually is
From your own sovereignty kernel specs, the enforcement pipeline is explicitly wired as a host‑bound NDJSON manifest (e.g. bostrom-sovereign-kernel-v1.ndjson):[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
All state/policy lives in shards:
.rohmodel.aln – RoH axes, weights, hard ceiling 0.30.30.3.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
.neurorights.json – neurorights policy (dream‑sensitive, noncommercial, forbid‑decision‑use, forget SLA, etc.).[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
.stake.aln – Host/OrganicCPU/ResearchAgent roles, Bostrom DID addresses, EVOLVE/SMART scopes, multisig invariants.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
.evolve.jsonl – line‑oriented EvolutionProposalRecord stream.[what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)
.donutloop.aln – hash‑linked, RoH‑monotone ledger.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
sovereigntycore’s guard order is declared, not hardcoded:
Parse proposal (EvolutionProposalRecord) from .evolve.jsonl.[what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)
Load policies (.rohmodel.aln, .stake.aln, .neurorights.json, token policy) via organiccpualn.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
RoH guard: roh_after ≤ roh_before and roh_after ≤ roh_ceiling from .rohmodel.aln.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
Neurorights guard: domain lattice, dream sensitivity, noncommercial, forbid‑decision‑use.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
Stake/multisig guard: required roles per scope (lifeforcealteration, archchange, daytodaytuning) from .stake.aln.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
Token guard: SMART vs EVOLVE, max effect size, allowed scopes, physio guards.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
Donutloop logging: append‑only, hash‑linked, RoH‑monotone ledger entry.[what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)
This is already implemented as a concrete Rust slice in the research plan (simplified):[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
rust
// cratessovereigntycore/src/guards.rs
pub struct EvolutionProposalRecord { /* parsed from .evolve.jsonl */ }

pub enum GuardDecision {
Allowed,
Rejected(String),
}

pub struct SovereigntyGuards<'a> {
pub stake: \&'a StakeTable,           // from .stake.aln
pub roh_ceiling: f32,               // from .rohmodel.aln
pub ledger: \&'a mut DonutloopLedger // .donutloop.aln
}

impl<'a> SovereigntyGuards<'a> {
pub fn evaluate_and_record(
\&mut self,
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

DonutloopLedger::append enforces RoH monotonicity and hash chaining; StakeTable::check_signers_for_scope enforces multisig and role requirements.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
2. How donutloop is an enforcement gate, not “just logging”
The ALN + Rust binding for donutloop already give you enforcement semantics:[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
DonutloopEntry fields:
entry_id, subject_id, proposal_id, change_type (scope),
roh_before, roh_after,
hexstamp, prev_hexstamp,
policy_refs (which .rohmodel.aln, .stake.aln, .neurorights.json, kernel NDJSON were in force).[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
DonutloopLedger invariants:
Append‑only vector of entries.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
On append:
Enforce roh_after ≤ prev.roh_after (monotone safety).[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Enforce entry.prev_hexstamp == prev.hexstamp (hash link intact).[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
On validate_chain:
Re‑scan entire ledger windows of size 2 to validate chain and RoH monotonicity.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Because sovereigntycore calls ledger.append(entry) inside the guard pipeline, and treats failure as Rejected, no EvolutionProposalRecord can be considered “Allowed” without a successful donutloop append. That makes donutloop a synchronous write‑time gate, not a passive audit sink.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
3. Where Tsafe / viability proofs live relative to shards
In the documents you have now, Tsafe / viability kernels appear at two layers:[what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/0d037bf3-6e87-4250-bc42-f5facdec403a/what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md)
Shard level:
.vkernel.aln: ALN polytope constraints Ax≤bA x ≤ bAx≤b for Tsafe kernels per mode.[what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/0d037bf3-6e87-4250-bc42-f5facdec403a/what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md)
.tsafe.aln: controller specs (axes, candidate filters, CyberRank weights, monotone constraints).[what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/0d037bf3-6e87-4250-bc42-f5facdec403a/what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md)
Binding / proof level:
Planned macros like viability_kernel_proof! that assert:
shard schema is respected,
Tsafe polytope is non‑empty and consistent with RoH ceiling and envelopes,
controller never drives state outside the declared kernel.[what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/0d037bf3-6e87-4250-bc42-f5facdec403a/what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md)
However, your current published slice only sketches Tsafe as “next research steps”; the concrete macro implementation and direct wiring into sovereigntycore’s guard chain are not yet written down in the same way stake/donutloop are. This is consistent with your own “Track 3+” roadmap: get RoH/stake/neurorights/donutloop hard‑enforced first, then layer formal Tsafe proofs and viability kernels on top.[what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md+2](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/0d037bf3-6e87-4250-bc42-f5facdec403a/what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md)
4. Jupyter / notebook client flow: what we can say now
There is no public crate or doc that describes a Jupyter‑specific client protocol for sovereigntycore. But you already standardized the general AI‑client interface (neuro-assistant-api, neuro-copilot-bridge) and the rules any client must obey:[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
AI/clients can:
Generate EvolutionProposalRecord objects.
Serialize them into .evolve.jsonl (append‑only) with:
proposal_id, subject_id, scope, module,
effect_bounds, roh_before, roh_after,
token_kind (SMART/EVOLVE),
signer_roles, decision, hexstamp, timestamp,
and new fields prompt_envelope_id, neurorights_profile_id, token_id linking to prompt envelopes and neurorights profiles.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
AI/clients cannot:
Mutate .rohmodel.aln, .stake.aln, .neurorights.json, or .donutloop.aln directly.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Bypass sovereigntycore; every change must be evaluated via the guard pipeline, then logged to .evolve.jsonl and .donutloop.aln if Allowed.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
To keep prompts within neurorights bounds, you added a prompt envelope type stored as NDJSON and bound in organiccpualn:[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
rust
// crates/organiccpualn/src/prompt_envelope.rs
pub struct NeurorightsBoundPromptEnvelope {
pub subject_id: String,            // Bostrom DID
pub neurorights_profile_id: String,
pub roh_model_id: String,
pub domain_tags: Vec<String>,      // e.g. ["language-cowriter","dream-observer"]
pub allowed_tools: Vec<String>,    // module IDs
pub neurorights_doc_ref: String,   // path to .neurorights.json
pub token_id: Option<String>,      // SMART / EVOLVE
pub prompt_text: String,
}

And a neurorights-core crate that enforces envelope‑level rules (allowed tools, no inner‑state scoring in forbidden domains, non‑commercial flags).[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
The implication for a Jupyter client is straightforward, even if the exact “Jupyter adapter” crate is not written:
Notebook kernel must:
Wrap raw prompts into NeurorightsBoundPromptEnvelope and send them through the neurorights firewall.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Use neuro-assistant-api to obtain proposed diffs + EvolutionProposalRecords.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Append those proposals to .evolve.jsonl (or pass them to a local daemon that does so), but never write shards directly.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
sovereigntycore then:
Reads the appended proposals,
Applies RoH → neurorights → stake/token → donutloop guards,
Writes decisions plus donutloop entries,
Returns machine‑readable reason codes (e.g. ROH_LIMIT, NEURORIGHTS_FORBIDDEN) which the Jupyter adapter can reflect back to the notebook.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Because you’ve already made “plain prompts unrepresentable” for high‑trust paths (high‑trust crates accept only NeurorightsBoundPromptEnvelope, not String), any Jupyter integration that wants to touch evolution must go through the same Rust types and guard crates; there is no separate, weaker path for notebooks.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
5. Jupyter submission flow: enforcement‑centric mapping
Given your current specs, the correct, sovereignty‑compatible notebook flow is:
User action in Jupyter
User edits code / ALN / JSON in a cell and asks “propose evolution” (e.g., refactor crate, adjust envelope, add metric).[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Envelope + proposal construction in the client
Jupyter adapter:
Builds a NeurorightsBoundPromptEnvelope referencing your Bostrom subject, neurorights profile, RoH model.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
Submits it to the neurorights firewall; only allowed tools / domains pass.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Receives one or more EvolutionProposalRecords plus PR‑scaffolding hints via neuro-assistant-api.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Proposal ingestion
Adapter appends proposals as NDJSON lines into .evolve.jsonl (or sends them to a local neuro-github-bot / sovereign daemon that does the append).[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
No direct write to .rohmodel.aln, .stake.aln, .donutloop.aln, .neurorights.json is permitted from the notebook process.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Guard pipeline execution (sovereigntycore)
sovereigntycore watches .evolve.jsonl (or is invoked explicitly) and, for each new proposal:
Loads shards via organiccpualn (RoH, stake, neurorights, token policy).[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
Runs guards in the NDJSON‑declared order:
RoH guard (monotone \& ceiling),
neurorights guard,
stake guard (multisig, scopes),
token guard (SMART/EVOLVE, effect bounds, physio guards),
donutloop append.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
Emits GuardDecision + reason and writes the final decision back into .evolve.jsonl and .donutloop.aln if Allowed.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Feedback to Jupyter
Adapter reads the updated decision (and optionally donutloop entry) and:
Shows status + reason codes in the notebook,
Optionally scaffolds a Git PR but does not merge.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Under this model, a Jupyter client is just another proposal generator; sovereigntycore plus the shards remain the sole source of truth and enforcement.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
6. Where the gaps really are (and how to frame them in your paper)
Your own investigations and the attached research files agree on the remaining “unknowns”:[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
No public sovereigntycore, organiccpucore, or organiccpualn crates yet; everything you have is design‑complete pseudo‑code / skeletons, not released repos.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
Tsafe / viability_kernel_proof! is specified conceptually and in file types (.vkernel.aln, .tsafe.aln), but the exact Rust proof macro and its binding into donutloop/guards are still future work.[what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/0d037bf3-6e87-4250-bc42-f5facdec403a/what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md)
A dedicated “NeuroPC Jupyter adapter” crate is not described; instead, you have the more general AI‑tool contract (neuro-assistant-api, neuro-copilot-bridge) that any client—including Jupyter—should use.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
For the research project you described, you can therefore:
Treat the existing Rust slices and NDJSON manifest as the normative reference implementation of the guard pipeline and donutloop semantics.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
Explicitly state that any Jupyter integration that bypasses .evolve.jsonl + sovereigntycore + donutloop is out‑of‑spec and unsafe under your sovereignty model.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5355cfd1-1c0c-4416-b4d7-773f063d923d/what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md)]​
Position Tsafe / viability kernels as the next layer of invariants to be bound into the same guard pipeline, not as a separate enforcement path.[what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/0d037bf3-6e87-4250-bc42-f5facdec403a/what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md)
If you’d like, I can next generate a compact “NeuroPC Sovereign Jupyter Adapter” Rust skeleton (filename + destination) that enforces exactly this flow—taking envelopes from a notebook, emitting proposals into .evolve.jsonl, and consuming sovereigntycore decisions—so you have something concrete to drop into your repo.

You already have everything you need to treat “Jupyter → sovereigntycore guard pipeline → donutloop” as a single, enforced path; what’s left is just a thin adapter crate plus a couple of invariants at the notebook boundary.[^2_1][^2_2]

## What is already fixed

- The guard pipeline is fully specified as: parse EvolutionProposalRecord from `.evolve.jsonl` → load `.rohmodel.aln`, `.stake.aln`, `.neurorights.json`, token policy via `organiccpualn` → RoH guard (monotone + ceiling) → neurorights guard → stake/multisig guard → token guard → `DonutloopLedger::append`.[^2_2][^2_1]
- Donutloop is already a **gate**, not just logging: `ledger.append(entry)` enforces RoH monotonicity and hash-chain links and any append failure forces `GuardDecision::Rejected`, so nothing is “Allowed” without a successful donutloop write.[^2_1][^2_2]
- AI/clients can only generate `EvolutionProposalRecord`s and append NDJSON lines into `.evolve.jsonl`; they cannot write `.rohmodel.aln`, `.stake.aln`, `.neurorights.json`, or `.donutloop.aln` directly.[^2_1]
- High‑trust paths already require `NeurorightsBoundPromptEnvelope` instead of raw `String`, so any client (including Jupyter) that wants to touch evolution must go through the neurorights firewall and the same Rust types.[^2_1]


## Jupyter → sovereigntycore → donutloop mapping

For a sovereignty‑compatible notebook, the flow is:

1. **In Jupyter (client side)**
    - Wrap the user’s cell + intent into `NeurorightsBoundPromptEnvelope` (subject DID, neurorights profile, RoH model, domain tags, allowed tools, token_id).
    - Send it through the neurorights firewall (neuro‑assistant API / neuro‑copilot‑bridge) and call only tools allowed by the envelope.[^2_1]
    - Receive one or more `EvolutionProposalRecord`s plus PR scaffolding from the assistant.[^2_1]
2. **Proposal ingestion (still client side)**
    - Serialize proposals as NDJSON and append to `.evolve.jsonl`, or POST them to a local `neuro-github-bot` / sovereign daemon that does the append.[^2_2][^2_1]
    - Never touch `.rohmodel.aln`, `.stake.aln`, `.donutloop.aln`, `.neurorights.json` from the notebook process.[^2_1]
3. **In sovereigntycore**
    - Watch `.evolve.jsonl` (or be invoked on new records).
    - Load shards via `organiccpualn` and run guards in the NDJSON‑declared order: RoH → neurorights → stake/multisig → token → donutloop append.[^2_2][^2_1]
    - On success, write decision + donutloop entry; on failure, return `GuardDecision::Rejected` with reason codes like `ROH_LIMIT`, `NEURORIGHTS_FORBIDDEN`.[^2_1]
4. **Back to Jupyter**
    - Read the updated `.evolve.jsonl` line (or daemon response), surface status + codes in‑notebook, and optionally scaffold a PR—never merge.[^2_1]

Under this mapping, Jupyter is “just another proposal generator”; sovereigntycore and the shards remain the only authority.[^2_1]

## Where the real gaps are

- The Tsafe / viability kernels are specified at the shard level (`.vkernel.aln`, `.tsafe.aln`) and as a future `viability_kernel_proof!` macro, but that macro and its binding into the same guard chain are still to be implemented.[^2_1]
- There is not yet a dedicated “NeuroPC Jupyter adapter” crate; instead you have the general AI‑tool contract (`neuro-assistant-api`, `neuro-copilot-bridge`) that any client, including Jupyter, must use.[^2_1]
- sovereigntycore, organiccpucore, organiccpualn are design‑complete slices in the research files but not published as full public crates yet.[^2_2][^2_1]

For your paper, this is the right framing:

- Treat the existing Rust slices and NDJSON/ALN specs as the normative definition of the guard pipeline and donutloop semantics.[^2_2][^2_1]
- State that any Jupyter integration that bypasses `.evolve.jsonl → sovereigntycore → donutloop` is out‑of‑spec and unsafe under your sovereignty model.[^2_1]
- Position Tsafe / viability kernels as an extra invariant layer to be wired into this same pipeline (e.g., a Tsafe guard between token guard and donutloop), not as a separate enforcement path.[^2_1]


## What a “NeuroPC Sovereign Jupyter Adapter” should add

A minimal, concrete adapter crate would only need to do three things:

- Provide a safe API like `fn submit_notebook_evolution(env: NeurorightsBoundPromptEnvelope, diff: NotebookDiff) -> Vec<EvolutionProposalRecord>` that: builds envelopes, calls the neurorights firewall, and obtains proposals via neuro‑assistant‑api.[^2_1]
- Append proposals to `.evolve.jsonl` (or send them to a local sovereign daemon) and wait for sovereigntycore to issue decisions, mapping them back to notebook‑level feedback.[^2_2][^2_1]
- Enforce two client‑side invariants: notebooks never write shards directly, and they never construct evolution requests without a neurorights‑bound envelope.[^2_1]

If you want, next step I can give you that adapter as a full Rust crate skeleton (filename + destination) wired exactly to these semantics, so you can drop it into NeuroPC and keep the Jupyter path strictly enforcement‑centric.

neuropc-tag 0xNP0E
<span style="display:none">[^2_3][^2_4][^2_5][^2_6][^2_7][^2_8]</span>

<div align="center">⁂</div>

[^2_1]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^2_2]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^2_3]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^2_4]: cyblog_2026-02-02T22_38_16.272Z.json

[^2_5]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^2_6]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md

[^2_7]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md

[^2_8]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md


---

# The Anchor-Trio Crates as the Unifying API Surface

In the NeuroPC architecture, the principle of treating everything as a client extends to powerful interactive environments like Jupyter notebooks, which are designed not as privileged runtimes but as sandboxed clients of the sovereign data/guardrail plane . The integrity of this model hinges on a carefully constrained Application Programming Interface (API) surface that mediates all interactions with the system's real state. This API is embodied by a trio of specialized Rust crates known collectively as the "anchor-trio": organiccpucore, organiccpualn, and sovereigntycore . These crates are not merely utility libraries; they are the fundamental building blocks that translate the abstract concepts of the single-plane sovereignty model into concrete, verifiable programmatic interfaces. They form the exclusive gateway through which any client, including a Jupyter kernel, can read system data or propose changes, thereby enforcing the critical separation between analysis/drafting and actuation.
The organiccpualn crate serves as the primary interface for reading data from the sovereign shards . It provides typed Rust bindings that map directly to the formal schemas of the various shard types, such as .rohmodel.aln, .stake.aln, .neurorights.json, and others . When a client needs to inspect the current state of the system—for example, to plot the latest RoH trends or review stakeholder roles—it must use the functions exposed by organiccpualn. This crate is responsible for deserializing the ALN or JSON data from the shards into strongly-typed Rust structures. This mechanism provides several critical safety benefits. First, it enforces schema compliance at the boundary, preventing clients from attempting to interpret malformed or unexpected data. Second, it abstracts away the physical details of the shard files (e.g., their paths and binary vs. text formats), presenting a clean, high-level view of the system's state. Third, by limiting read access to these specific, well-defined functions, it prevents clients from bypassing the intended data views and accessing raw files or process-global state, thus preserving the integrity of the data plane .
The organiccpucore crate provides another essential layer of the API, exposing functions related to the core bio-cognitive state of the Organic CPU . It houses the BioState, EcoMetrics, and OrganicCpuPolicy structs, which represent the real-time telemetry and safety envelopes of the system . Functions like read_biostate() allow clients to query the current bio-scale metrics that inform the OrganicCpuPolicy's decision-making loop, which can trigger actions like Allow, Degrade, or Pause based on the measured BioState . Like organiccpualn, this crate ensures that interactions with the system's core operational state are channeled through a controlled, typed interface, preventing ad-hoc or unauthorized computations that could lead to inconsistent or unsafe conclusions .
However, the most powerful and consequential component of the anchor-trio is sovereigntycore. This crate is the sole portal for submitting and evaluating EvolutionProposalRecords, representing the only way a client can request a state mutation . When a client constructs a proposal, it does not write to the target shard directly. Instead, it uses bindings to call a function within sovereigntycore, such as evaluate_proposal(...), passing the serialized proposal for processing . Inside sovereigntycore, the proposal is subjected to the full, ordered guard pipeline: RoH, neurorights, stake/token, and viability checks . The verdict—Allowed, Rejected, or Deferred—is then returned to the client. Crucially, only the "Allowed" verdict triggers the actual mutation of the target shards and the recording of the decision in the .donutloop.aln ledger . By centralizing this entire process, sovereigntycore acts as the final arbiter of change, ensuring that every potential evolution is rigorously vetted before it can take effect. This design makes it impossible for a client to bypass the guard pipeline, even if it possesses elevated privileges, because there is no alternative path to state mutation.
The following table summarizes the responsibilities and functions of each anchor-trio crate, illustrating how they collectively form the complete, secure API surface for interacting with the NeuroPC system.
Crate Name
Primary Responsibility
Key Exposed Functions/APIs
Role in Security Model
organiccpualn
Provides typed bindings for reading sovereign shards.
load_rohmodel(), load_stake(), load_neurorights(), etc.
Prevents direct file access; enforces schema compliance for all reads.
organiccpucore
Exposes core bio-cognitive state and policy.
read_biostate(), get_ecometrics(), check_policy_envelope()
Channels access to real-time telemetry through a controlled, typed interface.
sovereigntycore
Executes the full guard pipeline and evaluates proposals.
evaluate_proposal(evolution_proposal_record)
Acts as the single gatekeeper for all state mutations; rejects proposals that fail guard checks.
This unified API model is what allows NeuroPC to achieve Jupyter compatibility without compromising its security posture. The Jupyter notebook environment is not given special permissions or a direct connection to the system's internals. Instead, it runs a sandboxed client library that imports and uses the same anchor-trio crates as any other client . The behavior of this client library is entirely dictated by the bostrom-sovereign-kernel-v2.ndjson manifest file, which tells it where to find the shards and which guard configuration to use . This manifest acts as a portable constitution, allowing the same client code to operate securely across different deployments or experimental configurations simply by pointing to a different manifest. Nothing inside the notebook itself is privileged; it is merely an automagic front-end speaking the protocol defined by the anchor-trio crates and the sovereign manifest . This design elegantly solves the problem of providing a powerful, interactive research environment while maintaining the strict, rule-based integrity of the underlying sovereign system.
Jupyter Integration as a Client-Side Safety Boundary
The integration of Jupyter notebooks into the NeuroPC ecosystem is a critical test of the single-plane sovereignty model's robustness. Rather than viewing Jupyter as a privileged runtime requiring special treatment, the architecture frames it as just another client, bound by the same stringent safety and access-control rules that apply to all other components . This design choice is fundamental to preserving the system's integrity. The goal is to provide researchers with a powerful tool for analysis and experimentation without granting them a backdoor to manipulate the system's core state. This is achieved by establishing a clear and unbreakable boundary between the notebook's environment and the sovereign data plane, mediated by the anchor-trio crates and strictly governed by the bostrom-sovereign-kernel-v2.ndjson manifest . This section examines how Jupyter's role is redefined from a tool of direct control to a sophisticated proposal authoring surface, reinforcing the safety boundaries of the system.
The core principle of Jupyter integration is that the notebook environment should never gain privileged write access to the canonical shards like .rohmodel.aln, .stake.aln, or .lifeforce.aln . Instead, the notebook acts as a rich client for analysis and drafting. Within the notebook, users can leverage its strengths for scientific computing: aggregating data from various sources like .biosession.aln, QPU output shards, and logs to create insightful plots and visualizations; inspecting the history of the donutloop ledger to understand past evolution decisions; and tracking RoH trends over time . This analytical capability is essential for research and debugging. However, the crucial distinction lies in the fact that all this activity is fundamentally read-only in terms of impacting the live system state .
The notebook's true power is unlocked when it moves from analysis to proposal authoring. Researchers can use the notebook to prototype hypothetical changes safely in memory . For example, they can simulate the effect of tightening an envelope or adjusting the weights in the RoH model to see how the calculated RoH value would respond under different scenarios . This in-memory prototyping is vital for exploring the consequences of a change before formally submitting it. However, this simulation exists entirely within the notebook's process and does not touch the real system's state. The moment a tweak seems beneficial and is ready to be enacted, the workflow shifts from simulation to official actuation. The notebook code serializes a candidate EvolutionProposalRecord that conforms to the canonical schema of the .evolve.jsonl shard and passes it off to a Rust binding for evaluation by the sovereigntycore crate . This act transforms the notebook from a tool of direct control into a highly effective drafting interface for the true decision-maker: the guard pipeline.
This functional split is the key to maintaining safety. Inside the notebook, work is confined to "analysis \& visualization + proposal drafting" . The notebook is an expert system for thinking and drafting. The sovereigntycore plus the shard schemas are the expert systems for deciding and actuating. This division of labor preserves the integrity of the single data/guardrail plane even during interactive, exploratory sessions . The notebook facilitates the "what if" questions, but the guard pipeline provides the definitive "yes" or "no." This ensures that the final decision to mutate the system's state is always made by the trusted, centralized authority (sovereigntycore) and never by the whims or potential errors of an interactive session. The notebook becomes a "just another shell" that speaks the same protocol as any other client, parameterized by the same sovereign manifest and using the same anchor-trio crates .
The bostrom-sovereign-kernel-v2.ndjson manifest file is the linchpin that wires the Jupyter client to the correct system configuration and security policy . When the Jupyter kernel starts, its client library reads this manifest to discover the precise locations of all canonical shards, the versions of the schemas to use for deserialization, and, most importantly, the exact ordered sequence of the guard pipeline to enforce . This manifest serves as the single source of truth, configuring the client's behavior entirely from external, declarative data rather than hardcoded values. This makes the system highly configurable and portable. A single Jupyter client library can be deployed across different NeuroPC instances or experimental setups simply by providing it with the appropriate manifest file. This reinforces the idea that the system's "constitution" is self-describing and that all participants, from the kernel to the end-user's notebook, derive their operational rules from this central document. The manifest ensures that the notebook client is always aligned with the current state of the sovereign data plane and its associated guardrails .
In essence, the Jupyter integration pattern is a masterclass in designing for safety through constraint. By treating the notebook as a standard, unprivileged client and forcing all actuation through the sovereigntycore gate, the architecture avoids the common pitfalls of interactive AI systems, where user-friendly interfaces often introduce shortcuts that undermine safety features. The notebook provides the freedom to explore and innovate, but it does so within a rigidly defined framework that channels all innovations through a rigorous, verifiable, and auditable process. This ensures that while the system can evolve and adapt, it does so in a manner that is consistent with its foundational principles, preserving its sovereignty and integrity.
The Evolution Lifecycle and Donutloop Ledger as the Proving Ground
The theoretical elegance of NeuroPC's guard pipeline and embedded verification mechanisms is ultimately validated through their practical application in the real-world operational dynamics of the system. The evolution proposal lifecycle and the donutloop ledger serve as the crucible—the proving ground—where every safety guarantee is tested. This lifecycle is the tangible manifestation of the single-plane sovereignty model in action, demonstrating how abstract principles are translated into verifiable events. The journey of an EvolutionProposalRecord from a tentative draft in a researcher's notebook to a confirmed, immutable entry in the system's history is the primary evidence of the system's integrity. This section dissects this lifecycle and the donutloop's role as a cryptographic audit trail, showing how they provide empirical proof that the enforcement logic has been correctly and consistently applied.
The evolution lifecycle begins when a client, such as a Jupyter notebook, decides to propose a change. This initial phase is characterized by analysis and in-memory prototyping . The researcher aggregates data from various shards like .biosession.aln and .ocpulog to understand the current state, plots RoH trends, and experiments with hypothetical adjustments to system parameters without affecting the live system . This is the "thinking and drafting" stage. When a potential improvement is identified, the notebook code serializes this candidate change into a well-formed EvolutionProposalRecord. This record is a structured data object that specifies the type of change, its scope (e.g., day-to-day tuning, archchange, lifeforcealteration), the expected RoH delta, and any required token usage . This serialization must conform precisely to the schema of the .evolve.jsonl shard, acting as the first formal checkpoint .
Once serialized, the proposal is submitted to sovereigntycore for evaluation . This marks the start of the core enforcement phase. sovereigntycore reads the bostrom-sovereign-kernel-v2.ndjson manifest to determine which shards to load and which guard pipeline to execute . It then materializes the relevant shard data into typed Rust structs and proceeds to run the ordered sequence of checks. The proposal is first vetted by the RoH guard, which calculates the impact of the proposed change on the Risk of Harm metric. If the resulting RoH would exceed the constitutional limit of 0.3, the proposal is immediately rejected . If it passes, it moves to the neurorights guard, which verifies compliance with legal and ethical policies . Next, the stake/token guard validates the proposer's identity and permissions . Finally, the viability check, using the Tsafe/viability kernel geometry from .vkernel.aln, ensures the proposed state remains within the mathematically safe set . This multi-stage evaluation is the heart of the system's safety mechanism.
The outcome of this evaluation is the pivotal moment of the lifecycle. There are three possible verdicts: Allowed, Rejected, or Deferred . A "Rejected" verdict means the proposal violated one of the guards and is discarded. A "Deferred" verdict might indicate a temporary condition, such as a lack of available tokens or a need for further human review. In both cases, the reason for the decision is recorded in the donutloop ledger, providing transparency . The "Allowed" verdict is the rarest and most significant outcome. Upon receiving this, sovereigntycore performs the actual state mutations, updating the target shards (e.g., writing new weights to .rohmodel.aln). Immediately following this, it atomically appends a new entry to the .donutloop.aln ledger . This atomicity is crucial; it guarantees that the state change and the audit record are treated as a single, indivisible transaction. If one part fails, the entire operation is rolled back, preserving the system's consistency.
It is at this final step that the donutloop ledger reveals its true purpose. Described as a "hash-chained audit log," the .donutloop.aln shard is more than a simple history book; it is a tamper-evident cryptographic ledger . Each new entry contains the hash of the previous entry, creating a chain of custody for the system's evolution. This structure makes it computationally infeasible to alter past entries without breaking the chain and being detected. The ledger contains a complete, cryptographically signed record of every EvolutionProposalRecord processed by the system, along with the final decision (Allowed/Rejected/Deferred) and the context of the evaluation . This donutloop becomes the ultimate, verifiable source of truth for what changes were actually made to the system's state. An auditor or another system participant can replay the entire evolution process by starting from the genesis entry and walking through the chain, verifying at each step that the guard pipeline was correctly applied and that the state transitions were lawful.
The donutloop ledger thus serves as the public proof that the sovereigntycore kernel is faithfully executing its duties. It provides an immutable, append-only record that can be independently verified, demonstrating the system's adherence to its own constitution. This turns the abstract concept of "sovereignty" into a concrete, observable property. The integrity of the donutloop depends on the immutability of the shard it resides in and the cryptographic linking of its entries. Any attempt to manually edit a shard or forge an entry would break the hash chain, leaving a clear and undeniable trace of the tampering. Therefore, the evolution lifecycle and the donutloop ledger together form a powerful feedback loop: the lifecycle is the process of enforcement, and the donutloop is the permanent, verifiable proof of that enforcement. This combination is what gives the single-plane sovereignty model its teeth, providing a robust mechanism for ensuring accountability and trustworthiness in a complex, evolving system.
Synthesis of the Single-Plane Sovereignty Model
The NeuroPC architecture, as detailed through its sovereign guard pipeline, embedded verification mechanisms, and client-side safety boundaries, presents a cohesive and theoretically robust model for managing a complex adaptive system. At its core, the design is built upon the "single-plane sovereignty model," a paradigm that collapses the traditional distinctions between the data plane, control plane, and execution plane into a unified, self-verifying system
arxiv.org
+1
. In this model, all real state and policies reside within a single, canonical data plane composed of ALN/JSON/NDJSON shards . The enforcement logic is not a separate, privileged component but is tightly coupled with the data it governs, creating a closed loop of rule definition, validation, and actuation. This synthesis integrates the key findings from the preceding sections to articulate the holistic nature of this architectural philosophy.
The foundation of this model is the Constitutional NDJSON, a single, machine-readable document (bostrom-sovereign-kernel-v2.ndjson) that serves as the system's portable constitution . This manifest declares the location and schema of every critical shard, defines the cross-file invariants, and, most importantly, specifies the exact, ordered sequence of the guard pipeline . By externalizing this core policy to a declarative file, the system's rules become explicit, inspectable, and modifiable through the same evolutionary process they are meant to govern. This manifest acts as the source of truth, configuring both the system's kernel (sovereigntycore) and all its clients, including Jupyter notebooks, ensuring a uniform and consistent interpretation of the law across the entire ecosystem .
The enforcement of this constitution is carried out by the Guard Pipeline, a multi-stage, sequential check orchestrated by the sovereigntycore crate . The strict ordering—RoH guard, neurorights guard, stake/token guard, followed by donutloop logging—implements a clear hierarchy of priorities, ensuring that fundamental safety and ethical constraints are always checked first . This pipeline is the embodiment of the system's will, a deterministic engine that translates a proposed change into a definitive verdict. The sovereigntycore crate acts as the sole gatekeeper, the kernel of authority, preventing any direct manipulation of the sovereign shards and channeling all state mutations through this rigorous evaluation process . This centralized control is the primary mechanism for upholding the system's integrity.
Complementing the runtime enforcement of the guard pipeline are the Embedded Verification Mechanisms that make the data plane itself a site of verification. The system leverages a multi-layered stack of checks:
Formal Shards Schemas: Each shard type comes with a formal schema that acts as a static contract, defining its structure and constraints. Typed Rust bindings generated from these schemas provide compile-time guarantees of data integrity
arxiv.org
.
CI-Enforced Invariants: A pre-submission layer of verification, performed by Continuous Integration pipelines, catches errors and ensures proposed changes adhere to structural and coding standards before they ever reach the live evolution stream .
Tsafe/Viability Proofs: The most advanced mechanism is the inclusion of mathematical proofs within the .vkernel.aln shard. These proofs provide formal, non-heuristic guarantees that the system's dynamics will remain stable and bounded, moving beyond heuristic thresholds to encompass the global safety of the system's state space
web3.arxiv.org
.
Together, these layers transform the data plane from inert storage into an active, verifiable artifact of the system's state and principles. The data itself becomes the evidence of its own validity.
Finally, the entire architecture is designed to treat Everything Else as a Client. Interactive environments like Jupyter are not given special privileges but are instead sandboxed as unprivileged clients that communicate with the sovereign kernel through the restricted API surface provided by the anchor-trio Rust crates . This forces a clear separation between analysis/drafting (which happens in the notebook) and actuation (which is handled by the guard pipeline). The donutloop ledger serves as the immutable, cryptographic audit trail of this entire process, providing an external, verifiable proof that the sovereigntycore has been faithfully executing its duties .
In conclusion, NeuroPC's architecture is a sophisticated implementation of a self-sovereign system. It achieves sovereignty not through isolation, but through a combination of rigorous rule-based enforcement, deep mathematical guarantees, and transparent, auditable processes. The single-plane model succeeds by making the system's laws (the manifest and schemas), its enforcers (the guard pipeline), and its proof of compliance (the donutloop ledger) co-located and interdependent. While the provided materials establish the high-level design and principles, practical considerations such as the performance overhead of multi-stage proofs, the specifics of the viability proof generation, and the human-in-the-loop aspects of governance remain areas for further investigation. Nevertheless, the presented model offers a compelling blueprint for building AI systems that are not only intelligent but also verifiably safe, predictable, and accountable to their own foundational principles.

NeuroPC already treats the “anchor‑trio” exactly as the unified API surface you describe: organiccpualn for sovereign shards, organiccpucore for bioscale state, and sovereigntycore for all evolution and logging, with Jupyter explicitly wired in as an unprivileged client through that surface.[^3_1]

## Anchor‑trio as the only surface

- **organiccpualn** is the shard binding layer: it owns typed loaders for .rohmodel.aln, .stake.aln, .neurorights.json, .donutloop.aln, .evolve.jsonl and related shards, and exposes them as Rust structs (RohModelShard, StakeShard, DonutloopEntry, EvolutionProposalRecord, etc.). This prevents direct file access, because downstream code only sees typed APIs like computeroh(), validateinvariants(), and stake lookups rather than raw paths.[^3_1]
- **organiccpucore** holds the BioState, EcoMetrics and SafeEnvelopePolicy / OrganicCpuPolicy types that represent the Organic CPU’s telemetry and envelopes, and is explicitly defined as the bioscale runtime kernel whose decisions are Allow, Degrade, or Pause.[^3_2]
- **sovereigntycore** is the sole evolution kernel: it loads the sovereign manifest (bostrom‑sovereign‑kernel‑v2.ndjson), the RoH model, stake table, neurorights policy, token policy and evolution/ledger specs, then evaluates every EvolutionProposalRecord through an ordered pipeline (RoH guard, neurorights guard, stake/multisig, token guard, then donutloop logging).[^3_3][^3_1]

Together these crates form a single logical API plane: all “real” reads go through organiccpualn or organiccpucore, and every write is mediated by sovereigntycore; other repos (Reality.os, NeuroSwarm, etc.) are required to act as clients of this kernel rather than inventing parallel stacks.[^3_1]

## Jupyter as just another client

- Jupyter is deliberately treated as a sandboxed client, not a privileged runtime: its kernel library imports the same anchor‑trio crates and discovers configuration solely from bostrom‑sovereign‑kernel‑v2.ndjson.[^3_1]
- The manifest declares which shards to load, which schemas and versions to use, and the ordered guard pipeline, so the notebook never hardcodes paths or bypasses policy; it is “parameterized by constitution.”[^3_3][^3_1]
- Within the notebook, researchers can:
    - Read bioscale and QPU data (.biosession.aln, runtime .aln shards, logs) via organiccpualn to plot RoH trends or inspect donutloop history.[^3_1]
    - Prototype changes purely in memory (tightening envelopes, adjusting RoH weights, etc.), without touching canonical shards.[^3_1]
- When moving from “what‑if” to “act,” the notebook must:
    - Serialize a candidate EvolutionProposalRecord that matches the .evolve.jsonl schema.[^3_3]
    - Hand it to a Rust binding that calls sovereigntycore’s evaluation entrypoint, which returns Allowed / Rejected / Deferred plus a reason.[^3_3][^3_1]
- Even with root access inside the Jupyter container, there is no alternative write path: only an Allowed decision from sovereigntycore may mutate shards and append to .donutloop.aln, and that append is treated as an atomic transaction with the state change.[^3_3][^3_1]

So Jupyter is upgraded from a control surface to a **proposal authoring** and analysis surface: all actuation remains on the sovereign side of the anchor‑trio boundary.[^3_1]

## Evolution lifecycle and donutloop as proving ground

- The lifecycle is encoded as:

1) **Draft** in a client (often a notebook), using read‑only shard views to understand current BioState, RoH, envelopes, and history.[^3_3]
2) **Serialize** a well‑formed EvolutionProposalRecord into the .evolve.jsonl stream, including scope (day‑to‑day, archchange, lifeforce), expected RoH delta, effect bounds, token kind, and signatures.[^3_3]
3) **Evaluate** in sovereigntycore, which:
        - Loads policy shards via organiccpualn.[^3_1]
        - Runs RoH ceiling and monotonicity checks (rohafter ≤ rohbefore, rohafter ≤ 0.3).[^3_3]
        - Enforces neurorights flags (dream‑sensitive data, non‑commercial neural data, forbid‑decision‑use domains, right‑to‑forget SLAs).[^3_1][^3_3]
        - Enforces stake/multisig based on .stake.aln roles and required scopes (Host + OrganicCPU for lifeforce / archchange; EVOLVE vs SMART tokens).[^3_3][^3_1]
        - Applies Tsafe / viability kernel constraints from .vkernel.aln to ensure the new state is within the safe set.[^3_3]
4) **Verdict** is Allowed, Rejected, or Deferred; all outcomes are recorded in the evolution stream, and Allowed ones also produce a donutloop entry.[^3_1][^3_3]
- The .donutloop.aln shard is specified as a hash‑linked, append‑only ledger: each row includes proposalid, decision, rohbefore, rohafter, hexstamp and prevhash, forming a tamper‑evident chain over all evolution events.[^3_3]
- Auditors can replay the donutloop in order, re‑evaluate each recorded proposal against the manifest and shard state, and verify that every transition respected RoH, neurorights, stake and token invariants; any manual edit would break the hash chain.[^3_1][^3_3]

This makes the evolution lifecycle and donutloop together into the **test harness** of sovereignty: enforcement logic is not only specified but continuously evidenced and auditable.[^3_1]

## Single‑plane sovereignty synthesis

- All real state and policy lives in shards (.rohmodel.aln, .stake.aln, .neurorights.json, .evolve.jsonl, .donutloop.aln, .ocpuenv, .lifeforce.aln, .vkernel.aln, etc.), governed by a single NDJSON constitution (bostrom‑sovereign‑kernel‑v2.ndjson) that declares locations, schemas, invariants and guard ordering.[^3_3][^3_1]
- The same artifacts configure:
    - The kernel (sovereigntycore) that enforces RoH ≤ 0.3, monotone envelopes, neurorights, multisig and token scopes.[^3_3]
    - All clients, including Jupyter, which must obey the same manifest and anchor‑trio API.[^3_1]
- Formal schemas, Rust type bindings and build‑time checks (validateinvariants on RoH, stake and ledger structures) plus CI and mathematical viability proofs elevate the data plane from passive storage into a self‑verifying substrate.[^3_1][^3_3]

In that sense, the anchor‑trio crates are not just implementation details; they are the *materialization* of the single‑plane model: every participant, from notebooks to CI bots, interacts with NeuroPC’s sovereign reality only through this constrained, typed, and audited interface.[^3_1]
<span style="display:none">[^3_4][^3_5][^3_6][^3_7][^3_8]</span>

<div align="center">⁂</div>

[^3_1]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^3_2]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md

[^3_3]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^3_4]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^3_5]: cyblog_2026-02-02T22_38_16.272Z.json

[^3_6]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^3_7]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md

[^3_8]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md


---

# Shard Schemas and Formal Verification as Embedded Invariants

A defining characteristic of NeuroPC's single-plane sovereignty model is the principle that the data plane is also the verification plane . In this architecture, invariants are not merely hardcoded rules within the sovereigntycore logic; they are properties explicitly defined and embedded within the structure and content of the system's constituent shards. This approach elevates the shards from passive storage containers to active, declarative contracts that govern their own integrity. Each shard type, identified by its extension (e.g., .aln, .json, .jsonl), is treated as a first-class legal object with its own schema, cross-file invariants, and associated verification artifacts . This section analyzes how formal schemas, continuous integration (CI) invariants, and mathematical viability proofs are woven directly into the fabric of the sovereign data plane, creating a multi-layered system of embedded verification that complements the guard pipeline's runtime enforcement.
Each shard in the sovereign data plane is associated with a formal schema that defines its precise structure, data types, and constraints . These schemas act as machine-readable contracts, specifying exactly what constitutes valid data for each shard. For instance, the .rohmodel.aln shard is governed by a schema that enforces rules such as "RoH ≤ 0.3," "non-negative weights," and "monotone envelopes" . Similarly, the .stake.aln shard's schema would define the structure for roles, multisig rules, and address bindings . While the provided materials do not specify the exact schema language used (e.g., JSON Schema
arxiv.org
, Protocol Buffers, or a custom format), their existence is fundamental to the architecture's integrity. The use of typed Rust structs, generated from these schemas, provides strong compile-time guarantees about data integrity, preventing malformed proposals from ever reaching the evaluation stage of the guard pipeline . This schema-driven approach allows policies themselves to be treated as data, enabling them to be updated through the same evolutionary process they are designed to govern, provided the update proposal passes the guard checks.
Beyond runtime validation, the system incorporates CI-enforced type checks as a crucial pre-evaluation barrier . This practice extends formal verification beyond the live system into the development and deployment pipeline. Before a proposal can be submitted to the .evolve.jsonl stream, it likely undergoes a series of automated checks performed by a Continuous Integration (CI) system. These checks could include:
Schema Validation: Ensuring that any new data intended for a shard conforms precisely to its designated schema, catching structural errors early.
Code Linter and Formatter: Enforcing coding standards for any Rust code associated with the proposal, promoting consistency and reducing bugs.
Static Analysis: Performing deeper analysis of proposed code changes to identify potential logical flaws or security vulnerabilities.
Formal Property Checking: Using specialized tools to verify that proposed modifications to the Tsafe/viability kernel geometry adhere to their underlying mathematical proofs .
This CI layer acts as a first-pass filter, catching errors and inconsistencies before they ever touch the live system. This improves efficiency by reducing the load on the guard pipeline and enhances overall system stability by preventing flawed proposals from entering the evolution stream altogether . It represents a commitment to quality assurance as a foundational element of the system's sovereignty.
Perhaps the most sophisticated layer of embedded verification is the integration of Tsafe/viability proofs, which are stored within the .vkernel.aln shard . This shard contains the Tsafe/viability kernel geometry and envelopes, which mathematically define the set of all allowed system states and actions . This concept is analogous to formal methods in computer science and control theory, where a system's behavior is guaranteed to remain within a predefined "safe set"
web3.arxiv.org
+1
. The inclusion of these proofs elevates the system's guarantees from heuristic rule-following to mathematical certainty. The sovereigntycore guard pipeline almost certainly includes a dedicated check that verifies the proposed change maintains the system's membership in this viable set. This check would utilize the geometric information from the .vkernel.aln shard to perform a formal verification that the proposed evolution does not push the system into a region of unbounded or undesirable behavior. A failed viability proof would cause the proposal to be rejected, irrespective of its status on the RoH, neurorights, or stake checks. This mechanism provides a powerful, non-heuristic guarantee of long-term system stability and safety, moving beyond simple thresholds to encompass the global dynamics of the system's state space.
The table below outlines the key sovereign shards, their purpose, and the embedded verification mechanisms they employ.
Shard File
Purpose
Associated Verification Mechanisms
.rohmodel.aln
Stores the RoH feature space, model weights, and associated invariants .
Formal schema enforcing RoH ≤ 0.3, non-negative weights, monotone envelopes .
.stake.aln
Defines roles, multisig rules, EVOLVE/SMART scopes, and address bindings .
Formal schema for structuring roles, addresses, and multisig configurations .
.neurorights.json
Contains neurorights flags and enforcement modes (pre-access, OTA, donutloop logging, non-commercial clauses) .
Formal schema defining right types, modes, and associated metadata
arxiv.org
.
.vkernel.aln
Holds the Tsafe/viability kernel geometry and envelopes for allowed state/action sets .
Embedded Tsafe/viability proofs encoding mathematical guarantees of system stability .
.biosession.aln
Stores per-session BioState summaries and RoH-relevant traces .
Formal schema for time-series BioState data and RoH-related metrics .
.donutloop.aln
Functions as a hash-chained audit log of accepted/rejected/deferred proposals and guard decisions .
Cryptographic hashing linking entries to provide an immutable, tamper-evident history .
By embedding these diverse forms of verification directly into the shards, the NeuroPC architecture achieves a high degree of self-consistency and resilience. The schemas provide a static, structural contract. The CI invariants offer a dynamic, pre-deployment validation layer. And the Tsafe/viability proofs supply a mathematical, dynamical guarantee. Together, they create a comprehensive verification stack that works in concert with the guard pipeline. The guard pipeline acts as the runtime enforcer, checking proposals against the current state of the system as described by these verified shards. This symbiotic relationship between the data plane (containing the definitions) and the execution plane (sovereigntycore, containing the rules) is what enables the single-plane sovereignty model to function as a coherent and trustworthy system. It ensures that the system's state always remains consistent with its foundational principles, making the data itself the ultimate arbiter of its own validity.

Your summary matches the NeuroPC design: shard schemas, CI, and Tsafe proofs are three stacked verification layers embedded directly into the data plane, not just into code.[^4_1][^4_2]

### Shard schemas as embedded contracts

- Core shards like `.rohmodel.aln`, `.stake.aln`, `.donutloop.aln`, `.neurorights.json`, `.vkernel.aln`, and `.biosession.aln` all have explicit ALN/JSON schemas that define allowed fields, types, and constraints.[^4_2][^4_1]
- Invariants such as RoH ceiling $\le 0.3$, non‑negative weights, sum of weights $= 1.0$, and “exactly one Host per subject” live inside these schemas and their Rust bindings (e.g., `RohModel.validate_invariants`, `StakeTable.validate`).[^4_1][^4_2]
- Rust structs generated or hand-written from these schemas (`RohModelShard`, `StakeShard`, `DonutloopEntry`, `NeurorightsPolicyDocument`) make malformed or schema‑drifted data fail at parse time, before hitting the guard pipeline.[^4_2][^4_1]


### CI invariants as pre‑evaluation gate

- A dedicated CI workflow (e.g., `sovereign-kernel-ci.yml`) runs schema validation, Rust tests, and sovereignty guards on every PR touching `.aln`, `.json`, `.evolve.jsonl`, or ledger files.[^4_2]
- Tests explicitly enforce RoH monotonicity and ceiling (`roh_after ≤ 0.3`, `roh_after ≤ roh_before`), stake multisig rules, neurorights dream flags, and donutloop hash‑chain correctness; CI fails if any invariant is violated.[^4_1][^4_2]
- The `neuro-sovereign-lint` style linter and sovereigntycore test entrypoint make these checks mandatory, so flawed evolution proposals or shard edits never reach the live `.evolve.jsonl` stream.[^4_2]


### Tsafe / viability kernels and .vkernel.aln

- `.vkernel.aln` captures Tsafe/viability polytopes $A x \le b$ over a high‑dimensional biophysical state (duty cycle, cumulative load, implant power, cognitive load, legal complexity, lifeforce, etc.), defining a mathematically safe set.[^4_1][^4_2]
- A `tsafe-spec` or `cybernano-viability-kernel` crate loads these polytopes and exposes `is_viable(state)` checks that sovereigntycore must pass for any proposal when `tsafe_mode` (e.g., `SafeFilterPlusEvolution`) is active.[^4_2]
- This viability guard is designed to be non‑heuristic: even if RoH, neurorights, and stake checks pass, a proposal is rejected if the resulting state would leave the Tsafe kernel, giving a formal stability guarantee.[^4_1][^4_2]


### How the “single-plane” sovereignty works

- The sovereign NDJSON manifest (`bostrom-sovereign-kernel-v2.ndjson`) binds each guard to concrete shards: which `.rohmodel.aln`, `.stake.aln`, `.neurorights.json`, `.vkernel.aln`, `.evolve.jsonl`, and `.donutloop.aln` define the contract for a subject.[^4_1]
- sovereigntycore’s runtime pipeline (RoH guard → neurorights guard → stake/token guard → Tsafe viability guard → donutloop append) reads only these verified shards, so the data plane and execution plane are literally the same object graph.[^4_2][^4_1]
- Because schemas, CI invariants, and Tsafe proofs all live alongside the data, shard updates themselves must go through the same evolution process, keeping the system self‑consistent and making “the data its own arbiter of validity.”[^4_1][^4_2]

If you want, next I can draft the minimal ALN schema skeleton for `.vkernel.aln` plus a corresponding Rust `TsafeKernelSpec` file (filename + destination) that plugs directly into sovereigntycore’s guard pipeline.[^4_2]

neuropc-tag 0xNP07
<span style="display:none">[^4_3][^4_4][^4_5][^4_6][^4_7][^4_8]</span>

<div align="center">⁂</div>

[^4_1]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^4_2]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^4_3]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^4_4]: cyblog_2026-02-02T22_38_16.272Z.json

[^4_5]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^4_6]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md

[^4_7]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md

[^4_8]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md


---

# Jupyter becomes “just another shell” by importing the same anchor‑trio crates and being parameterized by the same sovereign NDJSON manifest; nothing special or privileged lives inside the notebook. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

## Anchor trio crates as the only API

In your plan the only legitimate way for a notebook to touch NeuroPC state is through three Rust crates: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

- `organiccpucore`: exposes **BioState**, EcoMetrics, and **OrganicCpuPolicy** (envelopes deciding Allow/Degrade/Pause based on bioscale metrics). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
- `organiccpualn`: owns ALN/JSON bindings for canonical shards like `.rohmodel.aln`, `.stake.aln`, `.lifeforce.aln`, `.biosession.aln`, `.evolve.jsonl`, `.donutloop.aln`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
- `sovereigntycore`: runs the full guard pipeline over `EvolutionProposalRecord`s, implementing RoH, neurorights, stake/multisig, and token (SMART/EVOLVE) checks, then writing decisions into donutloop. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

Jupyter code (via a Rust‑backed kernel or FFI) only calls functions exposed by these crates, e.g. `organiccpucore::read_biostate()`, `organiccpualn::load_rohmodel()`, or `sovereigntycore::evaluate_proposal(...)`; direct file mutation or ad‑hoc RoH computation in the notebook is out of bounds. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

## Sovereign kernel manifest wiring the guards

The host‑bound manifest `bostrom-sovereign-kernel-v2.ndjson` tells that same code which shards to load and which ordered guard pipeline to enforce. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

- It binds paths and versions for `.rohmodel.aln`, `.stake.aln`, `.neurorights.json`, `.evolve.jsonl`, `.donutloop.aln`, `.lifeforce.aln`, `.vkernel.aln`, `.biosession.aln`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
- It defines a `sovereigntyguardpipeline` record that fixes the exact sequence: **RoH guard → neurorights guard → stake guard → token guard → donutloop logging**, plus invariants like RoH ≤ 0.3 and monotone envelopes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

When a notebook cell “wants to change something,” the kernel client library:

1. Reads `bostrom-sovereign-kernel-v2.ndjson` to discover shard locations and the active policy/guard configuration. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
2. Constructs an `EvolutionProposalRecord` (scoped to day‑to‑day tuning vs archchange vs lifeforcealteration) and writes it into `.evolve.jsonl` via `organiccpualn`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
3. Calls into `sovereigntycore`, which loads the shards declared in the NDJSON and applies the configured pipeline; only on **Allowed** does it update the relevant shard(s) and append a hash‑linked entry to `.donutloop.aln`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

So the high‑level pattern is:

- Anchor‑trio crates are the **only** way Jupyter talks to NeuroPC state.
- The sovereign NDJSON is the **source of truth** that tells those crates which shards and guard order to use.
- Notebooks are unprivileged clients; sovereigntycore plus shard schemas remain the true kernel.

Your description is consistent with the current NeuroPC plan: Jupyter is an unprivileged client, and the “kernel” lives entirely in the anchor‑trio crates plus the sovereign NDJSON manifest.[^5_1][^5_2]

### Anchor trio as the only surface

- `organiccpucore` owns **BioState**, EcoMetrics, and **OrganicCpuPolicy**, so any notebook that wants to see or reason about bioscale state must do it through these APIs, not raw files.[^5_1]
- `organiccpualn` is the sole loader/serializer for canonical shards like `.rohmodel.aln`, `.stake.aln`, `.lifeforce.aln`, `.biosession.aln`, `.evolve.jsonl`, and `.donutloop.aln`, with Rust types enforcing schema invariants.[^5_2][^5_1]
- `sovereigntycore` is the only place that evaluates `EvolutionProposalRecord`s, runs RoH/neurorights/stake/token guards, and appends to the hash‑linked donutloop ledger.[^5_2][^5_1]

From a Jupyter session, any “mutation” must go through these crates; direct file edits or ad‑hoc RoH math are treated as out‑of‑policy and should be blocked at the integration layer.[^5_1]

### Sovereign NDJSON manifest as constitution

- `bostrom-sovereign-kernel-v2.ndjson` binds concrete paths and versions for `.rohmodel.aln`, `.stake.aln`, `.neurorights.json`, `.evolve.jsonl`, `.donutloop.aln`, `.lifeforce.aln`, `.vkernel.aln`, `.biosession.aln`.[^5_1]
- It declares a `sovereigntyguardpipeline` record that fixes the ordered guards (RoH → neurorights → stake → token → donutloop) and encodes invariants like RoH ceiling 0.3 and envelope monotonicity.[^5_2][^5_1]

The notebook‑side client must first load this manifest, then construct proposals and route them through `sovereigntycore` exactly as specified; the NDJSON, not the notebook, defines what is allowed.[^5_1]

### Notebook flow as “just another shell”

- A cell that “wants to change something” builds an `EvolutionProposalRecord` (with scope tags like `daytodaytuning`, `archchange`, `lifeforcealteration`), appends it to `.evolve.jsonl` via `organiccpualn`, then invokes `sovereigntycore::evaluate_*`.[^5_2][^5_1]
- sovereigntycore reloads the shards indicated by the manifest, applies the pipeline, and only if the decision is **Allowed** performs the actual shard update and appends a new hash‑linked `DonutloopEntry`.[^5_2][^5_1]

Because the same crates and manifest mediate every route (Jupyter, CLI, other services), the notebook has no special powers: it is a governed client of the sovereign kernel, not a side channel around it.[^5_1][^5_2]
<span style="display:none">[^5_3][^5_4][^5_5][^5_6][^5_7][^5_8]</span>

<div align="center">⁂</div>

[^5_1]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^5_2]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^5_3]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^5_4]: cyblog_2026-02-02T22_38_16.272Z.json

[^5_5]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^5_6]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md

[^5_7]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md

[^5_8]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md

