use serde::{Deserialize, Serialize};

use organiccpualn::rohmodel::{RohInputs, RohModelShard};
use organiccpualn::donutloop::DonutloopEntry;
use crate::riskofharm::RiskOfHarm;
use crate::stakegate::StakeGate;
use crate::smarttoken::{TokenKind, SmartTokenScope};
use crate::evolvestream::EvolutionProposal;

/// Marker for proposals that want to be treated as neuro round-ins.
pub const NEURO_ROUND_IN_KIND: &str = "NEURO_ROUND_IN";

/// Hard-coded mirror of qpudatashards/particles/neuro-round-in-v1.aln.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuroRoundInSpec {
    pub roh_max: f32,
    pub max_l2_delta: f32,
}

impl Default for NeuroRoundInSpec {
    fn default() -> Self {
        Self {
            roh_max: 0.30,
            max_l2_delta: 0.02,
        }
    }
}

impl NeuroRoundInSpec {
    pub fn is_neuro_round_in(&self, p: &EvolutionProposal) -> bool {
        p.kind == NEURO_ROUND_IN_KIND
    }

    pub fn check_bounds(&self, p: &EvolutionProposal) -> Result<(), String> {
        if !self.is_neuro_round_in(p) {
            return Ok(());
        }

        if p.effectbounds.l2deltanorm > self.max_l2_delta {
            return Err(format!(
                "NeuroRoundInEffectTooLarge: l2deltanorm={} > {}",
                p.effectbounds.l2deltanorm, self.max_l2_delta
            ));
        }

        if p.effectbounds.irreversible {
            return Err("NeuroRoundInIrreversibleNotAllowed".to_string());
        }

        if p.rohafter > p.rohbefore + 1e-6 {
            return Err(format!(
                "NeuroRoundInRoHNotMonotone: rohafter={} > rohbefore={}",
                p.rohafter, p.rohbefore
            ));
        }

        if p.rohafter > self.roh_max + 1e-6 {
            return Err(format!(
                "NeuroRoundInRoHExceedsMax: rohafter={} > roh_max={}",
                p.rohafter, self.roh_max
            ));
        }

        Ok(())
    }

    /// Optional: sanity check against live RoH model and state vectors.
    pub fn check_roh_model(
        &self,
        roh_model: &RohModelShard,
        before: RohInputs,
        after: RohInputs,
    ) -> Result<(), String> {
        let guard = RiskOfHarm::new(roh_model.clone());
        let roh_before = guard.estimate(before);
        let roh_after = guard.estimate(after);

        if roh_after > roh_before + 1e-6 {
            return Err("NeuroRoundInRoHModelNotMonotone".to_string());
        }
        if roh_after > self.roh_max + 1e-6 {
            return Err("NeuroRoundInRoHModelExceedsMax".to_string());
        }
        Ok(())
    }
}

/// Smart-token guard for neuro round-ins: SMART-only, no EVOLVE.
pub fn ensure_smart_only_scope(
    proposal: &EvolutionProposal,
    token_scope: &SmartTokenScope,
) -> Result<(), String> {
    if proposal.kind != NEURO_ROUND_IN_KIND {
        return Ok(());
    }

    if token_scope.kind != TokenKind::SMART {
        return Err("NeuroRoundInRequiresSMARTToken".to_string());
    }

    if token_scope
        .forbidden_domains
        .iter()
        .any(|d| d == "neuro_round_in")
    {
        return Err("NeuroRoundInForbiddenBySmartScope".to_string());
    }

    Ok(())
}

/// Donutloop append helper: enforces RoH monotone + Tsafe tag for NEURO_ROUND_IN.
pub fn append_neuro_round_in_entry(
    entry: &DonutloopEntry,
    spec: &NeuroRoundInSpec,
) -> Result<(), String> {
    if entry.changetype != NEURO_ROUND_IN_KIND {
        return Ok(());
    }

    if entry.rohafter > entry.rohbefore + 1e-6 {
        return Err("NeuroRoundInLedgerRoHNotMonotone".to_string());
    }

    if entry.rohafter > spec.roh_max + 1e-6 {
        return Err("NeuroRoundInLedgerRoHExceedsMax".to_string());
    }

    if !entry.tsafemode.starts_with("Tsafe") {
        return Err("NeuroRoundInTsafeModeMissingOrInvalid".to_string());
    }

    Ok(())
}

/// Integration hook inside sovereigntycore evaluate_update.
/// Called after RoH + neurorights + stake guards, before mutating any shards.
pub fn guard_neuro_round_in(
    spec: &NeuroRoundInSpec,
    stake_gate: &StakeGate,
    proposal: &EvolutionProposal,
    smart_scope: &SmartTokenScope,
) -> Result<(), String> {
    // Stake must already validate host DID, etc.
    stake_gate.verify_host()?;

    // SMART-only scope.
    ensure_smart_only_scope(proposal, smart_scope)?;

    // Static bounds and RoH fields.
    spec.check_bounds(proposal)
}
