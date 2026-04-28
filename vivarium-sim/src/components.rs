use bevy_mass::prelude::{Component, MassFragment, PrevTranslationLike, Resource, DVec3};

pub use bevy_mass::components::{Transform, Velocity, SimpleMovementTag};

/// Marker for insect entities.
#[derive(Component, MassFragment, Clone, Copy, Debug)]
#[cfg_attr(feature = "unreal", mass(group = "insects"))]
pub struct Insect;

/// Previous-frame translation. Required by `bevy_mass::MovementPlugin` in
/// standalone mode; UE mode integrates via `UMassSimpleMovementProcessor`.
#[repr(C)]
#[derive(Component, MassFragment, Clone, Copy, Debug, Default)]
pub struct PreviousTranslation {
    pub value: DVec3,
}

impl PrevTranslationLike for PreviousTranslation {
    fn prev(&self) -> DVec3 { self.value }
    fn set_prev(&mut self, v: DVec3) { self.value = v; }
}

/// Simulation bounds, read by `boundary_force_system`.
/// Overwritten from `MassInitSimulationParams` in UE mode; defaults here
/// match `config::WORLD_HALF_SIZE` for standalone.
#[derive(Resource, Clone, Copy, Debug)]
pub struct SimBounds {
    pub min: DVec3,
    pub max: DVec3,
}

impl Default for SimBounds {
    fn default() -> Self {
        let h = crate::config::WORLD_HALF_SIZE;
        Self {
            min: DVec3::new(-h, -h, -h),
            max: DVec3::new(h, h, h),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn previous_translation_layout() {
        assert_eq!(mem::size_of::<PreviousTranslation>(), 24);
        assert_eq!(mem::align_of::<PreviousTranslation>(), 8);
        assert_eq!(mem::offset_of!(PreviousTranslation, value), 0);
    }

    #[test]
    fn sim_bounds_default_matches_world_half_size() {
        let b = SimBounds::default();
        let h = crate::config::WORLD_HALF_SIZE;
        assert_eq!(b.min, DVec3::new(-h, -h, -h));
        assert_eq!(b.max, DVec3::new(h, h, h));
    }
}
