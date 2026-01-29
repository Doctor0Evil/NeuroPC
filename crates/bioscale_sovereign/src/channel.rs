use std::time::Duration;

/// DID / Bostrom-style identity for the sovereign owner.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SovereignId {
    // e.g., "bostrom18sd2u..." or DID URI; ALN shard holds canonical mapping.
    pub id: String,
}

/// Normalized eco-help vector, all values in [0.0, 1.0].
#[derive(Clone, Debug)]
pub struct EcoHelpVector {
    pub avg_daily_device_hours_reduced: f32,   // 0 = no change, 1 = large reduction
    pub annual_energy_saved_per_user: f32,     // normalized kWh savings
    pub embodied_devices_averted: f32,         // normalized device-count avoidance
}

/// Normalized safety envelope, software-only, monotone under OTAs.
#[derive(Clone, Debug)]
pub struct MuscleSafetyEnvelope {
    pub g_max: f32,       // max software control gain (0..1)
    pub d_max: f32,       // max duty cycle over window (0..1)
    pub f_warn: f32,      // fatigue index for warnings (0..1)
    pub f_stop: f32,      // fatigue index for mandatory stop (0..1)
    pub r_max: f32,       // modeled overuse risk (0..1)
}

/// Live host budget and corridor scores for routing decisions.
#[derive(Clone, Debug)]
pub struct HostBudget {
    pub duty_cycle: f32,         // current duty cycle (rolling window)
    pub fatigue_index: f32,      // current fatigue (0..1)
    pub eco_impact_score: f32,   // current eco impact (0..1, higher is better)
    pub window: Duration,        // aggregation window for duty/fatigue
}

/// Sovereign, neurorights-safe channel descriptor used by routers.
#[derive(Clone, Debug)]
pub struct SovereignChannel {
    pub owner: SovereignId,
    pub channel_id: String,
    pub safety_envelope: MuscleSafetyEnvelope,
    pub eco_vector: EcoHelpVector,
    pub min_decoder_accuracy: f32,    // e.g., >= 0.90 for sEMG intent
    pub max_avg_latency_ms: u32,      // upper bound on end-to-end latency
}

impl SovereignChannel {
    /// Check whether a proposed OTA (new envelope/eco vector) is allowed
    /// under monotone safety and eco constraints.
    pub fn ota_monotone_ok(&self, new_env: &MuscleSafetyEnvelope, new_eco: &EcoHelpVector) -> bool {
        // Safety: envelopes can only tighten or stay equal.
        let safety_ok =
            new_env.g_max <= self.safety_envelope.g_max &&
            new_env.d_max <= self.safety_envelope.d_max &&
            new_env.f_warn <= self.safety_envelope.f_warn &&
            new_env.f_stop <= self.safety_envelope.f_stop &&
            new_env.r_max <= self.safety_envelope.r_max;

        // Eco: must not regress along any eco-help dimension.
        let eco_ok =
            new_eco.avg_daily_device_hours_reduced >= self.eco_vector.avg_daily_device_hours_reduced &&
            new_eco.annual_energy_saved_per_user >= self.eco_vector.annual_energy_saved_per_user &&
            new_eco.embodied_devices_averted >= self.eco_vector.embodied_devices_averted;

        safety_ok && eco_ok
    }

    /// Runtime admission check for routing given current host budget and decoder metrics.
    pub fn runtime_admit(
        &self,
        host: &HostBudget,
        decoder_accuracy: f32,
        avg_latency_ms: u32,
    ) -> bool {
        // Decoder must meet or exceed required accuracy and latency bounds.
        if decoder_accuracy < self.min_decoder_accuracy {
            return false;
        }
        if avg_latency_ms > self.max_avg_latency_ms {
            return false;
        }

        // Host duty-cycle and fatigue must be within envelope.
        if host.duty_cycle > self.safety_envelope.d_max {
            return false;
        }
        if host.fatigue_index > self.safety_envelope.f_stop {
            return false;
        }

        // Eco corridor must not be violated (no eco backsliding).
        if host.eco_impact_score < 0.0 || host.eco_impact_score > 1.0 {
            return false;
        }

        true
    }
}
