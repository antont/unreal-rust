//! Framework-provided movement infrastructure.
//!
//! Game systems write desired velocity (`DesiredMovementLike::velocity()`).
//! The framework applies it to position — in Bevy mode via `apply_movement`,
//! in Unreal mode via C++ `UMassApplyMovementProcessor`.
//!
//! Game authors never see `#[cfg]` gates. The backend switch is internal.

use crate::prelude::*;
use bevy_ecs::component::Mutable;
use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// Traits — implemented by game fragment types
// ---------------------------------------------------------------------------

/// A component with a translatable position (maps to UE's FTransformFragment).
pub trait TransformLike: Component<Mutability = Mutable> {
    fn translation(&self) -> DVec3;
    fn set_translation(&mut self, v: DVec3);
}

/// A component storing the previous frame's translation (for spatial sweeps).
pub trait PrevTranslationLike: Component<Mutability = Mutable> {
    fn prev(&self) -> DVec3;
    fn set_prev(&mut self, v: DVec3);
}

/// A component expressing desired velocity (maps to UE's FMassDesiredMovementFragment).
pub trait DesiredMovementLike: Component<Mutability = Mutable> {
    fn velocity(&self) -> DVec3;
}

// ---------------------------------------------------------------------------
// apply_movement system — Bevy mode only
// ---------------------------------------------------------------------------

/// Apply desired velocity to position and save previous translation.
///
/// This is the Bevy-mode equivalent of UE's `UMassApplyMovementProcessor` +
/// `URustMassPostMovementProcessor`. In Unreal mode, those C++ processors
/// handle the same work.
///
/// Use this directly in a system chain, or via `MovementPlugin`.
#[cfg(not(feature = "unreal"))]
pub fn apply_movement<T, P, D>(
    mut entities: Query<(&mut T, &mut P, &D)>,
    time: Res<Time>,
) where
    T: TransformLike,
    P: PrevTranslationLike,
    D: DesiredMovementLike,
{
    let dt = time.delta_secs() as f64;
    for (mut transform, mut prev, movement) in &mut entities {
        // Save previous position for spatial sweep queries
        prev.set_prev(transform.translation());

        let vel = movement.velocity();
        let speed = vel.length();
        if speed < 1e-8 {
            continue;
        }

        let dir = vel / speed;
        let step = speed * dt;
        let new_pos = transform.translation() + dir * step;
        transform.set_translation(new_pos);
    }
}

// ---------------------------------------------------------------------------
// MovementPlugin — optional convenience for apps using Bevy's Plugin system
// ---------------------------------------------------------------------------

/// Plugin that adds `apply_movement` to the `Update` schedule.
///
/// In Bevy mode, adds the movement application system.
/// In Unreal mode, this is a no-op — C++ processors handle movement.
///
/// ```ignore
/// app.add_plugins(MovementPlugin::<Transform, PreviousTranslation, DesiredMovement>::default());
/// ```
pub struct MovementPlugin<T, P, D> {
    _marker: PhantomData<fn(T, P, D)>,
}

impl<T, P, D> Default for MovementPlugin<T, P, D> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T, P, D> Clone for MovementPlugin<T, P, D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, P, D> Copy for MovementPlugin<T, P, D> {}

#[cfg(not(feature = "unreal"))]
impl<T, P, D> bevy_app::Plugin for MovementPlugin<T, P, D>
where
    T: TransformLike,
    P: PrevTranslationLike,
    D: DesiredMovementLike,
{
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(bevy_app::Update, apply_movement::<T, P, D>);
    }
}

