//! brainprint: host-local, non-financial, soul-safe brain state capsule.
//!
//! Produces a fixed-layout, machine-readable binary "brainPrint" record
//! usable by any host service (Rust, JS via WASM, etc.) without exposing
//! souls or consciousness, and without enabling transfer/finance.

use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};

/// Awareness check: this is a host-bound, biophysical proxy only.
/// No ownership, no consciousness fields.[file:8]
#[derive(Clone, Debug)]
pub struct BrainPrintHeader {
    /// ALN/DID/Bostrom-style host identifier (trimmed or hashed to 32 bytes).
    pub host_id: [u8; 32],
    /// 64-bit unix epoch millis.
    pub timestamp_ms: u64,
    /// Schema version for forward-compat parsing.
    pub schema_version: u16,
    /// Plane / environment flags (bitfield) – e.g., bioscale, bci-hci-eeg, software-only.
    pub plane_flags: u16,
}

/// Biophysical token snapshot, aligned with your BioTokenState.[file:8]
#[derive(Clone, Debug)]
pub struct BrainPrintBiophysics {
    pub brain: f64,
    pub wave: f64,
    pub blood: f64,
    pub oxygen: f64,
    pub nano: f64,
    pub smart: f64,
}

/// Optional eco + lifeforce summary (normalized to 0–1 ranges where applicable).[file:7]
#[derive(Clone, Debug)]
pub struct BrainPrintLifeforce {
    pub lifeforce_index: f32,
    pub blood_level: f32,
    pub oxygen_level: f32,
    pub clarity_index: f32,
    /// Encoded eco-band: 0=low,1=medium,2=high.
    pub eco_band: u8,
}

/// Final packed struct in host memory.
#[derive(Clone, Debug)]
pub struct BrainPrint {
    pub header: BrainPrintHeader,
    pub biophysics: BrainPrintBiophysics,
    pub lifeforce: BrainPrintLifeforce,
    /// 32-byte quantum-safe hash of the preceding fields (host-local attestation).[file:8]
    pub state_hash: [u8; 32],
}

/// Fixed binary layout (little-endian):
///
/// bytes 0..32   host_id
/// bytes 32..40  timestamp_ms (u64)
/// bytes 40..42  schema_version (u16)
/// bytes 42..44  plane_flags (u16)
/// bytes 44..(44+8*6)  brain,wave,blood,oxygen,nano,smart (f64 each)
/// next 4*4      lifeforce_index,blood_level,oxygen_level,clarity_index (f32)
/// next 1        eco_band (u8)
/// padding 3     reserved (u8[3])
/// last 32       state_hash (u8[32])
///
/// Total size: 44 + 48 + 16 + 1 + 3 + 32 = 144 bytes.
pub const BRAINPRINT_BYTES: usize = 144;

/// Very small, host-local hash to avoid pulling in heavy PQ crates.
/// In your biophysical-runtime you can swap this with QuantumHash.[file:8]
fn simple_host_hash(input: &[u8]) -> [u8; 32] {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    let mut h = DefaultHasher::new();
    input.hash(&mut h);
    let raw = h.finish().to_le_bytes();
    let mut out = [0u8; 32];
    // Repeat raw into 32 bytes (deterministic, not crypto-strong).
    for i in 0..4 {
        out[i * 8..(i + 1) * 8].copy_from_slice(&raw);
    }
    out
}

impl BrainPrint {
    /// Construct a brainPrint from host state and lifeforce snapshot.
    pub fn new(
        host_id_bytes: &[u8],
        schema_version: u16,
        plane_flags: u16,
        biophysics: BrainPrintBiophysics,
        lifeforce: BrainPrintLifeforce,
    ) -> Self {
        // Awareness check: no negative BRAIN, no BLOOD/OXYGEN ≤ 0.[file:8]
        assert!(biophysics.brain >= 0.0);
        assert!(biophysics.blood > 0.0);
        assert!(biophysics.oxygen > 0.0);

        let mut host_id = [0u8; 32];
        if host_id_bytes.len() >= 32 {
            host_id.copy_from_slice(&host_id_bytes[0..32]);
        } else {
            host_id[0..host_id_bytes.len()].copy_from_slice(host_id_bytes);
        }

        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let header = BrainPrintHeader {
            host_id,
            timestamp_ms: now_ms,
            schema_version,
            plane_flags,
        };

        // First compute hash over all fields except state_hash.
        let mut tmp = Vec::with_capacity(BRAINPRINT_BYTES);
        BrainPrint::encode_without_hash(&header, &biophysics, &lifeforce, &mut tmp);
        let state_hash = simple_host_hash(&tmp);

        BrainPrint {
            header,
            biophysics,
            lifeforce,
            state_hash,
        }
    }

