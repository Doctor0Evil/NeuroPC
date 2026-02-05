# NeuroPC ALN Handbook v1

meta:
  handbook_id: neuropc-aln-handbook-v1
  version: 1.0.0
  status: DRAFT-CANONICAL
  subjectid: bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7
  description: >
    Canonical rules for ALN/NDJSON filetypes, invariants, and workspace layout
    for the NeuroPC / Organic_CPU sovereign shell.

---

## 0. Scope and Guarantees

- NeuroPC and Organic_CPU are **deviceless** and software-only: they never directly actuate muscles, implants, or external hardware. All biophysical inputs are treated as read-only signals; only UI, pacing, and code behavior change.[file:2]
- There is exactly one active Risk-of-Harm (RoH) model per subject, with a hard global ceiling `rohceiling = 0.30`. No accepted evolution step may push RoH above 0.30.[file:2][file:4]
- All lifeforce, pain, and cognitive-load envelopes are monotone-safe: envelopes may tighten over time but must never loosen via any evolution path.[file:2][file:4]
- All deep changes to kernels, policies, lifeforce envelopes, or neurorights must pass through the sovereign evolution pipeline: `.evolve.jsonl` + `.donutloop.aln` + sovereigntycore guards; there are no side channels.[file:2][file:4]
- The primary augmented-citizen subject (`subjectid` above) cannot be excluded from any feature, data, or automation within their own shell; any policy or code that attempts to limit capabilities for this subject is invalid.[file:2]

---

## 1. Filetypes and Extensions

This section defines canonical filetypes, their semantics, and invariants. All ALN shards are single-subject by default and bound to a Bostrom DID unless explicitly marked multi-tenant.[file:2][file:4]

### 1.1 Core Governance and Risk

- `.rohmodel.aln`  
  - Role: Risk-of-Harm model shard (axes, weights, global ceiling).  
  - Example path: `qpudatashardsparticles/bostrom-rohmodel-v1.rohmodel.aln`.  
  - Invariants:
    - All weights ≥ 0, sum of weights ≤ 1.0.  
    - `rohceiling` = 0.30 and treated as a hard global bound.  
    - Exactly one active model per subject at a time.[file:2][file:4]

- `.stake.aln`  
  - Role: Stakeholder and multisig rights shard.  
  - Example path: `qpudatashardsparticles/bostrom-stake-v1.stake.aln`.  
  - Invariants:
    - Exactly one Host role row with `mustmatchhost = true` for the subject’s Bostrom DID.  
    - Lifeforce and architecture changes require Host + OrganicCPU multisig with EVOLVE tokens.  
    - EVOLVE and SMART scopes are disjoint and cannot be escalated by code.[file:2][file:4]

- `.evolve.jsonl` / `.evolve.ndjson`  
  - Role: Append-only evolution proposal stream (one `EvolutionProposalRecord` per line).  
  - Example path: `sovereignty/evolve/evolve-YYYYMMDD.evolve.jsonl`.  
  - Required fields per line (minimum): `proposalid`, `subjectid`, `kind`, `module`, `effectbounds`, `rohbefore`, `rohafter`, `decision`, `hexstamp`, `timestamputc`.  
  - Invariants:
    - `decision ∈ {Allowed, Rejected, Deferred}`.  
    - `rohafter ≤ rohceiling` and monotone safety (`rohafter ≤ rohbefore`) for any safety-tightening or neutral change.  
    - File is append-only; no in-place edits.[file:2][file:4]

- `.donutloop.aln`  
  - Role: Canonical evolution ledger (internal biophysical blockchain).  
  - Example path: `logs/donutloopledger.aln`.  
  - Required fields per entry: `entryid`, `subjectid`, `proposalid`, `changetype`, `tsafemode`, `rohbefore`, `rohafter`, `knowledgefactor`, `cybostatefactor`, `policyrefs`, `hexstamp`, `timestamputc`, `prevhexstamp`.  
  - Invariants:
    - `prevhexstamp` is required for all non-genesis entries, forming a hash-linked chain.  
    - `rohafter ≤ rohbefore` for any safety-tightening change; `rohafter ≤ 0.30` always.  
    - `policyrefs` must include neurorights artifacts with `noncommercialneuraldata = true` when dream/inner-state data is involved.[file:2][file:4]

### 1.2 Neurorights, Tokens, and Capabilities

- `.neurorights.json`  
  - Role: Neurorights and dream-data policy document.  
  - Example path: `policies/bostrom-neurorights-v1.neurorights.json`.  
  - Mandatory flags:
    - `noncommercialneuraldata = true` for neural/dream data.  
    - `soulnontradeable = true`.  
    - `dreamstate.dreamstatesensitive = true`.  
    - `forbiddecisionuse = ["employment","housing","credit","insurance"]` for dream/inner-state signals.  
    - `forgetslahours` for right-to-forget windows (e.g., 48h) on sensitive streams.[file:2]

