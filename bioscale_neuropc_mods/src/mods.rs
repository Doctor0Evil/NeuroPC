use crate::metrics::{BioscaleMetric, MetricUnit, normalize_minmax, cognitive_load_index};
use crate::session::{NeuroPcSessionInfo, BioscaleSnapshot};
use serde::{Serialize, Deserialize};

/// What a bioscale_neuropc_mod is allowed to do: only suggest.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BioscaleSuggestion {
    ShowRestPrompt { reason: String },
    SuggestCommandMacro { pattern: String },
    SuggestSimplerTooling { reason: String },
    SuggestEcoReduction { reason: String },
}

/// Result of running a bioscale mod.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BioscaleModResult {
    pub suggestions: Vec<BioscaleSuggestion>,
    pub emitted_metrics: Vec<BioscaleMetric>,
}

/// Core trait for a biocompatible NeuroPC mod.
pub trait BioscaleNeuropcMod {
    fn name(&self) -> &str;
    fn apply(&self, snapshot: &BioscaleSnapshot) -> BioscaleModResult;
}

/// Example mod: watch fatigue and repetition, suggest macros and breaks.
pub struct FatigueAndRepetitionMod;

impl BioscaleNeuropcMod for FatigueAndRepetitionMod {
    fn name(&self) -> &str {
        "FatigueAndRepetitionMod"
    }

    fn apply(&self, snapshot: &BioscaleSnapshot) -> BioscaleModResult {
        let s = &snapshot.session;

        let load = cognitive_load_index(
            snapshot.session.recent_compile_fail_rate,      // proxy for error
            snapshot.session.recent_command_repetition,     // repetition
        );

        let device_hours = BioscaleMetric {
            name: "AvgDailyDeviceHours".to_string(),
            unit: MetricUnit::HoursPerUserDay,
            value_raw: s.device_hours_today,
            value_norm: normalize_minmax(s.device_hours_today, 0.0, 10.0),
        };

        let mut suggestions = Vec::new();

        if load.value_norm > 0.7 {
            suggestions.push(BioscaleSuggestion::ShowRestPrompt {
                reason: "High cognitive load detected from errors and repetitions.".to_string(),
            });
        }

        if s.recent_command_repetition > 0.6 {
            suggestions.push(BioscaleSuggestion::SuggestCommandMacro {
                pattern: "Repeated cargo/neuro commands in last N minutes".to_string(),
            });
        }

        if device_hours.value_norm > 0.7 {
            suggestions.push(BioscaleSuggestion::SuggestEcoReduction {
                reason: "High device-hours today; consider a low-strain mode or offline break."
                    .to_string(),
            });
        }

        BioscaleModResult {
            suggestions,
            emitted_metrics: vec![load, device_hours],
        }
    }
}
