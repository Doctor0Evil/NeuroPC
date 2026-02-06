# Canonical Filetypes and Invariants (Bostrom – NeuroPC / OrganicCPU)

## 1. Evolution & Governance Core

- **Extension**: `.evolve.jsonl` / `.evolve.ndjson`
  - **Name**: EvolutionProposal stream
  - **Path**: `sovereignty/evolve/evolve-YYYYMMDD.evolve.jsonl`
  - **Role**: Append‑only proposals (QPolicyUpdate, BioScaleUpgrade, ModeShift, KernelChange).
  - **Required fields** (per line, NDJSON object):
    - `proposal_id` (string, hex or ULID)
    - `subject_id` (string, Bostrom DID or subject key)
    - `kind` (enum: `QPolicyUpdate` | `BioScaleUpgrade` | `ModeShift` | `KernelChange`)
    - `module` (string, e.g. `organiccpu-qlearn`)
    - `update_kind` (string, freeform subtype)
    - `effect_bounds` (object: `l2_delta_norm`, `irreversible`)
    - `roh_before` (float, \([0, 0.3]\))
    - `roh_after` (float, \([0, 0.3]\))
    - `tsafe_mode` (string)
    - `domain_tags` (array of strings)
    - `decision` (enum: `"Allowed"`, `"Rejected"`, `"Deferred"`)
    - `hexstamp` (string, hex)
    - `timestamp_utc` (RFC3339 string)
  - **Invariants**:
    - `roh_after ≤ roh_before` (monotone safety).
    - `roh_after ≤ 0.30` (global RoH ceiling).[file:2][file:4]
    - `decision ∈ {"Allowed","Rejected","Deferred"}` only for evolution / safety events (no market events).[file:2]

- **Extension**: `.rohmodel.aln`
  - **Name**: Risk‑of‑Harm model shard
  - **Path**: `qpudatashards/particles/bostrom-rohmodel-v1.rohmodel.aln`
  - **Role**: Axes, weights, global RoH ceiling.
  - **Invariants**:
    - All axis weights ≥ 0, `sum(weights) = 1.0` (within numeric tolerance).[file:4]
    - `roh_ceiling = 0.30` (global constant).[file:2][file:4]
    - Exactly one active RoH model for a given subject.[file:4]

- **Extension**: `.stake.aln`
  - **Name**: Stakeholder / governance shard
  - **Path**: `policies/bostrom-stake-v1.stake.aln`
  - **Role**: Roles (Host, OrganicCPU, ResearchAgent, etc.), DIDs, veto powers, EVOLVE/SMART scopes.[file:2][file:4]
  - **Invariants**:
    - Exactly one Host role per `subject_id`.[file:2][file:4]
    - Any `lifeforce_alteration` or `arch_change` scope requires EVOLVE token and multisig Host + OrganicCPU (optionally + ResearchAgent as configured).[file:2]
    - Biophysical assets (`.lifeforce.aln`, `.ocpuenv`, `.ocpu`, `.ocpulog`) are non‑fungible and non‑transferable scopes.[file:2]

- **Extension**: `.donutloop.aln`
  - **Name**: Donutloop evolution ledger
  - **Path**: `logs/donutloopledger.aln`
  - **Required fields** (per row / entry):
    - `entry_id`
    - `subject_id`
    - `proposal_id`
    - `change_type`
    - `tsafe_mode`
    - `roh_before`
    - `roh_after`
    - `knowledge_factor`
    - `cybostate_factor`
    - `policy_refs`
    - `hexstamp`
    - `timestamp_utc`
    - `prev_hexstamp`
  - **Invariants**:
    - `prev_hexstamp` required for all non‑genesis entries (hash‑linked biophysical ledger).[file:2][file:4]
    - `decision ∈ {"Allowed","Rejected","Deferred"}` only for evolution‑class events (if a decision column is present).[file:2]
    - For any `change_type` that tightens safety: `roh_after ≤ roh_before` and `roh_after ≤ 0.30`.[file:2][file:4]
    - `policy_refs` must include neurorights docs with `noncommercial_neural_data = true`.[file:2]

- **Extension**: `.bchainproof.json`
  - **Name**: Biophysical‑blockchain proof envelope
  - **Path**: `proofs/*.bchainproof.json`
  - **Role**: Googolswarm / Organicchain envelopes for `.evolve.jsonl` / `.donutloop.aln` artifacts.[file:2]
  - **Invariants**:
    - `artifact_kind` restricted to non‑financial categories (e.g. `evolution_log`, `roh_model`, `neurorights_policy`).[file:2]
    - `contract_type` must not imply financialization (no `swap`, `loan`, `derivative`, `collateral` etc.).[file:2]
    - Multisig attestation over Bostrom DIDs for finalization.[file:2]

