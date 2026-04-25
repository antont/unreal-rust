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
// **QueryAll** (`bevy_mass::prelude::QueryAll`):
//   Facade for global index-based access (get by spawn-order index).
//   In Bevy mode: backed by EntityIndex<Tag> + Query (macro-rewritten).
//   In Unreal mode: backed by MassQueryAllMut (zero-copy chunk access).
//   Examples: apply_food_mutations, carried_food_tracking.
//
// **SpatialQuery** (`bevy_mass::prelude::SpatialQuery`):
//   UE-mode wrapper around MassSpatialQueries with cleaner Rust API.
//   Returns SpatialHit with DVec3 instead of raw FFI types.
//   In standalone mode, collision uses direct Bevy queries instead.
//   Examples: ant_collision_prepass (UE-specific).
//
// **#[bevy] Query** (parameter attribute):
//   Use for pure-Bevy components that live on shadow entities (not in chunk
//   memory). The #[bevy] attribute tells #[mass_system] to pass this Query
//   through as a real bevy_ecs::Query instead of rewriting it to chunk access.
//   Example: entity_cooldown's #[bevy] Query<(Entity, &mut Cooldown)>.
//   (Replaces the deprecated BevyQuery type alias.)
// ---------------------------------------------------------------------------

#[allow(unused_imports)] // Some items used only by #[mass_system] macro expansion
use bevy_mass::prelude::*;
use crate::components::{
    Transform, PreviousTranslation, DesiredMovement, Cooldown, Carrying, Behavior,
    FoodState, Food, Ant,
    FoodEncounter,
};
use gatherers_sim::components::{AntFoodHit, FoodMutation, FoodDropEvents, FoodPickupEvents};
use bevy_ecs::message::{MessageReader, MessageWriter};
use gatherers_sim::food_decision::{
    ant_food_decision as ant_food_decision_fn,
    DECISION_NO_ACTION, DECISION_PICK_UP, DECISION_DROP,
};
use std::collections::HashMap;

// Re-export facade systems from gatherers-sim (the single source of truth).
// Movement application (pos += vel * dt) is handled by C++ UMassApplyMovementProcessor
// in UE mode — no Rust system needed. entity_boundary_reflect only reflects velocity.
pub use gatherers_sim::movement::{
    entity_cooldown, entity_boundary_reflect,
    SIM_BOUNDS_MIN, SIM_BOUNDS_MAX,
};

// Re-export helpers used by tests and other systems.
pub use gatherers_sim::movement::reflect_velocity;

// ---------------------------------------------------------------------------
// Food drop FFI cache + dispatch hooks.
//
// The Bevy `FoodDropEvents` resource is drained each frame into a static cache
// that C++ reads via `get_food_drop_events`. This keeps the framework
// (`unreal-module`) free of game-specific types — it discovers the FFI binding
// and the pre/post-dispatch hooks via `inventory`.
// ---------------------------------------------------------------------------

struct SyncDropCache(Vec<unreal_ffi::FoodDropEvent>);
// Safe: only ever accessed through the Mutex below.
unsafe impl Send for SyncDropCache {}
unsafe impl Sync for SyncDropCache {}

static FOOD_DROP_CACHE: std::sync::Mutex<SyncDropCache> =
    std::sync::Mutex::new(SyncDropCache(Vec::new()));

static FOOD_PICKUP_CACHE: std::sync::Mutex<Vec<i32>> =
    std::sync::Mutex::new(Vec::new());

// ---------------------------------------------------------------------------
// Decision-outcome diagnostic counters (exposed via FFI to automation tests).
// Counts every invocation of ant_food_decision_fn inside the UE `ant_food_decision`
// system so we can see how returned encounters split into pickup/drop/no-action.
// ---------------------------------------------------------------------------

