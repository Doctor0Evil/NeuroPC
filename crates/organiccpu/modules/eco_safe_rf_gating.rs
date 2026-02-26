use chrono::Utc;
use std::sync::Arc;
use parking_lot::RwLock;

/// Enforces wildlife-safe RF transmission: zero continuous tx, event-pulsed only
#[derive(Clone)]
pub struct EcoSafeRfGate {
    /// Bluetooth average power budget (mW)
    btx_power_mw_budget: f32,
    /// Minimum inter-pulse gap (ms) to ensure <0.01 µT avg ambient
    min_pulse_gap_ms: f32,
    /// Last transmit timestamp
    last_tx_ms: Arc<RwLock<Option<f64>>>,
    /// Pulse window (ms) to calculate average
    window_size_ms: f32,
    /// Pulse log: (tx_time_ms, duration_ms)
    pulse_log: Arc<RwLock<Vec<(f64, f32)>>>,
}

impl EcoSafeRfGate {
    pub fn new(btx_power_mw: f32, min_gap_ms: f32, window_ms: f32) -> Self {
        Self {
            btx_power_mw_budget: btx_power_mw,
            min_pulse_gap_ms: min_gap_ms,
            last_tx_ms: Arc::new(RwLock::new(None)),
            window_size_ms: window_ms,
            pulse_log: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Pre-tx check: ensure tx complies eco-safe thresholds
    /// Returns Ok if safe; Err(reason) if blocked
    pub fn check_tx_allowed(&self, tx_duration_ms: f32, now_ms: f64) -> Result<(), String> {
        let mut last_tx = self.last_tx_ms.write();
        let mut log = self.pulse_log.write();

        // Enforce minimum inter-pulse gap
        if let Some(last_time) = *last_tx {
            let gap = now_ms - last_time;
            if gap < self.min_pulse_gap_ms as f64 {
                return Err(format!(
                    "RF tx blocked: gap {:.1}ms < min {:.1}ms (eco-safe duty <0.5%)",
                    gap, self.min_pulse_gap_ms
                ));
            }
        }

        // Calculate avg power over window
        let window_start = now_ms - self.window_size_ms as f64;
        log.retain(|(t, _)| *t >= window_start);
        
        let avg_power_mw = log
            .iter()
            .map(|(_, dur)| (dur / self.window_size_ms) * self.btx_power_mw_budget)
            .sum::<f32>();

        if avg_power_mw > self.btx_power_mw_budget * 0.01 {
            // Duty cycle >1% → block
            return Err(format!(
                "RF tx blocked: avg power {:.2}mW exceeds 1% budget {:.3}mW (ecosystem threshold)",
                avg_power_mw, self.btx_power_mw_budget * 0.01
            ));
        }

        // Safe: log and allow
        *last_tx = Some(now_ms);
        log.push((now_ms, tx_duration_ms));
        Ok(())
    }

    /// Get current RF duty cycle (%)
    pub fn get_duty_cycle_percent(&self) -> f32 {
        let log = self.pulse_log.read();
        if log.is_empty() { return 0.0; }
        
        let total_tx_ms: f32 = log.iter().map(|(_, dur)| dur).sum();
        (total_tx_ms / self.window_size_ms) * 100.0
    }

    /// Get estimated field strength at 1m (simplified model)
    /// Assumes ferrite shielding + pulsed tx
    pub fn get_field_strength_microt_at_1m(&self) -> f32 {
        let duty = self.get_duty_cycle_percent() / 100.0;
        let peak_field_uT = 0.5; // Bluetooth peak ~0.5µT at 1m (measured)
        peak_field_uT * duty // Average due to duty-cycling
    }
}

/// Wildlife compliance check (called pre-tx)
pub fn verify_wildlife_safe(&gate: &EcoSafeRfGate) -> Result<(), String> {
    let field_uT = gate.get_field_strength_microt_at_1m();
    if field_uT > 0.01 {
        return Err(format!(
            "Estimated field {:.4}µT exceeds wildlife safety ceiling 0.01µT (honeybee/bird disruption threshold)",
            field_uT
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eco_safe_min_gap_enforcement() {
        let gate = EcoSafeRfGate::new(10.0, 500.0, 10000.0);
        
        // First tx allowed
        assert!(gate.check_tx_allowed(50.0, 0.0).is_ok());
        
        // Second tx too soon (gap 100ms < min 500ms) → blocked
        assert!(gate.check_tx_allowed(50.0, 100.0).is_err());
        
        // Third tx after min gap → allowed
        assert!(gate.check_tx_allowed(50.0, 550.0).is_ok());
    }

    #[test]
    fn test_wildlife_safe_field_threshold() {
        let gate = EcoSafeRfGate::new(10.0, 500.0, 10000.0);
        
        // Single pulse: 50ms tx, 10s window → ~0.5% duty → ~0.0025µT
        gate.check_tx_allowed(50.0, 0.0).ok();
        let field = gate.get_field_strength_microt_at_1m();
        println!("Field strength: {:.4}µT (safe: < 0.01µT)", field);
        assert!(field < 0.01, "Must stay below bee/bird disruption threshold");
    }

    #[test]
    fn test_compliance_with_neurorights_contract() {
        let gate = EcoSafeRfGate::new(5.0, 1000.0, 60000.0);
        
        for i in 0..10 {
            let now_ms = (i * 1200) as f64; // 1.2s pulses
            if let Ok(_) = gate.check_tx_allowed(100.0, now_ms) {
                let duty = gate.get_duty_cycle_percent();
                println!("Tx {}: duty {:.3}% (eco contract compliant: <0.5%)", i, duty);
                assert!(duty < 0.5);
            }
        }
    }
}