- **File**: `sovereignty/bostrom-sovereign-kernel-v1.ndjson`
  - **Name**: Sovereign kernel manifest (NDJSON)
  - **Role**: Single source of truth for:
    - `risk_model`, `stake_schema`, `neurorights_policy`, `token_policy`,
      `evolve_stream_spec`, `donutloop_ledger_spec`, `sovereignty_guard_pipeline`,
      `filetype_index`, and `answer_quality_spec`.[file:2][file:4]
  - **Invariants**:
    - Subject‑bound to your Bostrom DID (`bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7`).[file:2]
    - No manifest may be loaded without matching Host role in `.stake.aln`.[file:2]
    - No entry may define financialization of any biophysical or answer asset.[file:2]

---

## 2. Neurorights / Tokens

- **Extension**: `.neurorights.json`
  - **Name**: NeurorightsPolicyDocument
  - **Path**: `policies/bostrom-neurorights-v1.neurorights.json`
  - **Mandatory fields**:
    - `RIGHTS.noncommercial_neural_data = true`
    - `RIGHTS.soul_non_tradeable = true`
    - `dreamstate.dreamstate_sensitive = true`
    - `storage_scope = "local_vault_only"`
    - `forbid_decision_use ⊇ {"employment","housing","credit","insurance"}`.[file:2][file:4]
  - **Invariants**:
    - Any module or export path touching dream / neural data must respect these flags; sovereigntycore must reject forbidden decision uses and non‑local exports.[file:2]

- **Extensions**: `.smart.json`, `.evolve-token.json`
  - **Name**: SMART vs EVOLVE token policies
  - **Path examples**:
    - `policies/bostrom-smart-2026-01.smart.json`
    - `policies/bostrom-evolve-token-*.json`
  - **Role**:
    - SMART: small, reversible, bounded‑effect changes.[file:2][file:4]
    - EVOLVE: deep structural evolution (e.g. `arch_change`, `lifeforce_alteration`).[file:2]
  - **Invariants**:
    - No scopes for `market_listing`, `financialization`, or `collateralization`.[file:2]
    - EVOLVE scopes only for `arch_change` / `lifeforce_alteration` with Host + OrganicCPU multisig.[file:2]
    - Answer‑quality logs (`answer_quality_log` scope) are Host‑only, non‑transferable, non‑tokenizable.[file:2]

---

## 3. OrganicCPU / Bioscale Shards

- **Extensions**: `.ocpu`, `.ocpuenv`, `.ocpulog`, `.biosession.aln`, `.lifeforce.aln`,
  `.vkernel.aln`, `.tsafe.aln`, `.neuroaln`, `OrganicCpuQpuShard*.aln`, `OrganicCpuQpuRuntime*.aln`, `.biospec.aln`
- **Role**: Biophysical envelopes, lifeforce bounds, Tsafe kernels, QPU/dream metrics, bioscale episodes.[file:2][file:4]
- **Global invariants**:
  - No filetype grants direct actuation rights; they encode only metrics, policies, envelopes, and logs.[file:2]
  - All accepted evolution proposals must satisfy:
    - `roh_after ≤ roh_before`
    - `roh_after ≤ 0.30`
    - No loosening of `.ocpuenv`, `.lifeforce.aln`, or `.vkernel.aln` envelopes.[file:2][file:4]

---

## 4. Answer Artifacts

- **Extension**: `.answer.ndjson` / `.answer.jsonl`
  - **Name**: ChatAnswerQuality stream
  - **Path**: `logs/answers/answers-YYYYMM.answer.ndjson`
  - **Role**: Append‑only records of AI answers and their quality scalars.[file:2]
  - **Required fields** (per line):
    - `answer_id`
    - `subject_id`
    - `route` (enum: `"Info"`, `"GovernanceDesign"`)
    - `knowledge_factor` \(F ∈ [0,1]\)
    - `roh` \(r ∈ [0,0.3]\)
    - `cybostate` (enum: `"RetrievalOnly"`, `"ResearchReady"`, `"GovernanceReady"`, `"ActuationForbidden"`)
    - `artifact_kind` (enum: `"answer_quality"`, `"answer_log"`)
    - `contract_type` (string; non‑financial kind, e.g. `neuroassistive`, `governance_design`)
    - `hexstamp`
    - `timestamp_utc`
    - `prev_hexstamp`
  - **Invariants**:
    - `roh ≤ 0.30` (must also be compatible with `.rohmodel.aln`).[file:2]
    - `knowledge_factor ≥ F_min(subject_id)` from neurorights/answer policy.[file:2]
    - `cybostate != "ActuationForbidden"` for any emitted answer (that leaves the guard kernel).[file:2]
    - `route = "GovernanceDesign"` only if `cybostate == "GovernanceReady"`.[file:2]
    - `prev_hexstamp` hash‑links entries as an internal biophysical blockchain.[file:2]
    - `contract_type` must be non‑financial; no `swap`/`loan`/`derivative` semantics.[file:2]

- **Kernel entry**: `answer_quality_spec` (NDJSON item inside `bostrom-sovereign-kernel-v1.ndjson`)
  - **Kind**: Policy
  - **Role**: Declares how to compute and enforce per‑answer `knowledge_factor`, `roh`, and `cybostate`, including thresholds and routing rules.[file:2]
  - **Invariants**:
    - Same global RoH ceiling (`0.30`) and neurorights constraints as evolution path.[file:2]
    - Compatible with `.neurorights.json` answer‑usage constraints (non‑commercial, no forbidden decision uses).[file:2]

