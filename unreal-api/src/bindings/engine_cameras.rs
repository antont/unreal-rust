#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FCameraAnimationParams {
    pub play_rate: f32,
    pub scale: f32,
    pub ease_in_type: ECameraAnimationEasingType,
    pub ease_in_duration: f32,
    pub ease_out_type: ECameraAnimationEasingType,
    pub ease_out_duration: f32,
    pub b_loop: bool,
    pub start_offset: i32,
    pub b_random_start_time: bool,
    pub duration_override: f32,
    pub play_space: ECameraAnimationPlaySpace,
    pub user_play_space_rot: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(2))]
pub struct FCameraAnimationHandle {}
#[repr(C, align(8))]
pub struct FActiveCameraAnimationInfo {
    pub sequence: UPtr<crate::bindings::template_sequence::UCameraAnimationSequence>,
    pub params: FCameraAnimationParams,
    pub handle: FCameraAnimationHandle,
    pub player: UPtr<crate::bindings::template_sequence::UCameraAnimationSequencePlayer>,
    pub camera_stand_in: UPtr<
        crate::bindings::template_sequence::UCameraAnimationSequenceCameraStandIn,
    >,
    pub ease_in_current_time: f32,
    pub ease_out_current_time: f32,
    pub b_is_easing_in: bool,
    pub b_is_easing_out: bool,
}
#[repr(C, align(4))]
pub struct FFOscillator {
    pub amplitude: f32,
    pub frequency: f32,
    pub initial_offset: EInitialOscillatorOffset,
    pub waveform: EOscillatorWaveform,
}
#[repr(C, align(4))]
pub struct FROscillator {
    pub pitch: FFOscillator,
    pub yaw: FFOscillator,
    pub roll: FFOscillator,
}
#[repr(C, align(4))]
pub struct FVOscillator {
    pub x: FFOscillator,
    pub y: FFOscillator,
    pub z: FFOscillator,
}
#[repr(C, align(4))]
pub struct FPerlinNoiseShaker {
    pub amplitude: f32,
    pub frequency: f32,
}
#[repr(C, align(4))]
pub struct FWaveOscillator {
    pub amplitude: f32,
    pub frequency: f32,
    pub initial_offset_type: EInitialWaveOscillatorOffsetType,
}
pub struct UCameraAnimationCameraModifier {
    pub active_animations: TArray<FActiveCameraAnimationInfo>,
    pub next_instance_serial_number: u16,
}
pub struct UEngineCameraAnimationFunctionLibrary {}
pub struct UEngineCamerasSubsystem {}
pub struct UCompositeCameraShakePattern {
    pub child_patterns: TArray<UPtr<crate::bindings::engine::UCameraShakePattern>>,
}
pub struct UDefaultCameraShakeBase {}
pub struct ULegacyCameraShake {
    pub oscillation_duration: f32,
    pub oscillation_blend_in_time: f32,
    pub oscillation_blend_out_time: f32,
    pub rot_oscillation: FROscillator,
    pub loc_oscillation: FVOscillator,
    pub fov_oscillation: FFOscillator,
    pub anim_play_rate: f32,
    pub anim_scale: f32,
    pub anim_blend_in_time: f32,
    pub anim_blend_out_time: f32,
    pub random_anim_segment_duration: f32,
    pub anim_sequence: UPtr<
        crate::bindings::template_sequence::UCameraAnimationSequence,
    >,
    pub flags_344: u8,
    pub oscillator_time_remaining: f32,
    pub sequence_shake_pattern: UPtr<
        crate::bindings::template_sequence::USequenceCameraShakePattern,
    >,
}
pub struct ULegacyCameraShakePattern {}
pub struct ULegacyCameraShakeFunctionLibrary {}
pub struct USimpleCameraShakePattern {
    pub duration: f32,
    pub blend_in_time: f32,
    pub blend_out_time: f32,
}
pub struct UPerlinNoiseCameraShakePattern {
    pub location_amplitude_multiplier: f32,
    pub location_frequency_multiplier: f32,
    pub x: FPerlinNoiseShaker,
    pub y: FPerlinNoiseShaker,
    pub z: FPerlinNoiseShaker,
    pub rotation_amplitude_multiplier: f32,
    pub rotation_frequency_multiplier: f32,
    pub pitch: FPerlinNoiseShaker,
    pub yaw: FPerlinNoiseShaker,
    pub roll: FPerlinNoiseShaker,
    pub fov: FPerlinNoiseShaker,
}
pub struct UWaveOscillatorCameraShakePattern {
    pub location_amplitude_multiplier: f32,
    pub location_frequency_multiplier: f32,
    pub x: FWaveOscillator,
    pub y: FWaveOscillator,
    pub z: FWaveOscillator,
    pub rotation_amplitude_multiplier: f32,
    pub rotation_frequency_multiplier: f32,
    pub pitch: FWaveOscillator,
    pub yaw: FWaveOscillator,
    pub roll: FWaveOscillator,
    pub fov: FWaveOscillator,
}
pub struct UTestCameraShake {}
pub struct UConstantCameraShakePattern {
    pub location_offset: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FRotator,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraAnimationEasingType(pub u8);
impl ECameraAnimationEasingType {
    pub const LINEAR: ECameraAnimationEasingType = ECameraAnimationEasingType(0);
    pub const SINUSOIDAL: ECameraAnimationEasingType = ECameraAnimationEasingType(1);
    pub const QUADRATIC: ECameraAnimationEasingType = ECameraAnimationEasingType(2);
    pub const CUBIC: ECameraAnimationEasingType = ECameraAnimationEasingType(3);
    pub const QUARTIC: ECameraAnimationEasingType = ECameraAnimationEasingType(4);
    pub const QUINTIC: ECameraAnimationEasingType = ECameraAnimationEasingType(5);
    pub const EXPONENTIAL: ECameraAnimationEasingType = ECameraAnimationEasingType(6);
    pub const CIRCULAR: ECameraAnimationEasingType = ECameraAnimationEasingType(7);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraAnimationPlaySpace(pub u8);
impl ECameraAnimationPlaySpace {
    pub const CAMERA_LOCAL: ECameraAnimationPlaySpace = ECameraAnimationPlaySpace(0);
    pub const WORLD: ECameraAnimationPlaySpace = ECameraAnimationPlaySpace(1);
    pub const USER_DEFINED: ECameraAnimationPlaySpace = ECameraAnimationPlaySpace(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInitialOscillatorOffset(pub u8);
impl EInitialOscillatorOffset {
    pub const EOO_OFFSET_RANDOM: EInitialOscillatorOffset = EInitialOscillatorOffset(0);
    pub const EOO_OFFSET_ZERO: EInitialOscillatorOffset = EInitialOscillatorOffset(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOscillatorWaveform(pub u8);
impl EOscillatorWaveform {
    pub const SINE_WAVE: EOscillatorWaveform = EOscillatorWaveform(0);
    pub const PERLIN_NOISE: EOscillatorWaveform = EOscillatorWaveform(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInitialWaveOscillatorOffsetType(pub u8);
impl EInitialWaveOscillatorOffsetType {
    pub const RANDOM: EInitialWaveOscillatorOffsetType = EInitialWaveOscillatorOffsetType(
        0,
    );
    pub const ZERO: EInitialWaveOscillatorOffsetType = EInitialWaveOscillatorOffsetType(
        1,
    );
}
