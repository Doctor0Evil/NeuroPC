# Canonical Filetypes and Invariants (Bostrom – NeuroPC / OrganicCPU)

## 1. Evolution & Governance Core

- **Extension**: `.evolve.jsonl`
  - **Name**: EvolutionProposal stream
  - **Path**: `sovereignty/evolve/evolution-proposals-YYYYMM.evolve.jsonl`
  - **Role**: Append-only proposals (QPolicyUpdate, BioScaleUpgrade, ModeShift, KernelChange).
  - **Required fields**: `proposalid`, `subjectid`, `kind`, `scope`, `effectbounds`, `rohbefore`, `rohafter`, `decision`, `hexstamp`, `timestamp`.
  - **Invariants**:
    - `rohafter ≤ rohbefore` (monotone safety) and `rohafter ≤ 0.3`.
    - `decision ∈ {"Allowed","Rejected","Deferred"}`.

- **Extension**: `.rohmodel.aln`
  - **Name**: Risk-of-Harm model shard
  - **Path**: `qpudatashards/particles/bostrom-rohmodel-v1.rohmodel.aln`
  - **Role**: Axes, weights, global RoH ceiling.
  - **Invariants**:
    - All weights ≥ 0, `sum(weights) = 1.0`.
    - `rohceiling = 0.3` (global constant).

- **Extension**: `.stake.aln`
  - **Name**: Stakeholder / governance shard
  - **Path**: `policies/bostrom-stake-v1.stake.aln`
  - **Invariants**:
    - Exactly one Host per `subjectid`.
    - Any `lifeforcealteration` or `archchange` scope requires Host + OrganicCPU multisig.
    - Biophysical assets (.lifeforce.aln, .ocpuenv, .ocpu, .ocpulog) are non-fungible, non-transferable.

- **Extension**: `.donutloop.aln`
  - **Name**: Donutloop evolution ledger
  - **Path**: `logs/donutloopledger.aln`
  - **Required fields**:
    - `entryid`, `subjectid`, `proposalid`, `changetype`, `tsafemode`,
      `rohbefore`, `rohafter`, `knowledgefactor`, `cybostatefactor`,
      `policyrefs`, `hexstamp`, `timestamputc`, `prevhexstamp`.
  - **Invariants**:
    - `prevhexstamp` required for all non-genesis entries.
    - `decision ∈ {"Allowed","Rejected","Deferred"}` for evolution events only.
    - `rohafter ≤ rohbefore`, `rohafter ≤ 0.3` for any `changetype` that tightens safety.
    - `policyrefs` must include neurorights docs with `noncommercial_neural_data = true`.

## 2. Neurorights / Tokens

- **Extension**: `.neurorights.json`
  - **Role**: NeurorightsPolicyDocument.
  - **Mandatory fields**:
    - `RIGHTS.noncommercial_neural_data = true`
    - `RIGHTS.soulnontradeable = true`
    - `dreamstate.dreamstate_sensitive = true`
    - `storage_scope = "local_vault_only"`
    - `forbid_decision_use ⊇ {"employment","housing","credit","insurance"}`.

- **Extension**: `.smart.json`, `.evolve-token.json`
  - **Role**: SMART vs EVOLVE token policies.
  - **Invariants**:
    - No scopes for `market_listing`, `financialization`, `collateralization`.
    - EVOLVE only for `archchange` / `lifeforcealteration` with multisig (Host + OrganicCPU).

## 3. OrganicCPU / Bioscale Shards

- **Extensions**: `.ocpu`, `.ocpuenv`, `.ocpulog`, `.biosession.aln`, `.lifeforce.aln`,
  `.vkernel.aln`, `.tsafe.aln`, `.neuroaln`, `OrganicCpuQpuShard*.aln`, etc.
- **Role**: Biophysical envelopes, lifeforce bounds, Tsafe kernels, QPU/dream metrics.
- **Global invariants**:
  - No filetype grants direct actuation rights (metrics, policies, envelopes, logs only).
  - All proposals must satisfy `roh_after ≤ roh_before`, `roh_after ≤ 0.3`, and may not loosen envelopes.

## 4. Answer Artifacts (New)

- **Extension**: `.answer.ndjson` (or `.answer.jsonl`)
  - **Name**: ChatAnswerQuality stream
  - **Path**: `logs/answers/answers-YYYYMM.answer.ndjson`
  - **Role**: Append-only records of AI answers and their quality scalars.
  - **Required fields**:
    - `answer_id`, `subjectid`, `route` ("Info","GovernanceDesign"),
      `knowledgefactor` (F ∈ [0,1]),
      `roh` (r ∈ [0,0.3]),
      `cybostate` ("RetrievalOnly","ResearchReady","GovernanceReady","ActuationForbidden"),
      `hexstamp`, `timestamputc`, `prev_hexstamp`.
  - **Invariants**:
    - `roh ≤ 0.3`.
    - `knowledgefactor ≥ F_min(subjectid)` from neurorights/answer policy.
    - `cybostate != "ActuationForbidden"` for any emitted answer.
    - `route = "GovernanceDesign"` only if `cybostate == "GovernanceReady"`.
    - `prev_hexstamp` hash-links entries (internal biophysical blockchain).
    - `artifact_kind ∈ {"answer_quality","answer_log"}`; `contract_type` must be non-financial.

## 5. Sovereign Kernel NDJSON

- **File**: `sovereigntyndjson/bostrom-sovereign-kernel-v1.ndjson`
- **Role**: Single source of truth for:
  - `riskmodel`, `stakeschema`, `neurorightspolicy`, `tokenpolicy`,
    `evolvestreamspec`, `donutloopledgerspec`, `sovereigntyguardpipeline`,
    and now **`answerqualityspec`**.
- **Invariants**:
  - Subject‑bound to your Bostrom DID.
  - No manifest may be loaded without matching Host role in `.stake.aln`.
  - No entry may define financialization of any biophysical or answer asset.

