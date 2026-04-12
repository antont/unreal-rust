// ---------------------------------------------------------------------------
// Query types: when to use which
// ---------------------------------------------------------------------------
//
// **Facade Query** (`bevy_mass::prelude::Query`):
//   Use for systems that should compile in both standalone Bevy and Unreal modes.
//   Supports single-component and tuple forms, With/Without filters, Entity.
//   The #[mass_system] macro rewrites these to chunk access in Unreal mode.
//   Examples: entity_movement, entity_cooldown, entity_boundary_reflect.
//
// **MassQuery / MassQueryAll** (`unreal_api::mass::{MassQuery, MassQueryAll}`):
//   Use for Unreal-only systems that need access unavailable in standalone mode:
//   - MassSpatialQueries (C++ collision pre-pass results)
//   - MassQueryAll (cross-archetype index-based access, e.g. food by index)
//   These do NOT compile without the `unreal` feature.
//   Examples: ant_collision_prepass, ant_food_decision, carried_food_tracking.
//
// **BevyQuery** (`bevy_ecs::system::Query`):
//   Use for pure-Bevy components that live on shadow entities (not in chunk
//   memory). Always resolves to real bevy_ecs::Query in both modes.
//   The #[mass_system] macro passes these through unchanged.
//   Example: entity_cooldown's BevyQuery<(Entity, &mut Cooldown)>.
// ---------------------------------------------------------------------------

#[allow(unused_imports)] // Used at runtime, not in test cfg
use unreal_api::mass::{MassQuery, MassQueryAll};
use unreal_api::mass_system;
use glam::DVec3;
#[allow(unused_imports)] // Used by #[mass_system] macro expansion
use bevy_ecs::prelude::{With, Without, Entity, Commands};
#[allow(unused_imports)] // Used by #[mass_system] macro expansion
use bevy_ecs::system::Query as BevyQuery;
use crate::fragments::{
    Position, Movement, Cooldown, Carrying,
    AntEncounterFragment, FoodFragment, BevyMassAntTag,
};

// Re-export facade systems from gatherers-sim (the single source of truth).
// These use facade Query<&mut T> and Res<DeltaTime> — standard Bevy syntax
// that compiles against both pure Bevy and Unreal Mass Entity.
pub use gatherers_sim::movement::{
    entity_movement, entity_cooldown, entity_boundary_reflect,
    SIM_BOUNDS_MIN, SIM_BOUNDS_MAX,
};

// Re-export helpers used by tests and other systems.
pub use gatherers_sim::movement::{reflect_direction, reverse_direction};

/// Cooldown applied after picking up or dropping food, in seconds.
const PICKUP_COOLDOWN_SECONDS: f32 = 0.5;

// ---------------------------------------------------------------------------
// System 3: Food decision — pickup/drop logic from pre-computed encounters
// (Unreal-only: uses MassQueryAll for index-based food access)
//
// Without<Cooldown> filter skips ants on cooldown — Cooldown is a pure-Bevy
// component on shadow entities, checked via the facade Query filter mask.
// ---------------------------------------------------------------------------

#[mass_system(order = 30, entity_group = "ants")]
fn ant_food_decision(
    mut ants: MassQuery<
        (Entity, &mut Position, &mut Movement, &mut Carrying, &AntEncounterFragment),
        (With<BevyMassAntTag>, Without<Cooldown>),
    >,
    foods: MassQueryAll<&mut FoodFragment>,
    mut commands: Commands,
) {
    for (entity, mut pos, mut mov, mut carry, encounter) in &mut ants {
        if !encounter.has_encounter {
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
        } else {
            // Pick up nearest food by index (from C++ collision pre-pass)
            let food_index = encounter.nearest_food_index;
            if food_index >= 0 {
                if let Some(food) = foods.get_mut(food_index as usize) {
                    if food.is_loose {
                        food.is_loose = false;
                        carry.food_index = food_index;
                        mov.direction = reverse_direction(mov.direction);
                    }
                }
            }
        }

        commands.entity(entity).insert(Cooldown { remaining_seconds: PICKUP_COOLDOWN_SECONDS });
    }
}

