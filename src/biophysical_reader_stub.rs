use crate::sovereignty_core::{BiophysicalStateReader, StateVector};

/// Stub reader for testing / deviceless simulations.
/// In a real setup, this would translate HRV/EMG/EEG/behavioral features
/// into the StateVector abstraction.
pub struct StubBiophysicalReader;

impl BiophysicalStateReader for StubBiophysicalReader {
    fn read_state(&self) -> StateVector {
        StateVector {
            muscular_pain: 3,
            cognitive_load: 4,
            emotional_stress: 2,
            fatigue_index: 0.30,
            hrv_lf_hf: 1.10,
            emg_fatigue: 0.20,
        }
    }
}