    fn encode_without_hash(
        header: &BrainPrintHeader,
        bio: &BrainPrintBiophysics,
        lf: &BrainPrintLifeforce,
        out: &mut Vec<u8>,
    ) {
        out.extend_from_slice(&header.host_id);
        out.extend_from_slice(&header.timestamp_ms.to_le_bytes());
        out.extend_from_slice(&header.schema_version.to_le_bytes());
        out.extend_from_slice(&header.plane_flags.to_le_bytes());

        out.extend_from_slice(&bio.brain.to_le_bytes());
        out.extend_from_slice(&bio.wave.to_le_bytes());
        out.extend_from_slice(&bio.blood.to_le_bytes());
        out.extend_from_slice(&bio.oxygen.to_le_bytes());
        out.extend_from_slice(&bio.nano.to_le_bytes());
        out.extend_from_slice(&bio.smart.to_le_bytes());

        out.extend_from_slice(&lf.lifeforce_index.to_le_bytes());
        out.extend_from_slice(&lf.blood_level.to_le_bytes());
        out.extend_from_slice(&lf.oxygen_level.to_le_bytes());
        out.extend_from_slice(&lf.clarity_index.to_le_bytes());
        out.push(lf.eco_band);
        // 3 bytes reserved padding
        out.extend_from_slice(&[0u8; 3]);
    }

    /// Serialize to fixed-size, machine-readable binary.
    pub fn to_bytes(&self) -> [u8; BRAINPRINT_BYTES] {
        let mut buf = Vec::with_capacity(BRAINPRINT_BYTES);
        Self::encode_without_hash(&self.header, &self.biophysics, &self.lifeforce, &mut buf);
        buf.extend_from_slice(&self.state_hash);
        let arr: [u8; BRAINPRINT_BYTES] = buf
            .as_slice()
            .try_into()
            .expect("brainPrint length mismatch");
        arr
    }

    /// Parse from binary and verify the embedded hash.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != BRAINPRINT_BYTES {
            return None;
        }
        let mut idx = 0;

        let mut host_id = [0u8; 32];
        host_id.copy_from_slice(&bytes[idx..idx + 32]);
        idx += 32;

        let mut ts_bytes = [0u8; 8];
        ts_bytes.copy_from_slice(&bytes[idx..idx + 8]);
        idx += 8;
        let timestamp_ms = u64::from_le_bytes(ts_bytes);

        let mut sv_bytes = [0u8; 2];
        sv_bytes.copy_from_slice(&bytes[idx..idx + 2]);
        idx += 2;
        let schema_version = u16::from_le_bytes(sv_bytes);

        let mut pf_bytes = [0u8; 2];
        pf_bytes.copy_from_slice(&bytes[idx..idx + 2]);
        idx += 2;
        let plane_flags = u16::from_le_bytes(pf_bytes);

        let mut take_f64 = |b: &[u8], i: &mut usize| -> f64 {
            let mut tmp = [0u8; 8];
            tmp.copy_from_slice(&b[*i..*i + 8]);
            *i += 8;
            f64::from_le_bytes(tmp)
        };
        let mut take_f32 = |b: &[u8], i: &mut usize| -> f32 {
            let mut tmp = [0u8; 4];
            tmp.copy_from_slice(&b[*i..*i + 4]);
            *i += 4;
            f32::from_le_bytes(tmp)
        };

        let brain = take_f64(bytes, &mut idx);
        let wave = take_f64(bytes, &mut idx);
        let blood = take_f64(bytes, &mut idx);
        let oxygen = take_f64(bytes, &mut idx);
        let nano = take_f64(bytes, &mut idx);
        let smart = take_f64(bytes, &mut idx);

        let lifeforce_index = take_f32(bytes, &mut idx);
        let blood_level = take_f32(bytes, &mut idx);
        let oxygen_level = take_f32(bytes, &mut idx);
        let clarity_index = take_f32(bytes, &mut idx);

        let eco_band = bytes[idx];
        idx += 1;
        // skip padding
        idx += 3;

        let mut state_hash = [0u8; 32];
        state_hash.copy_from_slice(&bytes[idx..idx + 32]);

        let header = BrainPrintHeader {
            host_id,
            timestamp_ms,
            schema_version,
            plane_flags,
        };
        let bio = BrainPrintBiophysics {
            brain,
            wave,
            blood,
            oxygen,
            nano,
            smart,
        };
        let lf = BrainPrintLifeforce {
            lifeforce_index,
            blood_level,
            oxygen_level,
            clarity_index,
            eco_band,
        };

        // Recompute hash and verify.
        let mut tmp = Vec::with_capacity(BRAINPRINT_BYTES);
        BrainPrint::encode_without_hash(&header, &bio, &lf, &mut tmp);
        let check = simple_host_hash(&tmp);
        if check != state_hash {
            return None;
        }

        // Invariant recheck to keep this soul-safe.[file:8]
        if bio.brain < 0.0 || bio.blood <= 0.0 || bio.oxygen <= 0.0 {
            return None;
        }

        Some(BrainPrint {
            header,
            biophysics: bio,
            lifeforce: lf,
            state_hash,
        })
    }
}