// ---------------------------------------------------------------------------
// System 4: Carried food tracking — update food position to follow carrying ant
// (Unreal-only: uses MassQueryAll for index-based food access)
// ---------------------------------------------------------------------------

#[mass_system(order = 45)]
fn carried_food_tracking(
    positions: MassQuery<&Position, With<BevyMassAntTag>>,
    carrying: MassQuery<&Carrying, With<BevyMassAntTag>>,
    foods: MassQueryAll<&mut FoodFragment>,
) {
    for (pos, carry) in positions.iter().zip(carrying.iter()) {
        if carry.food_index >= 0 {
            if let Some(food) = foods.get_mut(carry.food_index as usize) {
                food.position = pos.position + DVec3::new(0.0, 0.0, 15.0);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// System 2: Collision pre-pass — detect food encounters via spatial query
// (Unreal-only: uses Res<MassSpatialQueries>)
// ---------------------------------------------------------------------------

#[mass_system(order = 20)]
fn ant_collision_prepass(
    positions: MassQuery<&Position, With<BevyMassAntTag>>,
    encounters: MassQuery<&mut AntEncounterFragment, With<BevyMassAntTag>>,
    spatial: Res<unreal_api::mass::MassSpatialQueries>,
) {
    for (pos, enc) in positions.iter().zip(encounters.iter_mut()) {
        // Reset encounter
        enc.has_encounter = false;
        enc.nearest_food_index = -1;
        enc.encounter_position = DVec3::ZERO;

        if let Some(result) = spatial.call("food_pickup", pos.previous_position.as_ref(), pos.position.as_ref()) {
            if result.has_encounter {
                enc.has_encounter = true;
                enc.nearest_food_index = result.entity_index;
                enc.encounter_position = DVec3::from(result.encounter_position);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::DVec3;
    use crate::fragments::{Position, Movement, Carrying, AntEncounterFragment, FoodFragment};
    use unreal_api::mass::{MassGlobalChunkStorage, MassQueryAllMut, MassQueryMut, MassQueryRef};

    /// Helper: construct the facade struct for ant_food_decision tests.
    /// Creates a `__FQ_ant_food_decision_ants` from raw arrays.
    /// Also returns a World (needed to spawn valid entities) and the entity list.
    fn setup_facade_test(count: usize) -> (bevy_ecs::world::World, Vec<bevy_ecs::entity::Entity>) {
        let mut world = bevy_ecs::world::World::new();
        let entities: Vec<_> = (0..count).map(|_| world.spawn_empty().id()).collect();
        (world, entities)
    }

    unsafe fn make_ant_facade<'a>(
        positions: &'a mut [Position],
        movements: &'a mut [Movement],
        carrying: &'a mut [Carrying],
        encounters: &'a [AntEncounterFragment],
        entities: &'a [bevy_ecs::entity::Entity],
        filter_mask: Vec<bool>,
    ) -> __FQ_ant_food_decision_ants<'a> {
        __FQ_ant_food_decision_ants {
            __p0: positions.as_mut_ptr(),
            __p1: movements.as_mut_ptr(),
            __p2: carrying.as_mut_ptr(),
            __p3: encounters.as_ptr(),
            __entities: entities,
            __filter_mask: filter_mask,
            __len: positions.len(),
            __phantom: ::std::marker::PhantomData,
        }
    }

    // -----------------------------------------------------------------------
    // Food decision tests (facade Query)
    // -----------------------------------------------------------------------

    #[test]
    fn food_decision_pickup_when_encounter() {
        let (mut world, entities) = setup_facade_test(1);
        let mut positions = [Position::default()];
        positions[0].position = DVec3::new(100.0, 0.0, 0.0);
        let mut movements = [Movement { direction: DVec3::X, movement_speed: 100.0 }];
        let mut carrying = [Carrying::default()];
        let encounters = [AntEncounterFragment {
            nearest_food_index: 0,
            encounter_position: DVec3::new(100.0, 0.0, 0.0),
            has_encounter: true,
            ..Default::default()
        }];
        let mut foods = [FoodFragment {
            position: DVec3::new(101.0, 0.0, 0.0),
            is_loose: true,
        }];

        let ants = unsafe {
            make_ant_facade(&mut positions, &mut movements, &mut carrying, &encounters, &entities, vec![true])
        };
        let mut storage = MassGlobalChunkStorage::new();
        let food_q = unsafe {
            MassQueryAllMut::from_raw_single_chunk(foods.as_mut_ptr() as *mut _, foods.len(), &mut storage)
        };
        let mut command_queue = bevy_ecs::world::CommandQueue::default();
        let commands = bevy_ecs::system::Commands::new(&mut command_queue, &world);

        ant_food_decision(ants, food_q, commands);

        assert_eq!(carrying[0].food_index, 0);
        assert!(!foods[0].is_loose, "food should no longer be loose");
        assert!(movements[0].direction.x < 0.0, "direction should reverse");
    }

    #[test]
    fn food_decision_drop_when_carrying_and_encounter() {
        let (mut world, entities) = setup_facade_test(1);
        let mut positions = [Position::default()];
        positions[0].position = DVec3::new(200.0, 0.0, 0.0);
        let mut movements = [Movement { direction: DVec3::X, movement_speed: 100.0 }];
        let mut carrying = [Carrying { food_index: 0 }];
        let encounters = [AntEncounterFragment {
            nearest_food_index: 1,
            encounter_position: DVec3::new(200.0, 0.0, 0.0),
            has_encounter: true,
            ..Default::default()
        }];
        let mut foods = [
            FoodFragment { position: DVec3::ZERO, is_loose: false },
            FoodFragment { position: DVec3::new(201.0, 0.0, 0.0), is_loose: true },
        ];

        let ants = unsafe {
            make_ant_facade(&mut positions, &mut movements, &mut carrying, &encounters, &entities, vec![true])
        };
        let mut storage = MassGlobalChunkStorage::new();
        let food_q = unsafe {
            MassQueryAllMut::from_raw_single_chunk(foods.as_mut_ptr() as *mut _, foods.len(), &mut storage)
        };
        let mut command_queue = bevy_ecs::world::CommandQueue::default();
        let commands = bevy_ecs::system::Commands::new(&mut command_queue, &world);

        ant_food_decision(ants, food_q, commands);

        assert_eq!(carrying[0].food_index, -1);
        assert!(foods[0].is_loose, "dropped food should be loose");
        assert_eq!(foods[0].position, DVec3::new(200.0, 0.0, 0.0));
        assert!(movements[0].direction.x < 0.0);
    }

    #[test]
    fn food_decision_skips_when_cooldown_active() {
        let (world, entities) = setup_facade_test(1);
        let mut positions = [Position::default()];
        positions[0].position = DVec3::new(100.0, 0.0, 0.0);
        let mut movements = [Movement { direction: DVec3::X, movement_speed: 100.0 }];
        let mut carrying = [Carrying::default()];
        let encounters = [AntEncounterFragment {
            nearest_food_index: 0,
            encounter_position: DVec3::new(100.0, 0.0, 0.0),
            has_encounter: true,
            ..Default::default()
        }];
        let mut foods = [FoodFragment {
            position: DVec3::new(101.0, 0.0, 0.0),
            is_loose: true,
        }];

        // Filter mask = false → entity has Cooldown → skipped by Without<Cooldown>
        let ants = unsafe {
            make_ant_facade(&mut positions, &mut movements, &mut carrying, &encounters, &entities, vec![false])
        };
        let mut storage = MassGlobalChunkStorage::new();
        let food_q = unsafe {
            MassQueryAllMut::from_raw_single_chunk(foods.as_mut_ptr() as *mut _, foods.len(), &mut storage)
        };
        let mut command_queue = bevy_ecs::world::CommandQueue::default();
        let commands = bevy_ecs::system::Commands::new(&mut command_queue, &world);

        ant_food_decision(ants, food_q, commands);

        assert_eq!(carrying[0].food_index, -1);
        assert!(foods[0].is_loose);
        assert_eq!(movements[0].direction, DVec3::X);
    }

    #[test]
    fn food_decision_skips_when_no_encounter() {
        let (world, entities) = setup_facade_test(1);
        let mut positions = [Position::default()];
        positions[0].position = DVec3::new(100.0, 0.0, 0.0);
        let mut movements = [Movement::default()];
        let mut carrying = [Carrying::default()];
        let encounters = [AntEncounterFragment::default()];

        let mut foods = [FoodFragment { position: DVec3::new(101.0, 0.0, 0.0), is_loose: true }];

        let ants = unsafe {
            make_ant_facade(&mut positions, &mut movements, &mut carrying, &encounters, &entities, vec![true])
        };
        let mut storage = MassGlobalChunkStorage::new();
        let food_q = unsafe {
            MassQueryAllMut::from_raw_single_chunk(foods.as_mut_ptr() as *mut _, foods.len(), &mut storage)
        };
        let mut command_queue = bevy_ecs::world::CommandQueue::default();
        let commands = bevy_ecs::system::Commands::new(&mut command_queue, &world);

        ant_food_decision(ants, food_q, commands);

        assert_eq!(carrying[0].food_index, -1);
        assert!(foods[0].is_loose);
    }

    // -----------------------------------------------------------------------
    // Collision pre-pass tests
    // -----------------------------------------------------------------------

    #[test]
    fn collision_prepass_no_callback_clears_encounters() {
        let positions = [Position {
            position: DVec3::new(100.0, 0.0, 0.0),
            previous_position: DVec3::new(99.0, 0.0, 0.0),
        }];
        let mut encounters = [AntEncounterFragment {
            has_encounter: true,
            nearest_food_index: 5,
            encounter_position: DVec3::new(50.0, 0.0, 0.0),
            ..Default::default()
        }];

        let pos_q = unsafe { MassQueryRef::from_raw(positions.as_ptr() as *const _, positions.len()) };
        let enc_q = unsafe { MassQueryMut::from_raw(encounters.as_mut_ptr() as *mut _, encounters.len()) };

        // Empty MassSpatialQueries — no "food_pickup" registered
        let spatial = unreal_api::mass::MassSpatialQueries::default();
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
                (*out).entity_index = 3;
                (*out).encounter_position = [50.0, 50.0, 0.0];
            }
            1
        }

        let positions = [Position {
            position: DVec3::new(100.0, 0.0, 0.0),
            previous_position: DVec3::new(90.0, 0.0, 0.0),
        }];
        let mut encounters = [AntEncounterFragment::default()];

        let pos_q = unsafe { MassQueryRef::from_raw(positions.as_ptr() as *const _, positions.len()) };
        let enc_q = unsafe { MassQueryMut::from_raw(encounters.as_mut_ptr() as *mut _, encounters.len()) };

        let mut spatial = unreal_api::mass::MassSpatialQueries::default();
        spatial.insert("food_pickup".to_string(), mock_hit, 15.0);
        ant_collision_prepass(pos_q, enc_q, &spatial);

        assert!(encounters[0].has_encounter);
        assert_eq!(encounters[0].nearest_food_index, 3);
        assert_eq!(encounters[0].encounter_position, DVec3::new(50.0, 50.0, 0.0));
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
                (*out).entity_index = -1;
            }
            1
        }

        let positions = [Position {
            position: DVec3::new(100.0, 0.0, 0.0),
            previous_position: DVec3::new(90.0, 0.0, 0.0),
        }];
        let mut encounters = [AntEncounterFragment::default()];

        let pos_q = unsafe { MassQueryRef::from_raw(positions.as_ptr() as *const _, positions.len()) };
        let enc_q = unsafe { MassQueryMut::from_raw(encounters.as_mut_ptr() as *mut _, encounters.len()) };

        let mut spatial = unreal_api::mass::MassSpatialQueries::default();
        spatial.insert("food_pickup".to_string(), mock_miss, 15.0);
        ant_collision_prepass(pos_q, enc_q, &spatial);

        assert!(!encounters[0].has_encounter);
        assert_eq!(encounters[0].nearest_food_index, -1);
    }
}
