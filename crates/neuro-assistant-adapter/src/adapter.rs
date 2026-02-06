use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

use serde::{Deserialize, Serialize};

use organiccpualn::promptenvelope::NeurorightsBoundPromptEnvelope;
use organiccpualn::evolvestream::{EffectBounds, EvolutionProposalRecord, JsonlEvolutionLog, EvolutionLogWriter};
use organiccpualn::donutloopledger::DonutloopEntry;

use neurorights_core::{NeurorightsPolicyDocument, ToolCapability, guard_prompt_tool};
use neurorights_firewall::{NeurorightsSafeBackend, FirewallError, process_prompt};

use sovereigntycore::chatguard::{ChatFitness, RightsBoundChatExecutor};

use crate::config::AssistantAdapterConfig;
use crate::error::AssistantAdapterError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssistantAnswer {
    /// Raw assistant text answer.
    pub text: String,
    /// Fitness and RoH estimation coming from the backend.
    pub fitness: ChatFitness,
}

/// Trait for anything that can append to donutloop.
/// You can implement this inside sovereigntycore or as a thin wrapper.
pub trait DonutloopAppender {
    fn append_entry(&self, entry: &DonutloopEntry) -> Result<(), AssistantAdapterError>;
}

/// The core adapter: envelopes in, answers + logs out.
pub struct AssistantAdapter<B, D>
where
    B: RightsBoundChatExecutor<Answer = String>,
    D: DonutloopAppender,
{
    pub config: AssistantAdapterConfig,
    pub backend: B,
    pub donutloop: D,
}

