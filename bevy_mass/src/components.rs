//! Framework-provided component types that map to Unreal Engine's built-in
//! Mass Entity fragments and tags.
//!
//! Game code imports these from `bevy_mass::prelude::*` — just like vanilla Bevy's
//! `Transform` comes from `bevy::prelude`.

use crate::movement::{TransformLike, VelocityLike};
use glam::DVec3;

// ---------------------------------------------------------------------------
// Transform
// ---------------------------------------------------------------------------

crate::mass_fragment!(cpp_type = "FTransformFragment", existing, include = "MassCommonFragments.h",
    /// Transform matching UE's FTransformFragment layout (FQuat + FVector + FVector,
    /// each padded to 32 bytes for SIMD alignment). 96 bytes, align 16.
    #[repr(align(16))]
    pub struct Transform {
        pub rotation: [f64; 4],   // Quaternion XYZW (32 bytes)
        pub translation: DVec3,    // Position XYZ (24 bytes)
        _pad0: f64,                // SIMD padding (8 bytes)
        pub scale: DVec3,          // Scale XYZ (24 bytes)
        _pad1: f64,                // SIMD padding (8 bytes)
    }
);

impl Default for Transform {
    fn default() -> Self {
        Self {
            rotation: [0.0, 0.0, 0.0, 1.0], // Identity quaternion
            translation: DVec3::ZERO,
            _pad0: 0.0,
            scale: DVec3::ONE,
            _pad1: 0.0,
        }
    }
}

impl Transform {
    /// Create a transform with the given translation, identity rotation, unit scale.
    pub fn from_translation(t: DVec3) -> Self {
        Self {
            translation: t,
            ..Self::default()
        }
    }
}

impl TransformLike for Transform {
    fn translation(&self) -> DVec3 { self.translation }
    fn set_translation(&mut self, v: DVec3) { self.translation = v; }
}

// ---------------------------------------------------------------------------
// Velocity
// ---------------------------------------------------------------------------

crate::mass_fragment!(cpp_type = "FMassVelocityFragment", existing, include = "MassMovementFragments.h",
    /// Velocity authored by game systems and integrated into Transform each
    /// frame — in Bevy mode by `apply_movement`, in Unreal mode by
    /// `UMassSimpleMovementProcessor`. Magnitude is units per second.
    ///
    /// Maps to UE's `FMassVelocityFragment`. In Development builds the engine
    /// struct is 48 bytes (`Value` + `DebugPreviousValue`); game code only
    /// interacts with `value`.
    pub struct Velocity {
        pub value: DVec3,          // direction * speed (24 bytes)
        _debug_prev: DVec3,        // DebugPreviousValue padding (24 bytes, Development only)
    }
);

impl Default for Velocity {
    fn default() -> Self {
        Self {
            value: DVec3::ZERO,
            _debug_prev: DVec3::ZERO,
        }
    }
}

impl Velocity {
    /// Create a velocity from a direction vector and a speed in units/s.
    /// Normalizes the direction; returns zero if the input is near-zero.
    pub fn new(direction: DVec3, speed: f32) -> Self {
        let value = if direction.length() > 1e-8 {
            direction.normalize() * speed as f64
        } else {
            DVec3::ZERO
        };
        Self { value, _debug_prev: DVec3::ZERO }
    }

    /// Speed (magnitude of velocity vector).
    pub fn speed(&self) -> f32 {
        self.value.length() as f32
    }

    /// Normalized direction, or zero for a near-zero velocity.
    pub fn direction(&self) -> DVec3 {
        let len = self.value.length();
        if len > 1e-8 { self.value / len } else { DVec3::ZERO }
    }
}

impl VelocityLike for Velocity {
    fn velocity(&self) -> DVec3 { self.value }
}

// ---------------------------------------------------------------------------
// SimpleMovementTag
// ---------------------------------------------------------------------------

crate::mass_tag!(cpp_type = "FMassSimpleMovementTag", existing, include = "Example/MassSimpleMovementTrait.h",
    /// Tag indicating entity uses code-driven movement integration via
    /// `UMassSimpleMovementProcessor` in Unreal mode. No LOD gating — the
    /// processor integrates Velocity into Transform regardless of visual LOD.
    pub struct SimpleMovementTag;
);

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn transform_layout() {
        assert_eq!(mem::size_of::<Transform>(), 96);
        assert_eq!(mem::align_of::<Transform>(), 16);
        assert_eq!(mem::offset_of!(Transform, rotation), 0);
        assert_eq!(mem::offset_of!(Transform, translation), 32);
        assert_eq!(mem::offset_of!(Transform, scale), 64);
    }

    #[test]
    fn velocity_layout() {
        assert_eq!(mem::size_of::<Velocity>(), 48);
        assert_eq!(mem::align_of::<Velocity>(), 8);
        assert_eq!(mem::offset_of!(Velocity, value), 0);
    }

    #[test]
    fn transform_default() {
        let t = Transform::default();
        assert_eq!(t.rotation, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(t.translation, DVec3::ZERO);
        assert_eq!(t.scale, DVec3::ONE);
    }

    #[test]
    fn transform_from_translation() {
        let t = Transform::from_translation(DVec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.translation, DVec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.rotation, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(t.scale, DVec3::ONE);
    }

    #[test]
    fn velocity_default() {
        let v = Velocity::default();
        assert_eq!(v.value, DVec3::ZERO);
        assert!((v.speed() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn velocity_new() {
        let v = Velocity::new(DVec3::new(0.0, 1.0, 0.0), 50.0);
        assert!((v.speed() - 50.0).abs() < 1e-4);
        assert!((v.direction() - DVec3::Y).length() < 1e-8);
        assert!((v.value - DVec3::new(0.0, 50.0, 0.0)).length() < 1e-6);
    }

    #[test]
    fn velocity_new_zero_direction_is_zero() {
        let v = Velocity::new(DVec3::ZERO, 100.0);
        assert_eq!(v.value, DVec3::ZERO);
    }
}
