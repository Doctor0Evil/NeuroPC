#![forbid(unsafe_code)]

use std::time::SystemTime;

// Core imports from your existing stack (paths may be adjusted to your workspace)
use organic_cpu_roh::RoHBand;
use organic_cpu_roh::RoHGuardedHostState;
use organic_cpu_neurorights::NeuroRightsProfileId;
use organic_cpu_neurorights::NeuroRightsVerdict;
use organic_cpu_neurorights::NeuroRightsEngine;
use organic_cpu_stake::StakeRole;
use organic_cpu_stake::StakeVerdict;
use organic_cpu_stake::StakeEngine;
use sovereignty_core_donutloop::DonutloopEntry;
use sovereignty_core_donutloop::DonutloopWriter;
use sovereignty_core_envelopes::CorridorId;
use sovereignty_core_envelopes::SessionId;
use sovereignty_core_envelopes::BioStateSummary;
use sovereignty_core_tokens::TokenKind;
use sovereignty_core_tokens::TokenLedger;

// -----------------------------
// Guest cyberswarm capability
// -----------------------------

/// The only 3 modes a guest cyberswarm may operate in. All are strictly
/// subordinate to the sovereignty core.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwarmMode {
    /// Pure observation: can read summarized BioState and corridor IDs,
    /// but may not propose any actuation or evolution.
    Observe,

    /// Safe filter only: may propose filtered suggestions over external
    /// inputs (e.g., spam/abuse filters, routing), but no OrganicCPU-
    /// facing actuation or evolution.
    SafeFilterOnly,

    /// Safe filter plus evolution: may propose bounded evolution steps
    /// that still must pass RoH, neurorights, stake, and token guards.
    SafeFilterPlusEvolution,
}

/// Minimal, sovereignty-controlled view of the host for guest swarms.
#[derive(Clone, Debug)]
pub struct SovereignHostView {
    pub corridor_id: CorridorId,
    pub session_id: SessionId,
    pub bio_summary: BioStateSummary,
    pub roh_band: RoHBand,
    pub neurorights_profile: NeuroRightsProfileId,
}

/// Bounded action algebra exposed to guest cyberswarms.
///
/// NOTE: this is intentionally small. Any additional variant must be
/// justified with proofs and ALN policy updates.
#[derive(Clone, Debug)]
pub enum SwarmAction {
    /// Purely informational recommendation; no actuation implied.
    RecommendInfo {
        reason: String,
        tags: Vec<String>,
    },

    /// Safe filtering suggestion (e.g., block, mute, de-prioritize).
    /// Sovereignty core decides whether to enact.
    SuggestFilter {
        target_id: String,
        filter_kind: String,
        reason: String,
    },

    /// Bounded evolution proposal for OrganicCPU/NeuroPC.
    /// This is not a direct actuation: it must be wrapped and run
    /// through the full sovereignty pipeline.
    ProposeEvolution {
        proposal: SwarmEvolutionProposal,
    },
}

/// Minimal evolution proposal shape a guest is allowed to emit.
/// It does NOT contain the full EvolutionProposalRecord shape;
/// sovereignty core will wrap it and add roh_before/after, stake,
/// neurorights, and token data.
#[derive(Clone, Debug)]
pub struct SwarmEvolutionProposal {
    /// High-level label (e.g., "MicroEpochVisualFocusV1").
    pub protocol_id: String,
    /// Bounded expected effect norm (e.g., L2 delta bound).
    pub effect_l2_delta_norm: f32,
    /// Whether this proposal claims to be reversible.
    pub reversible: bool,
    /// Guest-provided justification.
    pub justification: String,
    /// Optional tags (e.g., research lane, HCI intent).
    pub tags: Vec<String>,
}

// -----------------------------
// Sovereign loader interface
// -----------------------------

/// Capabilities granted to guest swarms. The swarm implementation
/// must never see more than this surface.
pub trait GuestCyberswarm {
    /// Unique identifier for this swarm instance.
    fn id(&self) -> &str;

    /// Declared capabilities of this swarm (used for policy checks).
    fn declared_modes(&self) -> Vec<SwarmMode>;

    /// Main entrypoint: the sovereignty core calls this with a
    /// SovereignHostView and a mode token. The swarm may return
    /// zero or more bounded actions.
    fn handle_tick(
        &mut self,
        now: SystemTime,
        host_view: &SovereignHostView,
        mode: SwarmMode,
    ) -> Vec<SwarmAction>;
}

