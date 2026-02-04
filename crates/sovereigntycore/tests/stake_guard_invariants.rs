use sovereigntycore::stake::{ScopeKind, StakeRow, StakeTable};
use sovereigntycore::core_stake_guard::StakeGuard;

fn example_table() -> StakeTable {
    use std::collections::HashMap;

    let mut by_subject = HashMap::new();
    by_subject.insert(
        "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
        vec![
            StakeRow {
                roleid: "hostprimary".to_string(),
                subjectid: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
                bostromaddress:
                    "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
                rolekind: "Host".to_string(),
                canveto: true,
                caninitevolve: true,
                requiredforlifeforce: true,
                requiredforarchchange: true,
            },
            StakeRow {
                roleid: "organiccpu".to_string(),
                subjectid: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
                bostromaddress: "zeta12x0up66pzyeretzyku8p4ccuxrjqtqpdc4y4x8".to_string(),
                rolekind: "OrganicCPU".to_string(),
                canveto: true,
                caninitevolve: true,
                requiredforlifeforce: true,
                requiredforarchchange: true,
            },
        ],
    );

    // Bypass file loader invariants by constructing directly.
    StakeTable { by_subject }
}

#[derive(Clone)]
struct DummyProposal {
    subjectid: String,
    scope: Vec<String>,
    signers: Vec<String>,
}

#[test]
fn lifeforce_requires_host_and_organiccpu() {
    let table = example_table();
    let guard = StakeGuard::new(table);

    let subject =
        "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string();

    // Missing OrganicCPU signature: must fail.
    let p_missing_organic = DummyProposal {
        subjectid: subject.clone(),
        scope: vec!["lifeforce".to_string()],
        signers: vec![
            "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
        ],
    };

    let res = guard.enforce(&to_update(p_missing_organic));
    assert!(res.is_err());

    // Both Host + OrganicCPU present: must pass.
    let p_ok = DummyProposal {
        subjectid: subject.clone(),
        scope: vec!["lifeforce".to_string()],
        signers: vec![
            "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
            "zeta12x0up66pzyeretzyku8p4ccuxrjqtqpdc4y4x8".to_string(),
        ],
    };

    let res_ok = guard.enforce(&to_update(p_ok));
    assert!(res_ok.is_ok());
}

fn to_update(p: DummyProposal) -> sovereigntycore::update::UpdateProposal {
    sovereigntycore::update::UpdateProposal {
        id: "test-proposal".to_string(),
        subjectid: p.subjectid,
        scope: p.scope,
        signers: p.signers,
        // ...fill other fields with dummy values as needed...
        ..Default::default()
    }
}
