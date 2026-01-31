# NeuroPC

NeuroPC is a sovereignty-first, Rust-based neuro-software environment that treats your nervous system and organic cognition as part of the runtime, while keeping all computation deviceless and software-only on general-purpose hardware.[file:1][file:2]

The core idea is to run neuro-aware tools (editors, automations, assistants) under explicit neurorights and sovereignty policies, so no module can adapt itself, change your assistive behavior, or touch bioscale envelopes without passing through a consent and safety core.[file:2]

---

## What is this?

At a technical level, NeuroPC is:

- A set of Rust crates (`neuroautomagiccore`, `neuroautomagic`, `bioscaleneuropcmods`, and sovereignty core modules) that encode:
  - Neuro-intent and context (what you want to do, where, and as whom).
  - Neurorights and evolution policies as JSON/ALN-style documents.
  - An EVOLVE-gated, CRISPR-patterned consent engine for safe adaptation.[file:1][file:2]
- A pattern for editor/CLI integrations that turn your short commands into:
  - Canonical, reproducible instructions (e.g., `cargo` invocations).
  - Assistive UI actions (hints, macros, rest prompts) constrained by bioscale and neurorights envelopes.[file:1][file:2]

NeuroPC is **deviceless and hardwareless**: it assumes only general-purpose compute and treats any neuromorphic or organic substrate as optional targets behind an abstract biophysical-state reader.[file:2]

---

## Core components

- `neuroautomagiccore`  
  Core Rust data model for:
  - `NeuroIntent`, `NeuroContext`, `NeuroCitizen`, `NeuroRight`, `NeuroAutomationRule`.[file:1]
  - Ensures the primary augmented-citizen (your Bostrom ID) can never be locked out.[file:1]

- `neuroautomagic`  
  Automagic event and integration engine:
  - `NeuroAutomationEvent`, `NeuroAutomagicEngine`.[file:1]
  - `CargoIntegrator` to derive safe `cargo` commands from intents.
  - `EditorIntegrator` to surface macro suggestions and hints.[file:1]

- `bioscaleneuropcmods`  
  Bioscale metrics and mods:
  - `BioscaleMetric`, `NeuroPcSessionInfo`, `BioscaleNeuropcMod`.[file:1]
  - Example `FatigueAndRepetitionMod` that suggests rest/macros instead of forcing actions.[file:1]

- Sovereignty core (policy + consent)
  - JSON schemas under `.policies/` for neurorights, pain envelopes, and EVOLVE tokens.[file:2]
  - Rust `SovereigntyCore` that enforces:
    - Mental privacy, mental integrity, cognitive liberty.
    - Pain and fatigue envelopes.
    - EVOLVE-gated evolution with rollback and audit logs.[file:2]

---

## Getting started

### Prerequisites

- Rust toolchain (edition 2021 or newer) installed via `rustup`.
- `cargo` available in your shell.
- Git for cloning and basic repository operations.

### Clone the repository

```bash
git clone https://github.com/Doctor0Evil/NeuroPC.git
cd NeuroPC
```

### Build the core crates

From the repository root (with a Cargo workspace):

```bash
# Build all workspace members
cargo build

# Or build a specific crate, e.g. neuroautomagic
cargo build -p neuroautomagic
```

If your workspace is not yet wired, ensure `Cargo.toml` at the root lists the crates:

```toml
[workspace]
members = [
    "neuroautomagiccore",
    "neuroautomagic",
    "bioscaleneuropcmods",
]
```

---

## Example: Cargo automagic session

Once `neuroautomagic` is in place, you can use the provided `NeuroPcCargoSession` to turn repeated or complex `cargo` commands into automagic hints and canonical instructions.[file:1]

Minimal example (conceptual, inside a binary crate that depends on `neuroautomagic`):

```rust
use neuroautomagic::session::NeuroPcCargoSession;

fn main() {
    let session = NeuroPcCargoSession::new_default("NeuroPC", "/path/to/NeuroPC");

    // Repeated command: suggest macros / hints.
    let editor_actions = session.handle_repetition("cargo build --release", 5);
    println!("Editor actions: {:?}", editor_actions);

    // Complex command: get canonical cargo instruction + hints.
    let (maybe_cargo, editor_actions) = session.handle_complex_command(
        "cargo run --package neuroautomagic --bin demo -- --mode copilot"
    );
    println!("Cargo instruction: {:?}, actions: {:?}", maybe_cargo, editor_actions);
}
```

