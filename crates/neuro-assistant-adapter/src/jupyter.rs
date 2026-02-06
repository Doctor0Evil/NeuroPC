use alloc::string::String;
use serde::{Deserialize, Serialize};

use organiccpualn::promptenvelope::NeurorightsBoundPromptEnvelope;

use crate::{AssistantAdapter, AssistantAdapterConfig, AssistantAdapterError};
use sovereigntycore::chatguard::RightsBoundChatExecutor;
use crate::adapter::DonutloopAppender;

/// JSON contract for a Jupyter-side call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupyterRequest {
    pub subject_id: String,
    pub neurorights_profile_id: String,
    pub roh_model_id: String,
    pub domain_tags: Vec<String>,
    pub allowed_tools: Vec<String>,
    pub neurorights_doc_ref: String,
    pub token_id: Option<String>,
    pub prompt_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupyterResponse {
    pub answer_text: String,
    pub status: String,
}

/// Minimal, enforcement-centric bridge for Jupyter.
pub struct JupyterBridge<B, D>
where
    B: RightsBoundChatExecutor<Answer = String>,
    D: DonutloopAppender,
{
    pub adapter: AssistantAdapter<B, D>,
}

impl<B, D> JupyterBridge<B, D>
where
    B: RightsBoundChatExecutor<Answer = String>,
    D: DonutloopAppender,
{
    pub fn new(config: AssistantAdapterConfig, backend: B, donutloop: D) -> Self {
        Self {
            adapter: AssistantAdapter::new(config, backend, donutloop),
        }
    }

    pub fn handle_json(
        &self,
        req_json: &str,
    ) -> Result<String, AssistantAdapterError> {
        let req: JupyterRequest =
            serde_json::from_str(req_json).map_err(|e| AssistantAdapterError::Serde(e.to_string()))?;

        let env = NeurorightsBoundPromptEnvelope {
            subject_id: req.subject_id.clone(),
            neurorights_profile_id: req.neurorights_profile_id.clone(),
            roh_model_id: req.roh_model_id.clone(),
            domaintags: req.domain_tags.clone(),
            allowedtools: req.allowed_tools.clone(),
            neurorightsdocref: req.neurorights_doc_ref.clone(),
            token_id: req.token_id.clone(),
            prompttext: req.prompt_text.clone(),
        };

        let ans = self.adapter.handle_envelope(env)?;
        let resp = JupyterResponse {
            answer_text: ans.text,
            status: "ok".to_string(),
        };

        serde_json::to_string(&resp).map_err(|e| AssistantAdapterError::Serde(e.to_string()))
    }
}
