use biophysical_runtime::BioTokenState;
use brainprint::{
    BrainPrint, BrainPrintBiophysics, BrainPrintLifeforce
};

fn make_brain_print_from_state(
    state: &BioTokenState,
    lifeforce_index: f32,
    blood_level: f32,
    oxygen_level: f32,
    clarity_index: f32,
    eco_band: u8,
    plane_flags: u16,
) -> [u8; brainprint::BRAINPRINT_BYTES] {
    let bio = BrainPrintBiophysics {
        brain: state.brain,
        wave: state.wave,
        blood: state.blood,
        oxygen: state.oxygen,
        nano: state.nano,
        smart: state.smart,
    };

    let lf = BrainPrintLifeforce {
        lifeforce_index,
        blood_level,
        oxygen_level,
        clarity_index,
        eco_band,
    };

    let host_id_bytes = state.hostid.id.as_bytes();
    let bp = BrainPrint::new(
        host_id_bytes,
        1,          // schema_version
        plane_flags, // e.g. 0b0000_0001 for "bioscale + BCI/HCI/EEG"
        bio,
        lf,
    );
    bp.to_bytes()
}