impl<B, D> AssistantAdapter<B, D>
where
    B: RightsBoundChatExecutor<Answer = String>,
    D: DonutloopAppender,
{
    pub fn new(config: AssistantAdapterConfig, backend: B, donutloop: D) -> Self {
        Self { config, backend, donutloop }
    }

    /// Main entry: enforce neurorights + RoH/cybostate via backend guard,
    /// then log into evolution-proposals.evolve.jsonl and donutloop ledger.
    pub fn handle_envelope(
        &self,
        envelope: NeurorightsBoundPromptEnvelope,
    ) -> Result<AssistantAnswer, AssistantAdapterError> {
        // Domain guard: adapter-level allowlist.
        if !envelope.domaintags.iter().any(|t| self.config.allowed_domains.contains(t)) {
            return Err(AssistantAdapterError::DomainNotAllowed(
                envelope.domaintags.join(","),
            ));
        }

        // Load neurorights policy.
        let policy = self.load_policy(&self.config.neurorights_policy_path)?;

        // Map config -> ToolCapability.
        let tool = self.tool_capability_from_config();

        // Wrap backend into a NeurorightsSafeBackend facade.
        let safe_backend = BackendFacade {
            backend: &self.backend,
        };

        // Run neurorights firewall + backend.
        let response_text = match process_prompt(safe_backend, envelope.clone(), policy, tool) {
            Ok(r) => r,
            Err(FirewallError::Neurorights(e)) => return Err(AssistantAdapterError::Neurorights(e.to_string())),
            Err(FirewallError::Backend(e)) => return Err(AssistantAdapterError::Backend(e)),
        };

        // Backend should expose ChatFitness along with text; here we assume
        // a second call or a side-channel; for now, we synthesize minimal fitness.
        // If your RightsBoundChatExecutor already returns (String, ChatFitness),
        // adjust this to use the real value.
        let fitness = ChatFitness {
            fscore: 1.0,
            fitness: "ok".to_string(),
            roh: 0.0,
            cybostate: sovereigntycore::chatguard::CybostateClass::ObservationOnly,
        };

        // Emit evolution + donutloop entries for audit (non-mutating, informational).
        self.append_evolve_record(&envelope, &response_text, &fitness)?;
        self.append_donutloop_entry(&envelope, &response_text, &fitness)?;

        Ok(AssistantAnswer {
            text: response_text,
            fitness,
        })
    }

    fn load_policy(&self, path: &str) -> Result<NeurorightsPolicyDocument, AssistantAdapterError> {
        #[cfg(feature = "std")]
        {
            use std::fs::File;
            use std::io::Read;

            let mut f = File::open(path).map_err(|e| AssistantAdapterError::Io(e.to_string()))?;
            let mut buf = String::new();
            f.read_to_string(&mut buf)
                .map_err(|e| AssistantAdapterError::Io(e.to_string()))?;
            serde_json::from_str(&buf).map_err(|e| AssistantAdapterError::Serde(e.to_string()))
        }

        #[cfg(not(feature = "std"))]
        {
            let _ = path;
            Err(AssistantAdapterError::Config(
                "load_policy requires std feature".to_string(),
            ))
        }
    }

    fn tool_capability_from_config(&self) -> ToolCapability {
        ToolCapability {
            tool_id: self.config.tool_id.clone(),
            can_read_inner_state: false,
            can_score_user: false,
            can_write_long_term_profile: false,
            eco_cost_estimate: 0.2,
        }
    }

    fn append_evolve_record(
        &self,
        envelope: &NeurorightsBoundPromptEnvelope,
        answer: &str,
        fitness: &ChatFitness,
    ) -> Result<(), AssistantAdapterError> {
        #[cfg(feature = "std")]
        {
            use std::fs::OpenOptions;
            use std::io::BufWriter;

            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.config.evolve_log_path)
                .map_err(|e| AssistantAdapterError::Io(e.to_string()))?;
            let mut writer = BufWriter::new(file);

            let rec = EvolutionProposalRecord {
                proposal_id: format!("assist-{}", chrono::Utc::now().timestamp_millis()),
                subject_id: envelope.subject_id.clone(),
                kind: "AssistantAnswer".to_string(),
                module: "neuro-assistant-adapter".to_string(),
                update_kind: "None".to_string(),
                effect_bounds: EffectBounds {
                    l2_delta_norm: 0.0,
                    irreversible: false,
                },
                roh_before: fitness.roh,
                roh_after: fitness.roh,
                tsafe_mode: "Observe".to_string(),
                domain_tags: envelope.domaintags.clone(),
                decision: "Observed".to_string(),
                hexstamp: "0xNP0E".to_string(), // placeholder; real hexstamp wired elsewhere
                timestamp_utc: chrono::Utc::now().to_rfc3339(),
                prompt_envelope_id: None,
                neurorights_profile_id: Some(envelope.neurorights_profile_id.clone()),
                token_id: envelope.token_id.clone(),
            };

            let log = JsonlEvolutionLog;
            log.append(&mut writer, rec)
                .map_err(|e| AssistantAdapterError::Io(e.to_string()))
        }

        #[cfg(not(feature = "std"))]
        {
            let _ = (envelope, answer, fitness);
            Ok(())
        }
    }

    fn append_donutloop_entry(
        &self,
        envelope: &NeurorightsBoundPromptEnvelope,
        _answer: &str,
        fitness: &ChatFitness,
    ) -> Result<(), AssistantAdapterError> {
        let entry = DonutloopEntry {
            entry_id: format!("assist-{}", envelope.subject_id),
            subject_id: envelope.subject_id.clone(),
            proposal_id: "n/a".to_string(),
            change_type: "AssistantAnswer".to_string(),
            tsafe_mode: "Observe".to_string(),
            roh_before: fitness.roh,
            roh_after: fitness.roh,
            knowledge_factor: 0.0,
            cybostate_factor: 0.0,
            policy_refs: "policies/bostrom-neurorights-v1.neurorights.json".to_string(),
            hexstamp: "0xNP0E".to_string(),
            timestamp_utc: chrono::Utc::now().to_rfc3339(),
            prev_hexstamp: "0x0".to_string(),
        };

        self.donutloop
            .append_entry(&entry)
            .map_err(|e| AssistantAdapterError::Backend(e.to_string()))
    }
}

/// Lightweight facade to adapt a RightsBoundChatExecutor backend to
/// NeurorightsSafeBackend.
struct BackendFacade<'a, B>
where
    B: RightsBoundChatExecutor<Answer = String>,
{
    pub backend: &'a B,
}

impl<'a, B> neurorights_firewall::NeurorightsSafeBackend for BackendFacade<'a, B>
where
    B: RightsBoundChatExecutor<Answer = String>,
{
    type Response = String;

    fn handle_envelope(
        &self,
        envelope: NeurorightsBoundPromptEnvelope,
    ) -> Result<Self::Response, String> {
        self.backend
            .execute_guarded(envelope)
            .map_err(|e| e.to_string())
    }
}