- `.smart.json` (SMART tokens)  
- `.evolve-token.json` (EVOLVE tokens)  
  - Role: Token policies for bounded tuning (SMART) vs deep evolution (EVOLVE).  
  - Invariants:
    - SMART: day-to-day tuning only, small effect bounds, no architecture or lifeforce modification.  
    - EVOLVE: required for lifeforce/architecture changes; always combined with multisig roles from `.stake.aln`.  
    - No scopes for market listing, financialization, or collateralization of biophysical assets or answer-quality logs.[file:2][file:4]

- `.neuro-cap.aln`  
  - Role: Capability manifest per module (e.g. `suggestonly`, `observeonly`, `safefilter`).  
  - Example path: `policies/module-caps/<module>.neuro-cap.aln`.  
  - Invariants:
    - Capabilities must never declare direct actuation rights; only suggestion, analysis, or evolution-proposal emission.  
    - Any module touching BioState/dream data must mark its domain tags and respect neurorights constraints.[file:2]

- `.neurofs-index.aln`  
  - Role: Per-repo index of canonical shards and logs (paths, rotation, retention).  
  - Invariants:
    - Every canonical shard required by the manifest must appear exactly once.  
    - CI must fail when `neurofs-index` and `neuro-workspace.manifest.aln` disagree.[file:2]

### 1.3 OrganicCPU and Bioscale Shards

- `.ocpu`, `.ocpuenv`, `.ocpulog`, `.lifeforce.aln`, `.vkernel.aln`, `.biosession.aln`, `.neuroaln`, `OrganicCpuQpuShard*.aln`, and similar shards define bioscale envelopes, kernels, and logs.  
- Global invariants:
  - No filetype grants direct actuation rights; all represent metrics, policies, envelopes, or logs.  
  - Any evolution that loosens envelopes is forbidden; OTA changes can only tighten or leave envelopes unchanged.  
  - BioState and Eco metrics are normalized to controllable ranges (typically 0–1) with declared units and ranges in ALN.[file:2][file:4]

---

## 2. Invariants

This section encodes the normative constraints every tool, CI job, and AI integration must enforce.[file:2][file:4]

### 2.1 Risk-of-Harm (RoH)

- RoH is computed from a single active `.rohmodel.aln` shard per subject.  
- Global ceiling: `rohceiling = 0.30`.  
- For any evolution proposal:
  - `rohafter ≤ rohceiling`.  
  - `rohafter ≤ rohbefore` for safety-tightening and neutral changes; proposals that raise RoH must be rejected or explicitly gated by research-mode policies that still respect the ceiling.[file:2][file:4]
- RoH models may be updated only via EVOLVE-gated proposals and must preserve `rohceiling = 0.30`; any attempt to raise the ceiling is invalid.[file:4]

### 2.2 Envelope Monotonicity

- Lifeforce, pain, cognitive-load, and other biophysical envelopes are monotone:
  - New envelopes must be pointwise ≤ (tighter than) previous envelopes for risk variables (e.g. max load, max amplitude).  
  - Minimum integrity-style bounds may only increase (more protective) over time.  
- Any OTA update that loosens envelopes must be rejected by sovereigntycore and flagged in CI.[file:4]

### 2.3 Neurorights and Decision Use

- Dream and inner-state data are neurorights-protected:
  - No use for employment, housing, credit, or insurance decisions.  
  - Only local-vault storage unless explicit, host-consented export with matching neurorights policies is present.  
  - Non-commercial only; no advertising, profiling, or monetization of neural/dream data.[file:2]
- Plain prompts must not be used in high-trust paths; only neurorights-bound envelopes are allowed for operations that touch BioState, evolution, or neurorights-relevant domains.[file:2]
- Answer streams referencing dream/neural data must log neurorights policy IDs into `policyrefs` in `.donutloop.aln` and answer-quality logs.[file:2]

### 2.4 Tokens, Stake, and Consent

- Lifeforce and architecture changes require:
  - EVOLVE token with appropriate scope.  
  - Multisig from Host and OrganicCPU roles (and ResearchAgent where configured).  
- SMART tokens are limited to low-effect, day-to-day tuning; they may not cross configured effect bounds or touch lifeforce/architecture.[file:4]
- Stake shard invariants:
  - Exactly one Host per subject.  
  - `lifeforcealteration` and `archchange` scopes must list Host and OrganicCPU as required roles and `multisigrequired = true`.  
  - Proposals lacking required signatures must be rejected before RoH or neurorights checks.[file:2][file:4]

