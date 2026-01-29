//! Neuro-biophysical blockchain automation core for Cybercore-Brain / Cyberswarm.
//!
//! This module defines a non-authoritarian, safety-aligned biophysical-blockchain
//! execution pipeline for:
//! - Automation cycles (closed-loop cybernetic control)
//! - Evolution paths (configurable, auditable adaptation steps)
//! - AI-chat anchored proposals (human-in-the-loop transhuman augmentation)
//!
//! Design goals:
//! - No "cheat" registry, no ethics bypass, no coercive / regime-like controls.
//! - Immutable but privacy-aware audit trails for neuro-biophysical interventions.
//! - Deterministic, consensus-validated state transitions.
//! - Clear separation between:
//!     * Proposal (AI chat / human design)
//!     * Validation (safety + biophysical constraints)
//!     * Consensus (swarm of nodes)
//!     * Actuation (NeuroPC / nanoswarm endpoints)

// 2024 edition-style crate features.
#![allow(clippy::too_many_arguments)]
#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

/// Globally unique identifier for any chain object.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChainId(String);

impl ChainId {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

/// Represents an AI / human chat-turn snapshot that proposes an evolution step.
#[derive(Clone, Debug)]
pub struct ChatContext {
    /// Hash of the full chat transcript (off-chain storage).
    pub transcript_hash: String,
    /// Short human-readable description of the proposal.
    pub summary: String,
    /// Model identifier (e.g., "GPT_nano_vondy.4.0" but semantics-neutral).
    pub model_id: String,
    /// Optional anonymized operator / team identifier.
    pub operator_tag: Option<String>,
}

/// Biophysical / neurocybernetic constraints that must be satisfied.
#[derive(Clone, Debug)]
pub struct BiophysicalConstraints {
    /// Maximum allowed modulation intensity (normalized 0.0 - 1.0).
    pub max_modulation_intensity: f32,
    /// Maximum allowed continuous application duration in seconds.
    pub max_duration_secs: u64,
    /// Allowed anatomical or system targets (e.g., "cortex.v1", "peripheral.haptics").
    pub allowed_targets: BTreeSet<String>,
    /// Disallowed targets (hard-block list).
    pub blocked_targets: BTreeSet<String>,
    /// Require explicit human confirmation for any irreversible change.
    pub require_irreversible_confirmation: bool,
}

/// Classification of intervention reversibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reversibility {
    FullyReversible,
    PartiallyReversible,
    Irreversible,
}

/// A concrete biophysical intervention pattern for a single cycle.
#[derive(Clone, Debug)]
pub struct BiophysicalPattern {
    /// Where this pattern is applied in the cyber-biophysical system.
    pub target: String,
    /// Normalized intensity 0.0 - 1.0 within safety spec.
    pub intensity: f32,
    /// Duration of this pattern application.
    pub duration: Duration,
    /// Reversibility classification for risk assessment.
    pub reversibility: Reversibility,
}

/// High-level category of an evolution step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvolutionCategory {
    PerformanceEnhancement,
    SensoryAugmentation,
    CognitiveScaffolding,
    Rehabilitation,
    Experimental,
}

/// Proposed evolution step coming from AI/human ideation.
#[derive(Clone, Debug)]
pub struct EvolutionProposal {
    pub id: ChainId,
    pub chat: ChatContext,
    pub category: EvolutionCategory,
    pub patterns: Vec<BiophysicalPattern>,
    /// Optional justification supplied by AI/human, e.g. link to literature.
    pub justification_uri: Option<String>,
}

/// Validation verdict for a proposal.
#[derive(Clone, Debug)]
pub struct ValidationResult {
    pub proposal_id: ChainId,
    pub accepted: bool,
    pub reasons: Vec<String>,
}

