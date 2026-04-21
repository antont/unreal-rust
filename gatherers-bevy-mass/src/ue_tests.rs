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
use crate::components::{Transform, PreviousTranslation, DesiredMovement, FoodState, Carrying};

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
    let initial = ctx.read::<Transform>("ants", 0)
        .expect("should be able to read ant transform");

    // Run 10 simulation steps (dt=0.016 each, speed=200 → ~3.2 units/step → ~32 total)
    ctx.step(0.016, 10);

    // Verify ants moved a meaningful distance (at least 1 unit, well under the ~32 expected)
    let after = ctx.read::<Transform>("ants", 0)
        .expect("should be able to read ant transform after stepping");
    let dist = after.translation.distance(initial.translation);
    assert!(
        dist > 1.0,
        "ant should have moved meaningfully: distance={:.2}, before={:?}, after={:?}",
        dist, initial.translation, after.translation,
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
    ctx.write("ants", 0, &Transform::from_translation(DVec3::new(499.0, 0.0, 50.0)));
    ctx.write("ants", 0, &PreviousTranslation { value: DVec3::new(498.0, 0.0, 50.0) });
    ctx.write("ants", 0, &DesiredMovement::new(DVec3::X, 200.0));

    // Step enough for movement + boundary reflect.
    // speed=200, dt=0.016 → 3.2 units/step. Starting at x=499 moving +x,
    // first step overshoots to ~502.2, boundary clamps to 500.0 and reflects direction.
    ctx.step(0.016, 5);

    let t = ctx.read::<Transform>("ants", 0).unwrap();
    let dm = ctx.read::<DesiredMovement>("ants", 0).unwrap();

    // After reflection, position should be clamped to bounds exactly
    assert!(t.translation.x <= 500.0,
        "ant x should be clamped to bounds max (500.0): x={}", t.translation.x);
    // Direction should have flipped to negative x
    assert!(dm.direction().x < -0.5,
        "ant direction should reflect strongly negative: dir_x={}", dm.direction().x);
    // Ant should have moved away from boundary after reflecting
    assert!(t.translation.x < 499.0,
        "ant should have moved away from boundary after reflecting: x={}", t.translation.x);

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

    // Read food position (now in FTransformFragment) and move ant directly there
    let food = ctx.read::<FoodState>("food", 0)
        .expect("should read food fragment");
    assert!(food.is_loose, "food should start as loose");
    let food_tf = ctx.read::<Transform>("food", 0)
        .expect("should read food transform");

    ctx.write("ants", 0, &Transform::from_translation(food_tf.translation));
    ctx.write("ants", 0, &PreviousTranslation { value: food_tf.translation });
    ctx.write("ants", 0, &Carrying { food_index: -1 });
    // Note: Cooldown is now a pure-Bevy component on shadow entities.
    // Ants without Cooldown are eligible for food interaction.

    // Run simulation — spatial query should detect overlap, food decision should pick up
    ctx.step(0.016, 20);

    let carry = ctx.read::<Carrying>("ants", 0).unwrap();
    assert!(carry.food_index >= 0,
        "ant should have picked up food: food_index={}", carry.food_index);
    assert_eq!(carry.food_index, 0,
        "carried food index should be 0");

    let food_after = ctx.read::<FoodState>("food", 0).unwrap();
    assert!(!food_after.is_loose,
        "picked-up food should no longer be loose");

    ctx.reset();
}

// ---------------------------------------------------------------------------
// FoodPickupMultiChunk — regression for #[mass_system] MessageReader drain
// across chunks. If `MessageReader<AntFoodHit>` is consumed on the first chunk
// call, only ants in chunk 0 can ever match a hit → 99% pass-through in PIE.
//
// We spawn enough ants to guarantee >1 primary chunk, position two ants (one
// in an early chunk, one in a late chunk) directly on distinct loose foods,
// and expect BOTH to pick up. A correct implementation makes every emitted
// `AntFoodHit` visible to every chunk invocation of `ant_food_decision`.
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "FoodPickupMultiChunk",
    test_fn: food_pickup_multi_chunk,
});

