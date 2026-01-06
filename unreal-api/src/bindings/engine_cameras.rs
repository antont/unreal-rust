#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_STOP_CAMERA_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_STOP_ALL_CAMERA_ANIMATIONS_OF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_STOP_ALL_CAMERA_ANIMATIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_PLAY_CAMERA_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_IS_CAMERA_ANIMATION_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_GET_CAMERA_ANIMATION_CAMERA_MODIFIER_FROM_PLAYER_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_GET_CAMERA_ANIMATION_CAMERA_MODIFIER_FROM_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_GET_CAMERA_ANIMATION_CAMERA_MODIFIER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENGINE_CAMERA_ANIMATION_FUNCTION_LIBRARY_CONV_CAMERA_SHAKE_PLAY_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENGINE_CAMERA_ANIMATION_FUNCTION_LIBRARY_CONV_CAMERA_ANIMATION_PLAY_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENGINE_CAMERA_ANIMATION_FUNCTION_LIBRARY_CONV_CAMERA_ANIMATION_CAMERA_MODIFIER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENGINE_CAMERAS_SUBSYSTEM_STOP_CAMERA_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENGINE_CAMERAS_SUBSYSTEM_STOP_ALL_CAMERA_ANIMATIONS_OF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENGINE_CAMERAS_SUBSYSTEM_STOP_ALL_CAMERA_ANIMATIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENGINE_CAMERAS_SUBSYSTEM_PLAY_CAMERA_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ENGINE_CAMERAS_SUBSYSTEM_IS_CAMERA_ANIMATION_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEGACY_CAMERA_SHAKE_START_LEGACY_CAMERA_SHAKE_FROM_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEGACY_CAMERA_SHAKE_START_LEGACY_CAMERA_SHAKE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEGACY_CAMERA_SHAKE_RECEIVE_STOP_SHAKE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEGACY_CAMERA_SHAKE_RECEIVE_PLAY_SHAKE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEGACY_CAMERA_SHAKE_RECEIVE_IS_FINISHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEGACY_CAMERA_SHAKE_BLUEPRINT_UPDATE_CAMERA_SHAKE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEGACY_CAMERA_SHAKE_FUNCTION_LIBRARY_CONV_LEGACY_CAMERA_SHAKE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCameraAnimationCameraModifier::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopCameraAnimation"),
            &raw mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_STOP_CAMERA_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAllCameraAnimationsOf"),
            &raw mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_STOP_ALL_CAMERA_ANIMATIONS_OF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAllCameraAnimations"),
            &raw mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_STOP_ALL_CAMERA_ANIMATIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayCameraAnimation"),
            &raw mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_PLAY_CAMERA_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCameraAnimationActive"),
            &raw mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_IS_CAMERA_ANIMATION_ACTIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "GetCameraAnimationCameraModifierFromPlayerController",
            ),
            &raw mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_GET_CAMERA_ANIMATION_CAMERA_MODIFIER_FROM_PLAYER_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCameraAnimationCameraModifierFromID"),
            &raw mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_GET_CAMERA_ANIMATION_CAMERA_MODIFIER_FROM_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCameraAnimationCameraModifier"),
            &raw mut U_CAMERA_ANIMATION_CAMERA_MODIFIER_GET_CAMERA_ANIMATION_CAMERA_MODIFIER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEngineCameraAnimationFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_CameraShakePlaySpace"),
            &raw mut U_ENGINE_CAMERA_ANIMATION_FUNCTION_LIBRARY_CONV_CAMERA_SHAKE_PLAY_SPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_CameraAnimationPlaySpace"),
            &raw mut U_ENGINE_CAMERA_ANIMATION_FUNCTION_LIBRARY_CONV_CAMERA_ANIMATION_PLAY_SPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_CameraAnimationCameraModifier"),
            &raw mut U_ENGINE_CAMERA_ANIMATION_FUNCTION_LIBRARY_CONV_CAMERA_ANIMATION_CAMERA_MODIFIER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEngineCamerasSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopCameraAnimation"),
            &raw mut U_ENGINE_CAMERAS_SUBSYSTEM_STOP_CAMERA_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAllCameraAnimationsOf"),
            &raw mut U_ENGINE_CAMERAS_SUBSYSTEM_STOP_ALL_CAMERA_ANIMATIONS_OF,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAllCameraAnimations"),
            &raw mut U_ENGINE_CAMERAS_SUBSYSTEM_STOP_ALL_CAMERA_ANIMATIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayCameraAnimation"),
            &raw mut U_ENGINE_CAMERAS_SUBSYSTEM_PLAY_CAMERA_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCameraAnimationActive"),
            &raw mut U_ENGINE_CAMERAS_SUBSYSTEM_IS_CAMERA_ANIMATION_ACTIVE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULegacyCameraShake::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartLegacyCameraShakeFromSource"),
            &raw mut U_LEGACY_CAMERA_SHAKE_START_LEGACY_CAMERA_SHAKE_FROM_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartLegacyCameraShake"),
            &raw mut U_LEGACY_CAMERA_SHAKE_START_LEGACY_CAMERA_SHAKE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveStopShake"),
            &raw mut U_LEGACY_CAMERA_SHAKE_RECEIVE_STOP_SHAKE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceivePlayShake"),
            &raw mut U_LEGACY_CAMERA_SHAKE_RECEIVE_PLAY_SHAKE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveIsFinished"),
            &raw mut U_LEGACY_CAMERA_SHAKE_RECEIVE_IS_FINISHED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BlueprintUpdateCameraShake"),
            &raw mut U_LEGACY_CAMERA_SHAKE_BLUEPRINT_UPDATE_CAMERA_SHAKE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULegacyCameraShakeFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_LegacyCameraShake"),
            &raw mut U_LEGACY_CAMERA_SHAKE_FUNCTION_LIBRARY_CONV_LEGACY_CAMERA_SHAKE,
        );
    }
}
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
impl UCameraAnimationCameraModifier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAnimationCameraModifier")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEngineCameraAnimationFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UEngineCameraAnimationFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEngineCameraAnimationFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UEngineCamerasSubsystem {
    __padding_end: [u8; 64],
}
impl UEngineCamerasSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEngineCamerasSubsystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCompositeCameraShakePattern {
    __padding_end: [u8; 64],
}
impl UCompositeCameraShakePattern {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositeCameraShakePattern")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(16))]
pub struct UDefaultCameraShakeBase {
    __padding_end: [u8; 224],
}
impl UDefaultCameraShakeBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDefaultCameraShakeBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
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
impl ULegacyCameraShake {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyCameraShake")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULegacyCameraShakePattern {
    __padding_end: [u8; 48],
}
impl ULegacyCameraShakePattern {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyCameraShakePattern")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULegacyCameraShakeFunctionLibrary {
    __padding_end: [u8; 48],
}
impl ULegacyCameraShakeFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyCameraShakeFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USimpleCameraShakePattern {
    __padding_end: [u8; 96],
}
impl USimpleCameraShakePattern {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleCameraShakePattern")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
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
impl UPerlinNoiseCameraShakePattern {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPerlinNoiseCameraShakePattern")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
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
impl UWaveOscillatorCameraShakePattern {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaveOscillatorCameraShakePattern")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(16))]
pub struct UTestCameraShake {
    __padding_end: [u8; 224],
}
impl UTestCameraShake {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestCameraShake")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UConstantCameraShakePattern {
    __padding_end: [u8; 144],
}
impl UConstantCameraShakePattern {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstantCameraShakePattern")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
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
