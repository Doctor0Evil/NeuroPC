use std::fmt;

/// High-level categories of workloads that must pass neuroconsent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadClass {
    LightQuery,       // metadata reads, small embeddings
    ModerateCompute,  // re-ranking, light model passes
    HeavyUpdate,      // index/parameter updates
    DeepExcavation,   // N3/? dream-object or lifeforce-intensive ops
}

/// Snapshot of neuro-eco state derived from existing Biospectre modules.
#[derive(Debug, Clone, Copy)]
pub struct NeuroSnapshot {
    pub stage: SleepStage,
    pub sn3: f32,
    pub s_unknown: f32,
    pub eco_energy_nj: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepStage {
    Wake,
    N1,
    N2,
    N3,
    Rem,
    Unknown,
}

/// Cognitive twin view (clarity, fatigue, stress).
#[derive(Debug, Clone, Copy)]
pub struct TwinCognitiveState {
    pub clarity_score01: f32,
    pub fatigue_score01: f32,
    pub stress_score01: f32,
}

/// Lifeforce scalar and band from lifeforce governor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifeforceBand {
    Green,
    Yellow,
    Red,
}

#[derive(Debug, Clone, Copy)]
pub struct LifeforceState {
    pub lifeforce_scalar01: f32,
    pub band: LifeforceBand,
}

/// Minimal token bundle needed for consent decisions.
#[derive(Debug, Clone, Copy)]
pub struct TokenSlice {
    pub brain_tokens: f32,
    pub dracula_wave: f32,
    pub eco_nj_budget: f32,
}

/// Policy for neuroconsent thresholds.
#[derive(Debug, Clone)]
pub struct NeuroconsentPolicy {
    pub min_clarity_for_heavy: f32,
    pub max_fatigue_for_heavy: f32,
    pub max_stress_for_heavy: f32,
    pub min_lifeforce_for_heavy: f32,
    pub max_epoch_eco_nj: f32,
    pub min_lifeforce_for_deep_excavation: f32,
}

impl Default for NeuroconsentPolicy {
    fn default() -> Self {
        Self {
            min_clarity_for_heavy: 0.60,
            max_fatigue_for_heavy: 0.45,
            max_stress_for_heavy: 0.55,
            min_lifeforce_for_heavy: 0.55,
            max_epoch_eco_nj: 120.0,
            min_lifeforce_for_deep_excavation: 0.70,
        }
    }
}

/// Required token costs for a specific workload class.
#[derive(Debug, Clone, Copy)]
pub struct WorkloadCost {
    pub brain_cost: f32,
    pub dw_cost: f32,
    pub eco_cost_nj: f32,
}

/// A single neuroconsent request, akin to "#!Yield_Neuroconsent".
#[derive(Debug, Clone)]
pub struct NeuroconsentRequest {
    pub workload_id: String,
    pub class: WorkloadClass,
    pub cost: WorkloadCost,
    pub require_stable_stage: bool,
}

/// Output of the yield function, suitable for ALN logging.
#[derive(Debug, Clone)]
pub struct NeuroconsentOutcome {
    pub allowed: bool,
    pub reason: String,
}

impl fmt::Display for NeuroconsentOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "allowed={}, reason={}", self.allowed, self.reason)
    }
}

/// Core yield gate for neurological and biophysical consent.
#[derive(Debug, Clone)]
pub struct NeuroconsentGate {
    policy: NeuroconsentPolicy,
}

impl NeuroconsentGate {
    pub fn new(policy: NeuroconsentPolicy) -> Self {
        Self { policy }
    }

    fn is_heavy(class: WorkloadClass) -> bool {
        matches!(class, WorkloadClass::HeavyUpdate | WorkloadClass::DeepExcavation)
    }

    fn is_deep_excavation(class: WorkloadClass) -> bool {
        matches!(class, WorkloadClass::DeepExcavation)
    }

