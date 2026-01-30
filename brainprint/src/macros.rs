#[macro_export]
macro_rules! brainPrint {
    // Minimal telemetry, plane tagging only.
    (
        unleash: minimal,
        plane: $plane:ident
    ) => {
        impl $crate::unleash::BrainPrintUnleash for $crate::BrainPrint {
            fn unleash_level() -> $crate::unleash::BrainPrintUnleashLevel {
                $crate::unleash::BrainPrintUnleashLevel::TelemetryMinimal
            }

            fn to_research_view(bp: &$crate::BrainPrint) -> $crate::unleash::BrainPrintResearchView {
                use $crate::unleash::{BrainPrintPlane, BrainPrintResearchView, BrainPrintUnleashLevel};

                let state_hash_hex = hex::encode(bp.state_hash);

                BrainPrintResearchView {
                    schema_version: bp.header.schema_version,
                    plane: BrainPrintPlane::$plane,
                    unleash_level: BrainPrintUnleashLevel::TelemetryMinimal,
                    timestamp_ms: bp.header.timestamp_ms,
                    brain: bp.biophysics.brain,
                    wave: bp.biophysics.wave,
                    nano: bp.biophysics.nano,
                    smart: bp.biophysics.smart,
                    lifeforce_index: bp.lifeforce.lifeforce_index,
                    eco_band: bp.lifeforce.eco_band,
                    roh_index: None,
                    eco_energy_nj: None,
                    risk_of_harm: None,
                    knowledge_factor: None,
                    state_hash_hex,
                    profile_hex: None,
                }
            }
        }
    };

    // Research telemetry: opt-in to RoH, eco, knowledge-factor, etc.
    (
        unleash: research,
        plane: $plane:ident,
        roh_index: $roh_expr:expr,
        eco_energy_nj: $eco_expr:expr,
        risk_of_harm: $roharm_expr:expr,
        knowledge_factor: $kf_expr:expr,
        profile_hex: $profile_expr:expr
    ) => {
        impl $crate::unleash::BrainPrintUnleash for $crate::BrainPrint {
            fn unleash_level() -> $crate::unleash::BrainPrintUnleashLevel {
                $crate::unleash::BrainPrintUnleashLevel::TelemetryResearch
            }

            fn to_research_view(bp: &$crate::BrainPrint) -> $crate::unleash::BrainPrintResearchView {
                use $crate::unleash::{BrainPrintPlane, BrainPrintResearchView, BrainPrintUnleashLevel};

                let state_hash_hex = hex::encode(bp.state_hash);

                BrainPrintResearchView {
                    schema_version: bp.header.schema_version,
                    plane: BrainPrintPlane::$plane,
                    unleash_level: BrainPrintUnleashLevel::TelemetryResearch,
                    timestamp_ms: bp.header.timestamp_ms,
                    brain: bp.biophysics.brain,
                    wave: bp.biophysics.wave,
                    nano: bp.biophysics.nano,
                    smart: bp.biophysics.smart,
                    lifeforce_index: bp.lifeforce.lifeforce_index,
                    eco_band: bp.lifeforce.eco_band,
                    roh_index: Some(($roh_expr)(bp)),
                    eco_energy_nj: Some(($eco_expr)(bp)),
                    risk_of_harm: Some(($roharm_expr)(bp)),
                    knowledge_factor: Some(($kf_expr)(bp)),
                    state_hash_hex,
                    profile_hex: Some(($profile_expr)(bp)),
                }
            }
        }
    };
}
