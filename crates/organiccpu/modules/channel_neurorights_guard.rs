use std::collections::{HashMap, HashSet};

/// Enforces neurorights partition: blocks disallowed channel access
pub struct ChannelNeurorightsGuard {
    partitions: HashMap<String, ChannelPartition>,
}

#[derive(Clone)]
struct ChannelPartition {
    class_name: String,
    channel_range: (usize, usize),
    privacy_level: String,
    retention_policy: String,
    forbidden_operations: HashSet<String>,
}

impl ChannelNeurorightsGuard {
    pub fn new() -> Self {
        Self {
            partitions: HashMap::new(),
        }
    }

    /// Register a partition
    pub fn register_partition(&mut self, class: &str, start: usize, end: usize, privacy: &str, forbid: Vec<&str>) {
        self.partitions.insert(class.to_string(), ChannelPartition {
            class_name: class.to_string(),
            channel_range: (start, end),
            privacy_level: privacy.to_string(),
            retention_policy: String::new(),
            forbidden_operations: forbid.iter().map(|s| s.to_string()).collect(),
        });
    }

    /// Check if an operation is allowed on a channel
    pub fn check_operation_allowed(&self, channel_id: usize, operation: &str) -> Result<(), String> {
        for (_class, partition) in &self.partitions {
            let (start, end) = partition.channel_range;
            if channel_id >= start && channel_id <= end {
                if partition.forbidden_operations.contains(operation) {
                    return Err(format!(
                        "Operation '{}' forbidden on channel {} ({})",
                        operation, channel_id, partition.class_name
                    ));
                }
                return Ok(());
            }
        }
        Err(format!("Channel {} not in any partition", channel_id))
    }

    /// Batch check: Can we export this channel set?
    pub fn can_export_channels(&self, channels: &[usize]) -> Result<Vec<String>, String> {
        let mut exports = Vec::new();
        for &ch in channels {
            self.check_operation_allowed(ch, "export_to_external")?;
            for (_class, partition) in &self.partitions {
                let (start, end) = partition.channel_range;
                if ch >= start && ch <= end {
                    exports.push(partition.class_name.clone());
                    break;
                }
            }
        }
        Ok(exports)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cognitive_channels_forbidden_export() {
        let mut guard = ChannelNeurorightsGuard::new();
        guard.register_partition("cognitive_planning", 500, 799, "cognitive_liberty_protected", 
            vec!["export_to_external", "commercial_use", "dream_processing"]);

        // Channel 650 is in cognitive planning → export forbidden
        assert!(guard.check_operation_allowed(650, "export_to_external").is_err());
        assert!(guard.check_operation_allowed(650, "local_processing").is_ok());
    }

    #[test]
    fn test_motor_export_allowed() {
        let mut guard = ChannelNeurorightsGuard::new();
        guard.register_partition("motor_intent", 0, 299, "public_derivable", vec![]);

        // Channel 100 is motor → export allowed (assuming no blanket forbid)
        assert!(guard.check_operation_allowed(100, "export_intent_vector").is_ok());
    }
}
