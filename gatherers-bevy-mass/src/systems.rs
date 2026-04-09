use unreal_api::mass::{MassQuery, MassQueryAll};
use unreal_api::mass_system;
use bevy_ecs::prelude::With;
use crate::fragments::{
    Position, Movement, Cooldown, Carrying,
    AntEncounterFragment, FoodFragment, BevyMassAntTag,
};

// Re-export facade systems from gatherers-sim (the single source of truth).
// These use Query<&mut T> and Res<DeltaTime> — standard Bevy syntax that
// compiles against both pure Bevy and Unreal Mass Entity.
pub use gatherers_sim::movement::{
    entity_movement, entity_cooldown, entity_boundary_reflect,
    SIM_BOUNDS_MIN, SIM_BOUNDS_MAX,
};

// Re-export helpers used by tests and other systems.
pub use gatherers_sim::movement::{
    normalize_f64x3, is_nearly_zero_f64x3, dot_f64x3,
    reflect_direction, reverse_direction,
};

/// Cooldown applied after picking up or dropping food, in seconds.
const PICKUP_COOLDOWN_SECONDS: f32 = 0.5;

// ---------------------------------------------------------------------------
// System 3: Food decision — pickup/drop logic from pre-computed encounters
// (Unreal-only: uses MassQueryAll for index-based food access)
// ---------------------------------------------------------------------------

