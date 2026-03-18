use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::sovereignty_core::{SovereigntyCore, BiophysicalStateReader, StateVector, UpdateProposal, UpdateEffectBounds, UpdateKind, AuditEntry, DecisionOutcome};

/// NeuroPC .neurox file format – 5-D manifold binding + sovereign commitment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexusEndpointMapping {
    pub endpoint_id: String,                    // e.g. "bci-stream-frontal" or "chipset-core-A77-0"
    pub manifold_point: [f32; 5],               // (x,y,z region, t scale, φ phase/energy)
    pub component_type: String,                 // "bci_stream" | "organic_component" | "cybernetic_chipset"
    pub chipset_cores: Vec<CoreSpec>,           // your exact mt6883 organic config
    pub allowed_scopes: Vec<String>,            // "discover_only" | "ota_route" | "envelope_read"
    pub commitment: String,                     // hex commitment of (manifold_point || state_hash)
    pub reversal_conditions: Vec<String>,       // from your bioscale! macros
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSpec {
    pub core_type: String,  // "A77" | "A76" | "A74"
    pub count: u32,
    pub max_duty: f32,      // EnvelopePace bound per core
}

/// Secure discoverability & OTA routing engine – must be called after SovereigntyCore::evaluate_update
pub struct NexusEndpointMapper<S: BiophysicalStateReader> {
    mappings: HashMap<String, NexusEndpointMapping>,
    sovereignty: SovereigntyCore<S>,
}

impl<S: BiophysicalStateReader> NexusEndpointMapper<S> {
    pub fn new(sovereignty: SovereigntyCore<S>) -> Self {
        Self { mappings: HashMap::new(), sovereignty }
    }

    /// Load a .neurox file (production: use std::fs in your organic_cpu context)
    pub fn load_mapping(&mut self, mapping: NexusEndpointMapping) {
        self.mappings.insert(mapping.endpoint_id.clone(), mapping);
    }

    /// Secure discover – returns only corridor-safe scalars or rejects
    pub fn discover(&mut self, endpoint_id: &str, evolve_token_id: Option<&str>) -> Option<DiscoveryResult> {
        let Some(map) = self.mappings.get(endpoint_id) else { return None; };

        let proposal = UpdateProposal {
            id: format!("discover-{}", endpoint_id),
            module: "nexus_mapper".to_string(),
            kind: UpdateKind::ParamNudge,
            scope: vec!["discover_only".to_string()],
            description: format!("Discover endpoint {} (BCI/chipset config)", endpoint_id),
            effect_bounds: UpdateEffectBounds { l2_delta_norm: 0.0, irreversible: false },
            requires_evolve: false,
        };

        let audit = self.sovereignty.evaluate_update(&proposal, evolve_token_id);
        if !matches!(audit.decision, DecisionOutcome::Allowed) {
            return None;
        }

        // Biophysical 5-D commitment check (mathematically: ||p||_2 ≤ RoH bound)
        let norm: f32 = map.manifold_point.iter().map(|v| v * v).sum::<f32>().sqrt();
        if norm > 0.3 { return None; }  // RoH ≤ 0.3 enforced

        Some(DiscoveryResult {
            endpoint_id: map.endpoint_id.clone(),
            safe_scalars: SafeScalars {
                roh: 0.3 - norm,
                lifeforce_band: map.manifold_point[4],
                envelope_pace_window: map.chipset_cores.iter().map(|c| c.max_duty).sum(),
                neurorights_compliant: true,
            },
            chipset_summary: map.chipset_cores.clone(),
        })
    }

    /// OTA route through NOD – only after full neurorights + EnvelopePace + pain envelope
    pub fn route_ota(&mut self, endpoint_id: &str, upgrade_descriptor: UpgradeDescriptor, evolve_token_id: Option<&str>) -> AuditEntry {
        let proposal = UpdateProposal {
            id: format!("ota-route-{}", endpoint_id),
            module: "nexus_mapper".to_string(),
            kind: UpdateKind::RoutingChange,
            scope: vec!["ota_route".to_string()],
            description: format!("Route OTA evolution to BCI-stream/{} via nexus", endpoint_id),
            effect_bounds: UpdateEffectBounds { l2_delta_norm: upgrade_descriptor.effect_size, irreversible: false },
            requires_evolve: true,
        };

        let mut audit = self.sovereignty.evaluate_update(&proposal, evolve_token_id);
        if matches!(audit.decision, DecisionOutcome::Allowed) {
            // Biophysical-blockchain commit (append-only, no rollback unless ReversalConditions fire)
            if let Some(map) = self.mappings.get_mut(endpoint_id) {
                map.commitment = format!("0xNEUROX{:x}", upgrade_descriptor.evidence_bundle_hash); // custom neuro-hash
            }
        }
        audit
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryResult {
    pub endpoint_id: String,
    pub safe_scalars: SafeScalars,
    pub chipset_summary: Vec<CoreSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeScalars {
    pub roh: f32,
    pub lifeforce_band: f32,
    pub envelope_pace_window: f32,
    pub neurorights_compliant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeDescriptor {
    pub effect_size: f32,
    pub evidence_bundle_hash: u64,  // 10-tag bundle commitment
}

/// Your exact cybernetic-chipset organic abstraction (mt6883 config you provided)
pub fn default_chipset_mapping() -> NexusEndpointMapping {
    NexusEndpointMapping {
        endpoint_id: "cybernetic-chipset-mt6883".to_string(),
        manifold_point: [0.0, 0.0, 0.0, 0.0, 0.25],  // φ = 0.25 (energetic baseline)
        component_type: "cybernetic_chipset".to_string(),
        chipset_cores: vec![
            CoreSpec { core_type: "A77".to_string(), count: 4, max_duty: 0.35 },
            CoreSpec { core_type: "A76".to_string(), count: 2, max_duty: 0.30 },
            CoreSpec { core_type: "A74".to_string(), count: 2, max_duty: 0.25 },
        ],
        allowed_scopes: vec!["discover_only".to_string(), "ota_route".to_string()],
        commitment: "0xNEUROXINITIAL".to_string(),
        reversal_conditions: vec!["pain_envelope_exceeded".to_string(), "hrv_lf_hf_below_0.8".to_string()],
    }
}
