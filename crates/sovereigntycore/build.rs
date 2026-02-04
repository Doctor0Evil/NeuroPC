fn main() {
    // Load canonical shards using organiccpualn loaders.
    let roh = organiccpualn::rohmodel::load_default().expect("load RoH");
    assert!(roh.validate_invariants(), "RoH model invalid");

    let stake = organiccpualn::stake::load_default().expect("load stake");
    assert!(stake.validate_invariants(), "stake shard invalid");

    // Fail build if wiring is missing.
    sovereigntycore_schema::assert_guard_pipeline_complete()
        .expect("sovereignty guard pipeline incomplete");
}
