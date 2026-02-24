#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

use tissue_contact_envelopes::{
    BiomarkerSnapshot, EvidenceBundle, EvidenceTag, TissueClass, TissueContactEnvelope,
};
use cyberswarm_neurostack::dentocranial::{
    DentalCranialSnapshot, DentalCranialThresholds, DentalCranialDerived,
};
use bioscale_evolution_switch::{BciStar, RodScalar, LifeforceEnvelopeStatus};

/// Immune-defense evaluation for a given host region.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImmuneDefenseBand {
    Green,   // evolution allowed
    Yellow,  // low‑duty evolution / detox
    Orange,  // detox‑only
    Red,     // hard stop
}

/// Short evidence-backed thresholds for infection and organ stress.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ImmuneDefenseThresholds {
    pub il6_green_max_pg_ml: f32,   // e.g. 7 pg/mL
    pub il6_yellow_max_pg_ml: f32,  // e.g. 15 pg/mL
    pub crp_green_max_mg_l: f32,    // e.g. 2 mg/L
    pub crp_yellow_max_mg_l: f32,   // e.g. 8 mg/L
    pub crp_red_min_mg_l: f32,      // e.g. 100 mg/L brain‑abscess band
    pub thermo_yellow_delta_c: f32, // e.g. 1.5 °C
    pub thermo_red_delta_c: f32,    // e.g. 3.0 °C
    pub pain_yellow_vas: f32,       // e.g. 3.0/10
    pub pain_red_vas: f32,          // e.g. 7.0/10
}

impl ImmuneDefenseThresholds {
    pub const fn conservative_default() -> Self {
        Self {
            il6_green_max_pg_ml: 7.0,
            il6_yellow_max_pg_ml: 15.0,
            crp_green_max_mg_l: 2.0,
            crp_yellow_max_mg_l: 8.0,
            crp_red_min_mg_l: 100.0,
            thermo_yellow_delta_c: 1.5,
            thermo_red_delta_c: 3.0,
            pain_yellow_vas: 3.0,
            pain_red_vas: 7.0,
        }
    }
}

/// Region‑agnostic snapshot of immune stress, derived from dental‑cranial and organ biomarkers.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ImmuneDefenseSnapshot {
    pub il6_pg_ml: f32,
    pub crp_mg_l: f32,
    pub local_delta_t_c: f32,
    pub pain_vas: f32,
    pub bci_local: f32,
    pub rod_immune: f32,
    pub lifeforce: LifeforceEnvelopeStatus,
}

impl ImmuneDefenseSnapshot {
    /// Build from dental‑cranial telemetry plus global BCI/ROD.
    pub fn from_dental_cranial(
        snap: DentalCranialSnapshot,
        thr: DentalCranialThresholds,
        bci: BciStar,
        rod: RodScalar,
        lifeforce: LifeforceEnvelopeStatus,
    ) -> Self {
        let derived = DentalCranialDerived::from_snapshot(snap, thr);

        Self {
            il6_pg_ml: snap.il6_pg_ml,
            crp_mg_l: snap.crp_mg_l,
            local_delta_t_c: snap.dental_thermo_delta_t_c,
            pain_vas: snap.cranial_pain_vas,
            bci_local: derived.bci_local,
            rod_immune: derived.rod_dentocranial,
            lifeforce,
        }
    }

    /// Fold into a band for evolution logic.
    pub fn classify(&self, thr: ImmuneDefenseThresholds) -> ImmuneDefenseBand {
        // Hard systemic veto via LifeforceBand or extreme CRP.
        if matches!(self.lifeforce.as_lifeforce_band(), crate::tissue_contact_envelopes::LifeforceBand::HardStop)
            || self.crp_mg_l >= thr.crp_red_min_mg_l
        {
            return ImmuneDefenseBand::Red;
        }

        // Orange when immune ROD is high or strong local inflammation/pain.
        let strong_local = self.local_delta_t_c >= thr.thermo_red_delta_c
            || self.pain_vas >= thr.pain_red_vas;
        let rod_high = self.rod_immune >= 0.7;

        if strong_local || rod_high {
            return ImmuneDefenseBand::Orange;
        }

        // Yellow for low‑grade but non‑zero inflammatory burden.
        let mild_inflammation = (self.il6_pg_ml > thr.il6_green_max_pg_ml
            && self.il6_pg_ml <= thr.il6_yellow_max_pg_ml)
            || (self.crp_mg_l > thr.crp_green_max_mg_l
                && self.crp_mg_l <= thr.crp_yellow_max_mg_l)
            || (self.local_delta_t_c >= thr.thermo_yellow_delta_c
                && self.local_delta_t_c < thr.thermo_red_delta_c)
            || (self.pain_vas >= thr.pain_yellow_vas && self.pain_vas < thr.pain_red_vas);

        if mild_inflammation {
            ImmuneDefenseBand::Yellow
        } else {
            ImmuneDefenseBand::Green
        }
    }
}

/// What the neuromorphic immune trait must be able to decide for each actuation.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImmuneActuationMode {
    EvolutionAllowed,   // full evolution within global BCI/ROD/Lifeforce envelopes
    LowDutyEvolution,   // only low‑duty or “gentle” evolution
    DetoxOnly,          // detox / clearance, no new evolution
    HardStop,           // no actuation; clinical intervention required
}