    /// Main yield function: evaluates consent given current state and tokens.
    pub fn yield_neuroconsent(
        &self,
        neuro: NeuroSnapshot,
        twin: TwinCognitiveState,
        lifeforce: LifeforceState,
        tokens: TokenSlice,
        request: &NeuroconsentRequest,
    ) -> NeuroconsentOutcome {
        let heavy = Self::is_heavy(request.class);
        let deep = Self::is_deep_excavation(request.class);

        // Always block if eco per epoch is extreme.
        if neuro.eco_energy_nj > self.policy.max_epoch_eco_nj {
            return NeuroconsentOutcome {
                allowed: false,
                reason: format!(
                    "Denied: eco_energy_nj {:.1} exceeds safe per-epoch maximum {:.1}.",
                    neuro.eco_energy_nj, self.policy.max_epoch_eco_nj
                ),
            };
        }

        // Stage stability requirement for heavy workloads, if requested.
        if request.require_stable_stage && heavy {
            match neuro.stage {
                SleepStage::N3 | SleepStage::Wake => {} // allowed stages
                _ => {
                    return NeuroconsentOutcome {
                        allowed: false,
                        reason: format!(
                            "Denied: workload {:?} requires stable Wake/N3, found {:?}.",
                            request.class, neuro.stage
                        ),
                    };
                }
            }
        }

        // Additional requirement for deep excavation workloads.
        if deep {
            if lifeforce.lifeforce_scalar01 < self.policy.min_lifeforce_for_deep_excavation {
                return NeuroconsentOutcome {
                    allowed: false,
                    reason: format!(
                        "Denied: lifeforce {:.2} below {:.2} required for DeepExcavation.",
                        lifeforce.lifeforce_scalar01,
                        self.policy.min_lifeforce_for_deep_excavation
                    ),
                };
            }
            if neuro.s_unknown > 0.7 {
                return NeuroconsentOutcome {
                    allowed: false,
                    reason: format!(
                        "Denied: S? {:.2} too high for safe DeepExcavation in current epoch.",
                        neuro.s_unknown
                    ),
                };
            }
        }

        // Heavy workloads must obey clarity, fatigue, stress, lifeforce limits.
        if heavy {
            if twin.clarity_score01 < self.policy.min_clarity_for_heavy {
                return NeuroconsentOutcome {
                    allowed: false,
                    reason: format!(
                        "Denied: clarity {:.2} below minimum {:.2} for heavy workloads.",
                        twin.clarity_score01, self.policy.min_clarity_for_heavy
                    ),
                };
            }
            if twin.fatigue_score01 > self.policy.max_fatigue_for_heavy {
                return NeuroconsentOutcome {
                    allowed: false,
                    reason: format!(
                        "Denied: fatigue {:.2} above maximum {:.2} for heavy workloads.",
                        twin.fatigue_score01, self.policy.max_fatigue_for_heavy
                    ),
                };
            }
            if twin.stress_score01 > self.policy.max_stress_for_heavy {
                return NeuroconsentOutcome {
                    allowed: false,
                    reason: format!(
                        "Denied: stress {:.2} above maximum {:.2} for heavy workloads.",
                        twin.stress_score01, self.policy.max_stress_for_heavy
                    ),
                };
            }
            if lifeforce.lifeforce_scalar01 < self.policy.min_lifeforce_for_heavy {
                return NeuroconsentOutcome {
                    allowed: false,
                    reason: format!(
                        "Denied: lifeforce {:.2} below {:.2} for heavy workloads.",
                        lifeforce.lifeforce_scalar01,
                        self.policy.min_lifeforce_for_heavy
                    ),
                };
            }
        }

        // Token sufficiency check (Brain, DraculaWave, Eco).
        if tokens.brain_tokens < request.cost.brain_cost
            || tokens.dracula_wave < request.cost.dw_cost
            || tokens.eco_nj_budget < request.cost.eco_cost_nj
        {
            return NeuroconsentOutcome {
                allowed: false,
                reason: "Denied: insufficient Brain/DraculaWave/Eco budget for requested workload."
                    .into(),
            };
        }

        // If we reach here, consent is granted.
        let band_str = match lifeforce.band {
            LifeforceBand::Green => "green",
            LifeforceBand::Yellow => "yellow",
            LifeforceBand::Red => "red",
        };

        NeuroconsentOutcome {
            allowed: true,
            reason: format!(
                "Allowed: {:?} under lifeforce band {} (lf={:.2}), stage={:?}, clarity={:.2}, fatigue={:.2}, stress={:.2}, cost(brain={:.1}, dw={:.1}, eco_nj={:.1}).",
                request.class,
                band_str,
                lifeforce.lifeforce_scalar01,
                neuro.stage,
                twin.clarity_score01,
                twin.fatigue_score01,
                twin.stress_score01,
                request.cost.brain_cost,
                request.cost.dw_cost,
                request.cost.eco_cost_nj
            ),
        }
    }
}
