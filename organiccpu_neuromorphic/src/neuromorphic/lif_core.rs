#![no_std]
#![feature(const_trait_impl)]

use core::fmt::{Debug, Formatter};

/// Fixed-point 16-bit membrane potential (scaled: 0..65535 ≡ -8.0..8.0 V range, 0.000244 V resolution)
type Potential = u16;

/// LIF neuron state – single compartment (extensible to 9 via array for Loihi parity)
#[derive(Copy, Clone, Default)]
pub struct LifNeuron {
    /// Current membrane potential (fixed-point)
    pub u: Potential,
    /// Synaptic input accumulator (cleared post-tick)
    pub input: i32,
    /// Last spike timestamp (monotonic ns)
    pub last_spike_ns: u64,
}

/// Population of LIF neurons with compile-time size bound
pub struct LifPopulation<const N: usize> {
    neurons: [LifNeuron; N],
    /// Pre-computed decay factor (Q15 fixed-point: 0..32768 ≡ 0.0..1.0)
    decay: u16,
    /// Threshold (fixed-point)
    threshold: Potential,
    /// Reset value
    reset: Potential,
    /// Resting potential contribution
    rest_contrib: Potential,
}

impl<const N: usize> LifPopulation<N> {
    /// Construct with biophysical parameters
    /// tau_ms: membrane time constant in milliseconds (1..100 typical)
    /// threshold_v: spike threshold in scaled units
    /// reset_v: post-spike reset in scaled units
    /// rest_v: resting potential in scaled units
    #[inline(always)]
    pub const fn new(tau_ms: u16, threshold_v: Potential, reset_v: Potential, rest_v: Potential) -> Self {
        // decay = exp(-1/tau) approximated via Q15 lookup or const eval
        // For production: use pre-computed table or const fn approximation
        let decay = if tau_ms == 0 { 0 } else { 32768_u16.saturating_sub(32768 / tau_ms) };
        Self {
            neurons: [LifNeuron::default(); N],
            decay,
            threshold: threshold_v,
            reset: reset_v,
            rest_contrib: rest_v,
        }
    }

    /// Advance one timestep (Δt = 1 ms assumed)
    /// Returns count of emitted spikes this tick
    #[inline(always)]
    pub fn tick(&mut self, current_time_ns: u64) -> usize {
        let mut spike_count = 0;
        for neuron in self.neurons.iter_mut() {
            // Euler integration
            let decayed = ((neuron.u as u32) * (self.decay as u32) / 32768) as Potential;
            let integrated = decayed.saturating_add(self.rest_contrib);
            // Input scaled by synaptic weight factor (here ×1)
            let new_u = integrated.wrapping_add_signed(neuron.input as i16);

            neuron.input = 0; // clear accumulator

            if new_u >= self.threshold {
                spike_count += 1;
                neuron.u = self.reset;
                neuron.last_spike_ns = current_time_ns;
            } else {
                neuron.u = new_u;
            }
        }
        spike_count
    }

    /// Inject current into neuron index
    #[inline(always)]
    pub fn inject(&mut self, idx: usize, current: i32) {
        if idx < N {
            self.neurons[idx].input = self.neurons[idx].input.saturating_add(current);
        }
    }
}

impl Debug for LifNeuron {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "LIF(u={:.4}V, last_spike={})", (self.u as f32 / 8192.0 - 8.0), self.last_spike_ns)
    }
}
