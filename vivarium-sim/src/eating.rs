//! Bird eating (Phase 2b). When a bird gets within `BIRD_EATING_DISTANCE`
//! of an insect, despawn the insect and emit an `InsectEaten` message.
//! The bird's `BirdHuntStates` entry is removed so it reverts to
//! Searching on the next `hunt_system` tick.
//!
//! Ported from `/Users/tonialatalo/src/vivarium/src/systems/eating.rs`.

use crate::components::{Bird, Transform};
use crate::config::BIRD_EATING_DISTANCE;
use crate::hunt::BirdHuntStates;
use bevy_ecs::message::{Message, MessageWriter};
use bevy_mass::prelude::*;

/// Emitted when a bird eats an insect. Framework-visible so later
/// phases (stats HUD, squirrel-nest hatching) can consume it — no
/// consumer today.
#[derive(Message, Clone, Copy, Debug)]
pub struct InsectEaten {
    pub bird: Entity,
}

#[mass_system]
pub fn eating_system(
    spatial: SpatialQueries,
    mut commands: Commands,
    mut eaten: MessageWriter<InsectEaten>,
    mut hunt_states: ResMut<BirdHuntStates>,
    mut birds: Query<(Entity, &Transform), With<Bird>>,
) {
    for (bird_entity, transform) in &mut birds {
        let bird_pos = transform.translation;
        let neighbors = spatial.neighbors_within(
            "insects",
            &bird_pos,
            BIRD_EATING_DISTANCE,
            None,
        );
        if let Some(n) = neighbors.first() {
            commands.entity(n.entity).despawn();
            eaten.write(InsectEaten { bird: bird_entity });
            hunt_states.0.remove(&bird_entity);
        }
    }
}

#[cfg(all(test, not(feature = "unreal")))]
mod tests {
    use super::*;
    use crate::components::{Insect, PreviousTranslation, Velocity};
    use crate::config::{BIRD_SIGHT_RANGE, BIRD_SPEED};
    use crate::hunt::{HuntPhase, HuntState};
    use bevy_ecs::message::Messages;
    use bevy_ecs::prelude::*;
    use core::time::Duration;

    fn insert_time(world: &mut World, dt: f32) {
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(dt));
        world.insert_resource(time);
    }

    /// Build an app with the insects `SpatialGroupPlugin` so the grid is
    /// populated before `eating_system` runs. Returns the world after one tick.
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
        // `MessageWriter<InsectEaten>` requires `Messages<InsectEaten>` as a
        // resource. In UE mode `#[mass_system]` auto-registers via inventory;
        // here we register manually since tests build their own App.
        app.add_message::<InsectEaten>();
        app.add_systems(
            bevy_app::Update,
            eating_system.in_set(SpatialGroupSet::Query),
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
    fn bird_eats_nearby_insect() {
        let mut bird_entity = None;
        let mut insect_entity = None;
        let world = run_with_plugin(
            |w| {
                let bird = spawn_bird(w);
                let insect = spawn_insect(w, DVec3::new(1.0, 0.0, 0.0));
                // Pre-insert a Diving hunt state so we can assert removal.
                let mut states = w.resource_mut::<BirdHuntStates>();
                states.0.insert(
                    bird,
                    HuntState {
                        phase: HuntPhase::Diving,
                        timer: 0.0,
                        target: Some(insect),
                        target_pos: DVec3::new(1.0, 0.0, 0.0),
                    },
                );
                bird_entity = Some(bird);
                insect_entity = Some(insect);
            },
            1.0 / 60.0,
        );

        let bird = bird_entity.unwrap();
        let insect = insect_entity.unwrap();

        // 1) Insect entity is despawned.
        assert!(
            world.get_entity(insect).is_err(),
            "insect should be despawned after eating"
        );

        // 2) Exactly one InsectEaten { bird } message was written.
        let messages = world.resource::<Messages<InsectEaten>>();
        let mut cursor = messages.get_cursor();
        let collected: Vec<&InsectEaten> = cursor.read(messages).collect();
        assert_eq!(
            collected.len(),
            1,
            "expected exactly one InsectEaten message, got {}",
            collected.len()
        );
        assert_eq!(
            collected[0].bird, bird,
            "message should reference the eating bird"
        );

        // 3) Hunt state entry is removed — bird reverts to implicit Searching.
        let states = world.resource::<BirdHuntStates>();
        assert!(
            states.0.get(&bird).is_none(),
            "eating should remove the bird's BirdHuntStates entry"
        );
    }
}