#[cfg(feature = "unreal")]
impl<T, P, D> bevy_app::Plugin for MovementPlugin<T, P, D>
where
    T: TransformLike + Send + Sync + 'static,
    P: PrevTranslationLike + Send + Sync + 'static,
    D: DesiredMovementLike + Send + Sync + 'static,
{
    fn build(&self, _app: &mut bevy_app::App) {
        // UE's UMassApplyMovementProcessor + URustMassPostMovementProcessor
        // handle movement application and PreviousTranslation in C++.
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(all(test, not(feature = "unreal")))]
mod tests {
    use super::*;
    use bevy_ecs::prelude::*;
    use core::time::Duration;

    #[derive(Component, Clone, Copy, Debug)]
    struct TestTransform {
        pos: DVec3,
    }
    impl TransformLike for TestTransform {
        fn translation(&self) -> DVec3 { self.pos }
        fn set_translation(&mut self, v: DVec3) { self.pos = v; }
    }

    #[derive(Component, Clone, Copy, Debug)]
    struct TestPrev {
        value: DVec3,
    }
    impl PrevTranslationLike for TestPrev {
        fn prev(&self) -> DVec3 { self.value }
        fn set_prev(&mut self, v: DVec3) { self.value = v; }
    }

    #[derive(Component, Clone, Copy, Debug)]
    struct TestMovement {
        vel: DVec3,
    }
    impl DesiredMovementLike for TestMovement {
        fn velocity(&self) -> DVec3 { self.vel }
    }

    fn insert_time(world: &mut World, dt: f32) {
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(dt));
        world.insert_resource(time);
    }

    fn run<M>(world: &mut World, system: impl IntoSystem<(), (), M>) {
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(system);
        schedule.run(world);
    }

    #[test]
    fn applies_velocity_to_position() {
        let mut world = World::new();
        insert_time(&mut world, 0.1);
        world.spawn((
            TestTransform { pos: DVec3::ZERO },
            TestPrev { value: DVec3::ZERO },
            TestMovement { vel: DVec3::new(100.0, 0.0, 0.0) },
        ));

        run(&mut world, apply_movement::<TestTransform, TestPrev, TestMovement>);

        let mut q = world.query::<&TestTransform>();
        let t = q.single(&world).unwrap();
        assert!((t.pos.x - 10.0).abs() < 1e-6, "x: {}", t.pos.x);
        assert!(t.pos.y.abs() < 1e-6);
    }

    #[test]
    fn saves_previous_translation() {
        let mut world = World::new();
        insert_time(&mut world, 0.1);
        world.spawn((
            TestTransform { pos: DVec3::new(50.0, 100.0, 0.0) },
            TestPrev { value: DVec3::ZERO },
            TestMovement { vel: DVec3::new(0.0, 200.0, 0.0) },
        ));

        run(&mut world, apply_movement::<TestTransform, TestPrev, TestMovement>);

        let mut q = world.query::<&TestPrev>();
        let prev = q.single(&world).unwrap();
        assert_eq!(prev.value, DVec3::new(50.0, 100.0, 0.0));
    }

    #[test]
    fn zero_velocity_does_not_move() {
        let mut world = World::new();
        insert_time(&mut world, 0.1);
        world.spawn((
            TestTransform { pos: DVec3::new(5.0, 5.0, 0.0) },
            TestPrev { value: DVec3::ZERO },
            TestMovement { vel: DVec3::ZERO },
        ));

        run(&mut world, apply_movement::<TestTransform, TestPrev, TestMovement>);

        let mut q = world.query::<&TestTransform>();
        let t = q.single(&world).unwrap();
        assert_eq!(t.pos, DVec3::new(5.0, 5.0, 0.0));
    }

    #[test]
    fn multiple_entities_move_independently() {
        let mut world = World::new();
        insert_time(&mut world, 0.05);
        world.spawn((
            TestTransform { pos: DVec3::ZERO },
            TestPrev { value: DVec3::ZERO },
            TestMovement { vel: DVec3::new(200.0, 0.0, 0.0) },
        ));
        world.spawn((
            TestTransform { pos: DVec3::ZERO },
            TestPrev { value: DVec3::ZERO },
            TestMovement { vel: DVec3::new(0.0, 50.0, 0.0) },
        ));

        run(&mut world, apply_movement::<TestTransform, TestPrev, TestMovement>);

        let mut q = world.query::<&TestTransform>();
        let transforms: Vec<_> = q.iter(&world).collect();
        assert_eq!(transforms.len(), 2);

        let (t_x, t_y) = if transforms[0].pos.x > 1.0 {
            (transforms[0], transforms[1])
        } else {
            (transforms[1], transforms[0])
        };
        assert!((t_x.pos.x - 10.0).abs() < 1e-6);
        assert!((t_y.pos.y - 2.5).abs() < 1e-6);
    }
}
