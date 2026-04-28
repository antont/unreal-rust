//! Rust-authored UE automation tests for the vivarium simulation.
//!
//! These run inside the UE editor via the automation test framework.
//! `TestCtx` calls back into C++ for UE operations (spawn entities, step,
//! read/write fragments). Registered via `MassTestRegistration` — standard
//! `assert!` / `assert_eq!` panics become test failures.

use crate::components::{PreviousTranslation, Transform, Velocity};
use bevy_mass::prelude::DVec3;
use unreal_api::mass::{MassTestRegistration, TestCtx};

// ---------------------------------------------------------------------------
// SpawnAndSimulate — basic lifecycle: init, step, observe motion, reset.
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "VivariumSpawnAndSimulate",
    test_fn: spawn_and_simulate,
});

fn spawn_and_simulate(ctx: &TestCtx) {
    ctx.init_sim(
        &[("insects", 50)],
        [-200.0, -200.0, -200.0],
        [200.0, 200.0, 200.0],
        42,
    );

    assert!(ctx.has_managed_sim(), "sim should be running after init");
    assert_eq!(ctx.entity_count("insects"), 50);

    let initial = ctx
        .read::<Transform>("insects", 0)
        .expect("should read insect 0 transform");

    // ~60 frames at 1/60s dt. Speed is 30 units/s → ~30 units total before
    // boundary interactions; well above float noise.
    ctx.step(1.0 / 60.0, 60);

    let after = ctx
        .read::<Transform>("insects", 0)
        .expect("should read insect 0 transform after stepping");
    let dist = after.translation.distance(initial.translation);
    assert!(
        dist > 1.0,
        "insect should have moved meaningfully: distance={dist:.2}, before={:?}, after={:?}",
        initial.translation, after.translation,
    );

    ctx.reset();
    assert!(!ctx.has_managed_sim(), "sim should be inactive after reset");
}

// ---------------------------------------------------------------------------
// InsectsStayBounded — mirrors the standalone integration test:
// run many frames and assert no insect escapes the sim bounds. Guards
// against the boundary force being too weak / wrongly ordered in the UE
// pipeline.
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "VivariumInsectsStayBounded",
    test_fn: insects_stay_bounded,
});

fn insects_stay_bounded(ctx: &TestCtx) {
    const COUNT: i32 = 64;
    let bounds_min = [-200.0, -200.0, -200.0];
    let bounds_max = [200.0, 200.0, 200.0];
    ctx.init_sim(&[("insects", COUNT)], bounds_min, bounds_max, 7);

    // Park every insect inside the boundary margin, pointing outward on +X.
    // The force field must bend them back before they escape.
    let start = DVec3::new(160.0, 0.0, 0.0); // inside 40-unit margin at +X=200.
    for i in 0..COUNT as u32 {
        ctx.write("insects", i, &Transform::from_translation(start));
        ctx.write("insects", i, &PreviousTranslation { value: start });
        ctx.write("insects", i, &Velocity::new(DVec3::X, 30.0));
    }

    // 5 sim-seconds @ 1/60s.
    ctx.step(1.0 / 60.0, 300);

    for i in 0..COUNT as u32 {
        let t = ctx
            .read::<Transform>("insects", i)
            .expect("should read insect transform");
        assert!(
            t.translation.x >= bounds_min[0] - 1e-6
                && t.translation.x <= bounds_max[0] + 1e-6
                && t.translation.y >= bounds_min[1] - 1e-6
                && t.translation.y <= bounds_max[1] + 1e-6
                && t.translation.z >= bounds_min[2] - 1e-6
                && t.translation.z <= bounds_max[2] + 1e-6,
            "insect {i} escaped bounds: {:?}",
            t.translation,
        );
    }

    ctx.reset();
}