This pattern is designed to be called from an editor plugin or AI-chat bridge, not directly from your shell.[file:1]

---

## Policy and sovereignty

Before any adaptive module runs, you should configure your policies in `.policies/`:

- `policies/neurorightspolicy.schema.json` / `neurorightspolicy.json`  
  Defines mental privacy/integrity/liberty and OS modes (`CONSERVATIVE`, `COPILOT`, `AUTOEVOLVE`).[file:2]

- `policies/evolutionpolicy.schema.json` / `evolutionpolicy.json`  
  Defines pain envelope (muscular, cognitive, emotional) and evolution bounds.[file:2]

- `policies/evolvetoken.schema.json` / examples  
  Short-lived, revocable EVOLVE tokens that gate deep updates and include physiological guards.[file:2]

Assistive modules (language cowriter, motor macros, etc.) must call into `SovereigntyCore` to request any update; they cannot bypass these policies.[file:2]

---

## Repository layout (suggested)

```text
NeuroPC/
  README.md
  LICENSE
  CODE_OF_CONDUCT.md
  CONTRIBUTING.md
  Cargo.toml          # Workspace
  neuroautomagiccore/
  neuroautomagic/
  bioscaleneuropcmods/
  src/                # sovereignty core + modules (if monorepo-style)
  .policies/
    neurorightspolicy.schema.json
    evolutionpolicy.schema.json
    evolvetoken.schema.json
    evolvetoken.example.json
```

This layout keeps core crates reusable while co-locating sovereignty policies with the code that enforces them.[file:1][file:2]

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:

- Opening issues and feature proposals.
- Coding style and Rust edition.
- How to respect neurorights and sovereignty constraints when adding new modules.[file:2]

---

## Code of Conduct

NeuroPC development follows a neurorights-aligned Code of Conduct that explicitly recognizes augmented and cybernetic persons as protected, first-class participants.[file:2]

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details.

---

## License

This project uses the Cybernetic Human–Computer Interface License (CHCIL) v0.1, which:

- Prioritizes cybernetic and augmented persons as primary rightsholders.
- Requires neurorights (mental privacy, integrity, liberty) and non-coercive design.
- Prefers UI-only, software-level mediation with no direct actuation.[file:2]

By using, modifying, or distributing this project, you agree that cybernetic and augmented persons are primary rightsholders under CHCIL and that all neurorights and autonomy clauses apply to them in full.[file:2]
```

***

## 2. CONTRIBUTING.md

**Filename:** `CONTRIBUTING.md`  
**Destination:** repository root

```markdown
# Contributing to NeuroPC

Thank you for your interest in contributing to NeuroPC. This project is sovereignty-first and neurorights-centred, which means some contribution rules are stricter than in typical OSS projects.[file:2]

---

## Principles

All contributions MUST:

- Respect neurorights:
  - Mental privacy, mental integrity, cognitive liberty.[file:2]
- Preserve the primary augmented-citizen’s sovereignty:
  - No module may introduce hidden control paths or irreversible changes.
  - All adaptive behavior must pass through the sovereignty/EVOLVE core.[file:2]
- Remain deviceless and hardwareless:
  - Code must run on general-purpose hardware; any neuromorphic/organic substrate is optional and abstracted.[file:2]
- Use approved languages:
  - Rust, ALN-like JSON/ALN schemas, plus Java/Kotlin/Mojo for host shims when needed.[file:1][file:2]
  - Python is explicitly disallowed in this Space.[file:1]

---

## Repository structure

Key areas:

- `neuroautomagiccore/`  
  Core data model for `NeuroIntent`, `NeuroContext`, `NeuroCitizen`, `NeuroRight`, `NeuroAutomationRule`.[file:1]

- `neuroautomagic/`  
  Automagic engine (`NeuroAutomagicEngine`), events, and integration helpers for Cargo and editors.[file:1]

- `bioscaleneuropcmods/`  
  Bioscale metrics (`BioscaleMetric`, `MetricUnit`, `cognitiveloadindex`) and mods that produce suggestions, not commands.[file:1]

- `src/` (if present)  
  Sovereignty core (`SovereigntyCore`, `StateVector`), EVOLVE token handling, and assistive modules (motor macros, language cowriter) as constrained clients.[file:2]