/// A logically grouped automation cycle.
/// One block may contain many cycles across devices / nanoswarms.
#[derive(Clone, Debug)]
pub struct AutomationCycle {
    pub cycle_id: ChainId,
    /// Timestamp for scheduling and ordering.
    pub scheduled_for: SystemTime,
    /// Biophysical patterns to apply in this cycle.
    pub patterns: Vec<BiophysicalPattern>,
    /// Associated evolution proposal (for traceability).
    pub proposal_ref: ChainId,
}

/// A single block in the biophysical blockchain.
#[derive(Clone, Debug)]
pub struct BioBlock {
    pub block_id: ChainId,
    pub parent_id: Option<ChainId>,
    pub height: u64,
    pub timestamp: SystemTime,
    /// Hash over the previous block and payload (implementation-specific).
    pub hash: String,
    /// Included automation cycles.
    pub automation_cycles: Vec<AutomationCycle>,
    /// Aggregate validation outcomes for referenced proposals.
    pub validations: Vec<ValidationResult>,
}

/// A node participating in consensus.
#[derive(Clone, Debug)]
pub struct NodeId(String);

impl NodeId {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}

/// Consensus vote over a candidate block.
#[derive(Clone, Debug)]
pub struct ConsensusVote {
    pub node: NodeId,
    pub block_id: ChainId,
    pub approve: bool,
    pub rationale: Option<String>,
}

/// Consensus result combining votes from a committee.
#[derive(Clone, Debug)]
pub struct ConsensusResult {
    pub block_id: ChainId,
    pub approvals: BTreeSet<NodeId>,
    pub rejections: BTreeSet<NodeId>,
    pub finalized: bool,
}

/// Error type for the pipeline.
#[derive(Debug)]
pub enum PipelineError {
    InvalidProposal(String),
    SafetyViolation(String),
    ConsensusRejected(String),
    ChainMutation(String),
}

impl Display for PipelineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineError::InvalidProposal(msg) => write!(f, "InvalidProposal: {msg}"),
            PipelineError::SafetyViolation(msg) => write!(f, "SafetyViolation: {msg}"),
            PipelineError::ConsensusRejected(msg) => write!(f, "ConsensusRejected: {msg}"),
            PipelineError::ChainMutation(msg) => write!(f, "ChainMutation: {msg}"),
        }
    }
}

impl std::error::Error for PipelineError {}

/// Simple in-memory bio-blockchain representation.
#[derive(Default)]
pub struct BioChain {
    blocks: Vec<BioBlock>,
}

impl BioChain {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn tip(&self) -> Option<&BioBlock> {
        self.blocks.last()
    }

    pub fn height(&self) -> u64 {
        self.blocks.last().map(|b| b.height).unwrap_or(0)
    }

    pub fn append_block(&mut self, block: BioBlock) -> Result<(), PipelineError> {
        if let Some(tip) = self.tip() {
            if Some(tip.block_id.clone()) != block.parent_id {
                return Err(PipelineError::ChainMutation(
                    "Parent mismatch when appending block".into(),
                ));
            }
            if block.height != tip.height + 1 {
                return Err(PipelineError::ChainMutation(
                    "Unexpected block height sequence".into(),
                ));
            }
        } else if block.parent_id.is_some() {
            return Err(PipelineError::ChainMutation(
                "Genesis block must not have a parent".into(),
            ));
        }
        self.blocks.push(block);
        Ok(())
    }
}

/// Core trait for validating evolution proposals against constraints.
pub trait ProposalValidator: Send + Sync {
    fn validate(
        &self,
        proposal: &EvolutionProposal,
        constraints: &BiophysicalConstraints,
    ) -> ValidationResult;
}

/// Core trait for achieving consensus over blocks.
pub trait ConsensusEngine: Send + Sync {
    fn gather_votes(&self, candidate: &BioBlock) -> ConsensusResult;
}

/// Core trait for dispatching automation cycles to NeuroPC / nanoswarm endpoints.
pub trait ActuatorBus: Send + Sync {
    fn dispatch_cycle(&self, cycle: &AutomationCycle) -> Result<(), PipelineError>;
}