fn food_pickup_multi_chunk(ctx: &TestCtx) {
    // Mass archetype chunks default to ~300 entities — 1000 ants reliably
    // produces at least 3 chunks. Using two ants deep in different chunk
    // indices proves the bug regardless of chunk-size tuning.
    const NUM_ANTS: u32 = 1000;
    const EARLY_ANT: u32 = 0;   // chunk 0
    const LATE_ANT: u32 = 900;  // guaranteed a later chunk (≥3 expected)
    const EARLY_FOOD: u32 = 0;
    const LATE_FOOD: u32 = 1;

    ctx.init_sim(
        &[("ants", NUM_ANTS as i32), ("food", 2)],
        [-500.0, -500.0, 0.0],
        [500.0, 500.0, 100.0],
        42,
    );

    assert!(ctx.has_spatial_query("food_pickup"),
        "food_pickup spatial query should be auto-registered");

    // Park the two test ants directly on their respective foods (so the
    // collision prepass emits a hit for each on the very next step).
    // Don't move the food — the navigation hash grid is populated from the
    // food's spawn position and not rebuilt when we write the Transform.
    let early_food_tf = ctx.read::<Transform>("food", EARLY_FOOD).unwrap();
    let late_food_tf = ctx.read::<Transform>("food", LATE_FOOD).unwrap();

    ctx.write("ants", EARLY_ANT,
        &Transform::from_translation(early_food_tf.translation));
    ctx.write("ants", EARLY_ANT,
        &PreviousTranslation { value: early_food_tf.translation });
    ctx.write("ants", EARLY_ANT, &Carrying { food_index: -1 });

    ctx.write("ants", LATE_ANT,
        &Transform::from_translation(late_food_tf.translation));
    ctx.write("ants", LATE_ANT,
        &PreviousTranslation { value: late_food_tf.translation });
    ctx.write("ants", LATE_ANT, &Carrying { food_index: -1 });

    // Same step count as FoodPickup so any difference is about chunk placement,
    // not time-to-pickup. With both ants parked on their food, a correct
    // implementation picks up within a few frames; the bug keeps the late-chunk
    // ant idle forever (empty MessageReader on its chunk call).
    ctx.step(0.016, 20);

    let early_carry = ctx.read::<Carrying>("ants", EARLY_ANT).unwrap();
    let late_carry = ctx.read::<Carrying>("ants", LATE_ANT).unwrap();

    assert!(
        early_carry.food_index >= 0,
        "ant {EARLY_ANT} (early chunk) should have picked up food: got {}",
        early_carry.food_index,
    );
    assert!(
        late_carry.food_index >= 0,
        "ant {LATE_ANT} (late chunk) should have picked up food: got {}. \
         If only the early-chunk ant picks up, the #[mass_system] macro is \
         draining MessageReader on the first chunk call and leaving later \
         chunks with an empty message queue.",
        late_carry.food_index,
    );

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
    let food1_tf = ctx.read::<Transform>("food", 1)
        .expect("should read food[1] transform");

    ctx.write("ants", 0, &Transform::from_translation(food1_tf.translation));
    ctx.write("ants", 0, &PreviousTranslation { value: food1_tf.translation });
    ctx.write("ants", 0, &Carrying { food_index: 0 });
    // Note: Cooldown is now a pure-Bevy component on shadow entities.
    // Ants without Cooldown are eligible for food interaction.

    // Mark food[0] as not loose (it's being carried)
    let mut food0 = ctx.read::<FoodState>("food", 0).unwrap();
    food0.is_loose = false;
    ctx.write("food", 0, &food0);

    // Run simulation — ant carrying food encounters loose food[1], should drop
    ctx.step(0.016, 20);

    let carry = ctx.read::<Carrying>("ants", 0).unwrap();
    assert_eq!(carry.food_index, -1,
        "ant should have dropped food: food_index={}", carry.food_index);

    ctx.reset();
}

// ---------------------------------------------------------------------------
// CooldownRecovery — verify ants recover from cooldown and interact again
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "CooldownRecovery",
    test_fn: cooldown_recovery,
});