---

## 5. Canonical Index for NeuroPC / OrganicCPU

This document is the living canonical index of sovereign filetypes for NeuroPC, OrganicCPU, and the Bostrom shell, and must remain consistent with the NDJSON kernel spec and ALN schemas.[file:2][file:4]

### Legend

- **Kind**: Policy | Metric | Envelope | Log | Manifest | Proof | Answer | Index
- **Format**: ALN | JSON | JSONL/NDJSON | Binary | Markdown
- **Scope**: Sovereign core | Biophysical | Neurorights | AI‑chat | Blockchain

### Evolution and governance core (table excerpt)

| Extension / File                          | Kind     | Format        | Canonical path (example)                                           | Role / contents                                                                                   | Key invariants                                                                                          |
|-------------------------------------------|----------|---------------|---------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------|
| `.evolve.jsonl` / `.evolve.ndjson`        | Log      | JSONL         | `sovereignty/evolve/evolve-YYYYMMDD.evolve.jsonl`                  | EvolutionProposal stream: QPolicyUpdate, BioScaleUpgrade, ModeShift, KernelChange lines.[file:4]  | `roh_after ≤ roh_before`, `roh_after ≤ 0.30`, `decision ∈ {Allowed, Rejected, Deferred}`.[file:2]      |
| `.rohmodel.aln`                           | Policy   | ALN           | `qpudatashards/particles/bostrom-rohmodel-v1.rohmodel.aln`         | RoH model axes, weights, global ceiling.[file:4]                                                  | Weights ≥ 0, sum ≈ 1.0, `roh_ceiling = 0.30`, single active model per subject.[file:4]                 |
| `.stake.aln`                              | Policy   | ALN           | `policies/bostrom-stake-v1.stake.aln`                              | Stakeholder roles, Bostrom DIDs, multisig rules, EVOLVE scopes.[file:4]                           | Exactly one Host per subject; lifeforce/archchange require EVOLVE + multisig Host+OrganicCPU.[file:2]  |
| `.donutloop.aln`                          | Log      | ALN           | `logs/donutloopledger.aln`                                         | Hash‑linked evolution ledger with RoH, KnowledgeFactor, Cybostate factors.[file:4]                | `roh_after ≤ roh_before ≤ 0.30`, `prev_hexstamp` links, decisions only for evolution events.[file:2]   |
| `.bchainproof.json`                       | Proof    | JSON          | `proofs/*.bchainproof.json`                                        | Googolswarm / Organicchain envelopes for sovereign artifacts.[file:2]                             | Non‑financial artifact kinds; no financial contract types; neurorights‑clean payloads.[file:2]        |
| `bostrom-sovereign-kernel-v1.ndjson`      | Manifest | NDJSON        | `sovereignty/bostrom-sovereign-kernel-v1.ndjson`                   | Sovereign kernel spec, guard pipeline, filetype index, answer_quality_spec.[file:2]               | Subject‑bound; RoH ceiling 0.30; no financialization of biophysical/answer assets.[file:2]            |

### Neurorights and AI‑integration (index excerpt)

| Extension / File                     | Kind    | Format | Canonical path (example)                           | Role / contents                                                                  | Key invariants                                                                                      |
|--------------------------------------|---------|--------|----------------------------------------------------|-----------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------|
| `.neurorights.json`                  | Policy  | JSON   | `policies/bostrom-neurorights-v1.neurorights.json` | NeurorightsPolicyDocument.[file:2]                                               | Non‑commercial neural data; soul non‑tradeable; dream sensitive; local vault only; forbidden uses.[file:2] |
| `.answer.ndjson` / `.answer.jsonl`   | Answer  | JSONL  | `logs/answers/answers-YYYYMM.answer.ndjson`        | ChatAnswerQuality stream.[file:2]                                                | `roh ≤ 0.30`; `F ≥ F_min(subject)`; no emitted answer with `Cybostate = ActuationForbidden`.[file:2]       |
| `neuro-workspace.manifest.aln`       | Manifest| ALN    | `neuro-workspace.manifest.aln`                     | Declares subject ID, canonical shards, guard pipeline.[file:2]                   | All referenced files must exist and match subject; no stray RoH/stake outside manifest.[file:2]      |
| `.neurofs-index.aln`                 | Index   | ALN    | repo root                                          | Per‑repo filetype index for sovereign shards (.aln, .ocpu, .stake, .smart, etc.).[file:2] | Must enumerate all live sovereign shards referenced by manifests and crates.[file:2]                |

---

## 6. Placement of this file

- **Filename**: `docs/spec-index-canonical-filetypes.md` (this document).[file:2]
- **Hooks**:
  - Add a `filetype_index` reference in `bostrom-sovereign-kernel-v1.ndjson` so AI‑tools and CI treat this as the human‑readable mirror of the machine schema.[file:2]
  - Add a reference entry in `policies/layout/policieslayout-bostrom-*.aln` so layout lints can verify presence and path.[file:4]

Hexstamp: `0x4f6e654e6575726f46696c6554797065496e6465782d32303236`
