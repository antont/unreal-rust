//! Rust-authored UE automation tests for the gatherers simulation.
//!
//! These tests run inside the UE editor via the automation test framework.
//! They use `TestCtx` to call back into C++ for UE operations (spawn entities,
//! step simulation, read/write fragments).
//!
//! Register tests with `inventory::submit!(MassTestRegistration { ... })`.
//! Standard `assert!` / `assert_eq!` macros work — panics become test failures.

use unreal_api::mass::{MassTestRegistration, TestCtx};
use crate::fragments::{Position, Movement, FoodFragment, Carrying, Cooldown};

// ---------------------------------------------------------------------------
// SpawnAndSimulate — basic lifecycle test
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "SpawnAndSimulate",
    test_fn: spawn_and_simulate,
});

fn spawn_and_simulate(ctx: &TestCtx) {
    ctx.init_sim(
        &[("ants", 20), ("food", 10)],
        [-500.0, -500.0, 0.0],
        [500.0, 500.0, 100.0],
        456,
    );

    assert!(ctx.has_managed_sim(), "sim should be running after init");
    assert_eq!(ctx.entity_count("ants"), 20);
    assert_eq!(ctx.entity_count("food"), 10);

    // Record initial ant position
    let initial = ctx.read::<Position>("ants", 0)
        .expect("should be able to read ant position");

    // Run 10 simulation steps
    ctx.step(0.016, 10);

    // Verify ants moved
    let after = ctx.read::<Position>("ants", 0)
        .expect("should be able to read ant position after stepping");
    assert!(
        initial.position != after.position,
        "ant should have moved: before={:?}, after={:?}",
        initial.position, after.position,
    );

    // Reset and verify
    ctx.reset();
    assert!(!ctx.has_managed_sim(), "sim should be inactive after reset");
}

// ---------------------------------------------------------------------------
// BoundaryReflect — ants at bounds should reflect direction
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "BoundaryReflect",
    test_fn: boundary_reflect,
});

fn boundary_reflect(ctx: &TestCtx) {
    ctx.init_sim(
        &[("ants", 1), ("food", 0)],
        [-500.0, -500.0, 0.0],
        [500.0, 500.0, 100.0],
        123,
    );

    // Place ant at the boundary, moving outward
    ctx.write("ants", 0, &Position {
        position: [499.0, 0.0, 50.0],
        previous_position: [498.0, 0.0, 50.0],
    });
    ctx.write("ants", 0, &Movement {
        direction: [1.0, 0.0, 0.0],
        movement_speed: 200.0,
        _pad: [0; 4],
    });

    // Step enough for movement + boundary reflect
    ctx.step(0.016, 5);

    let pos = ctx.read::<Position>("ants", 0).unwrap();
    let mov = ctx.read::<Movement>("ants", 0).unwrap();

    assert!(pos.position[0] <= 500.0 + 1.0, "ant should be clamped to bounds: x={}", pos.position[0]);
    assert!(mov.direction[0] < 0.0, "ant direction should reflect: dir_x={}", mov.direction[0]);

    ctx.reset();
}

// ---------------------------------------------------------------------------
// CooldownDecrement — cooldown timer should decrease over time
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "CooldownDecrement",
    test_fn: cooldown_decrement,
});

fn cooldown_decrement(ctx: &TestCtx) {
    ctx.init_sim(
        &[("ants", 1), ("food", 0)],
        [-500.0, -500.0, 0.0],
        [500.0, 500.0, 100.0],
        42,
    );

    // Set a cooldown
    ctx.write("ants", 0, &Cooldown {
        remaining_seconds: 1.0,
        _pad: [0; 4],
    });

    // Step to let cooldown decrement
    ctx.step(0.016, 10);

    let cd = ctx.read::<Cooldown>("ants", 0).unwrap();
    assert!(
        cd.remaining_seconds < 1.0,
        "cooldown should have decreased: remaining={}",
        cd.remaining_seconds,
    );

    ctx.reset();
}
