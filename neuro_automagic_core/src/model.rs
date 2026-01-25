use core::fmt;

/// Platform where the intent is being applied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NeuroPlatform {
    NeuroPcKernel,
    NeuroPcUserland,
    RealityOs,
    ExternalTool,
}

/// Project-level scope (e.g., repository, workspace).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeuroProjectScope {
    pub name: heapless_string::HeaplessString,
    pub root_path: heapless_string::HeaplessString,
}

/// File-level scope (e.g., specific source file).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeuroFileScope {
    pub relative_path: heapless_string::HeaplessString,
    pub language_hint: Option<heapless_string::HeaplessString>,
}

/// Aggregate context for a given intent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeuroContext {
    pub platform: NeuroPlatform,
    pub project: Option<NeuroProjectScope>,
    pub file: Option<NeuroFileScope>,
}

/// Strongly-typed ID for an augmented-citizen.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NeuroCitizenId {
    /// External ID / address used by the user (e.g., Bostrom, DID).
    pub external_ref: heapless_string::HeaplessString,
}

/// Augmented-citizen description.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeuroCitizen {
    pub id: NeuroCitizenId,
    /// Whether this citizen is the primary augmented-citizen in this Space.
    pub is_primary_augmented_citizen: bool,
    /// Human-readable tag or label.
    pub label: heapless_string::HeaplessString,
}

/// High-level intent categories.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NeuroIntentKind {
    /// Navigate within code / projects.
    Navigate {
        target: heapless_string::HeaplessString,
    },
    /// Edit or refactor code.
    Edit {
        operation: heapless_string::HeaplessString,
    },
    /// Generate code, configuration, or documentation.
    Generate {
        artifact: heapless_string::HeaplessString,
    },
    /// Run commands (e.g., cargo, shell) in a controlled way.
    ExecuteCommand {
        command: heapless_string::HeaplessString,
    },
    /// Query information (e.g., “explain this file”, “show kernel config”).
    Query {
        subject: heapless_string::HeaplessString,
    },
}

/// Full neuro-intent structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeuroIntent {
    pub citizen: NeuroCitizen,
    pub context: NeuroContext,
    pub kind: NeuroIntentKind,
    /// Original text-like input from the user, for traceability.
    pub raw_input: heapless_string::HeaplessString,
}

/// Minimal heapless string abstraction.
/// In a real project, replace this with a concrete heapless or std-backed type.
pub mod heapless_string {
    use core::fmt;

    /// Simple wrapper around a fixed-size buffer String-like type.
    /// For `std` builds, this can just be `String`.
    #[cfg(feature = "std")]
    #[derive(Clone, Debug, Eq, PartialEq, Hash)]
    pub struct HeaplessString(pub String);

    #[cfg(feature = "std")]
    impl HeaplessString {
        pub fn new() -> Self {
            HeaplessString(String::new())
        }

        pub fn from_str(s: &str) -> Self {
            HeaplessString(s.to_owned())
        }
    }

    #[cfg(feature = "std")]
    impl fmt::Display for HeaplessString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    // For `no_std` targets, this module can be extended with an actual fixed-capacity string.
}
/// Helpers for constructing well-known NeuroCitizens for this Space.
impl NeuroCitizen {
    /// Primary augmented-citizen for this NeuroPC Space.
    pub fn primary_bostrom_augmented() -> Self {
        use crate::model::heapless_string::HeaplessString;

        NeuroCitizen {
            id: NeuroCitizenId {
                external_ref: HeaplessString::from_str(
                    "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7",
                ),
            },
            is_primary_augmented_citizen: true,
            label: HeaplessString::from_str("PrimaryAugmentedCitizen"),
        }
    }
}