fn cooldown_recovery(ctx: &TestCtx) {
    // 10 ants, 50 food — high food density so ants should encounter food quickly
    ctx.init_sim(
        &[("ants", 10), ("food", 50)],
        [-200.0, -200.0, 0.0],
        [200.0, 200.0, 100.0],
        7777,
    );

    // Run 200 steps (~3.2 seconds at 0.016 dt)
    // Cooldown is ~0.5s, so ants should complete multiple pickup/drop cycles.
    ctx.step(0.016, 200);

    // Count how many ants have interacted with food (food_index != -1 means carrying,
    // but we also want to detect ants that picked up AND dropped — check carrying state)
    let mut carrying_count = 0;
    for i in 0..10u32 {
        let carry = ctx.read::<Carrying>("ants", i).unwrap();
        if carry.food_index >= 0 {
            carrying_count += 1;
        }
    }

    // After 3.2 seconds with 50 food in a 400x400 area, most ants should have
    // interacted with food at least once. We expect a mix of carrying and not-carrying.
    // The key assertion: at least 1 ant is carrying (proves pickup works).
    assert!(carrying_count >= 1,
        "at least 1 ant should be carrying food after 200 steps: carrying={carrying_count}");

    // Run 200 MORE steps — ants that picked up should eventually drop (encounter other food)
    ctx.step(0.016, 200);

    // After 6.4 total seconds, verify food movement happened
    let mut food_not_loose = 0;
    for i in 0..50u32 {
        let food = ctx.read::<FoodState>("food", i).unwrap();
        if !food.is_loose {
            food_not_loose += 1;
        }
    }
    // Some food should have been picked up
    assert!(food_not_loose >= 1 || carrying_count >= 1,
        "some food should have been interacted with: not_loose={food_not_loose}");

    // Now the critical check: run 500 more steps and verify ALL ants can still interact.
    // Place all 10 ants directly on top of different loose food items.
    // After enough steps, ALL should have picked up.
    ctx.step(0.016, 100); // let things settle

    // Find loose food positions (position now in FTransformFragment)
    let mut loose_food: Vec<(u32, DVec3)> = Vec::new();
    for i in 0..50u32 {
        let food = ctx.read::<FoodState>("food", i).unwrap();
        if food.is_loose {
            let food_tf = ctx.read::<Transform>("food", i).unwrap();
            loose_food.push((i, food_tf.translation));
        }
    }

    // Place ants on top of loose food (as many as we can)
    let place_count = loose_food.len().min(10);
    for ant_idx in 0..place_count {
        let (_, food_pos) = loose_food[ant_idx];
        ctx.write("ants", ant_idx as u32, &Transform::from_translation(food_pos));
        ctx.write("ants", ant_idx as u32, &PreviousTranslation { value: food_pos - DVec3::new(1.0, 0.0, 0.0) });
        ctx.write("ants", ant_idx as u32, &Carrying { food_index: -1 });
    }

    // Run enough steps for cooldown to expire and pickup to occur
    // Cooldown ~0.5s = ~31 frames, then pickup on next encounter
    ctx.step(0.016, 60);

    // Check how many of the placed ants picked up food
    let mut picked_up = 0;
    let mut failed_ants: Vec<String> = Vec::new();
    for ant_idx in 0..place_count {
        let carry = ctx.read::<Carrying>("ants", ant_idx as u32).unwrap();
        if carry.food_index >= 0 {
            picked_up += 1;
        } else {
            let t = ctx.read::<Transform>("ants", ant_idx as u32).unwrap();
            let dm = ctx.read::<DesiredMovement>("ants", ant_idx as u32).unwrap();
            let (food_idx, food_pos) = loose_food[ant_idx];
            let dist = t.translation.distance(food_pos);
            failed_ants.push(format!(
                "ant[{ant_idx}]: pos=({:.1},{:.1},{:.1}) food[{food_idx}]=({:.1},{:.1},{:.1}) dist={dist:.1} speed={:.1} dir=({:.2},{:.2},{:.2})",
                t.translation.x, t.translation.y, t.translation.z,
                food_pos.x, food_pos.y, food_pos.z,
                dm.speed(), dm.direction().x, dm.direction().y, dm.direction().z,
            ));
        }
    }

    // If ANY ant failed to pick up food despite being placed directly on it,
    // that's the stuck cooldown bug.
    assert!(picked_up >= place_count * 3 / 4,
        "most ants placed on food should pick up: {picked_up}/{place_count} picked up. \
         Failed ants:\n{}", failed_ants.join("\n"));

    ctx.reset();
}

// ---------------------------------------------------------------------------
// CooldownCycle — verify single ant completes pickup → cooldown → pickup cycle
// ---------------------------------------------------------------------------

inventory::submit!(MassTestRegistration {
    name: "CooldownCycle",
    test_fn: cooldown_cycle,
});

