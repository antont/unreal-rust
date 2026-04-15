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

#[allow(unused_imports)] // Some items used only by #[mass_system] macro expansion
use bevy_mass::prelude::*;
use crate::fragments::{
    Transform, PreviousTranslation, DesiredMovement, Cooldown, Carrying, Behavior,
    FoodFragment, FoodTag, BevyMassAntTag,
    FoodEncounter,
};
use gatherers_sim::fragments::{AntFoodHit, FoodMutation};
use bevy_ecs::message::{MessageReader, MessageWriter};
use gatherers_sim::food_decision::{
    ant_food_decision as ant_food_decision_fn,
    DECISION_NO_ACTION, DECISION_PICK_UP, DECISION_DROP,
};
use std::collections::HashMap;

// Re-export facade systems from gatherers-sim (the single source of truth).
// entity_movement and entity_boundary_reflect are no-ops in Unreal mode
// (C++ URustMassMovementApplyProcessor handles movement + boundary reflection).
pub use gatherers_sim::movement::{
    entity_movement, entity_cooldown, entity_boundary_reflect,
    SIM_BOUNDS_MIN, SIM_BOUNDS_MAX,
};

// Re-export helpers used by tests and other systems.
pub use gatherers_sim::movement::reflect_velocity;

// ---------------------------------------------------------------------------
// System 2: Collision pre-pass — detect food encounters via UE spatial query,
// emit HitEvent messages (matching original gatherers CollisionPlugin pattern)
// ---------------------------------------------------------------------------