/// Sovereign loader / shell that mediates between OrganicCPU and
/// any guest cyberswarm implementation.
pub struct SovereignCyberswarmShell<'a, NE, SE, DW, TL> {
    pub neurorights_engine: &'a NE,
    pub stake_engine: &'a SE,
    pub donutloop_writer: &'a DW,
    pub token_ledger: &'a mut TL,
}

impl<'a, NE, SE, DW, TL> SovereignCyberswarmShell<'a, NE, SE, DW, TL>
where
    NE: NeuroRightsEngine,
    SE: StakeEngine,
    DW: DonutloopWriter,
    TL: TokenLedger,
{
    /// Enforce neurorights, RoH, stake, and token constraints around a
    /// single tick of the guest swarm.
    ///
    /// Returns: log entries and any accepted evolution proposals
    /// (in wrapped, sovereign form) that higher layers can route
    /// to OrganicCPU schedulers.
    pub fn run_swarm_tick<G>(
        &mut self,
        swarm: &mut G,
        now: SystemTime,
        host_state: &RoHGuardedHostState,
        host_view: &SovereignHostView,
        mode: SwarmMode,
    ) -> SwarmTickResult
    where
        G: GuestCyberswarm,
    {
        // 1. Neurorights gate: check that the requested mode is allowed
        // under current neurorights profile and context.
        let nr_verdict = self.neurorights_engine.check_swarm_mode(
            &host_view.neurorights_profile,
            swarm.id(),
            mode,
            host_view,
        );

        if !nr_verdict.allowed {
            let entry = DonutloopEntry::swarm_denied_mode(
                swarm.id().to_string(),
                mode,
                nr_verdict.reason.clone(),
                now,
            );
            self.donutloop_writer.append(entry);
            return SwarmTickResult::denied_by_neurorights(nr_verdict);
        }

        // 2. RoH gate: only allow SafeFilterPlusEvolution when RoH < 0.3
        // and monotone safety can be maintained.
        if matches!(mode, SwarmMode::SafeFilterPlusEvolution) {
            if !host_state.is_within_roh_ceiling() {
                let entry = DonutloopEntry::swarm_denied_roh(
                    swarm.id().to_string(),
                    host_view.roh_band,
                    now,
                );
                self.donutloop_writer.append(entry);
                return SwarmTickResult::denied_by_roh(host_view.roh_band);
            }
        }

        // 3. Stake/role gate: ensure swarm is allowed to operate in this
        // mode at all (e.g., only certain roles may touch evolution).
        let stake_verdict = self.stake_engine.check_swarm_mode(
            swarm.id(),
            mode,
            host_view,
        );

        if !stake_verdict.allowed {
            let entry = DonutloopEntry::swarm_denied_stake(
                swarm.id().to_string(),
                mode,
                stake_verdict.reason.clone(),
                now,
            );
            self.donutloop_writer.append(entry);
            return SwarmTickResult::denied_by_stake(stake_verdict);
        }

        // 4. Call into the guest swarm with only SovereignHostView and
        // mode. The swarm never sees raw telemetry or direct actuators.
        let actions = swarm.handle_tick(now, host_view, mode);

        // 5. Post-process actions: filter/encode evolutions, emit donutloop
        // logs, and apply token-level accounting.
        self.process_actions(swarm.id(), now, host_state, host_view, mode, actions)
    }

    fn process_actions(
        &mut self,
        swarm_id: &str,
        now: SystemTime,
        host_state: &RoHGuardedHostState,
        host_view: &SovereignHostView,
        mode: SwarmMode,
        actions: Vec<SwarmAction>,
    ) -> SwarmTickResult {
        let mut logs = Vec::new();
        let mut accepted_evolutions = Vec::new();

        for action in actions {
            match action {
                SwarmAction::RecommendInfo { reason, tags } => {
                    let entry = DonutloopEntry::swarm_recommend_info(
                        swarm_id.to_string(),
                        reason,
                        tags,
                        now,
                    );
                    self.donutloop_writer.append(entry.clone());
                    logs.push(entry);
                }
                SwarmAction::SuggestFilter {
                    target_id,
                    filter_kind,
                    reason,
                } => {
                    let entry = DonutloopEntry::swarm_suggest_filter(
                        swarm_id.to_string(),
                        target_id,
                        filter_kind,
                        reason,
                        now,
                    );
                    self.donutloop_writer.append(entry.clone());
                    logs.push(entry);
                }
                SwarmAction::ProposeEvolution { proposal } => {
                    if !matches!(mode, SwarmMode::SafeFilterPlusEvolution) {
                        let entry = DonutloopEntry::swarm_denied_evolution_wrong_mode(
                            swarm_id.to_string(),
                            proposal.protocol_id.clone(),
                            mode,
                            now,
                        );
                        self.donutloop_writer.append(entry.clone());
                        logs.push(entry);
                        continue;
                    }

                    // Wrap into a sovereign EvolutionProposalRecord-lite that
                    // still needs to go through the main sovereignty pipeline.
                    if let Some(wrapper) = SovereignEvolutionWrapper::from_swarm(
                        host_state,
                        host_view,
                        swarm_id,
                        proposal,
                        now,
                    ) {
                        // Token-level accounting: charge or earmark SMART/EVOLVE
                        // for this proposal, without finalizing anything yet.
                        self.token_ledger.reserve_for_proposal(
                            &wrapper.host_did,
                            TokenKind::Smart,
                            wrapper.estimated_smart_cost,
                        );

                        let entry = DonutloopEntry::swarm_proposed_evolution(
                            swarm_id.to_string(),
                            wrapper.protocol_id.clone(),
                            wrapper.roh_before,
                            wrapper.roh_after,
                            now,
                        );
                        self.donutloop_writer.append(entry.clone());
                        logs.push(entry);
                        accepted_evolutions.push(wrapper);
                    } else {
                        let entry = DonutloopEntry::swarm_denied_evolution_roh_guard(
                            swarm_id.to_string(),
                            host_view.roh_band,
                            now,
                        );
                        self.donutloop_writer.append(entry.clone());
                        logs.push(entry);
                    }
                }
            }
        }

        SwarmTickResult::accepted(logs, accepted_evolutions)
    }
}