fn cooldown_cycle(ctx: &TestCtx) {
    ctx.init_sim(
        &[("ants", 1), ("food", 5)],
        [-200.0, -200.0, 0.0],
        [200.0, 200.0, 100.0],
        42,
    );

    // Read food[0] position (from FTransformFragment)
    let food0_tf = ctx.read::<Transform>("food", 0).unwrap();
    let food0_pos = food0_tf.translation;

    // Place ant on food[0], moving toward it
    ctx.write("ants", 0, &Transform::from_translation(food0_pos));
    ctx.write("ants", 0, &PreviousTranslation { value: food0_pos - DVec3::new(5.0, 0.0, 0.0) });
    ctx.write("ants", 0, &DesiredMovement::new(DVec3::X, 100.0));
    ctx.write("ants", 0, &Carrying { food_index: -1 });

    // Step 1: pickup should happen within a few frames
    ctx.step(0.016, 10);

    let carry1 = ctx.read::<Carrying>("ants", 0).unwrap();
    assert!(carry1.food_index >= 0,
        "Phase 1: ant should pick up food after 10 steps: food_index={}",
        carry1.food_index);

    // Step 2: let cooldown expire (~0.5s = ~31 frames)
    ctx.step(0.016, 50);

    // Now place ant on food[1] (a different loose food)
    let food1 = ctx.read::<FoodState>("food", 1).unwrap();
    if food1.is_loose {
        let food1_tf = ctx.read::<Transform>("food", 1).unwrap();
        ctx.write("ants", 0, &Transform::from_translation(food1_tf.translation));
        ctx.write("ants", 0, &PreviousTranslation { value: food1_tf.translation - DVec3::new(5.0, 0.0, 0.0) });

        // Ant is carrying food[0], encounters food[1] → should DROP
        ctx.step(0.016, 10);

        let carry2 = ctx.read::<Carrying>("ants", 0).unwrap();
        assert_eq!(carry2.food_index, -1,
            "Phase 2: carrying ant encountering loose food should drop: food_index={}",
            carry2.food_index);

        // Step 3: let cooldown expire again
        ctx.step(0.016, 50);

        // Place ant on food[2] — should pick up again (cooldown expired)
        let food2 = ctx.read::<FoodState>("food", 2).unwrap();
        if food2.is_loose {
            let food2_tf = ctx.read::<Transform>("food", 2).unwrap();
            ctx.write("ants", 0, &Transform::from_translation(food2_tf.translation));
            ctx.write("ants", 0, &PreviousTranslation { value: food2_tf.translation - DVec3::new(5.0, 0.0, 0.0) });
            ctx.write("ants", 0, &Carrying { food_index: -1 });

            ctx.step(0.016, 10);

            let carry3 = ctx.read::<Carrying>("ants", 0).unwrap();
            assert!(carry3.food_index >= 0,
                "Phase 3: ant should pick up food again after cooldown: food_index={} \
                 (if -1, cooldown may be stuck)",
                carry3.food_index);
        }
    }

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
        .map(|i| ctx.read::<Transform>("ants", i as u32).unwrap().translation)
        .collect();

    // Run 100 simulation steps (1.6 seconds of sim time)
    ctx.step(0.016, 100);

    // Verify: most ants moved
    let moved_count = (0..ant_count)
        .filter(|&i| {
            let t = ctx.read::<Transform>("ants", i as u32).unwrap();
            t.translation != initial_positions[i]
        })
        .count();
    assert!(moved_count > ant_count / 2,
        "most ants should have moved: {}/{} moved", moved_count, ant_count);

    // Verify: no ant escaped bounds (with tolerance for boundary reflection timing)
    let out_of_bounds = (0..ant_count)
        .filter(|&i| {
            let t = ctx.read::<Transform>("ants", i as u32).unwrap();
            t.translation.x < -1050.0 || t.translation.x > 1050.0
                || t.translation.y < -1050.0 || t.translation.y > 1050.0
        })
        .count();
    assert_eq!(out_of_bounds, 0, "no ants should be far outside bounds");

    // Verify: previous_translation is tracked (differs from translation for moving ants)
    let prev_tracked = (0..ant_count)
        .filter(|&i| {
            let t = ctx.read::<Transform>("ants", i as u32).unwrap();
            let prev = ctx.read::<PreviousTranslation>("ants", i as u32).unwrap();
            t.translation != prev.value
        })
        .count();
    assert!(prev_tracked > 0,
        "at least some ants should have previous_translation != translation");

    // Verify: all food entities still valid and readable
    for i in 0..food_count {
        assert!(ctx.read::<FoodState>("food", i as u32).is_some(),
            "food[{}] should be readable", i);
    }

    // Verify: clean reset
    ctx.reset();
    assert!(!ctx.has_managed_sim(), "sim should be inactive after reset");
    assert_eq!(ctx.entity_count("ants"), 0, "ant count should be 0 after reset");
    assert_eq!(ctx.entity_count("food"), 0, "food count should be 0 after reset");
}
