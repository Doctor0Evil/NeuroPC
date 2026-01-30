#![forbid(unsafe_code)]

use chrono::{TimeZone, Utc};
use std::path::PathBuf;
use tempfile::TempDir;

use neuro_pc::evolution::controller::EvolutionController;
use neuro_pc::ota::controller::OtaController;
use neuro_pc::sovereignty::consent::{AwarenessToken, ConsentObject};
use neuro_pc::sovereignty::ota_io::{Caller, OtaAction, SovereignOtaIo};
use neuro_pc::sovereignty::policy::NrmlPolicy;
use neuro_pc::sovereignty::audit::AuditLogger;

fn make_policy(consent_dir: PathBuf, owner_id: &str) -> NrmlPolicy {
    NrmlPolicy {
        owner_id: owner_id.to_string(),
        trusted_ota_sources: vec!["https://trusted.vendor.example/ota".to_string()],
        sovereignty_core_enabled_flag: true,
        rollback_available_flag: true,
        user_control_channel_flag: true,
        consent_dir,
    }
}

#[test]
fn ota_and_evolve_require_explicit_consent() -> Result<(), Box<dyn std::error::Error>> {
    // Use a temp directory so tests don't touch real consent state.
    let tmp = TempDir::new()?;
    let consent_dir = tmp.path().join("consent");
    std::fs::create_dir_all(&consent_dir)?;

    let owner_id = "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7";

    // Policy + controllers.
    let policy = make_policy(consent_dir.clone(), owner_id);
    let mut logger = AuditLogger::new(tmp.path().join("audit.log"));
    let caller = Caller {
        module_id: "bf_ota_core".to_string(),
        instance_id: Some("instance_1".to_string()),
    };
    let mut io = SovereignOtaIo::new(&policy, &mut logger);
    let ota = OtaController::new(&policy);
    let evo = EvolutionController::new(&policy);

    let package_hash = "hash_ota_core_v2_abc123";

    // 1) OTA Commit with NO .cobj -> expect Err.
    let res_no_consent = ota.commit_package(&mut io, &caller, package_hash);
    assert!(res_no_consent.is_err(), "Commit without consent should fail");

    // 2) Add COMMIT .cobj, rerun -> expect Ok.
    let commit_cobj = ConsentObject {
        id: "cobj_ota_core_v2_commit".to_string(),
        token: AwarenessToken::Commit,
        target_id: package_hash.to_string(),
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

    let res_with_consent = ota.commit_package(&mut io, &caller, package_hash);
    assert!(res_with_consent.is_ok(), "Commit with consent should succeed");

    // 3) EVOLVE "bf_ota_core" with NO EVOLVE .cobj -> expect Err.
    let res_evo_no = evo.evolve_target("bf_ota_core");
    assert!(
        res_evo_no.is_err(),
        "Evolution without EVOLVE consent should fail"
    );

    // 4) Add EVOLVE .cobj, rerun -> expect Ok.
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

    let res_evo_yes = evo.evolve_target("bf_ota_core");
    assert!(
        res_evo_yes.is_ok(),
        "Evolution with EVOLVE consent should succeed"
    );

    // 5) Optional: simulate expired consent to confirm deny.
    let mut expired_cobj = commit_cobj;
    expired_cobj.valid_until = Some(
        Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
            .expect("valid past datetime"),
    );
    std::fs::write(
        &commit_path,
        serde_json::to_string_pretty(&expired_cobj)?,
    )?;

    let res_expired = ota.commit_package(&mut io, &caller, package_hash);
    assert!(
        res_expired.is_err(),
        "Commit with expired consent should fail"
    );

    Ok(())
}
