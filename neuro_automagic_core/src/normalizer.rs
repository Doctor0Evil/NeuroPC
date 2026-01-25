use crate::model::{
    NeuroIntent,
    NeuroIntentKind,
    NeuroContext,
    NeuroCitizen,
    heapless_string::HeaplessString,
};

/// A normalised intent representation ready for reproducible actions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedIntent {
    pub intent: NeuroIntent,
    /// Canonical, safe representation (e.g., validated cargo command).
    pub canonical_instruction: HeaplessString,
}

/// Trait: anything that can normalize raw text-like input into a NeuroIntent.
pub trait NeuroIntentNormalizer {
    fn normalize(
        &self,
        raw_input: &str,
        citizen: NeuroCitizen,
        context: NeuroContext,
    ) -> NormalizedIntent;
}

/// Example implementation can be provided in integration crates, not here.
/// This trait is meant to be implemented by higher layers (e.g., AI-chat,
/// editor plugin, or NeuroPC-specific normalizers).
