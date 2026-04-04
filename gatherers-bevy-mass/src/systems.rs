use unreal_api::mass::{MassQuery, MassQueryAll};
use unreal_api::mass_system;
use crate::fragments::{AntEncounterFragment, AntFragment, FoodFragment};

/// Default simulation bounds — Rust owns this, no C++ round-trip needed.
pub const SIM_BOUNDS_MIN: [f64; 3] = [-500.0, -500.0, -100.0];
pub const SIM_BOUNDS_MAX: [f64; 3] = [500.0, 500.0, 100.0];

// ---------------------------------------------------------------------------
// System 1: Movement — position += velocity * dt
// ---------------------------------------------------------------------------

#[mass_system]
fn ant_movement(ants: MassQuery<&mut AntFragment>, dt: f32) {
    let bounds_size_x = SIM_BOUNDS_MAX[0] - SIM_BOUNDS_MIN[0];
    let bounds_size_y = SIM_BOUNDS_MAX[1] - SIM_BOUNDS_MIN[1];
    let bounds_max_step = 0.5 * bounds_size_x.min(bounds_size_y);

    for ant in ants.iter_mut() {
        ant.previous_position = ant.position;

        let dir = normalize_f64x3(ant.direction);
        if is_nearly_zero_f64x3(dir) {
            continue;
        }
        let max_dist = (ant.movement_speed.max(0.0) * dt.max(0.0)) as f64;
        let step_dist = max_dist.min(bounds_max_step.max(0.0));
        ant.position[0] += dir[0] * step_dist;
        ant.position[1] += dir[1] * step_dist;
        ant.position[2] += dir[2] * step_dist;
    }
}

// ---------------------------------------------------------------------------
// System 4: Cooldown — decrement pickup cooldown timers
// ---------------------------------------------------------------------------

#[mass_system]
fn ant_cooldown(ants: MassQuery<&mut AntFragment>, dt: f32) {
    for ant in ants.iter_mut() {
        ant.pickup_cooldown_remaining_seconds =
            (ant.pickup_cooldown_remaining_seconds - dt.max(0.0)).max(0.0);
    }
}

// ---------------------------------------------------------------------------
// System 5: Boundary reflection — clamp position, reflect direction
// ---------------------------------------------------------------------------

#[mass_system]
fn ant_boundary_reflect(ants: MassQuery<&mut AntFragment>, dt: f32) {
    let _ = dt; // unused but required by macro signature

    for ant in ants.iter_mut() {
        let mut inward_normal = [0.0f64; 3];

        if ant.position[0] < SIM_BOUNDS_MIN[0] {
            ant.position[0] = SIM_BOUNDS_MIN[0];
            inward_normal[0] += 1.0;
        } else if ant.position[0] > SIM_BOUNDS_MAX[0] {
            ant.position[0] = SIM_BOUNDS_MAX[0];
            inward_normal[0] -= 1.0;
        }

        if ant.position[1] < SIM_BOUNDS_MIN[1] {
            ant.position[1] = SIM_BOUNDS_MIN[1];
            inward_normal[1] += 1.0;
        } else if ant.position[1] > SIM_BOUNDS_MAX[1] {
            ant.position[1] = SIM_BOUNDS_MAX[1];
            inward_normal[1] -= 1.0;
        }

        if !is_nearly_zero_f64x3(inward_normal) {
            ant.direction = reflect_direction(ant.direction, inward_normal);
        }
    }
}

// ---------------------------------------------------------------------------
// System 3: Food decision — pickup/drop logic from pre-computed encounters
// ---------------------------------------------------------------------------

/// Cooldown applied after picking up or dropping food, in seconds.
const PICKUP_COOLDOWN_SECONDS: f32 = 0.5;