/// Core guard struct: binds thresholds, dwell logic, and evidence tags.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuromorphicImmuneGuard {
    pub thresholds: ImmuneDefenseThresholds,
    pub last_green_since: Option<SystemTime>,
    pub dwell_to_full_evolution: Duration,
    pub evidence_tags: [String; 10],
}

impl NeuromorphicImmuneGuard {
    pub fn new_conservative(now: SystemTime) -> Self {
        Self {
            thresholds: ImmuneDefenseThresholds::conservative_default(),
            last_green_since: Some(now),
            dwell_to_full_evolution: Duration::from_secs(48 * 3600), // 48 h in green before full evolution
            evidence_tags: [
                "a1f3c9b2".into(), // IL‑6 corridor evidence
                "4be79d01".into(), // CRP thresholds
                "9cd4a7e8".into(), // thermography bands
                "2f8c6b44".into(), // pain / TSEP mapping
                "7e1da2ff".into(), // dental‑cranial infection link
                "5b93e0c3".into(), // BCIlocal / RODimmune math
                "d0174aac".into(), // duty vs inflammation
                "6ac2f9d9".into(), // LifeforceBand coupling
                "c4e61b20".into(), // RoH<=0.3 invariant
                "8f09d5ee".into(), // neurorights rollback
            ],
        }
    }

    /// Main decision function: what mode is permitted given current immune state.
    pub fn decide_mode(
        &mut self,
        snapshot: ImmuneDefenseSnapshot,
        now: SystemTime,
    ) -> ImmuneActuationMode {
        let band = snapshot.classify(self.thresholds);

        match band {
            ImmuneDefenseBand::Red => {
                // Reset dwell; require external care.
                self.last_green_since = None;
                ImmuneActuationMode::HardStop
            }
            ImmuneDefenseBand::Orange => {
                self.last_green_since = None;
                ImmuneActuationMode::DetoxOnly
            }
            ImmuneDefenseBand::Yellow => {
                self.last_green_since = None;
                ImmuneActuationMode::LowDutyEvolution
            }
            ImmuneDefenseBand::Green => {
                // Track continuous time in green before allowing full evolution.
                if self.last_green_since.is_none() {
                    self.last_green_since = Some(now);
                }
                if let Some(since) = self.last_green_since {
                    if now
                        .duration_since(since)
                        .unwrap_or_else(|_| Duration::from_secs(0))
                        >= self.dwell_to_full_evolution
                    {
                        ImmuneActuationMode::EvolutionAllowed
                    } else {
                        ImmuneActuationMode::LowDutyEvolution
                    }
                } else {
                    ImmuneActuationMode::LowDutyEvolution
                }
            }
        }
    }

    /// Export as an EvidenceBundle compatible with your existing crates.
    pub fn to_evidence_bundle(&self) -> EvidenceBundle {
        EvidenceBundle {
            sequences: self
                .evidence_tags
                .iter()
                .map(|h| EvidenceTag::from_short_hex(h.clone()))
                .collect(),
        }
    }

    /// Hard predicate for schedulers: evolution is forbidden under immune threat.
    pub fn evolution_forbidden(&self, snap: ImmuneDefenseSnapshot) -> bool {
        match snap.classify(self.thresholds) {
            ImmuneDefenseBand::Orange | ImmuneDefenseBand::Red => true,
            _ => false,
        }
    }
}

/// Trait that any neuromorphic‑immune‑aware scheduler must implement.
pub trait NeuromorphicImmunePath {
    /// Decide how a candidate upgrade may proceed, given immune state and tissue envelope.
    fn evaluate_with_immune(
        &self,
        immune_guard: &mut NeuromorphicImmuneGuard,
        immune_snapshot: ImmuneDefenseSnapshot,
        tissue: &TissueContactEnvelope,
        now: SystemTime,
    ) -> ImmuneActuationMode;
}

/// Example implementation for a generic cyberswarm upgrade scheduler.
pub struct CyberswarmImmuneScheduler;

impl NeuromorphicImmunePath for CyberswarmImmuneScheduler {
    fn evaluate_with_immune(
        &self,
        immune_guard: &mut NeuromorphicImmuneGuard,
        immune_snapshot: ImmuneDefenseSnapshot,
        tissue: &TissueContactEnvelope,
        now: SystemTime,
    ) -> ImmuneActuationMode {
        // First, respect organ/tissue hard stops.
        if tissue.roh_ceiling <= 0.0 {
            return ImmuneActuationMode::HardStop;
        }

        // Then, apply immune‑band logic.
        let mode = immune_guard.decide_mode(immune_snapshot, now);

        // Optionally co‑gate by tissue class (e.g. stricter for BrainCortex / CranialNerve).
        match tissue.tissue {
            TissueClass::BrainCortex | TissueClass::CranialNerve => match mode {
                ImmuneActuationMode::EvolutionAllowed => ImmuneActuationMode::LowDutyEvolution,
                other => other,
            },
            _ => mode,
        }
    }
}
