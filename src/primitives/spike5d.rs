use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Canonical 5-D spike event – direct biophysical encoding.
/// x,y,z = logical or embodied spatial coordinates
/// t       = precise event timestamp (seconds since UNIX epoch)
/// phi     = phase/energetic coordinate (radians, periodic 0–2π)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Spike5D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub t: f64,
    pub phi: f32,
}

impl Spike5D {
    pub fn now(x: f32, y: f32, z: f32, phi: f32) -> Self {
        let t = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs_f64();
        Self { x, y, z, t, phi }
    }

    /// Distance in 5-D manifold (Euclidean in space + phase-wrapped + scaled time)
    pub fn distance(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let dt = (self.t - other.t).abs() as f32;
        let dphi = (self.phi - other.phi).abs();
        let dphi_wrapped = dphi.min(2.0 * std::f32::consts::PI - dphi);

        (dx * dx + dy * dy + dz * dz + dt * dt + dphi_wrapped * dphi_wrapped).sqrt()
    }
}