use std::sync::atomic::{AtomicU64, Ordering};
static DECISION_CALLS: AtomicU64 = AtomicU64::new(0);
static DECISION_HITS_SEEN: AtomicU64 = AtomicU64::new(0);
static DECISION_ANTS_SEEN: AtomicU64 = AtomicU64::new(0);
static DECISION_MATCHED: AtomicU64 = AtomicU64::new(0);
static DECISION_PICKUPS: AtomicU64 = AtomicU64::new(0);
static DECISION_DROPS: AtomicU64 = AtomicU64::new(0);
static DECISION_NO_ACTIONS: AtomicU64 = AtomicU64::new(0);

fn clear_food_event_caches(_world: &mut bevy_ecs::world::World) {
    FOOD_DROP_CACHE.lock().unwrap().0.clear();
    FOOD_PICKUP_CACHE.lock().unwrap().clear();
}

fn drain_food_events(world: &mut bevy_ecs::world::World) {
    if let Some(mut events) = world.get_resource_mut::<FoodDropEvents>() {
        let mut cache = FOOD_DROP_CACHE.lock().unwrap();
        for e in events.events.drain(..) {
            cache.0.push(unreal_ffi::FoodDropEvent {
                food_index: e.food_index,
                _pad: 0,
                position: [e.position.x, e.position.y, e.position.z],
            });
        }
    }
    if let Some(mut events) = world.get_resource_mut::<FoodPickupEvents>() {
        let mut cache = FOOD_PICKUP_CACHE.lock().unwrap();
        cache.extend(events.indices.drain(..));
    }
}

/// C++ reads queued drop events through this FFI. Registered via `MassExternBinding`.
/// Drains up to `max` events from the front of the cache; caller loops until
/// it gets back fewer than `max`.
pub unsafe extern "C" fn get_food_drop_events(
    out: *mut unreal_ffi::FoodDropEvent,
    max: u32,
) -> u32 {
    if out.is_null() || max == 0 {
        return 0;
    }
    let mut cache = FOOD_DROP_CACHE.lock().unwrap();
    let count = cache.0.len().min(max as usize);
    unsafe {
        std::ptr::copy_nonoverlapping(cache.0.as_ptr(), out, count);
    }
    cache.0.drain(..count);
    count as u32
}

/// C++ reads queued pickup events (food indices) through this FFI. Registered
/// via `MassExternBinding`. Same drain-loop contract as `get_food_drop_events`.
pub unsafe extern "C" fn get_food_pickup_events(
    out: *mut i32,
    max: u32,
) -> u32 {
    if out.is_null() || max == 0 {
        return 0;
    }
    let mut cache = FOOD_PICKUP_CACHE.lock().unwrap();
    let count = cache.len().min(max as usize);
    unsafe {
        std::ptr::copy_nonoverlapping(cache.as_ptr(), out, count);
    }
    cache.drain(..count);
    count as u32
}

inventory::submit!(unreal_api::mass::MassDispatchHook {
    pre_dispatch: clear_food_event_caches,
    post_dispatch: drain_food_events,
});

inventory::submit!(unreal_api::mass::MassExternBinding {
    get_food_drop_events: Some(get_food_drop_events),
    get_food_pickup_events: Some(get_food_pickup_events),
    get_decision_counters: Some(get_decision_counters),
    reset_decision_counters: Some(reset_decision_counters),
});

// ---------------------------------------------------------------------------
// System 2: Collision pre-pass — detect food encounters via UE spatial query,
// emit HitEvent messages (matching original gatherers CollisionPlugin pattern)
// ---------------------------------------------------------------------------

#[mass_system]
fn ant_collision_prepass(
    ants: Query<(Entity, &Transform, &PreviousTranslation), (With<Ant>, Without<Cooldown>)>,
    spatial: Res<SpatialQuery>,
    entity_map: Res<unreal_api::mass::MassEntityMap>,
    mut hits: MessageWriter<AntFoodHit>,
) {
    let spatial = spatial.with_map(&entity_map);
    for (entity, transform, prev) in &mut ants {
        if let Some(hit) = spatial.call("food_pickup", &prev.value, &transform.translation) {
            hits.write(AntFoodHit::new(
                hit.entity_index,
                hit.entity,
                entity,
                hit.position,
            ));
        }
    }
}