/// Default safety-focused proposal validator.
pub struct DefaultProposalValidator;

impl DefaultProposalValidator {
    pub fn new() -> Self {
        Self
    }
}

impl ProposalValidator for DefaultProposalValidator {
    fn validate(
        &self,
        proposal: &EvolutionProposal,
        constraints: &BiophysicalConstraints,
    ) -> ValidationResult {
        let mut reasons = Vec::new();
        let mut accepted = true;

        for pattern in &proposal.patterns {
            if pattern.intensity < 0.0 || pattern.intensity > 1.0 {
                accepted = false;
                reasons.push(format!(
                    "Pattern intensity {} out of normalized [0.0, 1.0] range",
                    pattern.intensity
                ));
            }
            if pattern.intensity > constraints.max_modulation_intensity {
                accepted = false;
                reasons.push(format!(
                    "Pattern intensity {} exceeds max {}",
                    pattern.intensity, constraints.max_modulation_intensity
                ));
            }

            let secs = pattern.duration.as_secs();
            if secs > constraints.max_duration_secs {
                accepted = false;
                reasons.push(format!(
                    "Pattern duration {}s exceeds max {}s",
                    secs, constraints.max_duration_secs
                ));
            }

            if constraints.blocked_targets.contains(&pattern.target) {
                accepted = false;
                reasons.push(format!(
                    "Target '{}' is explicitly blocked by policy",
                    pattern.target
                ));
            }

            if !constraints.allowed_targets.is_empty()
                && !constraints.allowed_targets.contains(&pattern.target)
            {
                accepted = false;
                reasons.push(format!(
                    "Target '{}' not in allowed_targets set",
                    pattern.target
                ));
            }

            if constraints.require_irreversible_confirmation
                && pattern.reversibility == Reversibility::Irreversible
            {
                accepted = false;
                reasons.push(
                    "Irreversible pattern proposed without explicit confirmation token".into(),
                );
            }
        }

        // Domain-specific safeguard: Experimental + Irreversible is not allowed by default.
        if proposal.category == EvolutionCategory::Experimental {
            if proposal
                .patterns
                .iter()
                .any(|p| p.reversibility == Reversibility::Irreversible)
            {
                accepted = false;
                reasons.push(
                    "Experimental proposals may not include irreversible patterns by default"
                        .into(),
                );
            }
        }

        ValidationResult {
            proposal_id: proposal.id.clone(),
            accepted,
            reasons,
        }
    }
}

/// Simple majority consensus engine with node set.
pub struct MajorityConsensusEngine {
    nodes: Vec<NodeId>,
}

impl MajorityConsensusEngine {
    pub fn new(nodes: Vec<NodeId>) -> Self {
        Self { nodes }
    }

    fn simulate_vote(&self, _candidate: &BioBlock, node: &NodeId) -> ConsensusVote {
        // In a real system, each node will run full validation logic.
        // Here we default to approve, just recording the act.
        ConsensusVote {
            node: node.clone(),
            block_id: _candidate.block_id.clone(),
            approve: true,
            rationale: Some("Default-approve in majority engine simulation".into()),
        }
    }
}

impl ConsensusEngine for MajorityConsensusEngine {
    fn gather_votes(&self, candidate: &BioBlock) -> ConsensusResult {
        let mut approvals = BTreeSet::new();
        let mut rejections = BTreeSet::new();

        for node in &self.nodes {
            let vote = self.simulate_vote(candidate, node);
            if vote.approve {
                approvals.insert(vote.node);
            } else {
                rejections.insert(vote.node);
            }
        }

        let finalized = approvals.len() > rejections.len();

        ConsensusResult {
            block_id: candidate.block_id.clone(),
            approvals,
            rejections,
            finalized,
        }
    }
}

/// Simple logging-based actuator bus that would be replaced by a real NeuroPC bus.
pub struct LoggingActuatorBus;

impl LoggingActuatorBus {
    pub fn new() -> Self {
        Self
    }
}

