use serde::{Serialize, Deserialize};

/// Units for bioscale metrics.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MetricUnit {
    Dimensionless,
    Percent,
    HoursPerUserDay,
    KWhPerUserYear,
    CommandsPerMinute,
}

/// Core bioscale metric type, normalized to [0,1] for audits.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BioscaleMetric {
    pub name: String,
    pub unit: MetricUnit,
    /// Raw value in natural units.
    pub value_raw: f64,
    /// Normalized value in [0,1].
    pub value_norm: f64,
}

/// Simple min–max normalization into [0,1].
pub fn normalize_minmax(value: f64, min: f64, max: f64) -> f64 {
    if max <= min {
        return 0.0;
    }
    let clamped = value.max(min).min(max);
    (clamped - min) / (max - min)
}

/// Example: cognitive load estimated from error rate and command repetition.
/// In practice you would plug in real, validated formulas from your metrics pipeline.
pub fn cognitive_load_index(error_rate: f64, repetition_rate: f64) -> BioscaleMetric {
    // Both inputs expected in [0,1].
    let raw = 0.5 * error_rate + 0.5 * repetition_rate;
    BioscaleMetric {
        name: "CognitiveLoadIndex".to_string(),
        unit: MetricUnit::Dimensionless,
        value_raw: raw,
        value_norm: raw.max(0.0).min(1.0),
    }
}
