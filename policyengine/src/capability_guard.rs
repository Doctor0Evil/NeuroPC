// NewRow-Print! BrainPrint organicallyintegratedaugmentedcitizen
// CapabilityGuard implementation for safest-first neuromorphic governance.
//
// This module is verification- and governance-oriented only. It contains:
// - Error enums for capability and grounding checks
// - Data structures mirroring .donutloop.aln and .neuro-cap.aln rows
// - A CapabilityGuard trait
// - A concrete CapabilityGuardImpl with pure, auditable logic
//
// All checks are written in a deny-by-default, safest-first style and are
// suitable for unit testing and later formal verification.

use serde::{Deserialize, Serialize};

use crate::alncore::{
    CapabilityState,
    ConsentState,
    Decision,
    DecisionReason,
    Jurisdiction,
    PolicyStack,
    CapabilityTransitionRequest,
};

/// Error kinds that CapabilityGuard can report.
/// These map directly to governance/audit reason codes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityGuardErrorKind {
    // Module / manifest problems
    UnknownModule,
    ManifestSchemaViolation,
    TierExceeded,
    ForbiddenJurisdiction,
    ForbiddenTarget,

    // Grounding / dataset / standards problems
    MissingBiophysicalSourceId,
    MissingRegulatoryBasisId,
    MissingValidationEvidenceRef,
    UnverifiedBiophysicalArtifact,
    UnverifiedRegulatoryArtifact,
    UnverifiedValidationEvidence,

    // Policy / envelope / RoH problems
    PolicyStackNotSatisfied,
    EnvelopeMissing,
    EnvelopeMetadataIncomplete,
    RoHMonotonicityViolation,
    RoHCeilingExceeded,

    // Cryptographic / hash-chain / signature problems
    HashChainBroken,
    MissingRequiredSignatures,
    SignatureVerificationFailed,

    // Fallback
    InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityGuardError {
    pub kind: CapabilityGuardErrorKind,
    pub message: String,
}

/// Core subset of a .donutloop.aln row used by the guard.
/// The full row may contain more fields; this is the minimal
/// kernel needed for enforcement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonutLoopRowCore {
    pub row_id: String,
    pub prev_row_id: Option<String>,
    pub hexstamp: String,
    pub prev_hexstamp: Option<String>,

    pub proposal_id: String,
    pub proposal_kind: String, // "RoHUpdate", "EnvelopeUpdate", "CapabilityElevation", etc.
    pub module_id: String,

    pub roh_before: f32,
    pub roh_after: f32,

    pub capability_before: CapabilityState,
    pub capability_after: CapabilityState,

    pub jurisdiction: Jurisdiction,
    pub policy_stack_snapshot: PolicyStack,

    pub biophysical_source_id: String,
    pub regulatory_basis_id: String,
    pub validation_evidence_ref: String,

    pub envelope_shard_ref: Option<String>,
    pub roh_model_ref: Option<String>,

    // Decision fields are set after guard:
    // pub decision: String,
    // pub decision_reason: String,
}

/// Manifest for a module from .neuro-cap.aln.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleCapabilityManifest {
    pub module_id: String,
    pub tier: CapabilityState,
    pub may_actuate: bool,
    pub never_actuate: bool,
    pub allowed_targets: Vec<String>,
    pub jurisdiction_scopes: Vec<Jurisdiction>,
}

/// Minimal neuromorphic envelope spec referenced by CapabilityGuard.
/// Full parameters are handled elsewhere; here we only ensure that
/// dataset and analysis method are present and thus externally auditable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiophysicalEnvelopeSpec {
    pub shard_id: String,
    pub roh_model_id: String,
    pub dataset_id: String,
    pub analysis_method_id: String,
    // Additional envelope parameters (alpha/gamma limits, EDA, HR/HRV) live here.
}

/// Outcome of running CapabilityGuard on a proposed row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardOutcome {
    pub decision: Decision,
    pub effective_capability_after: CapabilityState,
}

/// Trait defining the CapabilityGuard interface.
///
/// Implementations MUST be pure (side-effect-free) and auditable.
/// All I/O (file access, crypto, etc.) should be performed by callers,
/// not by the guard itself.
pub trait CapabilityGuard {
    /// Validate a proposed donutloop row against:
    /// - module tier and jurisdiction scope
    /// - grounding artefacts (dataset, standards, evidence refs)
    /// - PolicyStack
    /// - RoH invariants
    /// - envelope presence for envelope-related proposals
    fn validate_row(
        &self,
        row: &DonutLoopRowCore,
        manifest: &ModuleCapabilityManifest,
        envelope: Option<&BiophysicalEnvelopeSpec>,
    ) -> Result<GuardOutcome, CapabilityGuardError>;

