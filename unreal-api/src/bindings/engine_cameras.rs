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
impl FCameraAnimationParams {}
#[repr(C, align(2))]
pub struct FCameraAnimationHandle {
    __padding_end: [u8; 4],
}
impl FCameraAnimationHandle {}
#[repr(C, align(4))]
pub struct FFOscillator {
    pub amplitude: f32,
    pub frequency: f32,
    #[doc(hidden)]
    __padding_9: [u8; 1],
    pub waveform: EOscillatorWaveform,
    __padding_end: [u8; 2],
}
impl FFOscillator {}
#[repr(C, align(4))]
pub struct FROscillator {
    pub pitch: FFOscillator,
    pub yaw: FFOscillator,
    pub roll: FFOscillator,
}
impl FROscillator {}
#[repr(C, align(4))]
pub struct FVOscillator {
    pub x: FFOscillator,
    pub y: FFOscillator,
    pub z: FFOscillator,
}
impl FVOscillator {}
#[repr(C, align(4))]
pub struct FPerlinNoiseShaker {
    pub amplitude: f32,
    pub frequency: f32,
}
impl FPerlinNoiseShaker {}
#[repr(C, align(4))]
pub struct FWaveOscillator {
    pub amplitude: f32,
    pub frequency: f32,
    pub initial_offset_type: EInitialWaveOscillatorOffsetType,
    __padding_end: [u8; 3],
}
impl FWaveOscillator {}
#[repr(C, align(8))]
pub struct UCameraAnimationCameraModifier {
    __padding_end: [u8; 104],
}
impl UCameraAnimationCameraModifier {}
#[repr(C, align(8))]
pub struct UEngineCameraAnimationFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UEngineCameraAnimationFunctionLibrary {}
#[repr(C, align(8))]
pub struct UEngineCamerasSubsystem {
    __padding_end: [u8; 64],
}
impl UEngineCamerasSubsystem {}
#[repr(C, align(8))]
pub struct UCompositeCameraShakePattern {
    __padding_end: [u8; 64],
}
impl UCompositeCameraShakePattern {}
#[repr(C, align(16))]
pub struct UDefaultCameraShakeBase {
    __padding_end: [u8; 224],
}
impl UDefaultCameraShakeBase {}
#[repr(C, align(16))]
pub struct ULegacyCameraShake {
    #[doc(hidden)]
    __padding_228: [u8; 228],
    pub rot_oscillation: FROscillator,
    pub loc_oscillation: FVOscillator,
    pub fov_oscillation: FFOscillator,
    #[doc(hidden)]
    __padding_348: [u8; 36],
    pub oscillator_time_remaining: f32,
    __padding_end: [u8; 144],
}
impl ULegacyCameraShake {}
#[repr(C, align(8))]
pub struct ULegacyCameraShakePattern {
    __padding_end: [u8; 48],
}
impl ULegacyCameraShakePattern {}
#[repr(C, align(8))]
pub struct ULegacyCameraShakeFunctionLibrary {
    __padding_end: [u8; 48],
}
impl ULegacyCameraShakeFunctionLibrary {}
#[repr(C, align(8))]
pub struct USimpleCameraShakePattern {
    __padding_end: [u8; 96],
}
impl USimpleCameraShakePattern {}
#[repr(C, align(8))]
pub struct UPerlinNoiseCameraShakePattern {
    #[doc(hidden)]
    __padding_96: [u8; 96],
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
    __padding_end: [u8; 56],
}
impl UPerlinNoiseCameraShakePattern {}
#[repr(C, align(8))]
pub struct UWaveOscillatorCameraShakePattern {
    #[doc(hidden)]
    __padding_96: [u8; 96],
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
    __padding_end: [u8; 60],
}
impl UWaveOscillatorCameraShakePattern {}
#[repr(C, align(16))]
pub struct UTestCameraShake {
    __padding_end: [u8; 224],
}
impl UTestCameraShake {}
#[repr(C, align(8))]
pub struct UConstantCameraShakePattern {
    __padding_end: [u8; 144],
}
impl UConstantCameraShakePattern {}
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
#[repr(transparent)]
pub struct ECameraAnimationPlaySpace(pub u8);
impl ECameraAnimationPlaySpace {
    pub const CAMERA_LOCAL: ECameraAnimationPlaySpace = ECameraAnimationPlaySpace(0);
    pub const WORLD: ECameraAnimationPlaySpace = ECameraAnimationPlaySpace(1);
    pub const USER_DEFINED: ECameraAnimationPlaySpace = ECameraAnimationPlaySpace(2);
}
#[repr(transparent)]
pub struct EInitialOscillatorOffset(pub u8);
impl EInitialOscillatorOffset {
    pub const EOO_OFFSET_RANDOM: EInitialOscillatorOffset = EInitialOscillatorOffset(0);
    pub const EOO_OFFSET_ZERO: EInitialOscillatorOffset = EInitialOscillatorOffset(1);
}
#[repr(transparent)]
pub struct EOscillatorWaveform(pub u8);
impl EOscillatorWaveform {
    pub const SINE_WAVE: EOscillatorWaveform = EOscillatorWaveform(0);
    pub const PERLIN_NOISE: EOscillatorWaveform = EOscillatorWaveform(1);
}
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
