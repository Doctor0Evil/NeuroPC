#![forbid(unsafe_code)]

use chrono::{TimeZone, Utc};
use std::path::PathBuf;

use neuro_pc::sovereignty::consent::{AwarenessToken, ConsentObject};
use neuro_pc::sovereignty::ota_io::{Caller, OtaAction};
use neuro_pc::sovereignty::policy::NrmlPolicy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test consent directory.
    let consent_dir = PathBuf::from("consent_test");
    std::fs::create_dir_all(&consent_dir)?;

    // Clean any previous files.
    for entry in std::fs::read_dir(&consent_dir)? {
        let entry = entry?;
        std::fs::remove_file(entry.path())?;
    }

    // Owner (you).
    let owner_id = "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7";

    // 1) Write a valid COMMIT .cobj for a specific package hash.
    let commit_cobj = ConsentObject {
        id: "cobj_ota_core_v2_commit".to_string(),
        token: AwarenessToken::Commit,
        target_id: "hash_ota_core_v2_abc123".to_string(),
        owner_id: owner_id.to_string(),
        created_at: Utc::now(),
        valid_until: Some(Utc::now() + chrono::Duration::hours(1)),
        revoked: false,
    };
    let commit_path = consent_dir.join("ota_commit_ota_core_v2.cobj");
    std::fs::write(
        &commit_path,
        serde_json::to_string_pretty(&commit_cobj)?,
    )?;

    // 2) Write a valid EVOLVE .cobj for evolving "bf_ota_core".
    let evolve_cobj = ConsentObject {
        id: "cobj_evolve_bf_ota_core".to_string(),
        token: AwarenessToken::Evolve,
        target_id: "bf_ota_core".to_string(),
        owner_id: owner_id.to_string(),
        created_at: Utc::now(),
        valid_until: Some(Utc::now() + chrono::Duration::hours(1)),
        revoked: false,
    };
    let evolve_path = consent_dir.join("evolve_bf_ota_core.cobj");
    std::fs::write(
        &evolve_path,
        serde_json::to_string_pretty(&evolve_cobj)?,
    )?;

    // Set up a policy instance.
    let policy = NrmlPolicy {
        owner_id: owner_id.to_string(),
        trusted_ota_sources: vec!["https://trusted.vendor.example/ota".to_string()],
        sovereignty_core_enabled_flag: true,
        rollback_available_flag: true,
        user_control_channel_flag: true,
        consent_dir: consent_dir.clone(),
    };

    // 3) Test OTA COMMIT consent: should be true.
    let caller = Caller {
        module_id: "bf_ota_core".to_string(),
        instance_id: Some("instance_1".to_string()),
    };
    let commit_action = OtaAction::Commit {
        package_hash: "hash_ota_core_v2_abc123".to_string(),
    };
    let has_commit_consent = policy.has_valid_ota_consent(&caller, &commit_action);
    println!(
        "Test 1: COMMIT consent (expect true) -> {}",
        has_commit_consent
    );

    // 4) Test OTA COMMIT with wrong hash: should be false.
    let commit_wrong = OtaAction::Commit {
        package_hash: "hash_wrong".to_string(),
    };
    let has_wrong = policy.has_valid_ota_consent(&caller, &commit_wrong);
    println!(
        "Test 2: COMMIT with wrong hash (expect false) -> {}",
        has_wrong
    );

    // 5) Test EVOLVE consent for bf_ota_core: should be true.
    let has_evolve = policy.has_valid_evolve_consent("bf_ota_core");
    println!(
        "Test 3: EVOLVE consent for bf_ota_core (expect true) -> {}",
        has_evolve
    );

    // 6) Test EVOLVE consent for some other target: should be false.
    let has_evolve_other = policy.has_valid_evolve_consent("sovereignty_core");
    println!(
        "Test 4: EVOLVE consent for sovereignty_core (expect false) -> {}",
        has_evolve_other
    );

    // 7) Simulate expired consent: modify file and re-check.
    let mut expired_cobj = commit_cobj;
    expired_cobj.valid_until = Some(Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap());
    std::fs::write(
        &commit_path,
        serde_json::to_string_pretty(&expired_cobj)?,
    )?;
    let has_expired = policy.has_valid_ota_consent(&caller, &commit_action);
    println!(
        "Test 5: COMMIT with expired consent (expect false) -> {}",
        has_expired
    );

    Ok(())
}
