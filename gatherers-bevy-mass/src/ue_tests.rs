//! Rust-authored UE automation tests for the gatherers simulation.
//!
//! These tests run inside the UE editor via the automation test framework.
//! They use `TestCtx` to call back into C++ for UE operations (spawn entities,
//! step simulation, read/write fragments).
//!
//! Register tests with `inventory::submit!(MassTestRegistration { ... })`.
//! Standard `assert!` / `assert_eq!` macros work — panics become test failures.

use glam::DVec3;
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
    let dist = after.position.distance(initial.position);
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
        position: DVec3::new(499.0, 0.0, 50.0),
        previous_position: DVec3::new(498.0, 0.0, 50.0),
    });
    ctx.write("ants", 0, &Movement {
        direction: DVec3::X,
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
    assert!(pos.position.x <= 500.0,
        "ant x should be clamped to bounds max (500.0): x={}", pos.position.x);
    // Direction should have flipped to negative x
    assert!(mov.direction.x < -0.5,
        "ant direction should reflect strongly negative: dir_x={}", mov.direction.x);
    // Ant should have moved away from boundary after reflecting
    assert!(pos.position.x < 499.0,
        "ant should have moved away from boundary after reflecting: x={}", pos.position.x);

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

// ---------------------------------------------------------------------------
// FoodPickup — ant at food position should pick it up
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "FoodPickup",
    test_fn: food_pickup,
});

fn food_pickup(ctx: &TestCtx) {
    ctx.init_sim(
        &[("ants", 1), ("food", 1)],
        [-500.0, -500.0, 0.0],
        [500.0, 500.0, 100.0],
        42,
    );

    assert!(ctx.has_spatial_query("food_pickup"),
        "food_pickup spatial query should be auto-registered from Rust config");

    // Read food position and move ant directly there
    let food = ctx.read::<FoodFragment>("food", 0)
        .expect("should read food fragment");
    assert!(food.is_loose, "food should start as loose");

    ctx.write("ants", 0, &Position {
        position: food.position,
        previous_position: food.position,
    });
    ctx.write("ants", 0, &Carrying { food_index: -1, _pad: 0 });
    ctx.write("ants", 0, &Cooldown { remaining_seconds: 0.0, _pad: [0; 4] });

    // Run simulation — spatial query should detect overlap, food decision should pick up
    ctx.step(0.016, 20);

    let carry = ctx.read::<Carrying>("ants", 0).unwrap();
    assert!(carry.food_index >= 0,
        "ant should have picked up food: food_index={}", carry.food_index);
    assert_eq!(carry.food_index, 0,
        "carried food index should be 0");

    let food_after = ctx.read::<FoodFragment>("food", 0).unwrap();
    assert!(!food_after.is_loose,
        "picked-up food should no longer be loose");

    let cd = ctx.read::<Cooldown>("ants", 0).unwrap();
    assert!(cd.remaining_seconds > 0.0,
        "pickup cooldown should be active: remaining={}", cd.remaining_seconds);

    ctx.reset();
}

// ---------------------------------------------------------------------------
// FoodDrop — carrying ant that encounters loose food should drop
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "FoodDrop",
    test_fn: food_drop,
});

fn food_drop(ctx: &TestCtx) {
    ctx.init_sim(
        &[("ants", 1), ("food", 2)],
        [-500.0, -500.0, 0.0],
        [500.0, 500.0, 100.0],
        99,
    );

    // Set ant as carrying food[0], move to food[1] position
    let food1 = ctx.read::<FoodFragment>("food", 1)
        .expect("should read food[1]");

    ctx.write("ants", 0, &Position {
        position: food1.position,
        previous_position: food1.position,
    });
    ctx.write("ants", 0, &Carrying { food_index: 0, _pad: 0 });
    ctx.write("ants", 0, &Cooldown { remaining_seconds: 0.0, _pad: [0; 4] });

    // Mark food[0] as not loose (it's being carried)
    let mut food0 = ctx.read::<FoodFragment>("food", 0).unwrap();
    food0.is_loose = false;
    ctx.write("food", 0, &food0);

    // Run simulation — ant carrying food encounters loose food[1], should drop
    ctx.step(0.016, 20);

    let carry = ctx.read::<Carrying>("ants", 0).unwrap();
    assert_eq!(carry.food_index, -1,
        "ant should have dropped food: food_index={}", carry.food_index);

    let cd = ctx.read::<Cooldown>("ants", 0).unwrap();
    assert!(cd.remaining_seconds > 0.0,
        "drop cooldown should be active: remaining={}", cd.remaining_seconds);

    ctx.reset();
}

// ---------------------------------------------------------------------------
// Integration — larger sim, verify bulk behavior after many steps
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "Integration",
    test_fn: integration,
});

fn integration(ctx: &TestCtx) {
    let ant_count: usize = 50;
    let food_count: usize = 20;
    ctx.init_sim(
        &[("ants", ant_count as i32), ("food", food_count as i32)],
        [-1000.0, -1000.0, 0.0],
        [1000.0, 1000.0, 100.0],
        12345,
    );

    assert_eq!(ctx.entity_count("ants"), ant_count as i32);
    assert_eq!(ctx.entity_count("food"), food_count as i32);

    // Record initial positions
    let initial_positions: Vec<DVec3> = (0..ant_count)
        .map(|i| ctx.read::<Position>("ants", i as u32).unwrap().position)
        .collect();

    // Run 100 simulation steps (1.6 seconds of sim time)
    ctx.step(0.016, 100);

    // Verify: most ants moved
    let moved_count = (0..ant_count)
        .filter(|&i| {
            let pos = ctx.read::<Position>("ants", i as u32).unwrap();
            pos.position != initial_positions[i]
        })
        .count();
    assert!(moved_count > ant_count / 2,
        "most ants should have moved: {}/{} moved", moved_count, ant_count);

    // Verify: no ant escaped bounds (with tolerance for boundary reflection timing)
    let out_of_bounds = (0..ant_count)
        .filter(|&i| {
            let pos = ctx.read::<Position>("ants", i as u32).unwrap();
            pos.position.x < -1050.0 || pos.position.x > 1050.0
                || pos.position.y < -1050.0 || pos.position.y > 1050.0
        })
        .count();
    assert_eq!(out_of_bounds, 0, "no ants should be far outside bounds");

    // Verify: previous_position is tracked (differs from position for moving ants)
    let prev_tracked = (0..ant_count)
        .filter(|&i| {
            let pos = ctx.read::<Position>("ants", i as u32).unwrap();
            pos.position != pos.previous_position
        })
        .count();
    assert!(prev_tracked > 0,
        "at least some ants should have previous_position != position");

    // Verify: all food entities still valid and readable
    for i in 0..food_count {
        assert!(ctx.read::<FoodFragment>("food", i as u32).is_some(),
            "food[{}] should be readable", i);
    }

    // Verify: clean reset
    ctx.reset();
    assert!(!ctx.has_managed_sim(), "sim should be inactive after reset");
    assert_eq!(ctx.entity_count("ants"), 0, "ant count should be 0 after reset");
    assert_eq!(ctx.entity_count("food"), 0, "food count should be 0 after reset");
}
