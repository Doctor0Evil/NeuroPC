fn main() -> Result<(), Box<dyn std::error::Error>> {
    let high_tolerance = ToleranceEnvelope {
        pain_threshold: 9.5,   // adult chosen elevated threshold
        fear_threshold: 8.0,
        psych_risk_threshold: 7.5,
    };

    let mut ledger = DonutLoopLedger::initialize("system.donutloop.aln", high_tolerance.clone())?;

    ledger.append_event(0.12, high_tolerance.clone(), "EVOLVE-gated config update".to_string())?;
    ledger.append_event(0.09, high_tolerance, "automation assistance session".to_string())?;

    ledger.verify_full_chain()?; // proves all invariants
    println!("Ledger valid with provable RoH ≤ 0.3 and monotone safety");
    Ok(())
}
