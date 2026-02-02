use serde::{Deserialize, Serialize};

/// On-wire representation for one dream-state snapshot.
/// CBOR-encoded, <=128 bytes per record.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuroAlnDreamShard {
    /// Monotonic timestamp (ns since boot).
    pub timestamp_ns: u64,
    /// Theta-gamma PAC coefficient, 0.0–1.0.
    pub tgcs: f32,
    /// REM density index, bursts per minute.
    pub rem_density: f32,
    /// NREM slow-wave / theta power ratios, precomputed.
    pub slow_wave_power: f32,
    pub theta_power: f32,
}

impl NeuroAlnDreamShard {
    /// Quick size sanity check before accept.
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.tgcs.is_finite()
            || !self.rem_density.is_finite()
            || !self.slow_wave_power.is_finite()
            || !self.theta_power.is_finite()
        {
            return Err("Non-finite dream-state value");
        }
        Ok(())
    }
}
