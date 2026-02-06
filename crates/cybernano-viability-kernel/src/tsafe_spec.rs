use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// One named axis in the viability state vector, normalized to [0.0, 1.0].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TsafeAxis {
    pub name: String,
    pub min: f32, // expected 0.0
    pub max: f32, // expected 1.0
}

/// One linear inequality row: sum_i a_i * x_i <= b.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TsafeConstraintRow {
    /// Sparse coefficients keyed by axis name.
    pub a: HashMap<String, f32>,
    pub b: f32,
}

/// Viability kernel for a single named mode (e.g., Rehab, Baseline).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TsafeModeKernel {
    pub id: String,
    pub description: String,
    pub constraints: Vec<TsafeConstraintRow>,
}

/// Top-level spec loaded from `bostrom-vkernel-v1.vkernel.aln`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TsafeKernelSpec {
    pub subjectid: String,
    pub version: String,
    pub axes: Vec<TsafeAxis>,
    pub modes: Vec<TsafeModeKernel>,
}

impl TsafeKernelSpec {
    /// Simple structural invariants mirroring `.vkernel.aln` invariants.
    pub fn validate_invariants(&self) -> anyhow::Result<()> {
        use anyhow::{bail, Context};

        if self.axes.is_empty() {
            bail!("TsafeKernelSpec must define at least one axis");
        }

        for axis in &self.axes {
            if axis.min < 0.0 || axis.max > 1.0 || axis.min >= axis.max {
                bail!(
                    "Axis {} has invalid range [{}, {}]",
                    axis.name,
                    axis.min,
                    axis.max
                );
            }
        }

        // Build axis name set for constraint validation.
        let axis_names: Vec<String> = self.axes.iter().map(|a| a.name.clone()).collect();

        if self.modes.is_empty() {
            bail!("TsafeKernelSpec must define at least one mode");
        }

        for mode in &self.modes {
            if mode.constraints.is_empty() {
                bail!("Mode {} must have at least one constraint row", mode.id);
            }

            let mut has_lifeforce_min = false;

            for (row_idx, row) in mode.constraints.iter().enumerate() {
                // All coeffs are for known axes and bounded in [-10, 10].
                for (axis, coeff) in &row.a {
                    if !axis_names.iter().any(|n| n == axis) {
                        bail!(
                            "Mode {} row {} references unknown axis {}",
                            mode.id,
                            row_idx,
                            axis
                        );
                    }
                    if *coeff < -10.0 || *coeff > 10.0 {
                        bail!(
                            "Mode {} row {} coefficient for {} out of bounds: {}",
                            mode.id,
                            row_idx,
                            axis,
                            coeff
                        );
                    }
                }

                // Default: b >= 0, except for lifeforce lower-bound rows.
                let lifeforce_coeff = row.a.get("lifeforce").copied().unwrap_or(0.0);
                if lifeforce_coeff < 0.0 {
                    // Encoding lifeforce >= L as (a=-1, b=-L).
                    if row.b > 0.0 {
                        bail!(
                            "Mode {} row {} lifeforce lower-bound must have b <= 0, got {}",
                            mode.id,
                            row_idx,
                            row.b
                        );
                    }
                    has_lifeforce_min = true;
                } else if row.b < 0.0 {
                    bail!(
                        "Mode {} row {} has b < 0 without lifeforce lower-bound semantics",
                        mode.id,
                        row_idx
                    );
                }
            }

            if !has_lifeforce_min {
                bail!(
                    "Mode {} must include at least one lifeforce lower-bound constraint",
                    mode.id
                );
            }
        }

        Ok(())
    }

    /// Check if a given state vector is inside the kernel for the given mode.
    /// `state` is a mapping from axis name to normalized value.
    pub fn is_viable(
        &self,
        mode_id: &str,
        state: &HashMap<String, f32>,
    ) -> anyhow::Result<bool> {
        use anyhow::{bail, Context};

        let mode = self
            .modes
            .iter()
            .find(|m| m.id == mode_id)
            .with_context(|| format!("Unknown Tsafe mode id {}", mode_id))?;

        for row in &mode.constraints {
            let mut lhs = 0.0_f32;
            for (axis, coeff) in &row.a {
                let x = *state
                    .get(axis)
                    .with_context(|| format!("Missing state value for axis {}", axis))?;
                lhs += coeff * x;
            }
            if lhs > row.b + 1e-6 {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
