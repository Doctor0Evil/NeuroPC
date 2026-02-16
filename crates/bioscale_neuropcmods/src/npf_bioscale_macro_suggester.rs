use crate::session::NeuroPcSessionInfo;
use crate::metrics::{BioscaleMetric, MetricUnit, normalize_minmax};
use crate::mods::{BioscaleSuggestion, BioscaleModResult, BioscaleNeuropcMod};

/// Biocompatibility rating in [0,1].
pub const NPF_BIOSCALE_MACRO_SUGGESTER_BCR: f32 = 0.27;

/// Lightweight summary of a repeated command pattern.
#[derive(Clone, Debug)]
pub struct CommandPatternSummary {
    pub pattern: String,
    pub recent_count: u32,
}

/// Neuroprint-function:
/// - If a command pattern repeats above a threshold,
///   suggest defining a macro / script instead of re-typing.
/// - Only returns suggestions + metrics; does not execute anything.
pub struct NpfBioscaleMacroSuggester {
    pub threshold: u32,
}

impl NpfBioscaleMacroSuggester {
    pub fn new(threshold: u32) -> Self {
        Self { threshold }
    }

    pub fn run(
        &self,
        session: &NeuroPcSessionInfo,
        pattern: &CommandPatternSummary,
    ) -> BioscaleModResult {
        let mut suggestions = Vec::new();
        let mut metrics = Vec::new();

        // Normalize repetition into [0,1] as a bioscale metric.
        let repetition_metric = BioscaleMetric {
            name: "CommandRepetitionIndex".to_string(),
            unit: MetricUnit::CommandsPerMinute,
            value_raw: pattern.recent_count as f64,
            value_norm: normalize_minmax(pattern.recent_count as f64, 0.0, 40.0),
        };
        metrics.push(repetition_metric.clone());

        if pattern.recent_count >= self.threshold {
            suggestions.push(BioscaleSuggestion::SuggestCommandMacro {
                pattern: pattern.pattern.clone(),
            });
        }

        // Optionally, also encourage eco / fatigue-aware pacing.
        let eco_metric = BioscaleMetric {
            name: "AvgDailyDeviceHours".to_string(),
            unit: MetricUnit::HoursPerUserDay,
            value_raw: session.device_hours_today,
            value_norm: normalize_minmax(session.device_hours_today, 0.0, 10.0),
        };
        metrics.push(eco_metric.clone());

        BioscaleModResult {
            suggestions,
            emitted_metrics: metrics,
        }
    }
}

/// Bridge into the existing trait, so this npf can plug into the mods registry.
pub struct FatigueMacroWrapper {
    inner: NpfBioscaleMacroSuggester,
}

impl FatigueMacroWrapper {
    pub fn new(threshold: u32) -> Self {
        Self {
            inner: NpfBioscaleMacroSuggester::new(threshold),
        }
    }
}

impl BioscaleNeuropcMod for FatigueMacroWrapper {
    fn name(&self) -> &str {
        "npf_bioscale_macro_suggester"
    }

    fn apply(&self, snapshot: crate::session::BioscaleSnapshot) -> BioscaleModResult {
        let pattern = CommandPatternSummary {
            pattern: "cargo neuro build".to_string(), // to be filled by caller in real use
            recent_count: (snapshot.session.recent_command_repetition * 100.0) as u32,
        };
        self.inner.run(&snapshot.session, &pattern)
    }
}
