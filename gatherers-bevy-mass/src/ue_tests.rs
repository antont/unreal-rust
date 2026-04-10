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

    // Run 10 simulation steps (dt=0.016 each, speed=200 → ~3.2 units/step → ~32 total)
    ctx.step(0.016, 10);

    // Verify ants moved a meaningful distance (at least 1 unit, well under the ~32 expected)
    let after = ctx.read::<Position>("ants", 0)
        .expect("should be able to read ant position after stepping");
    let dx = after.position[0] - initial.position[0];
    let dy = after.position[1] - initial.position[1];
    let dz = after.position[2] - initial.position[2];
    let dist = (dx * dx + dy * dy + dz * dz).sqrt();
    assert!(
        dist > 1.0,
        "ant should have moved meaningfully: distance={:.2}, before={:?}, after={:?}",
        dist, initial.position, after.position,
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

    // Step enough for movement + boundary reflect.
    // speed=200, dt=0.016 → 3.2 units/step. Starting at x=499 moving +x,
    // first step overshoots to ~502.2, boundary clamps to 500.0 and reflects direction.
    ctx.step(0.016, 5);

    let pos = ctx.read::<Position>("ants", 0).unwrap();
    let mov = ctx.read::<Movement>("ants", 0).unwrap();

    // After reflection, position should be clamped to bounds exactly
    assert!(pos.position[0] <= 500.0,
        "ant x should be clamped to bounds max (500.0): x={}", pos.position[0]);
    // Direction should have flipped to negative x
    assert!(mov.direction[0] < -0.5,
        "ant direction should reflect strongly negative: dir_x={}", mov.direction[0]);
    // Ant should have moved away from boundary after reflecting
    assert!(pos.position[0] < 499.0,
        "ant should have moved away from boundary after reflecting: x={}", pos.position[0]);

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

    // Step to let cooldown decrement: dt=0.016 × 10 steps = 0.16s elapsed
    // Expected remaining: 1.0 - 0.16 = 0.84
    ctx.step(0.016, 10);

    let cd = ctx.read::<Cooldown>("ants", 0).unwrap();
    let expected = 1.0 - 0.016 * 10.0;
    assert!(
        (cd.remaining_seconds - expected).abs() < 0.01,
        "cooldown should be ~{:.2} after 10 steps of dt=0.016: actual={}",
        expected, cd.remaining_seconds,
    );

    ctx.reset();
}
