use serde::{Deserialize, Serialize};

/// Neurorights-bound wrapper around any prompt/text payload.
/// All backends must accept this type, never a bare String.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsBoundPromptEnvelope {
    /// Host DID / Bostrom address.
    pub subjectid: String, // e.g. "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7"

    /// Neurorights policy document ID in your policies dir.
    /// e.g. "bostrom-neurorights-v1".
    pub neurorights_profile_id: String,

    /// RoH model ID to be used for any risk estimation on this request.
    /// e.g. "bostrom-rohmodel-v1".
    pub roh_model_id: String,

    /// High-level domain tags, used for neurorights / firewall checks:
    /// e.g. ["languagecowriter","dreamobserver","sensitive-inner-state"].
    pub domain_tags: Vec<String>,

    /// Allowed tools / modules for this envelope, by module ID.
    /// e.g. ["languagecowriter","motormacros","organiccpuqlearn-advisor"].
    pub allowed_tools: Vec<String>,

    /// Reference to neurorights ALN or JSON policy file path or ID.
    /// This is what neurorights-core loads to enforce constraints.
    pub neurorights_doc_ref: String, // e.g. "policies/bostrom-neurorights-v1.neurorights.json"

    /// Optional SMART or EVOLVE token ID authorizing higher-impact work.
    pub token_id: Option<String>, // e.g. "SMART-2026-02-01-UI"

    /// Raw user/system prompt content, or already-normalized text.
    /// Never processed without passing through neurorights-core guards.
    pub prompt_text: String,
}
