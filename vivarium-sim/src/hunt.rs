//! Bird hunt FSM (Phase 2b — Searching → Circling → Diving).
//!
//! Uses a shadow-side `BirdHuntStates` resource keyed by `Entity` rather
//! than a chunk-backed fragment. See Phase 2b plan Part A for rationale.
//!
//! Ported from `/Users/tonialatalo/src/vivarium/src/systems/hunt.rs`
//! with the `BirdLifecycle` phase gate removed (all birds hunt in 2b)
//! and wind drift set to zero (no wind system yet).

use crate::components::{Bird, Predator, Transform, Velocity};
use crate::config::{
    BIRD_EATING_DISTANCE, BIRD_SPEED, HUNT_CIRCLE_DURATION, HUNT_CIRCLE_RADIUS,
    HUNT_DIVE_SPEED_MULT,
};
use bevy_mass::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HuntPhase {
    Searching,
    Circling,
    Diving,
}

/// Per-bird hunt FSM state. Lives in `BirdHuntStates` (shadow-side)
/// because the FSM phase + target entity don't fit the chunk-backed
/// POD fragment mold cleanly and would force a `repr(u8)` enum dance.
#[derive(Clone, Copy, Debug)]
pub struct HuntState {
    pub phase: HuntPhase,
    pub timer: f32,
    pub target: Option<Entity>,
    pub target_pos: DVec3,
}

impl Default for HuntState {
    fn default() -> Self {
        Self {
            phase: HuntPhase::Searching,
            timer: 0.0,
            target: None,
            target_pos: DVec3::ZERO,
        }
    }
}

/// Shadow map of bird `Entity` → current `HuntState`. Birds without an
/// entry are implicitly `Searching` — the map only holds active hunts.
#[derive(Resource, Default)]
pub struct BirdHuntStates(pub HashMap<Entity, HuntState>);

#[mass_system]
pub fn hunt_system(
    spatial: SpatialQueries,
    time: Res<Time>,
    mut hunt_states: ResMut<BirdHuntStates>,
    mut birds: Query<(Entity, &Transform, &mut Velocity, &Predator), With<Bird>>,
) {
    let dt = time.delta_secs();

    for (entity, transform, mut velocity, predator) in &mut birds {
        let bird_pos = transform.translation;
        let forward = velocity.value.normalize_or_zero();

        // Find closest insect inside the sight cone.
        let closest = find_closest_in_cone(
            &spatial,
            bird_pos,
            forward,
            predator.sight_range,
            predator.sight_half_angle,
        );

        // Birds with no entry are implicitly `Searching`. Only insert on
        // the Searching → Circling transition so idle birds don't bloat
        // the map (matches the "map only holds active hunts" invariant).
        let current_phase = hunt_states
            .0
            .get(&entity)
            .map(|s| s.phase)
            .unwrap_or(HuntPhase::Searching);

        match current_phase {
            HuntPhase::Searching => {
                if let Some((target, target_pos)) = closest {
                    hunt_states.0.insert(
                        entity,
                        HuntState {
                            phase: HuntPhase::Circling,
                            timer: 0.0,
                            target: Some(target),
                            target_pos,
                        },
                    );
                }
                // No wander behaviour here — `wander_system` handles that.
                continue;
            }
            HuntPhase::Circling => {
                // Safe to unwrap: current_phase came from an existing entry.
                let state = hunt_states
                    .0
                    .get_mut(&entity)
                    .expect("Circling phase implies an existing HuntStates entry");
                state.timer += dt;

                if let Some((target, target_pos)) = closest {
                    state.target = Some(target);
                    state.target_pos = target_pos;
                }

                // Perpendicular + radial blend steering around target.
                let target_pos = state.target_pos;
                let timer_overflow = state.timer >= HUNT_CIRCLE_DURATION as f32;
                if timer_overflow {
                    state.phase = HuntPhase::Diving;
                    state.timer = 0.0;
                }

                let to_target = target_pos - bird_pos;
                let dist_to_target = to_target.length();
                if dist_to_target > f64::EPSILON {
                    let to_target_norm = to_target / dist_to_target;
                    let perp_seed = to_target_norm.cross(DVec3::Y);
                    let perp = if perp_seed.length_squared() > 1e-12 {
                        perp_seed.normalize()
                    } else {
                        to_target_norm.cross(DVec3::X).normalize_or_zero()
                    };
                    let radial = if dist_to_target > HUNT_CIRCLE_RADIUS {
                        to_target_norm * 0.5
                    } else {
                        -to_target_norm * 0.3
                    };
                    let steer = perp * 2.0 + radial;
                    let speed = velocity.value.length();
                    velocity.value = (velocity.value + steer).normalize_or_zero() * speed;
                }
            }
            HuntPhase::Diving => {
                let target_pos = hunt_states
                    .0
                    .get(&entity)
                    .expect("Diving phase implies an existing HuntStates entry")
                    .target_pos;

                let to_target = target_pos - bird_pos;
                let dist = to_target.length();

                if dist > f64::EPSILON {
                    let dive_speed = BIRD_SPEED as f64 * HUNT_DIVE_SPEED_MULT;
                    velocity.value = (to_target / dist) * dive_speed;
                }

                // Exit dive on proximity or target loss — drop back to
                // Searching and restore cruise speed.
                if dist < BIRD_EATING_DISTANCE * 2.0 || closest.is_none() {
                    velocity.value = velocity.value.normalize_or_zero() * BIRD_SPEED as f64;
                    hunt_states.0.remove(&entity);
                }
            }
        }
    }
}

