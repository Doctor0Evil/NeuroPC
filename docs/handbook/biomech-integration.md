# Biomech integration

This section defines how biomechanical assist modules integrate with NeuroPC’s neurorights, SovereigntyCore, and EVOLVE policies.

## Roles

- **observeronly**: Module may read signals and write analytics; it may not change mappings or actuation.
- **advisor**: Module may propose UpdateProposals for new macros or parameter changes, but decisions remain human- or SovereigntyCore-gated.
- **boundedauto**: Module may propose bounded automatic adjustments within maxeffectsize, maxupdatesperday, and EVOLVE token constraints.
- **forbidden**: Module is not permitted to operate for the current subjectid and context.

## Risk classes

- **R0_observer**: Purely observational or analytic, no direct actuation.
- **R1_advisor**: Provides suggestions or candidate macros, always gated by SovereigntyCore and human intent.
- **R2_bounded_auto**: Enables tightly bounded automatic adjustments that remain inside the user’s painenvelope and neurorights constraints.

All biomech-scoped UpdateProposals must be evaluated by SovereigntyCore, logged to .evolve.jsonl, and reflected into .donutloop.aln for later audit.
