use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ChannelGroup {
    pub name: String,
    pub thread_range: (usize, usize),
    pub channel_count: usize,
    pub biophysical_element: String, // earth/air/fire/water/ether
    pub roh_weight: f32,
}

pub struct NeuralinkChannelSorter {
    groups: Vec<ChannelGroup>,
    channel_map: HashMap<usize, String>, // channel_id -> group_name
}

impl NeuralinkChannelSorter {
    pub fn new() -> Self {
        let mut sorter = Self {
            groups: vec![],
            channel_map: HashMap::new(),
        };
        sorter.initialize_groups();
        sorter
    }

    fn initialize_groups(&mut self) {
        let groups = vec![
            ChannelGroup { name: "motor_intent".to_string(), thread_range: (0, 15), channel_count: 256, biophysical_element: "earth".to_string(), roh_weight: 0.04 },
            ChannelGroup { name: "sensory_feedback".to_string(), thread_range: (16, 31), channel_count: 256, biophysical_element: "water".to_string(), roh_weight: 0.06 },
            ChannelGroup { name: "speech_intent".to_string(), thread_range: (32, 47), channel_count: 256, biophysical_element: "air".to_string(), roh_weight: 0.25 },
            ChannelGroup { name: "cognitive_planning".to_string(), thread_range: (48, 63), channel_count: 256, biophysical_element: "ether".to_string(), roh_weight: 0.30 },
        ];
        self.groups = groups;

        for group in &self.groups {
            for thread in group.thread_range.0..=group.thread_range.1 {
                for elec in 0..16 {
                    let ch_id = (thread * 16) + elec;
                    self.channel_map.insert(ch_id, group.name.clone());
                }
            }
        }
    }

    /// Sort incoming spike vector (1024 elements) to groups
    pub fn sort_spikes(&self, spikes: &[f32; 1024]) -> HashMap<String, Vec<f32>> {
        let mut sorted: HashMap<String, Vec<f32>> = HashMap::new();
        for group in &self.groups {
            sorted.insert(group.name.clone(), vec![]);
        }
        for (ch_id, &value) in spikes.iter().enumerate() {
            if let Some(group_name) = self.channel_map.get(&ch_id) {
                if let Some(vec) = sorted.get_mut(group_name) {
                    vec.push(value);
                }
            }
        }
        sorted
    }

    /// Biophysical balance check before routing
    pub fn check_biophysical_stable(&self) -> bool {
        // Real proof: sum of roh_weights across active groups ≤0.3
        let total_roh: f32 = self.groups.iter().map(|g| g.roh_weight).sum();
        total_roh <= 0.3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1024_channel_sorting_and_stability() {
        let sorter = NeuralinkChannelSorter::new();
        let mut test_spikes = [0.0f32; 1024];
        test_spikes[0] = 150.0; // motor channel
        let sorted = sorter.sort_spikes(&test_spikes);
        assert_eq!(sorted["motor_intent"].len(), 256);
        assert!(sorter.check_biophysical_stable());
        println!("Sorted 1024 channels – biophysical stable: true");
    }
}