/// Scan the "insects" spatial group around `bird_pos`, return the closest
/// insect whose direction is within `sight_half_angle` of `forward`.
fn find_closest_in_cone(
    spatial: &SpatialQueries,
    bird_pos: DVec3,
    forward: DVec3,
    sight_range: f64,
    sight_half_angle: f64,
) -> Option<(Entity, DVec3)> {
    if forward.length_squared() < 1e-12 {
        return None;
    }
    let neighbors = spatial.neighbors_within("insects", &bird_pos, sight_range, None);
    let mut closest: Option<(Entity, DVec3, f64)> = None;
    for n in &neighbors {
        let to_insect = n.position - bird_pos;
        let dist = to_insect.length();
        if dist < f64::EPSILON {
            continue;
        }
        let to_norm = to_insect / dist;
        let cos_angle = forward.dot(to_norm).clamp(-1.0, 1.0);
        let angle = cos_angle.acos();
        if angle < sight_half_angle
            && (closest.is_none() || dist < closest.unwrap().2)
        {
            closest = Some((n.entity, n.position, dist));
        }
    }
    closest.map(|(e, p, _)| (e, p))
}

#[cfg(all(test, not(feature = "unreal")))]
mod tests {
    use super::*;
    use crate::components::{Insect, PreviousTranslation};
    use crate::config::{BIRD_SIGHT_HALF_ANGLE, BIRD_SIGHT_RANGE};
    use bevy_ecs::prelude::*;
    use core::time::Duration;

