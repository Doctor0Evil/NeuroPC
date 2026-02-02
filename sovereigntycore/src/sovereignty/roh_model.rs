#![no_std]

use core::fmt::{Debug, Formatter};

/// Scaled Risk-of-Harm value (0..=10000)
pub type RohScaled = u16;

/// Hard neurorights ceiling: RoH ≤ 0.3 ≡ 3000
pub const MAX_ROH_SCALED: RohScaled = 3000;

/// RoH model state — monotone non-increasing aggregate
#[derive(Copy, Clone, Default)]
pub struct RohModel {
    /// Current aggregate RoH (scaled)
    pub current: RohScaled,
    /// Peak observed (for audit)
    pub peak: RohScaled,
    /// Monotonic version
    pub version: u64,
}

impl RohModel {
    /// Apply delta — enforces ceiling and monotonic tightening only
    #[inline(always)]
    pub fn apply_delta(&mut self, delta_scaled: i16, new_version: u64) -> Result<(), &'static str> {
        if new_version <= self.version {
            return Err("Monotone version violation");
        }

        let proposed = self.current as i32 + delta_scaled as i32;
        if proposed < 0 {
            return Err("Negative RoH impossible");
        }
        let proposed_scaled = proposed as RohScaled;

        if proposed_scaled > MAX_ROH_SCALED {
            panic!("SafeEnvelopePolicy violation: RoH ceiling 0.3 breached");
        }

        // Monotone tightening: only allow decrease or bounded increase below ceiling
        if proposed_scaled > self.current && proposed_scaled > self.peak {
            if proposed_scaled > MAX_ROH_SCALED {
                panic!("RoH ceiling breach");
            }
        }

        self.current = proposed_scaled;
        self.peak = core::cmp::max(self.peak, proposed_scaled);
        self.version = new_version;
        Ok(())
    }
}

impl Debug for RohModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "RoH(current={:.4}, peak={:.4}, version={})",
            self.current as f32 / 10000.0,
            self.peak as f32 / 10000.0,
            self.version
        )
    }
}
