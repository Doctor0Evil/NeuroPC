use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NwaveAxis {
    pub name: String,
    pub unit: String,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NwaveChannel {
    pub id: String,
    pub role: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NwaveMeta {
    pub version: String,
    pub kind: String,
    pub description: String,
    pub subjectid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NwaveInvariants {
    pub roh_ceiling: f32,
    pub neurorights_noncommercialneuraldata: bool,
    pub dreamstatesensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NwaveSpec {
    pub meta: NwaveMeta,
    pub axes: Vec<NwaveAxis>,
    pub channels: Vec<NwaveChannel>,
    pub invariants: NwaveInvariants,
}

impl NwaveSpec {
    pub fn validate(&self) -> anyhow::Result<()> {
        use anyhow::{anyhow, bail};

        if (self.invariants.roh_ceiling - 0.30).abs() > f32::EPSILON {
            bail!("NwaveSpec invariant failed: roh_ceiling must be 0.30");
        }
        if !self.invariants.neurorights_noncommercialneuraldata {
            bail!("NwaveSpec invariant failed: noncommercialneuraldata must be true");
        }
        if self.meta.subjectid.is_empty() {
            bail!("NwaveSpec invariant failed: subjectid must be non-empty");
        }
        if self.axes.is_empty() {
            bail!("NwaveSpec invariant failed: at least one axis required");
        }
        if self.channels.is_empty() {
            bail!("NwaveSpec invariant failed: at least one channel required");
        }
        for a in &self.axes {
            if a.min >= a.max {
                return Err(anyhow!("Axis {} has invalid range", a.name));
            }
        }
        Ok(())
    }
}

pub struct Nwave {
    pub spec: NwaveSpec,
    pub data: Vec<u8>,
}

impl Nwave {
    pub fn load_from_files(
        spec_bytes: &[u8],
        wave_bytes: Vec<u8>,
    ) -> anyhow::Result<Self> {
        let spec: NwaveSpec = organiccpualn::from_aln_bytes(spec_bytes)?;
        spec.validate()?;
        Ok(Self { spec, data: wave_bytes })
    }
}