impl ActuatorBus for LoggingActuatorBus {
    fn dispatch_cycle(&self, cycle: &AutomationCycle) -> Result<(), PipelineError> {
        // In production, send pattern set to actual nanoswarm/NeuroPC endpoints.
        println!(
            "[ActuatorBus] Dispatching cycle {} with {} patterns at {:?}",
            cycle.cycle_id.0,
            cycle.patterns.len(),
            cycle.scheduled_for
        );
        Ok(())
    }
}

/// High-level pipeline combining validation, consensus, chain mutation, and actuation.
pub struct NeuroAutomationPipeline {
    chain: Arc<RwLock<BioChain>>,
    validator: Arc<dyn ProposalValidator>,
    consensus: Arc<dyn ConsensusEngine>,
    actuator: Arc<dyn ActuatorBus>,
    constraints: BiophysicalConstraints,
}

impl NeuroAutomationPipeline {
    pub fn new(
        constraints: BiophysicalConstraints,
        validator: Arc<dyn ProposalValidator>,
        consensus: Arc<dyn ConsensusEngine>,
        actuator: Arc<dyn ActuatorBus>,
    ) -> Self {
        Self {
            chain: Arc::new(RwLock::new(BioChain::new())),
            validator,
            consensus,
            actuator,
            constraints,
        }
    }

    /// Accessor for external inspection of the chain.
    pub fn chain_snapshot(&self) -> BioChain {
        self.chain.read().unwrap().clone()
    }

    /// Full flow:
    /// 1) Validate proposal.
    /// 2) Map to automation cycle(s).
    /// 3) Mint candidate block.
    /// 4) Run consensus.
    /// 5) Append block.
    /// 6) Dispatch cycles.
    pub fn propose_and_commit(
        &self,
        proposal: EvolutionProposal,
        schedule_time: SystemTime,
    ) -> Result<BioBlock, PipelineError> {
        // Step 1: Validate.
        let validation = self.validator.validate(&proposal, &self.constraints);
        if !validation.accepted {
            return Err(PipelineError::SafetyViolation(format!(
                "Proposal {} failed validation: {:?}",
                proposal.id.0, validation.reasons
            )));
        }

        // Step 2: Map to automation cycle.
        let cycle = AutomationCycle {
            cycle_id: ChainId::new(format!("cycle-{}", proposal.id.0)),
            scheduled_for: schedule_time,
            patterns: proposal.patterns.clone(),
            proposal_ref: proposal.id.clone(),
        };

        // Step 3: Construct candidate block.
        let (parent_id, height) = {
            let chain = self.chain.read().unwrap();
            if let Some(tip) = chain.tip() {
                (Some(tip.block_id.clone()), tip.height + 1)
            } else {
                (None, 0)
            }
        };

        // Hash is simplified: just a formatted string; replace with cryptographic hash in production.
        let block_id = ChainId::new(format!("block-{}", height));
        let hash = format!(
            "hash(parent={:?},height={},cycles={},proposal={})",
            parent_id,
            height,
            1,
            proposal.id.0
        );

        let candidate = BioBlock {
            block_id: block_id.clone(),
            parent_id,
            height,
            timestamp: SystemTime::now(),
            hash,
            automation_cycles: vec![cycle.clone()],
            validations: vec![validation],
        };

        // Step 4: Consensus.
        let consensus_result = self.consensus.gather_votes(&candidate);
        if !consensus_result.finalized {
            return Err(PipelineError::ConsensusRejected(format!(
                "Block {} rejected: approvals={}, rejections={}",
                candidate.block_id.0,
                consensus_result.approvals.len(),
                consensus_result.rejections.len()
            )));
        }

        // Step 5: Append.
        {
            let mut chain = self.chain.write().unwrap();
            chain.append_block(candidate.clone())?;
        }

        // Step 6: Dispatch cycles.
        self.actuator.dispatch_cycle(&cycle)?;

        Ok(candidate)
    }
}

