mod neuro_pc_min_core;
use neuro_pc_min_core::*;

fn main() {
    let policy = SafeEnvelopePolicy {
        limits: BioLimits {
            max_fatigue: 0.6,
            max_duty_cycle: 0.5,
            max_cognitive_load: 0.7,
        },
        min_intent_confidence: 0.7,
    };

    let samples = vec![
        BioState {
            fatigue_index: 0.3,
            duty_cycle: 0.2,
            cognitive_load_index: 0.4,
            intent_confidence: 0.9,
            eco: EcoMetrics {
                eco_impact_score: 0.8,
                device_hours: 3.0,
            },
        },
        BioState {
            fatigue_index: 0.7,
            duty_cycle: 0.6,
            cognitive_load_index: 0.8,
            intent_confidence: 0.9,
            eco: EcoMetrics {
                eco_impact_score: 0.9,
                device_hours: 7.0,
            },
        },
        BioState {
            fatigue_index: 0.4,
            duty_cycle: 0.3,
            cognitive_load_index: 0.5,
            intent_confidence: 0.5,
            eco: EcoMetrics {
                eco_impact_score: 0.85,
                device_hours: 4.0,
            },
        },
    ];

    for (i, s) in samples.iter().enumerate() {
        let decision = policy.decide(s);
        println!("tick {i}: state = {:?}, decision = {:?}", s, decision);
    }
}

main();
