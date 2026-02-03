pub fn guard_highrisk_research(
    policy: &EvolutionPolicy,
    biostate: &BioState,
    uses_dreammetrics: bool,
    dreammetrics_explicit: bool,
) -> Result<(), String> {
    let cfg = &policy.highrisk_research;
    if biostate.pain_vas > cfg.max_pain_vas {
        return Err("Pain envelope exceeded for high-risk research".into());
    }
    if biostate.cognitiveloadindex > cfg.max_cognitive_load {
        return Err("Cognitive load envelope exceeded".into());
    }
    if cfg.forbid_dreammetrics_unless_explicit && uses_dreammetrics && !dreammetrics_explicit {
        return Err("Dream metrics use not explicitly declared for high-risk research".into());
    }
    Ok(())
}