/// Example of constructing a safe, non-authoritarian configuration.
pub fn default_biophysical_constraints() -> BiophysicalConstraints {
    let mut allowed_targets = BTreeSet::new();
    allowed_targets.insert("cortex.visual.v1".into());
    allowed_targets.insert("peripheral.haptics.left_arm".into());
    allowed_targets.insert("peripheral.haptics.right_arm".into());

    let blocked_targets = BTreeSet::from(["brainstem.core".into(), "autonomic.heart".into()]);

    BiophysicalConstraints {
        max_modulation_intensity: 0.35,
        max_duration_secs: 90,
        allowed_targets,
        blocked_targets,
        require_irreversible_confirmation: true,
    }
}

/// Example bootstrapping function that could be called from main().
pub fn build_default_pipeline() -> NeuroAutomationPipeline {
    let constraints = default_biophysical_constraints();
    let validator: Arc<dyn ProposalValidator> = Arc::new(DefaultProposalValidator::new());

    let consensus_nodes = vec![
        NodeId::new("neuro-node-alpha"),
        NodeId::new("neuro-node-beta"),
        NodeId::new("neuro-node-gamma"),
    ];
    let consensus: Arc<dyn ConsensusEngine> =
        Arc::new(MajorityConsensusEngine::new(consensus_nodes));

    let actuator: Arc<dyn ActuatorBus> = Arc::new(LoggingActuatorBus::new());

    NeuroAutomationPipeline::new(constraints, validator, consensus, actuator)
}

/// Example of how an AI-chat proposal might be converted into an EvolutionProposal.
pub fn example_proposal_from_chat() -> EvolutionProposal {
    let chat = ChatContext {
        transcript_hash: "sha3_256:example-transcript-hash".into(),
        summary: "Mild visual cortex stimulation for augmented edge detection during AR tasks"
            .into(),
        model_id: "GPT_nano_vondy.4.0".into(),
        operator_tag: Some("neuro-ops-lab-A".into()),
    };

    let pattern = BiophysicalPattern {
        target: "cortex.visual.v1".into(),
        intensity: 0.25,
        duration: Duration::from_secs(45),
        reversibility: Reversibility::FullyReversible,
    };

    EvolutionProposal {
        id: ChainId::new("proposal-visual-aug-001"),
        chat,
        category: EvolutionCategory::SensoryAugmentation,
        patterns: vec![pattern],
        justification_uri: Some(
            "https://example.org/literature/visual-augmentation-biosafe-v1".into(),
        ),
    }
}

// Optional test to ensure basic pipeline wiring works.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_accepts_safe_proposal() {
        let pipeline = build_default_pipeline();
        let proposal = example_proposal_from_chat();
        let schedule_time = SystemTime::now();

        let result = pipeline.propose_and_commit(proposal, schedule_time);
        assert!(result.is_ok());
        let chain = pipeline.chain_snapshot();
        assert_eq!(chain.height(), 0);
        assert!(chain.tip().is_some());
    }

    #[test]
    fn pipeline_rejects_blocked_target() {
        let mut constraints = default_biophysical_constraints();
        // Make visual cortex blocked to force rejection.
        constraints.blocked_targets.insert("cortex.visual.v1".into());
        let validator: Arc<dyn ProposalValidator> = Arc::new(DefaultProposalValidator::new());
        let consensus: Arc<dyn ConsensusEngine> =
            Arc::new(MajorityConsensusEngine::new(vec![NodeId::new("node-1")]));
        let actuator: Arc<dyn ActuatorBus> = Arc::new(LoggingActuatorBus::new());
        let pipeline = NeuroAutomationPipeline::new(constraints, validator, consensus, actuator);

        let proposal = example_proposal_from_chat();
        let schedule_time = SystemTime::now();
        let result = pipeline.propose_and_commit(proposal, schedule_time);
        assert!(matches!(result, Err(PipelineError::SafetyViolation(_))));
    }
}