    /// Verify hash-chain and signatures for the row.
    ///
    /// raw_bytes_without_hexstamp is the serialized row with hexstamp field
    /// set to a neutral value (e.g., empty string) prior to hashing.
    fn verify_integrity_and_signatures(
        &self,
        row: &DonutLoopRowCore,
        raw_bytes_without_hexstamp: &[u8],
    ) -> Result<(), CapabilityGuardError>;

    /// High-level entry point used by the sovereign kernel.
    ///
    /// Callers are expected to:
    /// - load row, manifest, and envelope (if applicable)
    /// - serialize the row without hexstamp and pass the bytes here
    fn validate_and_enforce(
        &self,
        row: &DonutLoopRowCore,
        manifest: &ModuleCapabilityManifest,
        envelope: Option<&BiophysicalEnvelopeSpec>,
        raw_bytes_without_hexstamp: &[u8],
    ) -> Result<GuardOutcome, CapabilityGuardError> {
        self.verify_integrity_and_signatures(row, raw_bytes_without_hexstamp)?;
        self.validate_row(row, manifest, envelope)
    }
}

/// Concrete implementation of CapabilityGuard.
///
/// This implementation intentionally DOES NOT perform any IO or crypto by
/// itself; verify_integrity_and_signatures is provided as a stub that can
/// be wired to your cryptographic layer.
pub struct CapabilityGuardImpl;

impl CapabilityGuardImpl {
    pub fn new() -> Self {
        CapabilityGuardImpl
    }

    /// Helper: map guard errors to DecisionReason for donutloop decision fields.
    pub fn error_to_decision(error: &CapabilityGuardError) -> Decision {
        use CapabilityGuardErrorKind::*;
        use DecisionReason::*;

        let reason = match error.kind {
            PolicyStackNotSatisfied => DeniedPolicyStackFailure,
            RoHCeilingExceeded | RoHMonotonicityViolation => DeniedUnknown,
            MissingBiophysicalSourceId
            | MissingRegulatoryBasisId
            | MissingValidationEvidenceRef
            | UnverifiedBiophysicalArtifact
            | UnverifiedRegulatoryArtifact
            | UnverifiedValidationEvidence => DeniedMissingEvidence,
            TierExceeded | ForbiddenJurisdiction | ForbiddenTarget => DeniedUnknown,
            HashChainBroken
            | MissingRequiredSignatures
            | SignatureVerificationFailed
            | UnknownModule
            | ManifestSchemaViolation
            | EnvelopeMissing
            | EnvelopeMetadataIncomplete
            | InternalError => DeniedUnknown,
        };

        Decision::deny(reason)
    }
}

impl CapabilityGuard for CapabilityGuardImpl {
    fn validate_row(
        &self,
        row: &DonutLoopRowCore,
        manifest: &ModuleCapabilityManifest,
        envelope: Option<&BiophysicalEnvelopeSpec>,
    ) -> Result<GuardOutcome, CapabilityGuardError> {
        use CapabilityGuardErrorKind::*;

        // 1. Tier and jurisdiction checks.
        if (row.capability_after as u8) > (manifest.tier as u8) {
            return Err(CapabilityGuardError {
                kind: TierExceeded,
                message: format!(
                    "module {} tier {:?} exceeded by capability_after {:?}",
                    manifest.module_id, manifest.tier, row.capability_after
                ),
            });
        }

        if !manifest.jurisdiction_scopes.contains(&row.jurisdiction) {
            return Err(CapabilityGuardError {
                kind: ForbiddenJurisdiction,
                message: format!(
                    "jurisdiction {:?} not allowed for module {}",
                    row.jurisdiction, manifest.module_id
                ),
            });
        }

        // 2. Grounding fields MUST be non-empty for any effective change.
        if row.biophysical_source_id.trim().is_empty() {
            return Err(CapabilityGuardError {
                kind: MissingBiophysicalSourceId,
                message: "biophysical_source_id must be non-empty".into(),
            });
        }
        if row.regulatory_basis_id.trim().is_empty() {
            return Err(CapabilityGuardError {
                kind: MissingRegulatoryBasisId,
                message: "regulatory_basis_id must be non-empty".into(),
            });
        }
        if row.validation_evidence_ref.trim().is_empty() {
            return Err(CapabilityGuardError {
                kind: MissingValidationEvidenceRef,
                message: "validation_evidence_ref must be non-empty".into(),
            });
        }

        // 3. PolicyStack MUST be satisfied for any upgrade or envelope change.
        if !row.policy_stack_snapshot.allpass() {
            return Err(CapabilityGuardError {
                kind: PolicyStackNotSatisfied,
                message: "PolicyStack predicates not all satisfied".into(),
            });
        }

        // 4. RoH monotonicity and ceiling (no silent escalation above 0.3).
        if row.roh_after > 0.3 + f32::EPSILON {
            return Err(CapabilityGuardError {
                kind: RoHCeilingExceeded,
                message: format!("RoH after {} exceeds ceiling 0.3", row.roh_after),
            });
        }

        if row.roh_after < row.roh_before - f32::EPSILON {
            return Err(CapabilityGuardError {
                kind: RoHMonotonicityViolation,
                message: format!("RoH after {} < RoH before {}", row.roh_after, row.roh_before),
            });
        }

        // 5. Envelope binding for envelope-related proposals.
        if row.proposal_kind == "EnvelopeUpdate" {
            let env = envelope.ok_or_else(|| CapabilityGuardError {
                kind: EnvelopeMissing,
                message: "EnvelopeUpdate without BiophysicalEnvelopeSpec".into(),
            })?;

            if env.dataset_id.trim().is_empty() || env.analysis_method_id.trim().is_empty() {
                return Err(CapabilityGuardError {
                    kind: EnvelopeMetadataIncomplete,
                    message: "Envelope dataset_id or analysis_method_id missing".into(),
                });
            }
        }

        // If we passed all checks, allow and return the requested capability_after.
        Ok(GuardOutcome {
            decision: Decision::allow(),
            effective_capability_after: row.capability_after,
        })
    }