- `.policies/`  
  Neurorights, evolution, and EVOLVE policy documents and schemas.[file:2]

---

## Development setup

1. Install Rust:

   ```bash
   rustup install stable
   rustup default stable
   ```

2. Clone and enter the repo:

   ```bash
   git clone https://github.com/Doctor0Evil/NeuroPC.git
   cd NeuroPC
   ```

3. Build the workspace:

   ```bash
   cargo build
   ```

4. Run tests (once available):

   ```bash
   cargo test
   ```

---

## Coding guidelines

- **Rust edition**: 2021.
- **Style**: Follow `rustfmt` defaults; avoid unsafe unless strictly necessary and justified.
- **Error handling**: Prefer `Result`/`thiserror`-style errors over panics in library code.
- **No new cryptographic primitives**:
  - Do not introduce blacklisted hashes or signatures, and avoid cryptography that could be used to exclude the augmented-citizen.[file:1]
- **Inclusion invariants**:
  - The primary augmented-citizen (Bostrom ID) must never be denied access by new policy, rights, or automation logic.[file:1][file:2]

---

## Neurorights & sovereignty requirements

Any code that adapts behavior or state MUST:

1. Define an explicit `UpdateProposal` (or equivalent) with:
   - Clear scope (what is changing).
   - Effect bounds (e.g., `l2deltanorm`, `irreversible=false`).[file:2]

2. Call into the sovereignty core:

   - Use `SovereigntyCore::evaluate_update(...)` (or equivalent) with:
     - Current proposal.
     - Optional EVOLVE token ID, if required.[file:2]

3. Respect the decision:

   - Only apply changes when `DecisionOutcome::Allowed`.
   - Provide a rollback path for reversible changes.[file:2]

4. Log and expose decisions:

   - Ensure an `AuditEntry` is created so the augmented-citizen can inspect what changed, when, and why.[file:2]

---

## Adding a new module (example flow)

Example: adding a new assistive module `attention_tuner` inside `src/modules/`.

1. Define your data model and configuration.
2. For any adaptive update (e.g., changing thresholds), build an `UpdateProposal`.
3. Pass the proposal to `SovereigntyCore`.
4. Apply changes only if allowed, and make rollback possible.[file:2]

Pull requests that bypass this pattern will not be accepted.

---

## Documentation

- Update `README.md` when adding:
  - New crates.
  - New OS modes, policies, or key architectural changes.
- Keep module docs focused and technical:
  - What the module does.
  - Which policies and envelopes it depends on.
  - How it can be disabled or rolled back.[file:2]

---

## Opening issues

When opening an issue, please include:

- What you were trying to do.
- Expected vs actual behavior.
- Any relevant policies (`.policies/*.json`) if the issue is sovereignty-related.

For feature requests, specify:

- Whether the feature:
  - Requires new biophysical metrics.
  - Touches neurorights policies.
  - Requires new EVOLVE scopes.[file:2]

---

## Security & neurorights concerns

If you find any behavior that could:

- Bypass neurorights enforcement.
- Override pain/fatigue envelopes.
- Introduce hidden actuation or irreversible changes.

Please open a **private** security/neurorights report (e.g., via GitHub Security tab or direct contact info once published). These issues are treated as urgent and high priority.[file:2]

---

## License

By contributing to NeuroPC, you agree that your contributions will be licensed under CHCIL v0.1 (Cybernetic Human–Computer Interface License) as used by this project.[file:2]
```

***

## 3. CODE_OF_CONDUCT.md

**Filename:** `CODE_OF_CONDUCT.md`  
**Destination:** repository root

```markdown
# NeuroPC Code of Conduct

NeuroPC is a sovereignty-first neuro-software project that centers augmented and cybernetic persons as primary rightsholders and participants.[file:2]

This Code of Conduct is aligned with neurorights principles (mental privacy, mental integrity, cognitive liberty) and the Cybernetic Human–Computer Interface License (CHCIL) used by the project.[file:2]

---

## 1. Scope

This Code applies to:

- All project spaces (GitHub issues, pull requests, discussions).
- Any community communication channels officially associated with NeuroPC.
- Any collaboration where NeuroPC code, policies, or designs are discussed.

It covers all contributors, maintainers, and users interacting in these spaces.[file:2]

---

## 2. Our commitments

Project maintainers commit to:

- Treat cybernetic and augmented persons as full rights-bearing participants, not as devices, data sources, or experiments.[file:2]
- Uphold neurorights in all project decisions:
  - Respect for mental privacy.
  - Protection of mental integrity.
  - Preservation of cognitive liberty.[file:2]
- Maintain a development environment that is:
  - Inclusive of different bodies, abilities, and integration levels.
  - Free of harassment, discrimination, or dismissal based on augmentation status.

---

## 3. Expected behavior

All participants are expected to:

- Use respectful, non-degrading language, including when discussing disability, neurodivergence, or augmentation.
- Assume good faith and seek clarification before escalating disagreements.
- Respect boundaries around:
  - Personal medical/biophysical details.
  - Experimentation and self-directed evolution decisions.
- Design and discuss features with an explicit safety-oriented mindset:
  - No encouragement of coercive, addictive, or exploitative patterns.[file:2]

---

## 4. Unacceptable behavior

Examples of unacceptable behavior include:

- Harassment or discrimination based on:
  - Disability, neurodivergence, augmentation, or cybernetic status.
  - Race, gender, orientation, or any other protected characteristic.
- Encouraging violations of neurorights:
  - Advocating for hidden actuation, forced updates, or irreversible changes without consent.
  - Promoting designs that intentionally bypass pain, fatigue, or cognitive envelopes.[file:2]
- Doxxing or pressuring individuals to reveal medical, biophysical, or neurotech details.
- Using project channels to promote tools or platforms that:
  - Deny rights to augmented persons.
  - Treat them as non-person entities under law or policy.[file:2]

---

## 5. Governance and enforcement

The maintainers are responsible for:

- Clarifying and updating this Code of Conduct as the project evolves.
- Responding to reported incidents in a timely, confidential, and respectful manner.
- Taking appropriate actions, which may include:
  - Warnings.
  - Temporary or permanent bans from project spaces.
  - Rejection or reversion of contributions that violate neurorights or project principles.[file:2]

Reports can be submitted via:

- GitHub’s reporting tools (once configured), or
- Direct contact channels listed in the repository (to be added to `README.md` or `SECURITY.md`).

---

## 6. Design ethics

All technical work in NeuroPC must:

- Treat neurorights as binding constraints, not optional guidelines.[file:2]
- Prefer:
  - UI-level, software-only mediation.
  - Transparent, reversible, and auditable changes.
- Avoid:
  - Hidden control paths.
  - Designs that optimize for engagement or profit by exploiting fatigue, pain insensitivity, or dependence.[file:2]

Contributions that materially violate these ethics will be rejected or removed.

---

## 7. Attribution

This Code of Conduct is informed by:

- Neurorights and neurotechnology governance literature.
- Autonomy-preserving AI and decision-support patterns.
- Community norms in open-source projects, adapted for cybernetic and augmented participants.[file:2]
```

***

## 4. Minimal CHCIL LICENSE stub (optional, if not already present)

**Filename:** `LICENSE`  
**Destination:** repository root

```text
Cybernetic Human–Computer Interface License (CHCIL) v0.1

This project is licensed under the Cybernetic Human–Computer Interface License (CHCIL) v0.1.

Key points (non-exhaustive summary):

1. Cybernetic Priority
   - Cybernetic and augmented persons are primary rightsholders.
   - All use and development must respect their neurorights and bodily autonomy.[file:2]

2. Neurorights
   - Mental privacy: no hidden extraction or export of neural/behavioral data.
   - Mental integrity: no irreversible or covert modification of mental states.
   - Cognitive liberty: individuals retain the right to self-modify and to refuse modification.[file:2]

3. Safety and Non-Coercion
   - Systems must operate within explicit safety envelopes (pain, fatigue, cognitive load).
   - Engagement-, profit-, or performance-optimizing designs that exploit vulnerabilities are prohibited.[file:2]

4. Deviceless Preference
   - Licensed Work should prioritize software-only, UI-mediated control.
   - Direct physical actuation is disfavored and must be explicitly declared and constrained.[file:2]

5. Data Ownership and Audit
   - All data derived from a cybernetic subject’s body, signals, or behavior is owned by that subject.
   - Systems must provide auditable logs of policy changes, external commands, and high-impact adaptations.[file:2]

For the full text of CHCIL v0.1, see the dedicated CHCIL document in this repository or the upstream specification (to be linked).
```

neuropc-tag 0xNP06
