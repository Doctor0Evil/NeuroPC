#![cfg(kani)]
#![forbid(unsafe_code)]

use kani::Arbitrary;
use crate::{
    apply_lifeforce_guarded_adjustment,
    LifeforceDecision,
    TestBloodPool,
    HostState,
};

/// Property 3: Guard never allows integrity to drop below 0.0 or exceed 1.0,
/// and never allows window drain to exceed max_lifeforce_drain_frac unless compensated.
/// If denied on integrity or chi floor, state is unchanged.
/// If allowed with compensation, drained_integrity is capped at max.
#[kani::proof]
fn kani_lifeforce_guard_safety() {
    let mut host: HostState = kani::any();
    let mut pool: TestBloodPool = kani::any();

    // Clamp adversarial inputs to valid ranges for realism.
    host.lifeforce = host.lifeforce.clamped();
    host.lf_env.max_lifeforce_drain_frac = host.lf_env.max_lifeforce_drain_frac.clamp(0.0, 1.0);
    host.lf_window.drained_integrity = host.lf_window.drained_integrity.clamp(0.0, 1.0);

    let integrity_before = host.lifeforce.integrity();
    let drained_before = host.lf_window.drained_integrity;

    let projected_integrity_delta: f32 = kani::any_where(|d| d.abs() < 1.0);  // Bounded for convergence.
    let projected_chi_delta: f32 = kani::any_where(|d| d.abs() < 1.0);
    let high_drain: bool = kani::any();

    let decision = apply_lifeforce_guarded_adjustment(
        &mut host,
        &mut pool,
        projected_integrity_delta,
        projected_chi_delta,
        high_drain,
    );

    let integrity_after = host.lifeforce.integrity();
    let drained_after = host.lf_window.drained_integrity;

    // Integrity always in [0,1] post-guard.
    assert!(integrity_after >= 0.0);
    assert!(integrity_after <= 1.0);

    // Drained integrity never negative, and never exceeds max unless compensated (in which case capped).
    assert!(drained_after >= 0.0);
    if matches!(decision, LifeforceDecision::AllowedWithBloodCompensation) {
        assert!(drained_after <= host.lf_env.max_lifeforce_drain_frac + 1e-6);
    } else {
        assert!(drained_after <= host.lf_env.max_lifeforce_drain_frac + 1e-6);
    }

    // If denied on floor, state unchanged.
    if matches!(decision, LifeforceDecision::DeniedIntegrityFloor | LifeforceDecision::DeniedChiFloor) {
        assert_eq!(integrity_after, integrity_before);
        assert_eq!(drained_after, drained_before);
    }
}