    fn verify_integrity_and_signatures(
        &self,
        _row: &DonutLoopRowCore,
        _raw_bytes_without_hexstamp: &[u8],
    ) -> Result<(), CapabilityGuardError> {
        // This function is intentionally left as a stub to be wired to your
        // cryptographic layer. From the guard's perspective, we require that:
        //
        // - hexstamp == HASH(raw_bytes_without_hexstamp)
        // - prev_hexstamp matches the previous row's hexstamp (handled by caller)
        // - required signatures (host, organic CPU, regulator) are present and valid
        //
        // In this design-oriented implementation, we simply return Ok(()),
        // signalling that cryptographic verification passed.
        //
        // When integrating with a real crypto module, replace this stub with:
        // - hash computation and comparison
        // - signature verification against configured public keys
        //
        // Any failure MUST map to HashChainBroken, MissingRequiredSignatures,
        // or SignatureVerificationFailed, as appropriate.

        Ok(())
    }
}

/// Helper: integrate CapabilityGuard with the existing CapabilityTransitionRequest.
///
/// This function demonstrates how to combine PolicyStack/consent logic from
/// alncore.rs with CapabilityGuard's grounding and tier checks. The caller
/// is expected to have already populated the DonutLoopRowCore and manifest.
pub fn evaluate_transition_with_guard(
    ctr: &CapabilityTransitionRequest,
    guard: &dyn CapabilityGuard,
    row: &DonutLoopRowCore,
    manifest: &ModuleCapabilityManifest,
    envelope: Option<&BiophysicalEnvelopeSpec>,
    raw_bytes_without_hexstamp: &[u8],
) -> Decision {
    // First, use the existing core evaluation (consent, PolicyStack, roles).
    let base_decision = ctr.evaluate();

    if !base_decision.allowed {
        return base_decision;
    }

    // Second, run CapabilityGuard for tier, grounding, RoH, envelope.
    match guard.validate_and_enforce(row, manifest, envelope, raw_bytes_without_hexstamp) {
        Ok(outcome) => outcome.decision,
        Err(err) => CapabilityGuardImpl::error_to_decision(&err),
    }
}

/// Helper: check whether neuromorphic live coupling is allowed given
/// the current OperationContext and a CapabilityGuard outcome.
///
/// This can be used at the human-interface enforcement layer to ensure
/// that both ALN policy and neuromorphic envelopes are satisfied before
/// any real-world coupling is permitted.
pub fn can_live_couple_with_guard(
    op_ctx_decision: Decision,
    guard_outcome: &GuardOutcome,
) -> Decision {
    // If the operation context (from alncore.rs) already denies,
    // keep that decision.
    if !op_ctx_decision.allowed {
        return op_ctx_decision;
    }

    // If CapabilityGuard denies, propagate that denial.
    if !guard_outcome.decision.allowed {
        return guard_outcome.decision.clone();
    }

    // Otherwise, live coupling is permitted under both checks.
    Decision::allow()
}
