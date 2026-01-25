use neuro_automagic_core::{
    NeuroAutomationTrigger,
    NeuroIntent,
};

/// Simple repetition statistics.
#[derive(Clone, Debug)]
pub struct RepetitionStats {
    pub recent_count: u32,
}

/// Simple complexity statistics.
#[derive(Clone, Debug)]
pub struct ComplexityStats {
    pub token_count: u32,
    pub nested_level: u8,
}

/// Event emitted by higher layers to signal possible automagic assistance.
#[derive(Clone, Debug)]
pub struct NeuroAutomationEvent {
    pub trigger: NeuroAutomationTrigger,
    pub intent: NeuroIntent,
    pub repetition: Option<RepetitionStats>,
    pub complexity: Option<ComplexityStats>,
}