    fn insert_time(world: &mut World, dt: f32) {
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(dt));
        world.insert_resource(time);
    }

    /// Build an app with the insects `SpatialGroupPlugin` so the grid is
    /// populated before `hunt_system` runs. Returns the world after one tick.
    fn run_with_plugin<F>(setup: F, dt: f32) -> World
    where
        F: FnOnce(&mut World),
    {
        use bevy_app::App;
        let mut app = App::new();
        app.add_plugins(SpatialGroupPlugin::<Insect, Transform>::new(
            "insects",
            BIRD_SIGHT_RANGE,
        ));
        app.insert_resource(BirdHuntStates::default());
        app.add_systems(
            bevy_app::Update,
            hunt_system.in_set(SpatialGroupSet::Query),
        );
        insert_time(app.world_mut(), dt);
        setup(app.world_mut());
        app.update();
        let mut temp_world = World::new();
        std::mem::swap(app.world_mut(), &mut temp_world);
        temp_world
    }

    fn spawn_bird(world: &mut World) -> Entity {
        world
            .spawn((
                Bird,
                Transform::from_translation(DVec3::ZERO),
                PreviousTranslation::default(),
                Velocity::new(DVec3::X, BIRD_SPEED),
                Predator {
                    sight_range: BIRD_SIGHT_RANGE,
                    sight_half_angle: BIRD_SIGHT_HALF_ANGLE,
                },
            ))
            .id()
    }

    fn spawn_insect(world: &mut World, pos: DVec3) -> Entity {
        world
            .spawn((
                Insect,
                Transform::from_translation(pos),
                PreviousTranslation { value: pos },
            ))
            .id()
    }

    #[test]
    fn searching_to_circling_on_insect_in_cone() {
        let mut bird_entity = None;
        let insect_pos = DVec3::new(40.0, 0.0, 0.0);
        let world = run_with_plugin(
            |w| {
                bird_entity = Some(spawn_bird(w));
                spawn_insect(w, insect_pos);
            },
            1.0 / 60.0,
        );

        let states = world.resource::<BirdHuntStates>();
        let state = states
            .0
            .get(&bird_entity.unwrap())
            .expect("bird should have a hunt state entry after spotting insect");
        assert_eq!(state.phase, HuntPhase::Circling, "phase should flip to Circling");
        assert!(
            (state.target_pos - insect_pos).length() < 1e-6,
            "target_pos should equal insect position: got {:?}, want {:?}",
            state.target_pos,
            insect_pos,
        );
    }

    #[test]
    fn insect_outside_cone_stays_searching() {
        let mut bird_entity = None;
        let world = run_with_plugin(
            |w| {
                bird_entity = Some(spawn_bird(w));
                // 90° off +X forward — well outside a 0.7 rad half-angle cone.
                spawn_insect(w, DVec3::new(0.0, 40.0, 0.0));
            },
            1.0 / 60.0,
        );

        let states = world.resource::<BirdHuntStates>();
        assert!(
            !states.0.contains_key(&bird_entity.unwrap()),
            "bird stays implicitly Searching — no entry should be inserted",
        );
    }

    #[test]
    fn circling_to_diving_on_timer() {
        let mut bird_entity = None;
        let dt: f32 = 0.02;
        let world = run_with_plugin(
            |w| {
                let bird = spawn_bird(w);
                let mut states = w.resource_mut::<BirdHuntStates>();
                states.0.insert(
                    bird,
                    HuntState {
                        phase: HuntPhase::Circling,
                        timer: (HUNT_CIRCLE_DURATION as f32) - 0.01,
                        target: None,
                        target_pos: DVec3::ZERO,
                    },
                );
                bird_entity = Some(bird);
            },
            dt,
        );

        let states = world.resource::<BirdHuntStates>();
        let state = states
            .0
            .get(&bird_entity.unwrap())
            .expect("bird should retain hunt state through Circling → Diving transition");
        assert_eq!(state.phase, HuntPhase::Diving, "timer overflow should advance to Diving");
        assert_eq!(state.timer, 0.0, "timer should reset on phase transition");
    }

    #[test]
    fn diving_target_lost_reverts_to_searching() {
        let mut bird_entity = None;
        // Pick any valid dummy entity to serve as the prior target.
        let dummy_target = Entity::from_raw_u32(999).unwrap();
        let world = run_with_plugin(
            |w| {
                let bird = spawn_bird(w);
                let mut states = w.resource_mut::<BirdHuntStates>();
                states.0.insert(
                    bird,
                    HuntState {
                        phase: HuntPhase::Diving,
                        timer: 0.0,
                        target: Some(dummy_target),
                        target_pos: DVec3::new(1000.0, 1000.0, 1000.0),
                    },
                );
                bird_entity = Some(bird);
                // No insect spawned — `closest` will be None.
            },
            1.0 / 60.0,
        );

        let states = world.resource::<BirdHuntStates>();
        assert!(
            states.0.get(&bird_entity.unwrap()).is_none(),
            "target loss should remove entry, reverting bird to implicit Searching",
        );
    }
}
