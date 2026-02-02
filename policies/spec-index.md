# NeuroPC Policy & Shard Index (v1)

- `qpudatashards/particles/bostrom-rohmodel-v1.rohmodel.aln`  
  RoH model; one active row, ceiling ≤ 0.3, weights ≥ 0, sum(weights) ≤ 1.

- `qpudatashards/particles/evolution-proposals.evolve.jsonl`  
  Append-only evolution proposals; invariant: roh_after ≤ roh_before ≤ 0.3.

- `policies/bostrom-stake-v1.stake.aln`  
  Stakeholder roles; one Host per subject; lifeforce/arch-change require Host + OrganicCPU signatures.

- `policies/bostrom-neurorights-v1.neurorights.json`  
  Neurorights + dream flags; dreamstate is advisory, not for employment/housing/credit/insurance decisions.

- `policies/bostrom-smart-2026-01.smart.json`  
  SMART token; small, UI-level evolution envelope.

- `logs/donutloopledger.aln`  
  Canonical, append-only donutloop ledger; each entry links to a proposal and policies via hexstamp and policy_refs.