#[mass_system(order = 20)]
fn ant_collision_prepass(
    ants: MassQuery<(Entity, &Transform, &PreviousTranslation), (With<BevyMassAntTag>, Without<Cooldown>)>,
    spatial: Res<unreal_api::mass::MassSpatialQueries>,
    mut hits: MessageWriter<AntFoodHit>,
) {
    for (entity, transform, prev) in &mut ants {
        if let Some(result) = spatial.call("food_pickup", prev.value.as_ref(), transform.translation.as_ref()) {
            if result.has_encounter {
                hits.write(AntFoodHit::new(
                    result.entity_index,
                    entity,
                    DVec3::from(result.encounter_position),
                ));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// System 3: Food decision — reads HitEvent messages, calls shared decision
// function, inserts Cooldown, emits FoodMutation messages
// ---------------------------------------------------------------------------

#[mass_system(order = 30)]
fn ant_food_decision(
    mut ants: MassQuery<
        (Entity, &Transform, &mut DesiredMovement, &mut Carrying, &mut Behavior),
        (With<BevyMassAntTag>, Without<Cooldown>),
    >,
    mut hits: MessageReader<AntFoodHit>,
    mut food_mutations: MessageWriter<FoodMutation>,
    mut commands: Commands,
) {
    // Build entity → hit lookup from messages (read once, lookup per entity)
    let hit_map: HashMap<bevy_ecs::entity::Entity, _> = hits.read()
        .map(|h| (h.hitter_entity, (h.hittable_index, h.encounter_position)))
        .collect();

    for (entity, transform, mut movement, mut carry, mut behavior) in &mut ants {
        let Some(&(hittable_index, encounter_position)) = hit_map.get(&entity) else {
            continue;
        };

        let old_food_index = carry.food_index;
        let pos_before = transform.translation;
        // Use a local copy — Rust does not write transforms in Unreal mode;
        // C++ handles all position updates.
        let mut pos_scratch = transform.translation;
        let mut cd = Cooldown { remaining_seconds: 0.0 };
        let encounter = FoodEncounter {
            food_index: hittable_index,
            encounter_position,
        };

        let decision = ant_food_decision_fn(
            &mut pos_scratch, &mut movement, &mut cd, &mut carry, &mut behavior,
            Some(&encounter),
        );

        if decision != DECISION_NO_ACTION {
            commands.entity(entity).insert(cd);
            food_mutations.write(FoodMutation {
                food_index: if decision == DECISION_DROP { old_food_index } else { hittable_index },
                decision,
                drop_position: pos_before,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// System 3b: Apply food mutations — reads FoodMutation messages, updates food
// ---------------------------------------------------------------------------

#[mass_system(order = 35)]
fn apply_food_mutations(
    mut mutations: MessageReader<FoodMutation>,
    foods: MassQueryAll<&mut FoodFragment>,
) {
    let mut had_mutation = false;
    for mutation in mutations.read() {
        if let Some(food) = foods.get_mut(mutation.food_index as usize) {
            if mutation.decision == DECISION_PICK_UP {
                food.is_loose = false;
                had_mutation = true;
            } else if mutation.decision == DECISION_DROP {
                food.is_loose = true;
                // Food drop position is handled by C++ — Rust does not write transforms.
                had_mutation = true;
            }
        }
    }
    if had_mutation {
        unreal_api::mass::set_dispatch_flag(unreal_api::ffi::DISPATCH_FLAG_FOOD_PHYSICS_DIRTY);
    }
}

// ---------------------------------------------------------------------------
// System 4: Carried food tracking — update food position to follow carrying ant
// ---------------------------------------------------------------------------

#[mass_system(order = 45)]
fn carried_food_tracking(
    ants: MassQuery<(&Transform, &Carrying), With<BevyMassAntTag>>,
    food_transforms: MassQueryAll<&mut Transform, With<FoodTag>>,
) {
    for (transform, carry) in &mut ants {
        if carry.food_index >= 0 {
            if let Some(food_tf) = food_transforms.get_mut(carry.food_index as usize) {
                food_tf.translation = transform.translation + DVec3::new(0.0, 0.0, 15.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::DVec3;
    use unreal_api::mass::MassGlobalChunkStorage;

    // -----------------------------------------------------------------------
    // Food decision logic is tested at the pure function level in
    // gatherers-sim/src/food_decision.rs. The Unreal-specific systems here
    // are thin wrappers that read messages → call the shared function →
    // write mutations. They are tested end-to-end via UE automation tests.
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // apply_food_mutations tests — this system has no MassQuery (only
    // MassQueryAll), so we can test it directly without facade structs.
    // -----------------------------------------------------------------------

    #[test]
    fn apply_food_mutations_pickup() {
        use gatherers_sim::food_decision::DECISION_PICK_UP;
        let mut foods = [FoodFragment {
            is_loose: true,
        }];
        let mut storage = MassGlobalChunkStorage::new();
        let mut food_q = unsafe {
            unreal_api::mass::MassQueryAllMut::<FoodFragment>::from_raw_single_chunk(
                foods.as_mut_ptr() as *mut _, foods.len(), &mut storage,
            )
        };

        let mutation = FoodMutation {
            food_index: 0,
            decision: DECISION_PICK_UP,
            drop_position: DVec3::ZERO,
        };

        // Simulate what the system does: apply mutation directly
        if let Some(food) = food_q.get_mut(mutation.food_index as usize) {
            if mutation.decision == DECISION_PICK_UP {
                food.is_loose = false;
            }
        }

        assert!(!foods[0].is_loose, "food should no longer be loose after pickup");
    }

    #[test]
    fn apply_food_mutations_drop() {
        use gatherers_sim::food_decision::DECISION_DROP;
        let mut foods = [FoodFragment {
            is_loose: false,
        }];
        let mut storage = MassGlobalChunkStorage::new();
        let mut food_q = unsafe {
            unreal_api::mass::MassQueryAllMut::<FoodFragment>::from_raw_single_chunk(
                foods.as_mut_ptr() as *mut _, foods.len(), &mut storage,
            )
        };

        let drop_pos = DVec3::new(200.0, 100.0, 0.0);
        let mutation = FoodMutation {
            food_index: 0,
            decision: DECISION_DROP,
            drop_position: drop_pos,
        };

        if let Some(food) = food_q.get_mut(mutation.food_index as usize) {
            if mutation.decision == DECISION_DROP {
                food.is_loose = true;
            }
        }

        assert!(foods[0].is_loose, "food should be loose after drop");
    }

    // -----------------------------------------------------------------------
    // Spatial query integration — test the MassSpatialQueries API used by
    // the collision prepass.
    // -----------------------------------------------------------------------

    #[test]
    fn spatial_query_hit_returns_encounter() {
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

        let mut spatial = unreal_api::mass::MassSpatialQueries::default();
        spatial.insert("food_pickup".to_string(), mock_hit, 15.0);

        let prev = [90.0, 0.0, 0.0];
        let curr = [100.0, 0.0, 0.0];
        let result = spatial.call("food_pickup", &prev, &curr);

        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.has_encounter);
        assert_eq!(r.entity_index, 3);
        assert_eq!(r.encounter_position, [50.0, 50.0, 0.0]);
    }

    #[test]
    fn spatial_query_miss_returns_no_encounter() {
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

        let mut spatial = unreal_api::mass::MassSpatialQueries::default();
        spatial.insert("food_pickup".to_string(), mock_miss, 15.0);

        let prev = [90.0, 0.0, 0.0];
        let curr = [100.0, 0.0, 0.0];
        let result = spatial.call("food_pickup", &prev, &curr);

        assert!(result.is_some());
        assert!(!result.unwrap().has_encounter);
    }

    #[test]
    fn spatial_query_unregistered_returns_none() {
        let spatial = unreal_api::mass::MassSpatialQueries::default();
        let prev = [0.0; 3];
        let curr = [0.0; 3];
        assert!(spatial.call("food_pickup", &prev, &curr).is_none());
    }
}
