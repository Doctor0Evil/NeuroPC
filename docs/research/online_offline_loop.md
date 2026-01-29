# Online Circadian Micro-Epochs and Offline Dreaming in NeuroPC

This document specifies the schemas and control flow that bridge online
circadian-aligned micro-epochs with offline "dreaming" consolidation
via Organichain-visible learning loops.

## Core Filetypes

- `.bioflux/v1.0`: biophysical state traces (dopamine/H2O2, fatigue, focus...)
- `.evo/v1.0`: evolution pattern definitions (family, risk, intervals).
- `.evobudget/v1.0`: daily caps per pattern and globally.
- `.circmap/v1.0`: circadian states and allowed pattern families.
- `.neuro_rights/v1.0`: neurorights constraints and integrity limits.
- `.ghost_evo/v1.0`: offline replay episodes referencing online experiments.
- `.sim_organic/v1.0`: simulator outputs for predicted biophysical effects.

## Online Loop

1. Scheduler reads `.circmap`, `.evobudget`, `.neuro_rights`, and live `.bioflux`.
2. For each candidate `.evo` interval, it:
   - checks budget caps,
   - checks circadian state eligibility,
   - enforces neurorights (risk, reversibility, logging).
3. Approved intervals are tagged with `experiment_id` and logged for Organichain.

## Offline Loop ("Dreaming")

1. During eligible circadian phases, a ghost scheduler selects prior `experiment_id`s.
2. It runs `.ghost_evo` and/or `.sim_organic` to evaluate alternative policies
   without biophysical actuation.
3. Proposed changes to `.evo`, `.evobudget`, `.circmap` are accepted only if
   they pass `.neuro_rights` constraints.
4. All updates are recorded as Organichain-visible events linked by `experiment_id`.

This closes the loop between online task execution and offline policy refinement,
grounded in documented closed-loop organic neuromorphic platforms, offline replay,
and low-energy event-based sensing.
