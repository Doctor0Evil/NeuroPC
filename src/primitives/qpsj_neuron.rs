use rand::Rng;
use crate::primitives::spike5d::Spike5D;

/// Quantum-phase-slip-junction analog neuron for organic_cpu.
/// Membrane potential integrates incoming currents with biophysical noise.
/// Phase-slip stochasticity provides native hardware randomness source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QpsjNeuron {
    pub position: (f32, f32, f32),   // 3-D embedding for 5-D addressing
    membrane: f32,
    threshold_base: f32,             // nominal critical current analog
    refractory_cycles: u32,
    leak_rate: f32,                  // passive decay (biological realism)
    noise_amplitude: f32,            // biophysical fluctuation strength
}

impl QpsjNeuron {
    pub fn new(position: (f32, f32, f32), threshold_base: f32) -> Self {
        Self {
            position,
            membrane: 0.0,
            threshold_base,
            refractory_cycles: 0,
            leak_rate: 0.05,             // tunable biophysical parameter
            noise_amplitude: 0.12,       // organic stochasticity level
        }
    }

    /// Integrate over dt (seconds). Returns Some(Spike5D) on fire.
    pub fn step(&mut self, input_current: f32, dt: f64) -> Option<Spike5D> {
        if self.refractory_cycles > 0 {
            self.refractory_cycles -= 1;
            self.membrane *= (1.0 - self.leak_rate); // decay during refractory
            return None;
        }

        // Passive leak + input
        self.membrane *= (1.0 - self.leak_rate);
        self.membrane += input_current * dt as f32;

        // Biophysical phase-slip noise (organic randomness source)
        let mut rng = rand::thread_rng();
        let noise = rng.gen_range(-self.noise_amplitude..self.noise_amplitude);
        let effective_threshold = self.threshold_base + noise;

        if self.membrane >= effective_threshold {
            let phi = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
            let spike = Spike5D::now(self.position.0, self.position.1, self.position.2, phi);
            self.membrane = 0.0;                     // reset
            self.refractory_cycles = 8;              // example biophysical refractory
            Some(spike)
        } else {
            None
        }
    }
}