#[mass_system(order = 30)]
fn ant_food_decision(
    positions: MassQuery<&mut Position, With<BevyMassAntTag>>,
    movements: MassQuery<&mut Movement, With<BevyMassAntTag>>,
    cooldowns: MassQuery<&mut Cooldown, With<BevyMassAntTag>>,
    carrying: MassQuery<&mut Carrying, With<BevyMassAntTag>>,
    encounters: MassQuery<&AntEncounterFragment, With<BevyMassAntTag>>,
    foods: MassQueryAll<&mut FoodFragment>,
    dt: f32,
) {
    let _ = dt;

    for ((((pos, mov), cd), carry), encounter) in positions.iter_mut()
        .zip(movements.iter_mut())
        .zip(cooldowns.iter_mut())
        .zip(carrying.iter_mut())
        .zip(encounters.iter())
    {
        if !encounter.has_encounter {
            continue;
        }
        if cd.remaining_seconds > 0.0 {
            continue;
        }

        let is_carrying = carry.food_index >= 0;

        if is_carrying {
            // Drop carried food at ant's current position
            if let Some(food) = foods.get_mut(carry.food_index as usize) {
                food.is_loose = true;
                food.position = pos.position;
            }
            carry.food_index = -1;
            mov.direction = reverse_direction(mov.direction);
            cd.remaining_seconds = PICKUP_COOLDOWN_SECONDS;
        } else {
            // Pick up nearest food by index (from C++ collision pre-pass)
            let food_index = encounter.nearest_food_index;
            if food_index >= 0 {
                if let Some(food) = foods.get_mut(food_index as usize) {
                    if food.is_loose {
                        food.is_loose = false;
                        carry.food_index = food_index;
                        mov.direction = reverse_direction(mov.direction);
                        cd.remaining_seconds = PICKUP_COOLDOWN_SECONDS;
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// System 2: Collision pre-pass — detect food encounters via spatial query
// (Unreal-only: uses Res<MassSpatialQueryCallback>)
// ---------------------------------------------------------------------------

#[mass_system(order = 20)]
fn ant_collision_prepass(
    positions: MassQuery<&Position, With<BevyMassAntTag>>,
    encounters: MassQuery<&mut AntEncounterFragment, With<BevyMassAntTag>>,
    spatial: Res<unreal_api::mass::MassSpatialQueryCallback>,
) {
    for (pos, enc) in positions.iter().zip(encounters.iter_mut()) {
        // Reset encounter
        enc.has_encounter = false;
        enc.nearest_food_index = -1;
        enc.encounter_position = [0.0; 3];

        let Some(callback) = spatial.query_fn else {
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
                pos.previous_position.as_ptr(),
                pos.position.as_ptr(),
                spatial.pickup_radius,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fragments::{Position, Movement, Cooldown, Carrying, AntEncounterFragment, FoodFragment};
    use unreal_api::mass::{MassChunks, MassGlobalChunkStorage, MassQueryAllMut, MassQueryMut, MassQueryRef};

    // -----------------------------------------------------------------------
    // Food decision tests (Unreal query types)
    // -----------------------------------------------------------------------

    #[test]
    fn food_decision_pickup_when_encounter() {
        let mut positions = [Position::default()];
        positions[0].position = [100.0, 0.0, 0.0];
        let mut movements = [Movement { direction: [1.0, 0.0, 0.0], movement_speed: 100.0, _pad: [0; 4] }];
        let mut cooldowns = [Cooldown::default()];
        let mut carrying = [Carrying::default()]; // food_index = -1
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

        let pos_q = unsafe { MassQueryMut::from_raw(positions.as_mut_ptr() as *mut _, positions.len()) };
        let mov_q = unsafe { MassQueryMut::from_raw(movements.as_mut_ptr() as *mut _, movements.len()) };
        let cd_q = unsafe { MassQueryMut::from_raw(cooldowns.as_mut_ptr() as *mut _, cooldowns.len()) };
        let carry_q = unsafe { MassQueryMut::from_raw(carrying.as_mut_ptr() as *mut _, carrying.len()) };
        let enc_q = unsafe { MassQueryRef::from_raw(encounters.as_ptr() as *const _, encounters.len()) };
        let mut storage = MassGlobalChunkStorage::new();
        let food_q = unsafe {
            MassQueryAllMut::from_raw_single_chunk(
                foods.as_mut_ptr() as *mut _,
                foods.len(),
                &mut storage,
            )
        };

        ant_food_decision(pos_q, mov_q, cd_q, carry_q, enc_q, food_q, 0.1);

        assert_eq!(carrying[0].food_index, 0);
        assert!(!foods[0].is_loose, "food should no longer be loose");
        assert!(movements[0].direction[0] < 0.0, "direction should reverse");
        assert!(cooldowns[0].remaining_seconds > 0.0);
    }

    #[test]
    fn food_decision_drop_when_carrying_and_encounter() {
        let mut positions = [Position::default()];
        positions[0].position = [200.0, 0.0, 0.0];
        let mut movements = [Movement { direction: [1.0, 0.0, 0.0], movement_speed: 100.0, _pad: [0; 4] }];
        let mut cooldowns = [Cooldown::default()];
        let mut carrying = [Carrying { food_index: 0, _pad: 0 }];
        let encounters = [AntEncounterFragment {
            nearest_food_index: 1,
            encounter_position: [200.0, 0.0, 0.0],
            has_encounter: true,
            ..Default::default()
        }];
        let mut foods = [
            FoodFragment { position: [0.0; 3], is_loose: false, _pad: [0; 7] },
            FoodFragment { position: [201.0, 0.0, 0.0], is_loose: true, _pad: [0; 7] },
        ];

        let pos_q = unsafe { MassQueryMut::from_raw(positions.as_mut_ptr() as *mut _, positions.len()) };
        let mov_q = unsafe { MassQueryMut::from_raw(movements.as_mut_ptr() as *mut _, movements.len()) };
        let cd_q = unsafe { MassQueryMut::from_raw(cooldowns.as_mut_ptr() as *mut _, cooldowns.len()) };
        let carry_q = unsafe { MassQueryMut::from_raw(carrying.as_mut_ptr() as *mut _, carrying.len()) };
        let enc_q = unsafe { MassQueryRef::from_raw(encounters.as_ptr() as *const _, encounters.len()) };
        let mut storage = MassGlobalChunkStorage::new();
        let food_q = unsafe {
            MassQueryAllMut::from_raw_single_chunk(
                foods.as_mut_ptr() as *mut _,
                foods.len(),
                &mut storage,
            )
        };

        ant_food_decision(pos_q, mov_q, cd_q, carry_q, enc_q, food_q, 0.1);

        assert_eq!(carrying[0].food_index, -1);
        assert!(foods[0].is_loose, "dropped food should be loose");
        assert_eq!(foods[0].position, [200.0, 0.0, 0.0]);
        assert!(movements[0].direction[0] < 0.0);
    }

    #[test]
    fn food_decision_skips_when_cooldown_active() {
        let mut positions = [Position::default()];
        positions[0].position = [100.0, 0.0, 0.0];
        let mut movements = [Movement { direction: [1.0, 0.0, 0.0], movement_speed: 100.0, _pad: [0; 4] }];
        let mut cooldowns = [Cooldown { remaining_seconds: 1.0, _pad: [0; 4] }];
        let mut carrying = [Carrying::default()];
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

        let pos_q = unsafe { MassQueryMut::from_raw(positions.as_mut_ptr() as *mut _, positions.len()) };
        let mov_q = unsafe { MassQueryMut::from_raw(movements.as_mut_ptr() as *mut _, movements.len()) };
        let cd_q = unsafe { MassQueryMut::from_raw(cooldowns.as_mut_ptr() as *mut _, cooldowns.len()) };
        let carry_q = unsafe { MassQueryMut::from_raw(carrying.as_mut_ptr() as *mut _, carrying.len()) };
        let enc_q = unsafe { MassQueryRef::from_raw(encounters.as_ptr() as *const _, encounters.len()) };
        let mut storage = MassGlobalChunkStorage::new();
        let food_q = unsafe {
            MassQueryAllMut::from_raw_single_chunk(
                foods.as_mut_ptr() as *mut _,
                foods.len(),
                &mut storage,
            )
        };

        ant_food_decision(pos_q, mov_q, cd_q, carry_q, enc_q, food_q, 0.1);

        assert_eq!(carrying[0].food_index, -1);
        assert!(foods[0].is_loose);
        assert_eq!(movements[0].direction, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn food_decision_skips_when_no_encounter() {
        let mut positions = [Position::default()];
        positions[0].position = [100.0, 0.0, 0.0];
        let mut movements = [Movement::default()];
        let mut cooldowns = [Cooldown::default()];
        let mut carrying = [Carrying::default()];
        let encounters = [AntEncounterFragment::default()]; // has_encounter = false

        let mut foods = [FoodFragment { position: [101.0, 0.0, 0.0], is_loose: true, _pad: [0; 7] }];

        let pos_q = unsafe { MassQueryMut::from_raw(positions.as_mut_ptr() as *mut _, positions.len()) };
        let mov_q = unsafe { MassQueryMut::from_raw(movements.as_mut_ptr() as *mut _, movements.len()) };
        let cd_q = unsafe { MassQueryMut::from_raw(cooldowns.as_mut_ptr() as *mut _, cooldowns.len()) };
        let carry_q = unsafe { MassQueryMut::from_raw(carrying.as_mut_ptr() as *mut _, carrying.len()) };
        let enc_q = unsafe { MassQueryRef::from_raw(encounters.as_ptr() as *const _, encounters.len()) };
        let mut storage = MassGlobalChunkStorage::new();
        let food_q = unsafe {
            MassQueryAllMut::from_raw_single_chunk(
                foods.as_mut_ptr() as *mut _,
                foods.len(),
                &mut storage,
            )
        };

        ant_food_decision(pos_q, mov_q, cd_q, carry_q, enc_q, food_q, 0.1);

        assert_eq!(carrying[0].food_index, -1);
        assert!(foods[0].is_loose);
    }

    // -----------------------------------------------------------------------
    // Collision pre-pass tests
    // -----------------------------------------------------------------------

    #[test]
    fn collision_prepass_no_callback_clears_encounters() {
        let positions = [Position {
            position: [100.0, 0.0, 0.0],
            previous_position: [99.0, 0.0, 0.0],
        }];
        let mut encounters = [AntEncounterFragment {
            has_encounter: true,
            nearest_food_index: 5,
            encounter_position: [50.0, 0.0, 0.0],
            ..Default::default()
        }];

        let pos_q = unsafe { MassQueryRef::from_raw(positions.as_ptr() as *const _, positions.len()) };
        let enc_q = unsafe { MassQueryMut::from_raw(encounters.as_mut_ptr() as *mut _, encounters.len()) };

        let spatial = unreal_api::mass::MassSpatialQueryCallback {
            query_fn: None,
            pickup_radius: 15.0,
        };
        ant_collision_prepass(pos_q, enc_q, &spatial);

        assert!(!encounters[0].has_encounter);
        assert_eq!(encounters[0].nearest_food_index, -1);
    }

    #[test]
    fn collision_prepass_finds_nearest_food() {
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

        let positions = [Position {
            position: [100.0, 0.0, 0.0],
            previous_position: [90.0, 0.0, 0.0],
        }];
        let mut encounters = [AntEncounterFragment::default()];

        let pos_q = unsafe { MassQueryRef::from_raw(positions.as_ptr() as *const _, positions.len()) };
        let enc_q = unsafe { MassQueryMut::from_raw(encounters.as_mut_ptr() as *mut _, encounters.len()) };

        let spatial = unreal_api::mass::MassSpatialQueryCallback {
            query_fn: Some(mock_hit),
            pickup_radius: 15.0,
        };
        ant_collision_prepass(pos_q, enc_q, &spatial);

        assert!(encounters[0].has_encounter);
        assert_eq!(encounters[0].nearest_food_index, 3);
        assert_eq!(encounters[0].encounter_position, [50.0, 50.0, 0.0]);
    }

    #[test]
    fn collision_prepass_no_hit_clears() {
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

        let positions = [Position {
            position: [100.0, 0.0, 0.0],
            previous_position: [90.0, 0.0, 0.0],
        }];
        let mut encounters = [AntEncounterFragment::default()];

        let pos_q = unsafe { MassQueryRef::from_raw(positions.as_ptr() as *const _, positions.len()) };
        let enc_q = unsafe { MassQueryMut::from_raw(encounters.as_mut_ptr() as *mut _, encounters.len()) };

        let spatial = unreal_api::mass::MassSpatialQueryCallback {
            query_fn: Some(mock_miss),
            pickup_radius: 15.0,
        };
        ant_collision_prepass(pos_q, enc_q, &spatial);

        assert!(!encounters[0].has_encounter);
        assert_eq!(encounters[0].nearest_food_index, -1);
    }
}
