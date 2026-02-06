use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvisMeta {
    pub version: String,
    pub kind: String,
    pub description: String,
    pub subjectid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvisView {
    pub id: String,
    pub role: String,
    pub width_px: u32,
    pub height_px: u32,
    pub bg_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvisGlyph {
    pub id: String,
    pub r#type: String,
    pub text: String,
    pub font_size_px: u32,
    pub position_x: i32,
    pub position_y: i32,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvisBinding {
    pub glyph_id: String,
    pub source: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvisInvariants {
    pub roh_ceiling: f32,
    pub neurorights_noncommercialneuraldata: bool,
    pub hardware_actuation_forbidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvisLayout {
    pub meta: NvisMeta,
    pub view: NvisView,
    pub glyphs: Vec<NvisGlyph>,
    pub bindings: Vec<NvisBinding>,
    pub invariants: NvisInvariants,
}

impl NvisLayout {
    pub fn validate(&self) -> anyhow::Result<()> {
        use anyhow::bail;

        if (self.invariants.roh_ceiling - 0.30).abs() > f32::EPSILON {
            bail!("NvisLayout invariant failed: roh_ceiling must be 0.30");
        }
        if !self.invariants.neurorights_noncommercialneuraldata {
            bail!("NvisLayout invariant failed: noncommercialneuraldata must be true");
        }
        if !self.invariants.hardware_actuation_forbidden {
            bail!("NvisLayout invariant failed: hardware_actuation_forbidden must be true");
        }
        if self.meta.subjectid.is_empty() {
            bail!("NvisLayout invariant failed: subjectid must be non-empty");
        }
        Ok(())
    }
}
