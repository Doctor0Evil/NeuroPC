use crate::model::{NeuroIntent, NeuroIntentKind, NeuroContext};
use crate::automation::{NeuroAutomationAction};
use crate::right::{NeuroRightSet, NeuroRight, NeuroAccessDecision, check_access};
use crate::model::heapless_string::HeaplessString;

/// Biocompatibility rating in [0,1].
pub const NPF_FOCUS_HINT_BCR: f32 = 0.22;

/// Simple struct describing a "focus window" in a file (start..end lines).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusWindow {
    pub file_path: HeaplessString,
    pub start_line: u32,
    pub end_line: u32,
}

/// Output of npf_focus_hint: optional window + a suggested action (hint only).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusHint {
    pub window: Option<FocusWindow>,
    pub action: Option<NeuroAutomationAction>,
}

/// New neuroprint-function (npf):
/// - Reads recent navigation intents in the same file.
/// - If it detects "bouncing", suggests a smaller focus window and a gentle hint.
/// - Never edits code; only suggests hints/macros.
pub fn npf_focus_hint(
    citizen_rights: &NeuroRightSet,
    context: &NeuroContext,
    recent_intents: &[NeuroIntent],
) -> FocusHint {
    // Guard: must at least be allowed to read project.
    let dummy_citizen = if let Some(intent) = recent_intents.last() {
        intent.citizen.clone()
    } else {
        return FocusHint { window: None, action: None };
    };

    if let NeuroAccessDecision::Denied =
        check_access(dummy_citizen.clone(), citizen_rights.clone(), NeuroRight::ReadProject)
    {
        return FocusHint { window: None, action: None };
    }

    // Only consider intents within a single file.
    let file_path = match &context.file {
        Some(f) => f.relative_path.clone(),
        None => return FocusHint { window: None, action: None },
    };

    // Heuristic: look at last N navigation intents, estimate "bounce span".
    const N: usize = 16;
    let mut line_min: Option<u32> = None;
    let mut line_max: Option<u32> = None;

    for intent in recent_intents.iter().rev().take(N) {
        if let NeuroIntentKind::Navigate { target } = &intent.kind {
            // Expect "line:NNN" markers from editor integration; ignore others.
            if let Some(rest) = target.as_str().strip_prefix("line:") {
                if let Ok(line) = rest.parse::<u32>() {
                    line_min = Some(line_min.map(|m| m.min(line)).unwrap_or(line));
                    line_max = Some(line_max.map(|m| m.max(line)).unwrap_or(line));
                }
            }
        }
    }

    // If we have no navigation info, no hint.
    let (line_min, line_max) = match (line_min, line_max) {
        (Some(a), Some(b)) => (a, b),
        _ => return FocusHint { window: None, action: None },
    };

    // If the span is already small, no hint needed.
    if line_max.saturating_sub(line_min) < 40 {
        return FocusHint { window: None, action: None };
    }

    // Shrink to a narrower "focus band" around the median.
    let mid = (line_min + line_max) / 2;
    let start_line = mid.saturating_sub(20);
    let end_line = mid + 20;

    let window = FocusWindow {
        file_path,
        start_line,
        end_line,
    };

    // Hint-only action for an editor integration to surface.
    let message = HeaplessString::from_str(
        "npf_focus_hint: consider focusing edits in this 40-line window for a few minutes.",
    );

    let action = NeuroAutomationAction::SuggestHigherLevelCommand;

    FocusHint {
        window: Some(window),
        action: Some(action),
    }
}
