//! Framework-provided component types that map to Unreal Engine's built-in
//! Mass Entity fragments and tags.
//!
//! Game code imports these from `bevy_mass::prelude::*` — just like vanilla Bevy's
//! `Transform` comes from `bevy::prelude`.

use crate::movement::{TransformLike, DesiredMovementLike};
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
    /// UE's native velocity fragment (internal — written by UE's movement processor).
    /// Game code should use DesiredMovement instead.
    /// In Development builds: 48 bytes (Value + DebugPreviousValue).
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

// ---------------------------------------------------------------------------
// DesiredMovement
// ---------------------------------------------------------------------------

crate::mass_fragment!(cpp_type = "FMassDesiredMovementFragment", existing, include = "MassMovementFragments.h",
    /// Desired movement output. Game systems write velocity here;
    /// the engine applies it to position each frame.
    /// Maps to UE's FMassDesiredMovementFragment (80 bytes, align 16).
    /// Layout: FVector(24) + pad(8) + FQuat(32) + f32(4) + pad(12)
    #[repr(align(16))]
    pub struct DesiredMovement {
        pub velocity: DVec3,                    // FVector DesiredVelocity (24 bytes)
        _pad0: f64,                             // alignment padding to 16 for FQuat (8 bytes)
        pub facing: [f64; 4],                   // FQuat DesiredFacing XYZW (32 bytes)
        pub max_speed_override: f32,            // DesiredMaxSpeedOverride (4 bytes)
        _pad1: [u8; 12],                        // struct tail padding to 80 (12 bytes)
    }
);

impl Default for DesiredMovement {
    fn default() -> Self {
        Self {
            velocity: DVec3::new(100.0, 0.0, 0.0), // direction X * speed 100
            _pad0: 0.0,
            facing: [0.0, 0.0, 0.0, 1.0],           // identity quaternion
            max_speed_override: f32::MAX,
            _pad1: [0; 12],
        }
    }
}

impl DesiredMovement {
    /// Create a desired movement from direction and speed.
    pub fn new(direction: DVec3, speed: f32) -> Self {
        let v = if direction.length() > 1e-8 {
            direction.normalize() * speed as f64
        } else {
            DVec3::ZERO
        };
        Self { velocity: v, ..Self::default() }
    }

    /// Get the speed (magnitude of velocity vector).
    pub fn speed(&self) -> f32 {
        self.velocity.length() as f32
    }

    /// Get the normalized direction.
    pub fn direction(&self) -> DVec3 {
        let len = self.velocity.length();
        if len > 1e-8 { self.velocity / len } else { DVec3::ZERO }
    }
}

impl DesiredMovementLike for DesiredMovement {
    fn velocity(&self) -> DVec3 { self.velocity }
}

// ---------------------------------------------------------------------------
// CodeDrivenMovementTag
// ---------------------------------------------------------------------------

crate::mass_tag!(cpp_type = "FMassCodeDrivenMovementTag", existing, include = "MassMovementFragments.h",
    /// Tag indicating entity uses code-driven movement (required by UE's UMassApplyMovementProcessor).
    pub struct CodeDrivenMovementTag;
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
    fn desired_movement_layout() {
        assert_eq!(mem::size_of::<DesiredMovement>(), 80);
        assert_eq!(mem::align_of::<DesiredMovement>(), 16);
        assert_eq!(mem::offset_of!(DesiredMovement, velocity), 0);
        assert_eq!(mem::offset_of!(DesiredMovement, facing), 32);
        assert_eq!(mem::offset_of!(DesiredMovement, max_speed_override), 64);
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
    }

    #[test]
    fn desired_movement_default() {
        let dm = DesiredMovement::default();
        assert_eq!(dm.velocity, DVec3::new(100.0, 0.0, 0.0));
        assert!((dm.speed() - 100.0).abs() < 1e-4);
        assert!((dm.direction() - DVec3::X).length() < 1e-8);
        assert_eq!(dm.facing, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(dm.max_speed_override, f32::MAX);
    }

    #[test]
    fn desired_movement_new() {
        let dm = DesiredMovement::new(DVec3::new(0.0, 1.0, 0.0), 50.0);
        assert!((dm.speed() - 50.0).abs() < 1e-4);
        assert!((dm.direction() - DVec3::Y).length() < 1e-8);
        assert!((dm.velocity - DVec3::new(0.0, 50.0, 0.0)).length() < 1e-6);
    }
}