#[mass_system]
fn ant_food_decision(
    ants: MassQuery<&mut AntFragment>,
    encounters: MassQuery<&AntEncounterFragment>,
    foods: MassQueryAll<&mut FoodFragment>,
    dt: f32,
) {
    let _ = dt;

    for (ant, encounter) in ants.iter_mut().zip(encounters.iter()) {
        if !encounter.has_encounter {
            continue;
        }
        if ant.pickup_cooldown_remaining_seconds > 0.0 {
            continue;
        }

        let is_carrying = ant.carried_food_index >= 0;

        if is_carrying {
            // Drop carried food at ant's current position
            if let Some(food) = foods.get_mut(ant.carried_food_index as usize) {
                food.is_loose = true;
                food.position = ant.position;
            }
            ant.carried_food_index = -1;
            ant.direction = reverse_direction(ant.direction);
            ant.pickup_cooldown_remaining_seconds = PICKUP_COOLDOWN_SECONDS;
        } else {
            // Pick up nearest food by index (from C++ collision pre-pass)
            let food_index = encounter.nearest_food_index;
            if food_index >= 0 {
                if let Some(food) = foods.get_mut(food_index as usize) {
                    if food.is_loose {
                        food.is_loose = false;
                        ant.carried_food_index = food_index;
                        ant.direction = reverse_direction(ant.direction);
                        ant.pickup_cooldown_remaining_seconds = PICKUP_COOLDOWN_SECONDS;
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// System 2: Collision pre-pass — detect food encounters via spatial query
// ---------------------------------------------------------------------------

/// Pure logic for collision pre-pass. Testable with mock callbacks.
pub fn ant_collision_prepass_impl(
    ants: &unreal_api::mass::MassQueryRef<'_, AntFragment>,
    encounters: &mut unreal_api::mass::MassQueryMut<'_, AntEncounterFragment>,
    query_fn: Option<unreal_api::ffi::MassSpatialQueryFn>,
    pickup_radius: f32,
) {
    for (ant, enc) in ants.iter().zip(encounters.iter_mut()) {
        // Reset encounter
        enc.has_encounter = false;
        enc.nearest_food_index = -1;
        enc.encounter_position = [0.0; 3];

        let Some(callback) = query_fn else {
            continue;
        };

        let mut result = unreal_api::ffi::MassSpatialQueryResult {
            food_index: -1,
            _pad: 0,
            encounter_position: [0.0; 3],
            has_encounter: false,
            _result_pad: [0; 7],
        };

        let ok = unsafe {
            callback(
                ant.previous_position.as_ptr(),
                ant.position.as_ptr(),
                pickup_radius,
                &mut result,
            )
        };

        if ok != 0 && result.has_encounter {
            enc.has_encounter = true;
            enc.nearest_food_index = result.food_index;
            enc.encounter_position = result.encounter_position;
        }
    }
}

/// Bevy system wrapper for collision pre-pass.
/// Iterates over all primary chunks of ants/encounters and calls the impl.
pub fn ant_collision_prepass_bevy(
    ants: unreal_api::ecs::prelude::Res<unreal_api::mass::MassChunks<AntFragment>>,
    mut encounters: unreal_api::ecs::prelude::ResMut<unreal_api::mass::MassChunks<AntEncounterFragment>>,
    spatial: unreal_api::ecs::prelude::Res<unreal_api::mass::MassSpatialQueryCallback>,
) {
    for chunk_idx in 0..ants.primary_chunk_count() {
        let ant_query = unsafe { ants.primary_chunk_ref(chunk_idx) };
        let mut enc_query = unsafe { encounters.primary_chunk_mut(chunk_idx) };
        ant_collision_prepass_impl(
            &ant_query,
            &mut enc_query,
            spatial.query_fn,
            spatial.pickup_radius,
        );
    }
}

fn reverse_direction(dir: [f64; 3]) -> [f64; 3] {
    [-dir[0], -dir[1], -dir[2]]
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn normalize_f64x3(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-8 {
        return [0.0; 3];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

fn is_nearly_zero_f64x3(v: [f64; 3]) -> bool {
    v[0].abs() < 1e-8 && v[1].abs() < 1e-8 && v[2].abs() < 1e-8
}

fn dot_f64x3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn reflect_direction(dir: [f64; 3], normal: [f64; 3]) -> [f64; 3] {
    let safe_dir = normalize_f64x3(dir);
    let safe_normal = normalize_f64x3(normal);
    if is_nearly_zero_f64x3(safe_dir) || is_nearly_zero_f64x3(safe_normal) {
        return [0.0; 3];
    }
    let d = dot_f64x3(safe_dir, safe_normal);
    let reflected = [
        safe_dir[0] - 2.0 * d * safe_normal[0],
        safe_dir[1] - 2.0 * d * safe_normal[1],
        safe_dir[2] - 2.0 * d * safe_normal[2],
    ];
    normalize_f64x3(reflected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fragments::{AntEncounterFragment, AntFragment, FoodFragment};
    use unreal_api::mass::{MassGlobalChunkStorage, MassQueryAllMut, MassQueryMut, MassQueryRef};

    #[test]
    fn ant_movement_moves_forward() {
        let mut ants = [AntFragment {
            direction: [1.0, 0.0, 0.0],
            movement_speed: 100.0,
            ..Default::default()
        }];
        let mut query = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_movement(&mut query, 0.1);
        assert!(
            (ants[0].position[0] - 10.0).abs() < 1e-6,
            "expected ~10.0, got {}",
            ants[0].position[0]
        );
    }

    #[test]
    fn ant_movement_stores_previous_position() {
        let mut ants = [AntFragment {
            position: [100.0, 200.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            movement_speed: 50.0,
            ..Default::default()
        }];
        let mut query = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_movement(&mut query, 0.1);
        assert_eq!(ants[0].previous_position, [100.0, 200.0, 0.0]);
    }

    #[test]
    fn ant_cooldown_decrements() {
        let mut ants = [AntFragment {
            pickup_cooldown_remaining_seconds: 1.0,
            movement_speed: 100.0,
            direction: [1.0, 0.0, 0.0],
            ..Default::default()
        }];
        let mut query = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_cooldown(&mut query, 0.3);
        assert!(
            (ants[0].pickup_cooldown_remaining_seconds - 0.7).abs() < 1e-5,
            "cooldown should be ~0.7, got {}",
            ants[0].pickup_cooldown_remaining_seconds
        );
    }

    #[test]
    fn ant_cooldown_floors_at_zero() {
        let mut ants = [AntFragment {
            pickup_cooldown_remaining_seconds: 0.1,
            movement_speed: 100.0,
            direction: [1.0, 0.0, 0.0],
            ..Default::default()
        }];
        let mut query = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_cooldown(&mut query, 1.0);
        assert_eq!(ants[0].pickup_cooldown_remaining_seconds, 0.0);
    }

    #[test]
    fn boundary_clamp_x_max() {
        let mut ants = [AntFragment {
            position: [600.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            movement_speed: 100.0,
            ..Default::default()
        }];
        let mut query = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_boundary_reflect(&mut query, 0.0);
        assert!(
            ants[0].position[0] <= 500.0,
            "X should be clamped to 500, got {}",
            ants[0].position[0]
        );
    }

    #[test]
    fn direction_reflects_at_boundary() {
        let mut ants = [AntFragment {
            position: [600.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            movement_speed: 100.0,
            ..Default::default()
        }];
        let mut query = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_boundary_reflect(&mut query, 0.0);
        assert!(
            ants[0].direction[0] < 0.0,
            "direction X should reflect negative, got {}",
            ants[0].direction[0]
        );
    }

    #[test]
    fn combined_systems_match_original_behavior() {
        // Run all 3 systems in sequence, verify same result as original combined system
        let mut ants = [AntFragment {
            position: [499.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            movement_speed: 100.0,
            pickup_cooldown_remaining_seconds: 1.0,
            ..Default::default()
        }];

        // Step 1: movement
        let mut q = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_movement(&mut q, 1.0);
        assert!(ants[0].position[0] > 500.0, "should overshoot");
        assert_eq!(ants[0].previous_position, [499.0, 0.0, 0.0]);

        // Step 2: boundary reflect
        let mut q = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_boundary_reflect(&mut q, 1.0);
        assert!(ants[0].position[0] <= 500.0, "should be clamped");
        assert!(ants[0].direction[0] < 0.0, "should reflect");

        // Step 3: cooldown
        let mut q = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_cooldown(&mut q, 1.0);
        assert_eq!(ants[0].pickup_cooldown_remaining_seconds, 0.0);
    }

    // -----------------------------------------------------------------------
    // Food decision tests
    // -----------------------------------------------------------------------

    #[test]
    fn food_decision_pickup_when_encounter() {
        let mut ants = [AntFragment {
            position: [100.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            carried_food_index: -1,
            pickup_cooldown_remaining_seconds: 0.0,
            ..Default::default()
        }];
        let encounters = [AntEncounterFragment {
            nearest_food_index: 0,
            encounter_position: [100.0, 0.0, 0.0],
            has_encounter: true,
            ..Default::default()
        }];
        let mut foods = [FoodFragment {
            position: [101.0, 0.0, 0.0],
            is_loose: true,
            _pad: [0; 7],
        }];

        let mut ant_q = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        let enc_q = unsafe { MassQueryRef::from_raw(encounters.as_ptr() as *const _, encounters.len()) };
        let mut storage = MassGlobalChunkStorage::new();
        let mut food_q = unsafe {
            MassQueryAllMut::from_raw_single_chunk(
                foods.as_mut_ptr() as *mut _,
                foods.len(),
                &mut storage,
            )
        };

        ant_food_decision(&mut ant_q, &enc_q, &mut food_q, 0.1);

        assert_eq!(ants[0].carried_food_index, 0);
        assert!(!foods[0].is_loose, "food should no longer be loose");
        assert!(ants[0].direction[0] < 0.0, "direction should reverse");
        assert!(ants[0].pickup_cooldown_remaining_seconds > 0.0);
    }

    #[test]
    fn food_decision_drop_when_carrying_and_encounter() {
        let mut ants = [AntFragment {
            position: [200.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            carried_food_index: 0, // carrying food at index 0
            pickup_cooldown_remaining_seconds: 0.0,
            ..Default::default()
        }];
        let encounters = [AntEncounterFragment {
            nearest_food_index: 1, // different food nearby
            encounter_position: [200.0, 0.0, 0.0],
            has_encounter: true,
            ..Default::default()
        }];
        let mut foods = [
            FoodFragment { position: [0.0; 3], is_loose: false, _pad: [0; 7] }, // carried food
            FoodFragment { position: [201.0, 0.0, 0.0], is_loose: true, _pad: [0; 7] }, // nearby food
        ];

        let mut ant_q = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        let enc_q = unsafe { MassQueryRef::from_raw(encounters.as_ptr() as *const _, encounters.len()) };
        let mut storage = MassGlobalChunkStorage::new();
        let mut food_q = unsafe {
            MassQueryAllMut::from_raw_single_chunk(
                foods.as_mut_ptr() as *mut _,
                foods.len(),
                &mut storage,
            )
        };

        ant_food_decision(&mut ant_q, &enc_q, &mut food_q, 0.1);

        assert_eq!(ants[0].carried_food_index, -1);
        assert!(foods[0].is_loose, "dropped food should be loose");
        assert_eq!(foods[0].position, [200.0, 0.0, 0.0]);
        assert!(ants[0].direction[0] < 0.0);
    }

    #[test]
    fn food_decision_skips_when_cooldown_active() {
        let mut ants = [AntFragment {
            position: [100.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            carried_food_index: -1,
            pickup_cooldown_remaining_seconds: 1.0,
            ..Default::default()
        }];
        let encounters = [AntEncounterFragment {
            nearest_food_index: 0,
            encounter_position: [100.0, 0.0, 0.0],
            has_encounter: true,
            ..Default::default()
        }];
        let mut foods = [FoodFragment {
            position: [101.0, 0.0, 0.0],
            is_loose: true,
            _pad: [0; 7],
        }];

        let mut ant_q = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        let enc_q = unsafe { MassQueryRef::from_raw(encounters.as_ptr() as *const _, encounters.len()) };
        let mut storage = MassGlobalChunkStorage::new();
        let mut food_q = unsafe {
            MassQueryAllMut::from_raw_single_chunk(
                foods.as_mut_ptr() as *mut _,
                foods.len(),
                &mut storage,
            )
        };

        ant_food_decision(&mut ant_q, &enc_q, &mut food_q, 0.1);

        assert_eq!(ants[0].carried_food_index, -1);
        assert!(foods[0].is_loose);
        assert_eq!(ants[0].direction, [1.0, 0.0, 0.0]);
    }

    // -----------------------------------------------------------------------
    // Bevy schedule integration tests
    // -----------------------------------------------------------------------

    #[test]
    fn bevy_schedule_ant_movement_matches_direct() {
        use unreal_api::mass::{MassChunks, MassSchedule};
        use unreal_api::ffi::MassFragmentSlice;

        // Prepare identical ant data for both paths
        let mut ants_direct = [AntFragment {
            position: [100.0, 200.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            movement_speed: 50.0,
            ..Default::default()
        }];
        let mut ants_bevy = ants_direct;

        let dt = 0.1f32;

        // --- Direct path ---
        {
            let mut q = unsafe {
                MassQueryMut::from_raw(ants_direct.as_mut_ptr() as *mut _, ants_direct.len())
            };
            ant_movement(&mut q, dt);
        }

        // --- Bevy schedule path ---
        {
            let mut sched = MassSchedule::new();
            sched.world_mut().insert_resource(MassChunks::<AntFragment>::new());
            sched.schedule_mut().add_systems(ant_movement_bevy);
            sched.set_dt(dt);

            // Populate MassChunks with pointer to ants_bevy
            {
                let mut chunks = sched.world_mut().resource_mut::<MassChunks<AntFragment>>();
                unsafe {
                    chunks.push_primary_slice(MassFragmentSlice {
                        data: ants_bevy.as_mut_ptr() as *mut _,
                        count: ants_bevy.len() as i32,
                        stride: std::mem::size_of::<AntFragment>() as u32,
                    });
                }
            }

            sched.run();
        }

        // Both paths should produce identical results
        assert_eq!(ants_direct[0].position, ants_bevy[0].position,
            "Bevy schedule should produce same position as direct call");
        assert_eq!(ants_direct[0].previous_position, ants_bevy[0].previous_position,
            "Bevy schedule should produce same previous_position as direct call");
    }

    #[test]
    fn bevy_schedule_cooldown_matches_direct() {
        use unreal_api::mass::{MassChunks, MassSchedule};
        use unreal_api::ffi::MassFragmentSlice;

        let mut ants_direct = [AntFragment {
            pickup_cooldown_remaining_seconds: 1.0,
            ..Default::default()
        }];
        let mut ants_bevy = ants_direct;
        let dt = 0.3f32;

        // Direct
        {
            let mut q = unsafe {
                MassQueryMut::from_raw(ants_direct.as_mut_ptr() as *mut _, ants_direct.len())
            };
            ant_cooldown(&mut q, dt);
        }

        // Bevy
        {
            let mut sched = MassSchedule::new();
            sched.world_mut().insert_resource(MassChunks::<AntFragment>::new());
            sched.schedule_mut().add_systems(ant_cooldown_bevy);
            sched.set_dt(dt);

            {
                let mut chunks = sched.world_mut().resource_mut::<MassChunks<AntFragment>>();
                unsafe {
                    chunks.push_primary_slice(MassFragmentSlice {
                        data: ants_bevy.as_mut_ptr() as *mut _,
                        count: ants_bevy.len() as i32,
                        stride: std::mem::size_of::<AntFragment>() as u32,
                    });
                }
            }

            sched.run();
        }

        assert!(
            (ants_direct[0].pickup_cooldown_remaining_seconds
                - ants_bevy[0].pickup_cooldown_remaining_seconds).abs() < 1e-6,
            "Cooldown values should match: direct={}, bevy={}",
            ants_direct[0].pickup_cooldown_remaining_seconds,
            ants_bevy[0].pickup_cooldown_remaining_seconds
        );
    }

    #[test]
    fn bevy_schedule_boundary_reflect_matches_direct() {
        use unreal_api::mass::{MassChunks, MassSchedule};
        use unreal_api::ffi::MassFragmentSlice;

        let mut ants_direct = [AntFragment {
            position: [600.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            ..Default::default()
        }];
        let mut ants_bevy = ants_direct;
        let dt = 0.0f32;

        // Direct
        {
            let mut q = unsafe {
                MassQueryMut::from_raw(ants_direct.as_mut_ptr() as *mut _, ants_direct.len())
            };
            ant_boundary_reflect(&mut q, dt);
        }

        // Bevy
        {
            let mut sched = MassSchedule::new();
            sched.world_mut().insert_resource(MassChunks::<AntFragment>::new());
            sched.schedule_mut().add_systems(ant_boundary_reflect_bevy);
            sched.set_dt(dt);

            {
                let mut chunks = sched.world_mut().resource_mut::<MassChunks<AntFragment>>();
                unsafe {
                    chunks.push_primary_slice(MassFragmentSlice {
                        data: ants_bevy.as_mut_ptr() as *mut _,
                        count: ants_bevy.len() as i32,
                        stride: std::mem::size_of::<AntFragment>() as u32,
                    });
                }
            }

            sched.run();
        }

        assert_eq!(ants_direct[0].position, ants_bevy[0].position);
        assert_eq!(ants_direct[0].direction, ants_bevy[0].direction);
    }

    #[test]
    fn bevy_schedule_preserves_ordering() {
        use unreal_api::mass::{MassChunks, MassSchedule};
        use unreal_api::ffi::MassFragmentSlice;
        use unreal_api::ecs::schedule::IntoScheduleConfigs;

        // An ant near the boundary: movement → boundary produces clamped state
        let mut ants = [AntFragment {
            position: [499.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            movement_speed: 100.0,
            ..Default::default()
        }];

        let dt = 1.0f32;
        let mut sched = MassSchedule::new();
        sched.world_mut().insert_resource(MassChunks::<AntFragment>::new());

        // Add systems with explicit ordering: movement before boundary
        sched.schedule_mut().add_systems(
            (ant_movement_bevy, ant_boundary_reflect_bevy).chain()
        );
        sched.set_dt(dt);

        {
            let mut chunks = sched.world_mut().resource_mut::<MassChunks<AntFragment>>();
            unsafe {
                chunks.push_primary_slice(MassFragmentSlice {
                    data: ants.as_mut_ptr() as *mut _,
                    count: ants.len() as i32,
                    stride: std::mem::size_of::<AntFragment>() as u32,
                });
            }
        }

        sched.run();

        assert!(ants[0].position[0] <= 500.0,
            "Position should be clamped after movement+boundary: got {}", ants[0].position[0]);
        assert!(ants[0].direction[0] < 0.0,
            "Direction should reflect after hitting boundary: got {}", ants[0].direction[0]);
    }

    // -----------------------------------------------------------------------
    // Collision pre-pass tests
    // -----------------------------------------------------------------------

    #[test]
    fn collision_prepass_no_callback_clears_encounters() {
        let ants = [AntFragment {
            position: [100.0, 0.0, 0.0],
            previous_position: [99.0, 0.0, 0.0],
            ..Default::default()
        }];
        let mut encounters = [AntEncounterFragment {
            has_encounter: true,
            nearest_food_index: 5,
            encounter_position: [50.0, 0.0, 0.0],
            ..Default::default()
        }];

        let ant_q = unsafe { MassQueryRef::from_raw(ants.as_ptr() as *const _, ants.len()) };
        let mut enc_q = unsafe { MassQueryMut::from_raw(encounters.as_mut_ptr() as *mut _, encounters.len()) };

        ant_collision_prepass_impl(&ant_q, &mut enc_q, None, 15.0);

        assert!(!encounters[0].has_encounter);
        assert_eq!(encounters[0].nearest_food_index, -1);
    }

    #[test]
    fn collision_prepass_finds_nearest_food() {
        // Mock callback that always returns a hit
        unsafe extern "C" fn mock_hit(
            _prev: *const f64,
            _curr: *const f64,
            _radius: f32,
            out: *mut unreal_api::ffi::MassSpatialQueryResult,
        ) -> u32 {
            unsafe {
                (*out).has_encounter = true;
                (*out).food_index = 3;
                (*out).encounter_position = [50.0, 50.0, 0.0];
            }
            1
        }

        let ants = [AntFragment {
            position: [100.0, 0.0, 0.0],
            previous_position: [90.0, 0.0, 0.0],
            ..Default::default()
        }];
        let mut encounters = [AntEncounterFragment::default()];

        let ant_q = unsafe { MassQueryRef::from_raw(ants.as_ptr() as *const _, ants.len()) };
        let mut enc_q = unsafe { MassQueryMut::from_raw(encounters.as_mut_ptr() as *mut _, encounters.len()) };

        ant_collision_prepass_impl(&ant_q, &mut enc_q, Some(mock_hit), 15.0);

        assert!(encounters[0].has_encounter);
        assert_eq!(encounters[0].nearest_food_index, 3);
        assert_eq!(encounters[0].encounter_position, [50.0, 50.0, 0.0]);
    }

    #[test]
    fn collision_prepass_no_hit_clears() {
        // Mock callback that returns no hit
        unsafe extern "C" fn mock_miss(
            _prev: *const f64,
            _curr: *const f64,
            _radius: f32,
            out: *mut unreal_api::ffi::MassSpatialQueryResult,
        ) -> u32 {
            unsafe {
                (*out).has_encounter = false;
                (*out).food_index = -1;
            }
            1
        }

        let ants = [AntFragment {
            position: [100.0, 0.0, 0.0],
            previous_position: [90.0, 0.0, 0.0],
            ..Default::default()
        }];
        let mut encounters = [AntEncounterFragment::default()];

        let ant_q = unsafe { MassQueryRef::from_raw(ants.as_ptr() as *const _, ants.len()) };
        let mut enc_q = unsafe { MassQueryMut::from_raw(encounters.as_mut_ptr() as *mut _, encounters.len()) };

        ant_collision_prepass_impl(&ant_q, &mut enc_q, Some(mock_miss), 15.0);

        assert!(!encounters[0].has_encounter);
        assert_eq!(encounters[0].nearest_food_index, -1);
    }

    #[test]
    fn collision_prepass_bevy_integration() {
        use unreal_api::mass::{MassChunks, MassSchedule, MassSpatialQueryCallback};
        use unreal_api::ffi::MassFragmentSlice;

        unsafe extern "C" fn mock_hit(
            _prev: *const f64,
            _curr: *const f64,
            _radius: f32,
            out: *mut unreal_api::ffi::MassSpatialQueryResult,
        ) -> u32 {
            unsafe {
                (*out).has_encounter = true;
                (*out).food_index = 0;
                (*out).encounter_position = [100.0, 0.0, 0.0];
            }
            1
        }

        let mut ants = [AntFragment {
            position: [100.0, 0.0, 0.0],
            previous_position: [90.0, 0.0, 0.0],
            ..Default::default()
        }];
        let mut encounters = [AntEncounterFragment::default()];

        let mut sched = MassSchedule::new();
        sched.world_mut().insert_resource(MassChunks::<AntFragment>::new());
        sched.world_mut().insert_resource(MassChunks::<AntEncounterFragment>::new());
        sched.world_mut().insert_resource(MassSpatialQueryCallback {
            query_fn: Some(mock_hit),
            pickup_radius: 15.0,
        });
        sched.schedule_mut().add_systems(ant_collision_prepass_bevy);

        // Populate chunks
        {
            let mut ant_chunks = sched.world_mut().resource_mut::<MassChunks<AntFragment>>();
            unsafe {
                ant_chunks.push_primary_slice(MassFragmentSlice {
                    data: ants.as_mut_ptr() as *mut _,
                    count: ants.len() as i32,
                    stride: std::mem::size_of::<AntFragment>() as u32,
                });
            }
        }
        {
            let mut enc_chunks = sched.world_mut().resource_mut::<MassChunks<AntEncounterFragment>>();
            unsafe {
                enc_chunks.push_primary_slice(MassFragmentSlice {
                    data: encounters.as_mut_ptr() as *mut _,
                    count: encounters.len() as i32,
                    stride: std::mem::size_of::<AntEncounterFragment>() as u32,
                });
            }
        }

        sched.run();

        assert!(encounters[0].has_encounter);
        assert_eq!(encounters[0].nearest_food_index, 0);
    }

    #[test]
    fn food_decision_skips_when_no_encounter() {
        let mut ants = [AntFragment {
            position: [100.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            ..Default::default()
        }];
        let encounters = [AntEncounterFragment::default()]; // has_encounter = false

        let mut foods = [FoodFragment { position: [101.0, 0.0, 0.0], is_loose: true, _pad: [0; 7] }];

        let mut ant_q = unsafe { MassQueryMut::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        let enc_q = unsafe { MassQueryRef::from_raw(encounters.as_ptr() as *const _, encounters.len()) };
        let mut storage = MassGlobalChunkStorage::new();
        let mut food_q = unsafe {
            MassQueryAllMut::from_raw_single_chunk(
                foods.as_mut_ptr() as *mut _,
                foods.len(),
                &mut storage,
            )
        };

        ant_food_decision(&mut ant_q, &enc_q, &mut food_q, 0.1);

        assert_eq!(ants[0].carried_food_index, -1);
        assert!(foods[0].is_loose);
    }
}