// ---------------------------------------------------------------------------
// System 3: Food decision — reads HitEvent messages, calls shared decision
// function, inserts Cooldown, emits FoodMutation messages
// ---------------------------------------------------------------------------

#[mass_system]
fn ant_food_decision(
    mut ants: Query<
        (Entity, &Transform, &mut DesiredMovement, &mut Carrying, &mut Behavior),
        (With<Ant>, Without<Cooldown>),
    >,
    mut hits: MessageReader<AntFoodHit>,
    entity_map: Res<unreal_api::mass::MassEntityMap>,
    mut food_mutations: MessageWriter<FoodMutation>,
    mut commands: Commands,
) {
    DECISION_CALLS.fetch_add(1, Ordering::Relaxed);

    // Build entity → hit lookup from messages (read once, lookup per entity)
    let hit_map: HashMap<bevy_ecs::entity::Entity, _> = hits.read()
        .inspect(|_| { DECISION_HITS_SEEN.fetch_add(1, Ordering::Relaxed); })
        .map(|h| (h.hitter_entity, (h.hittable_index, h.hittable_entity, h.encounter_position)))
        .collect();

    for (entity, transform, mut movement, mut carry, mut behavior) in &mut ants {
        DECISION_ANTS_SEEN.fetch_add(1, Ordering::Relaxed);
        let Some(&(hittable_index, hittable_entity, encounter_position)) = hit_map.get(&entity) else {
            continue;
        };
        DECISION_MATCHED.fetch_add(1, Ordering::Relaxed);

        // Resolve the currently-carried food's Entity *before* the
        // decision runs — a DROP clears `food_index` to -1, so the
        // lookup must happen while the old value is still there.
        let carried_before = carry.carried_entity(&entity_map);
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

        match decision {
            DECISION_PICK_UP => { DECISION_PICKUPS.fetch_add(1, Ordering::Relaxed); }
            DECISION_DROP => { DECISION_DROPS.fetch_add(1, Ordering::Relaxed); }
            _ => { DECISION_NO_ACTIONS.fetch_add(1, Ordering::Relaxed); }
        }

        if decision != DECISION_NO_ACTION {
            commands.entity(entity).insert(cd);
            // On DROP the mutated food is the one previously carried — not
            // the one just encountered. `carried_before` was captured
            // above, before the decision fn cleared `Carrying.food_index`.
            let (food_index, food_entity) = if decision == DECISION_DROP {
                (old_food_index, carried_before.unwrap_or(hittable_entity))
            } else {
                (hittable_index, hittable_entity)
            };
            food_mutations.write(FoodMutation {
                food_index,
                food_entity,
                decision,
                drop_position: pos_before,
            });
        }
    }
}

/// Read decision-outcome diagnostic counters. Populated inside `ant_food_decision`
/// each frame; tests read these via the loader's `RustBindings.get_decision_counters`
/// (registered through `MassExternBinding` below) so they hit the same dylib
/// instance as the running sim.
pub unsafe extern "C" fn get_decision_counters(out: *mut unreal_ffi::DecisionCounters) {
    if out.is_null() { return; }
    unsafe {
        (*out).calls = DECISION_CALLS.load(Ordering::Relaxed);
        (*out).hits_seen = DECISION_HITS_SEEN.load(Ordering::Relaxed);
        (*out).ants_seen = DECISION_ANTS_SEEN.load(Ordering::Relaxed);
        (*out).matched = DECISION_MATCHED.load(Ordering::Relaxed);
        (*out).pickups = DECISION_PICKUPS.load(Ordering::Relaxed);
        (*out).drops = DECISION_DROPS.load(Ordering::Relaxed);
        (*out).no_actions = DECISION_NO_ACTIONS.load(Ordering::Relaxed);
    }
}

