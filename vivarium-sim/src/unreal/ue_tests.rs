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
// BirdsFlockAndStayBounded — the bird pipeline (wander + flocking +
// boundary force) must produce measurable motion without letting birds
// escape the sim bounds. Also a smoke check that the new `"birds"`
// visualizer group + spawn loop work end-to-end in UE.
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "VivariumBirdsFlockAndStayBounded",
    test_fn: birds_flock_and_stay_bounded,
});

fn birds_flock_and_stay_bounded(ctx: &TestCtx) {
    const COUNT: i32 = 20;
    let bounds_min = [-200.0, -200.0, -200.0];
    let bounds_max = [200.0, 200.0, 200.0];
    ctx.init_sim(
        &[("insects", 0), ("birds", COUNT)],
        bounds_min,
        bounds_max,
        99,
    );

    assert_eq!(ctx.entity_count("birds"), COUNT);

    let initial: Vec<DVec3> = (0..COUNT as u32)
        .map(|i| {
            ctx.read::<Transform>("birds", i)
                .expect("bird transform")
                .translation
        })
        .collect();

    // 3 sim-seconds at 1/60s. BIRD_SPEED=60 → ~180 units travelled per bird
    // if uninterrupted; easily above the 5-unit motion threshold.
    ctx.step(1.0 / 60.0, 180);

    let mut any_moved = false;
    for i in 0..COUNT as u32 {
        let t = ctx
            .read::<Transform>("birds", i)
            .expect("bird transform after step");
        // In bounds.
        assert!(
            t.translation.x >= bounds_min[0] - 1e-6
                && t.translation.x <= bounds_max[0] + 1e-6
                && t.translation.y >= bounds_min[1] - 1e-6
                && t.translation.y <= bounds_max[1] + 1e-6
                && t.translation.z >= bounds_min[2] - 1e-6
                && t.translation.z <= bounds_max[2] + 1e-6,
            "bird {i} escaped bounds: {:?}",
            t.translation,
        );
        if t.translation.distance(initial[i as usize]) > 5.0 {
            any_moved = true;
        }
    }
    assert!(any_moved, "at least one bird should have moved >5 units");

    ctx.reset();
}

// ---------------------------------------------------------------------------
// BirdsHuntAndEat — end-to-end check that `hunt_system` + `eating_system`
// run in UE mode: 20 birds amongst 200 insects over 5 sim-seconds should
// result in at least one insect eaten, and no bird should die (birds
// don't die in Phase 2b). Guards against the Predator fragment not
// being spawned, hunt/eating not being in the schedule, or
// `BirdHuntStates` / `SpatialQueries` not being wired in UE mode.
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "VivariumBirdsHuntAndEat",
    test_fn: birds_hunt_and_eat,
});

fn birds_hunt_and_eat(ctx: &TestCtx) {
    const BIRDS: i32 = 20;
    const INSECTS: i32 = 200;
    let bounds_min = [-200.0, -200.0, -200.0];
    let bounds_max = [200.0, 200.0, 200.0];
    ctx.init_sim(
        &[("insects", INSECTS), ("birds", BIRDS)],
        bounds_min,
        bounds_max,
        123,
    );

    let initial_insects = ctx.entity_count("insects");
    assert_eq!(
        initial_insects, INSECTS,
        "expected {INSECTS} insects spawned, got {initial_insects}",
    );
    assert_eq!(ctx.entity_count("birds"), BIRDS);

    // 5 sim-seconds @ 1/60s. With 20 birds hunting 200 insects in a small
    // 400-unit cube, at least one dive-to-eat should resolve.
    ctx.step(1.0 / 60.0, 300);

    let after_insects = ctx.entity_count("insects");
    assert!(
        after_insects < initial_insects,
        "expected at least one insect eaten over 5 sim-seconds: \
         before={initial_insects}, after={after_insects}",
    );
    assert_eq!(
        ctx.entity_count("birds"),
        BIRDS,
        "birds should not die in Phase 2b",
    );

    ctx.reset();
}

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
