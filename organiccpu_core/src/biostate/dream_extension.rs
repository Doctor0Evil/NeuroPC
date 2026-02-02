use core::fmt::{Debug, Formatter};

/// Dream-state metrics as fixed-point 16-bit scalars for BioState ingestion.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct DreamStateScalars {
    /// Theta-gamma PAC coefficient, scaled 0..10000 ≡ 0.0000–1.0000.
    pub tgcs: u16,
    /// REM density index, bursts per minute × 100.
    pub rdi: u16,
    /// NREM latency variance in seconds × 100.
    pub nlv: u16,
    /// Monotonic timestamp since boot (nanoseconds).
    pub timestamp_ns: u64,
}

impl DreamStateScalars {
    /// Max allowed staleness for dream-state scalars (nanoseconds).
    pub const MAX_AGE_NS: u64 = 200_000_000; // 200 ms

    /// Basic well-formedness: ranges and freshness only.
    #[inline(always)]
    pub fn is_fresh_and_bounded(
        &self,
        now_ns: u64,
    ) -> Result<(), &'static str> {
        // All three main scalars are bounded by construction (u16).
        // We only enforce staleness here.
        if now_ns.saturating_sub(self.timestamp_ns) > Self::MAX_AGE_NS {
            return Err("Stale dream-state data (>200ms) rejected");
        }
        Ok(())
    }

    /// Map into normalized 0.0–1.0 floats for higher layers.
    #[inline(always)]
    pub fn to_normalized(&self) -> DreamStateNormalized {
        DreamStateNormalized {
            tgcs: self.tgcs as f32 / 10_000.0,
            rdi: self.rdi as f32 / 100.0,    // bursts per minute
            nlv: self.nlv as f32 / 100.0,    // seconds
        }
    }
}

impl Debug for DreamStateScalars {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "DreamStateScalars {{ tgcs: {}.{:04}, rdi: {}.{:02}, nlv: {}.{:02}, ts_ns: {} }}",
            self.tgcs / 10_000,
            self.tgcs % 10_000,
            self.rdi / 100,
            self.rdi % 100,
            self.nlv / 100,
            self.nlv % 100,
            self.timestamp_ns,
        )
    }
}

/// Normalized dream metrics; these can be mixed into BioState-derived indices.
#[derive(Copy, Clone, Debug)]
pub struct DreamStateNormalized {
    pub tgcs: f32,
    pub rdi: f32,
    pub nlv: f32,
}
