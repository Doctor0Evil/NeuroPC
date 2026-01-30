use brainprint::{BrainPrint, brainPrint};
use brainprint::unleash::{BrainPrintResearchView, BrainPrintUnleash};

fn roh_index(bp: &BrainPrint) -> f32 {
    // Example: simple normalized RoH from WAVE/BRAIN and eco_band.
    let load = if bp.biophysics.brain > 0.0 {
        (bp.biophysics.wave / bp.biophysics.brain).min(2.0)
    } else {
        0.0
    };
    (0.1 + 0.4 * load as f32 + 0.1 * bp.lifeforce.eco_band as f32)
        .min(1.0)
}

fn eco_energy_nj(bp: &BrainPrint) -> f64 {
    // Map NANO + WAVE to notional nanojoule-equivalent; host-tunable.
    (bp.biophysics.nano * 1e6) + (bp.biophysics.wave * 1e3)
}

fn risk_of_harm(bp: &BrainPrint) -> f32 {
    // Higher when BLOOD/OXYGEN are near floors or WAVE is high.
    let blood_risk = if bp.biophysics.blood < 0.4 { 0.5 } else { 0.1 };
    let oxy_risk = if bp.biophysics.oxygen < 0.95 { 0.3 } else { 0.05 };
    let wave_ratio = if bp.biophysics.brain > 0.0 {
        (bp.biophysics.wave / bp.biophysics.brain).min(2.0)
    } else {
        0.0
    };
    (blood_risk + oxy_risk + 0.2 * wave_ratio as f32).min(1.0)
}

fn knowledge_factor(_bp: &BrainPrint) -> f32 {
    // Host can inject current ALN/DID knowledgefactor here if desired.
    0.93
}

fn profile_hex(_bp: &BrainPrint) -> String {
    "0xBCI-FULL-P4TH-27JAN26".to_string()
}

// Declare the unleash profile for this host-node:
brainPrint! {
    unleash: research,
    plane: Biophysics,
    roh_index: roh_index,
    eco_energy_nj: eco_energy_nj,
    risk_of_harm: risk_of_harm,
    knowledge_factor: knowledge_factor,
    profile_hex: profile_hex
}

// Export helper to JSON for researchers / dashboards.
pub fn export_brainprint_research_json(bp: &BrainPrint) -> String {
    let view: BrainPrintResearchView = BrainPrint::to_research_view(bp);
    serde_json::to_string(&view).unwrap_or_else(|_| "{}".to_string())
}
