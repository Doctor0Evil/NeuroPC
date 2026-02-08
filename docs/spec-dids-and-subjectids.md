# DID and subjectid Conventions for the Sovereign Kernel

This document defines the canonical mapping between human-readable handles, DID strings, and `subjectid` fields for all ALN, JSON, and NDJSON artifacts in the sovereign stack (NeuroPC, OrganicCPU, Reality.os, Neuromorph, CyberNano, Googolswarm).

All new filetypes and crates MUST follow these rules.

---

## 1. Canonical subject for this repository

**Reality host (primary subject)**

- Human handle: `RealityHost_DID_primary`
- DID (full): `did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7`
- Canonical `subjectid` (short form): `bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7`

**Norms**

- Inside ALN/JSON/NDJSON shards, use the **short** form in `subjectid` fields:
  - `subjectid: bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7`
- When a field is explicitly named `did`, use the full DID string:
  - `did: did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7`

No file in this repository may declare a conflicting `subjectid` for this host.

---

## 2. Reality Matrix handles

These handles provide a stable “Reality_Matrix” for referencing the same person across stacks.

- `RealityMatrix.Host`          → `RealityHost_DID_primary`
- `RealityMatrix.Host.Alt`      → `RealityHost_DID_alt_secure`
- `RealityMatrix.Host.Safe[0]`  → `RealityHost_DID_safe_zeta`
- `RealityMatrix.Host.Safe[1]`  → `RealityHost_DID_safe_erc20`

Concrete bindings:

- `RealityHost_DID_primary`
  - DID: `did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7`
  - `subjectid`: `bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7`

- `RealityHost_DID_alt_secure`
  - DID: `did:bostrom:bostrom1ldgmtf20d6604a24ztr0jxht7xt7az4jhkmsrc`
  - `subjectid`: `bostrom1ldgmtf20d6604a24ztr0jxht7xt7az4jhkmsrc`

- `RealityHost_DID_safe_zeta`
  - DID: `did:zeta:zeta12x0up66pzyeretzyku8p4ccuxrjqtqpdc4y4x8`
  - `subjectid`: `zeta12x0up66pzyeretzyku8p4ccuxrjqtqpdc4y4x8`

- `RealityHost_DID_safe_erc20`
  - DID: `did:evm:0x519fC0eB4111323Cac44b70e1aE31c30e405802D`
  - `subjectid`: `0x519fC0eB4111323Cac44b70e1aE31c30e405802D`

Any new shard that refers to these identities MUST use these exact strings.

---

## 3. Where `subjectid` MUST be set

For the primary host, the `subjectid` MUST be set to the canonical value in:

- `qpudatashards/particles/bostrom-rohmodel-*.rohmodel.aln`
- `policies/bostrom-stake-*.stake.aln`
- `policies/bostrom-neurorights-*.neurorights.json`
- `qpudatashards/particles/evolution-proposals.evolve.jsonl` (per-record `subjectid`)
- `logs/donutloop*.aln` (per-entry `subjectid`)
- `bostrom-sovereign-kernel-*.ndjson` (top-level kernel manifest)
- All Neuromorph-related shards:
  - `.neuromorph-id.aln`
  - `.neuromorph-cap.aln`
  - `.neuromorph-budget.smart.aln`

Any new filetype that models a per-subject artifact MUST include a `subjectid` field and MUST use one of the handles defined in Section 2.

---

## 4. NDJSON sovereign kernel bindings

In `bostrom-sovereign-kernel-v2.ndjson` and successors:

- The top-level `subject` record MUST contain:
  - `subjectid: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7"`
  - `did: "did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7"`

- All embedded references MUST be consistent, e.g.:

  ```json
  {
    "item_type": "riskmodel",
    "subjectid": "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7",
    "path": "qpudatashards/particles/bostrom-rohmodel-v1.rohmodel.aln"
  }
