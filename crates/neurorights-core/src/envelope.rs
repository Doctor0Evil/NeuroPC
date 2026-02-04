use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsBoundPromptEnvelope {
    /// Subject / augmented-citizen DID or Bostrom address.
    pub subject_id: String,              // e.g. "bostrom18sd2u..."
    /// Stable neurorights policy profile ID for this subject/session.
    pub neurorights_profile_id: String,  // e.g. "bostrom-neurorights-v1"
    /// Risk-of-harm model identifier to use for this call.
    pub roh_model_id: String,            // e.g. "bostrom-rohmodel-v1"
    /// High-level domain tags: "legal", "medical", "dreamstate", "devtools", etc.
    pub domain_tags: Vec<String>,
    /// Allowed tool IDs (must map to a curated, policy-checked registry).
    pub allowed_tools: Vec<String>,
    /// Cybostate gate for this request (retrieval-only, research-ready, etc.).
    pub cybostate: CybostateClass,
    /// Original user text; never sent to ungoverned paths.
    pub prompt_text: String,
}

/// Trust / actuation class for the current request.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CybostateClass {
    RetrievalOnly,
    ResearchReady,
    GovernanceReady,
    ActuationForbidden,
}
