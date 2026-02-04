use crate::donutloop::{DonutLoop};
use crate::hashcheck::{hash_canonical, verify_neurorights, verify_roh_invariants};
use crate::schema::{
    AuditEventPayload, EventType, NeurorightsDocument, RohModel, StakeConfig, UpdateProposal,
};
use ed25519_dalek::Keypair;
use std::fs;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SovereignKernel {
    pub roh_model: RohModel,
    pub stake: StakeConfig,
    pub neurorights: NeurorightsDocument,
    /// Raw JSONL of proposals; you will process them via Tsafe/CyberRank.
    pub evolve_proposals: Vec<UpdateProposal>,
    pub donutloop: DonutLoop,
    pub host_did: String,
}

fn load_rohmodel(path: &str) -> Result<RohModel, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read rohmodel: {e}"))?;
    // Treat .aln as YAML/JSON hybrid; adjust parser as needed.
    serde_yaml::from_str(&text).map_err(|e| format!("parse rohmodel YAML: {e}"))
}

fn load_stake(path: &str) -> Result<StakeConfig, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read stake: {e}"))?;
    serde_yaml::from_str(&text).map_err(|e| format!("parse stake YAML: {e}"))
}

fn load_neurorights(path: &str) -> Result<NeurorightsDocument, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read neurorights: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("parse neurorights JSON: {e}"))
}

fn load_evolve_jsonl(path: &str) -> Result<Vec<UpdateProposal>, String> {
    if !std::path::Path::new(path).exists() {
        return Ok(Vec::new());
    }
    let file = fs::File::open(path).map_err(|e| format!("open evolve.jsonl: {e}"))?;
    let reader = std::io::BufReader::new(file);
    let mut proposals = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| format!("read evolve.jsonl: {e}"))?;
        if line.trim().is_empty() {
            continue;
        }
        let p: UpdateProposal =
            serde_json::from_str(&line).map_err(|e| format!("parse UpdateProposal: {e}"))?;
        proposals.push(p);
    }
    Ok(proposals)
}

/// Search StakeConfig for Host role DID and enforce solo-host condition.
fn extract_host_did(stake: &StakeConfig) -> Result<String, String> {
    let hosts: Vec<_> = stake
        .roles
        .iter()
        .filter(|r| matches!(r.role, crate::schema::RoleKind::Host))
        .collect();
    if hosts.len() != 1 {
        return Err(format!(
            "stake.aln must define exactly one Host role, found {}",
            hosts.len()
        ));
    }
    Ok(hosts[0].did.clone())
}

/// Boot-time loader: fail fast if any constitutional file is invalid.
pub fn boot_sovereign_kernel(
    rohmodel_path: &str,
    stake_path: &str,
    neurorights_path: &str,
    evolve_path: &str,
    donutloop_path: &str,
    host_keypair: &Keypair,
) -> Result<SovereignKernel, String> {
    // 1. Load files.
    let roh_model = load_rohmodel(rohmodel_path)?;
    let stake = load_stake(stake_path)?;
    let neurorights = load_neurorights(neurorights_path)?;
    let evolve_proposals = load_evolve_jsonl(evolve_path)?;
    let mut donutloop = DonutLoop::load(donutloop_path)?;

    // 2. Verify invariants.
    verify_roh_invariants(&roh_model)?;
    verify_neurorights(&neurorights)?;
    let host_did = extract_host_did(&stake)?;

    // 3. Hex-stamped genesis/boot entry if ledger empty.
    if donutloop.entries.is_empty() {
        let payload = AuditEventPayload {
            roh_before: None,
            roh_after: None,
            knowledge_factor_before: None,
            knowledge_factor_after: None,
            cybostate_factor_before: None,
            cybostate_factor_after: None,
            reason: Some("Initialize sovereign kernel at boot".to_string()),
            reference_id: None,
        };
        let trace_id = Uuid::now_v7().to_string();
        donutloop.append_signed(
            donutloop_path,
            EventType::ArchitectureChangeApproved,
            payload,
            &host_did,
            host_keypair,
            Some(trace_id),
        )?;
    } else {
        // Verify no tampering before continuing.
        donutloop.verify_chain()?;
    }

    Ok(SovereignKernel {
        roh_model,
        stake,
        neurorights,
        evolve_proposals,
        donutloop,
        host_did,
    })
}