// -----------------------------
// Sovereign evolution wrapper
// -----------------------------

/// Sovereignty-level representation of a swarm-originated evolution
/// suggestion. This is what your main sovereignty pipeline consumes.
#[derive(Clone, Debug)]
pub struct SovereignEvolutionWrapper {
    pub host_did: String,
    pub protocol_id: String,
    pub roh_before: f32,
    pub roh_after: f32,
    pub reversible: bool,
    pub justification: String,
    pub tags: Vec<String>,
    pub corridor_id: CorridorId,
    pub session_id: SessionId,
    pub estimated_smart_cost: u64,
}

impl SovereignEvolutionWrapper {
    /// Build a wrapper only if RoH monotonicity and ceiling are preserved.
    pub fn from_swarm(
        host_state: &RoHGuardedHostState,
        host_view: &SovereignHostView,
        swarm_id: &str,
        proposal: SwarmEvolutionProposal,
        _now: SystemTime,
    ) -> Option<Self> {
        // Use host_state methods to compute predicted RoH delta for this
        // protocol, based on your existing rohfrombiokarma or similar.
        let roh_before = host_state.current_roh();
        let roh_predicted = host_state.predict_roh_for_protocol(&proposal.protocol_id);

        // Enforce RoH monotone safety and ceiling 0.3.
        if roh_predicted > roh_before {
            return None;
        }
        if roh_predicted > 0.30 {
            return None;
        }

        Some(SovereignEvolutionWrapper {
            host_did: host_state.host_did().to_string(),
            protocol_id: proposal.protocol_id,
            roh_before,
            roh_after: roh_predicted,
            reversible: proposal.reversible,
            justification: format!(
                "swarm={} justification={}",
                swarm_id, proposal.justification
            ),
            tags: proposal.tags,
            corridor_id: host_view.corridor_id.clone(),
            session_id: host_view.session_id.clone(),
            estimated_smart_cost: 1, // placeholder, tune with real cost model
        })
    }
}

// -----------------------------
// Result type
// -----------------------------

#[derive(Clone, Debug)]
pub enum SwarmTickResult {
    DeniedByNeuroRights {
        verdict: NeuroRightsVerdict,
    },
    DeniedByRoH {
        roh_band: RoHBand,
    },
    DeniedByStake {
        verdict: StakeVerdict,
    },
    Accepted {
        logs: Vec<DonutloopEntry>,
        evolutions: Vec<SovereignEvolutionWrapper>,
    },
}

impl SwarmTickResult {
    pub fn denied_by_neurorights(verdict: NeuroRightsVerdict) -> Self {
        SwarmTickResult::DeniedByNeuroRights { verdict }
    }

    pub fn denied_by_roh(roh_band: RoHBand) -> Self {
        SwarmTickResult::DeniedByRoH { roh_band }
    }

    pub fn denied_by_stake(verdict: StakeVerdict) -> Self {
        SwarmTickResult::DeniedByStake { verdict }
    }

    pub fn accepted(
        logs: Vec<DonutloopEntry>,
        evolutions: Vec<SovereignEvolutionWrapper>,
    ) -> Self {
        SwarmTickResult::Accepted { logs, evolutions }
    }
}
