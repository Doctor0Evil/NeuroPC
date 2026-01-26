use std::fmt;

use crate::neuroconsent_yield::{
    LifeforceBand,
    LifeforceState,
    NeuroSnapshot,
    TwinCognitiveState,
};
use crate::brain_tokens::tokens::BiophysicalTokenBundle;
use crate::brain_tokens::neuralrope::{NeuralRope5D, NeuralRope7D};

/// High-level AI-Chat capability families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiCapabilityKind {
    LowImpactQuery,
    HistorySummarization,
    DreamObjectExcavation,
    ModelUpgrade,
    SecureDataExport,
}

/// Cross-platform sensitivity level for operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSensitivity {
    PublicMeta,
    PersonalMeta,
    NeuralDerived,
    TokenLedger,
}

/// Descriptor of what an AI-Chat action wants to do.
#[derive(Debug, Clone)]
pub struct AiChatCapabilityRequest {
    pub kind: AiCapabilityKind,
    pub sensitivity: DataSensitivity,
    pub estimated_flops: u64,
    pub rope7d: NeuralRope7D,
}

/// Resulting capability budget to enforce downstream.
#[derive(Debug, Clone)]
pub struct AiChatCapabilityGrant {
    pub allowed: bool,
    pub max_flops: u64,
    pub max_latency_ms: u32,
    pub reason: String,
}

impl fmt::Display for AiChatCapabilityGrant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "allowed={}, max_flops={}, max_latency_ms={}, reason={}",
            self.allowed, self.max_flops, self.max_latency_ms, self.reason
        )
    }
}

/// Engine that turns neurostate + tokens + rope into enforceable chat budgets.
#[derive(Debug, Clone)]
pub struct AiChatCapabilityEngine {
    pub base_max_flops: u64,
    pub base_max_latency_ms: u32,
}

impl Default for AiChatCapabilityEngine {
    fn default() -> Self {
        Self {
            base_max_flops: 150_000_000, // 1.5e8 FLOPs baseline
            base_max_latency_ms: 2000,
        }
    }
}

impl AiChatCapabilityEngine {
    pub fn decide(
        &self,
        neuro: NeuroSnapshot,
        twin: TwinCognitiveState,
        lifeforce: LifeforceState,
        tokens: &BiophysicalTokenBundle,
        req: &AiChatCapabilityRequest,
    ) -> AiChatCapabilityGrant {
        let rope5d: NeuralRope5D = req.rope7d.project_to_5d();

        let stage_factor = match neuro.stage {
            crate::neuroconsent_yield::SleepStage::Wake => 1.0,
            crate::neuroconsent_yield::SleepStage::N1 => 0.7,
            crate::neuroconsent_yield::SleepStage::N2 => 0.8,
            crate::neuroconsent_yield::SleepStage::N3 => 1.1,
            crate::neuroconsent_yield::SleepStage::Rem => 0.9,
            crate::neuroconsent_yield::SleepStage::Unknown => 0.5,
        };

        let lifeforce_factor = match lifeforce.band {
            LifeforceBand::Green => 1.0,
            LifeforceBand::Yellow => 0.7,
            LifeforceBand::Red => 0.3,
        };

        let clarity = twin.clarity_score01;
        let fatigue = twin.fatigue_score01;

        let mut capacity_factor = stage_factor * lifeforce_factor * (0.5 + 0.5 * clarity);
        capacity_factor *= (1.0 - 0.5 * fatigue).max(0.2);

        let eco_factor = if tokens.eco.balancenjequiv > 0.0 {
            let relative = (tokens.eco.balancenjequiv / 1000.0).min(1.5);
            (0.7 + 0.3 * relative).max(0.4)
        } else {
            0.4
        };

        let variance_weight = rope5d.components[0].abs().min(2.0);
        let mut effective_capacity =
            (self.base_max_flops as f32) * capacity_factor * eco_factor * (0.7 + 0.3 * variance_weight);

        match req.kind {
            AiCapabilityKind::LowImpactQuery => {
                effective_capacity *= 0.4;
            }
            AiCapabilityKind::HistorySummarization => {
                effective_capacity *= 0.7;
            }
            AiCapabilityKind::DreamObjectExcavation => {
                effective_capacity *= 1.0;
            }
            AiCapabilityKind::ModelUpgrade => {
                effective_capacity *= 1.3;
            }
            AiCapabilityKind::SecureDataExport => {
                effective_capacity *= 0.5;
            }
        }

        let max_flops = effective_capacity.max(10_000.0) as u64;
        let mut max_latency = self.base_max_latency_ms;

        if matches!(req.kind, AiCapabilityKind::LowImpactQuery) {
            max_latency = (self.base_max_latency_ms as f32 * 0.5) as u32;
        }

        let allowed = req.estimated_flops <= max_flops;

        let reason = if allowed {
            format!(
                "Allowed {:?} with max_flops={} under stage={:?}, lifeforce_band={:?}, clarity={:.2}, eco_nj={:.1}.",
                req.kind, max_flops, neuro.stage, lifeforce.band, clarity, neuro.eco_energy_nj
            )
        } else {
            format!(
                "Denied {:?}: requested_flops={} exceeds max_flops={} at stage={:?}, band={:?}.",
                req.kind, req.estimated_flops, max_flops, neuro.stage, lifeforce.band
            )
        };

        AiChatCapabilityGrant {
            allowed,
            max_flops,
            max_latency_ms: max_latency,
            reason,
        }
    }
}