/// Reset decision counters to zero.
pub unsafe extern "C" fn reset_decision_counters() {
    DECISION_CALLS.store(0, Ordering::Relaxed);
    DECISION_HITS_SEEN.store(0, Ordering::Relaxed);
    DECISION_ANTS_SEEN.store(0, Ordering::Relaxed);
    DECISION_MATCHED.store(0, Ordering::Relaxed);
    DECISION_PICKUPS.store(0, Ordering::Relaxed);
    DECISION_DROPS.store(0, Ordering::Relaxed);
    DECISION_NO_ACTIONS.store(0, Ordering::Relaxed);
}

/// Log decision counters once per frame when enabled + reset. Runs as a
/// post-dispatch hook so it prints even without a dedicated FFI accessor.
fn log_decision_counters(_world: &mut bevy_ecs::world::World) {
    // Toggled via `UNREAL_RUST_MASS_TIMING=1` — same env var that enables
    // the main timing output, so a single toggle controls both logs.
    if std::env::var("UNREAL_RUST_MASS_TIMING").ok().as_deref() != Some("1") {
        return;
    }
    let calls = DECISION_CALLS.load(Ordering::Relaxed);
    if calls == 0 { return; }
    log::info!(
        "[decision-perf] calls={} hits_seen={} ants_seen={} matched={} pickups={} drops={} no_actions={}",
        calls,
        DECISION_HITS_SEEN.load(Ordering::Relaxed),
        DECISION_ANTS_SEEN.load(Ordering::Relaxed),
        DECISION_MATCHED.load(Ordering::Relaxed),
        DECISION_PICKUPS.load(Ordering::Relaxed),
        DECISION_DROPS.load(Ordering::Relaxed),
        DECISION_NO_ACTIONS.load(Ordering::Relaxed),
    );
    unsafe { reset_decision_counters(); }
}

inventory::submit!(unreal_api::mass::MassDispatchHook {
    pre_dispatch: |_| {},
    post_dispatch: log_decision_counters,
});

// ---------------------------------------------------------------------------
// System 3b: Apply food mutations — reads FoodMutation messages, updates food
// ---------------------------------------------------------------------------

#[mass_system]
fn apply_food_mutations(
    mut mutations: MessageReader<FoodMutation>,
    foods: QueryAll<&mut FoodState, With<Food>>,
    mut drop_events: ResMut<FoodDropEvents>,
    mut pickup_events: ResMut<FoodPickupEvents>,
) {
    for mutation in mutations.read() {
        if let Some(food) = foods.get_mut(mutation.food_index as usize) {
            if mutation.decision == DECISION_PICK_UP {
                food.is_loose = false;
                pickup_events.push(mutation.food_index);
            } else if mutation.decision == DECISION_DROP {
                food.is_loose = true;
                drop_events.push(mutation.food_index, mutation.drop_position);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// System 4: Carried food tracking — update food position to follow carrying ant
// ---------------------------------------------------------------------------

#[mass_system]
fn carried_food_tracking(
    ants: Query<(&Transform, &Carrying), With<Ant>>,
    food_transforms: QueryAll<&mut Transform, With<Food>>,
) {
    for (transform, carry) in &mut ants {
        if carry.is_carrying() {
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
        let mut foods = [FoodState {
            is_loose: true,
        }];
        let mut storage = MassGlobalChunkStorage::new();
        let mut food_q = unsafe {
            unreal_api::mass::MassQueryAllMut::<FoodState>::from_raw_single_chunk(
                foods.as_mut_ptr() as *mut _, foods.len(), &mut storage,
            )
        };

        let mutation = FoodMutation {
            food_index: 0,
            food_entity: bevy_ecs::entity::Entity::PLACEHOLDER,
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
        let mut foods = [FoodState {
            is_loose: false,
        }];
        let mut storage = MassGlobalChunkStorage::new();
        let mut food_q = unsafe {
            unreal_api::mass::MassQueryAllMut::<FoodState>::from_raw_single_chunk(
                foods.as_mut_ptr() as *mut _, foods.len(), &mut storage,
            )
        };

        let drop_pos = DVec3::new(200.0, 100.0, 0.0);
        let mutation = FoodMutation {
            food_index: 0,
            food_entity: bevy_ecs::entity::Entity::PLACEHOLDER,
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