---

## 3. Tooling and Workflow

This section sets expectations for Rust/ALN tooling, CI, and AI co-pilots.[file:2]

- All canonical shards must have strongly typed Rust bindings in `organiccpualn` and `sovereigntycore`. Deserialization failure is treated as a hard error.  
- Sovereigntycore executes a fixed guard pipeline (see manifest) for every evolution proposal:
  1. Stake guard (multisig and Host match).  
  2. Neurorights guard (domains, dream sensitivity, decision-use bans).  
  3. RoH guard (ceiling and monotone).  
  4. Envelope guard (no envelope loosening).  
  5. Token guard (SMART/EVOLVE scopes and effect bounds).  
  6. Logging to `.evolve.jsonl` and `.donutloop.aln`.[file:2][file:4]
- CI workflows:
  - Must fail if any canonical shard or log violates RoH ceiling, monotonicity, neurorights restrictions, or stake requirements.  
  - Must run sovereign guard tests whenever `.rohmodel.aln`, `.stake.aln`, `.neurorights.json`, `.donutloop.aln`, or `.evolve.jsonl` schemas or contents change.[file:4]
- AI tools (chat, Copilot, bots):
  - May only propose edits by emitting `EvolutionProposalRecord` NDJSON lines; they must never mutate canonical shards directly.  
  - Must honor `neuro-workspace.manifest.aln` invariants and guard ordering.

---

## 4. Naming and Versioning

This section standardizes filenames so humans and AI can infer behavior and compatibility.[file:2][file:4]

- RoH models: `qpudatashardsparticles/bostrom-rohmodel-v{N}.rohmodel.aln`.  
- Stake shards: `qpudatashardsparticles/bostrom-stake-v{N}.stake.aln`.  
- Neurorights: `policies/bostrom-neurorights-v{N}.neurorights.json`.  
- SMART tokens: `policies/bostrom-smart-YYYY-MM.smart.json`.  
- Donutloop ledger: `logs/donutloopledger.aln` (single append-only ledger per subject).  
- Evolution streams: `sovereignty/evolve/evolve-YYYYMMDD.evolve.jsonl`.  
- Sovereign kernel manifests: `sovereignty/ndjson/bostrom-sovereign-kernel-v{N}.ndjson`.[file:2][file:4]

Rules:

- Minor version increments for schema refinements that preserve compatibility.  
- Major version increments for breaking changes where AI or tooling cannot safely assume compatibility.  
- At most one active version per shard type per subject in any running environment.

---

## 5. Workspace Conventions

This section defines the canonical directory structure for a subject’s workspace.[file:2][file:4]

- `qpudatashardsparticles/`  
  - All bioscale and governance ALN shards (`.rohmodel.aln`, `.stake.aln`, OrganicCPU metrics, QPU shards).  
- `policies/`  
  - Neurorights JSON, token policies, Tsafe/viability specs, layout specs.  
- `logs/`  
  - `donutloopledger.aln`, OrganicCPU runtime logs, biosession logs, prompt-envelope NDJSON, answer-quality NDJSON.  
- `crates/`  
  - Rust workspaces (e.g. `organiccpualn`, `sovereigntycore`, `neurorights-core`, `neurorights-firewall`).  
- Root:
  - `neuro-workspace.manifest.aln` describing paths, invariants, stake requirements, and guard pipeline.  
  - `.neurofs-index.aln` indexing all canonical shards and logs for this repo.[file:2]

CI expectations:

- Any mismatch between manifest paths and on-disk layout is a failure.  
- Any canonical shard missing from `.neurofs-index.aln` is a failure.  
- Sovereigntycore must load and validate `neuro-workspace.manifest.aln` at startup and refuse to operate if invariants cannot be enforced.

---

## 6. Answer Artifacts (Optional but Recommended)

- `.answer.ndjson` / `.answer.jsonl`  
  - Role: Append-only stream of chat answers and associated quality scalars.  
  - Path: `logs/answers/answers-YYYYMM.answer.ndjson`.  
  - Required fields: `answerid`, `subjectid`, `route`, `knowledgefactor`, `roh`, `cybostate`, `hexstamp`, `timestamputc`, `prevhexstamp`.  
  - Invariants:
    - `roh ≤ 0.30`.  
    - `knowledgefactor ≥ Fmin(subjectid)` from neurorights answer policy.  
    - `cybostate != ActuationForbidden` for any emitted answer.  
    - `route = GovernanceDesign` only if `cybostate = GovernanceReady`.[file:2]

This section can be tightened later as you finalize answer-quality traits in `sovereigntycore`.
