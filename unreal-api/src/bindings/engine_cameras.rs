#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_camera_animation_camera_modifier_stop_camera_animation: *mut crate::ffi::UFunctionOpague,
    pub u_camera_animation_camera_modifier_stop_all_camera_animations_of: *mut crate::ffi::UFunctionOpague,
    pub u_camera_animation_camera_modifier_stop_all_camera_animations: *mut crate::ffi::UFunctionOpague,
    pub u_camera_animation_camera_modifier_play_camera_animation: *mut crate::ffi::UFunctionOpague,
    pub u_camera_animation_camera_modifier_is_camera_animation_active: *mut crate::ffi::UFunctionOpague,
    pub u_camera_animation_camera_modifier_get_camera_animation_camera_modifier_from_player_controller: *mut crate::ffi::UFunctionOpague,
    pub u_camera_animation_camera_modifier_get_camera_animation_camera_modifier_from_id: *mut crate::ffi::UFunctionOpague,
    pub u_camera_animation_camera_modifier_get_camera_animation_camera_modifier: *mut crate::ffi::UFunctionOpague,
    pub u_engine_camera_animation_function_library_conv_camera_shake_play_space: *mut crate::ffi::UFunctionOpague,
    pub u_engine_camera_animation_function_library_conv_camera_animation_play_space: *mut crate::ffi::UFunctionOpague,
    pub u_engine_camera_animation_function_library_conv_camera_animation_camera_modifier: *mut crate::ffi::UFunctionOpague,
    pub u_engine_cameras_subsystem_stop_camera_animation: *mut crate::ffi::UFunctionOpague,
    pub u_engine_cameras_subsystem_stop_all_camera_animations_of: *mut crate::ffi::UFunctionOpague,
    pub u_engine_cameras_subsystem_stop_all_camera_animations: *mut crate::ffi::UFunctionOpague,
    pub u_engine_cameras_subsystem_play_camera_animation: *mut crate::ffi::UFunctionOpague,
    pub u_engine_cameras_subsystem_is_camera_animation_active: *mut crate::ffi::UFunctionOpague,
    pub u_legacy_camera_shake_start_legacy_camera_shake_from_source: *mut crate::ffi::UFunctionOpague,
    pub u_legacy_camera_shake_start_legacy_camera_shake: *mut crate::ffi::UFunctionOpague,
    pub u_legacy_camera_shake_receive_stop_shake: *mut crate::ffi::UFunctionOpague,
    pub u_legacy_camera_shake_receive_play_shake: *mut crate::ffi::UFunctionOpague,
    pub u_legacy_camera_shake_receive_is_finished: *mut crate::ffi::UFunctionOpague,
    pub u_legacy_camera_shake_blueprint_update_camera_shake: *mut crate::ffi::UFunctionOpague,
    pub u_legacy_camera_shake_function_library_conv_legacy_camera_shake: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_camera_animation_camera_modifier_stop_camera_animation: std::ptr::null_mut(),
            u_camera_animation_camera_modifier_stop_all_camera_animations_of: std::ptr::null_mut(),
            u_camera_animation_camera_modifier_stop_all_camera_animations: std::ptr::null_mut(),
            u_camera_animation_camera_modifier_play_camera_animation: std::ptr::null_mut(),
            u_camera_animation_camera_modifier_is_camera_animation_active: std::ptr::null_mut(),
            u_camera_animation_camera_modifier_get_camera_animation_camera_modifier_from_player_controller: std::ptr::null_mut(),
            u_camera_animation_camera_modifier_get_camera_animation_camera_modifier_from_id: std::ptr::null_mut(),
            u_camera_animation_camera_modifier_get_camera_animation_camera_modifier: std::ptr::null_mut(),
            u_engine_camera_animation_function_library_conv_camera_shake_play_space: std::ptr::null_mut(),
            u_engine_camera_animation_function_library_conv_camera_animation_play_space: std::ptr::null_mut(),
            u_engine_camera_animation_function_library_conv_camera_animation_camera_modifier: std::ptr::null_mut(),
            u_engine_cameras_subsystem_stop_camera_animation: std::ptr::null_mut(),
            u_engine_cameras_subsystem_stop_all_camera_animations_of: std::ptr::null_mut(),
            u_engine_cameras_subsystem_stop_all_camera_animations: std::ptr::null_mut(),
            u_engine_cameras_subsystem_play_camera_animation: std::ptr::null_mut(),
            u_engine_cameras_subsystem_is_camera_animation_active: std::ptr::null_mut(),
            u_legacy_camera_shake_start_legacy_camera_shake_from_source: std::ptr::null_mut(),
            u_legacy_camera_shake_start_legacy_camera_shake: std::ptr::null_mut(),
            u_legacy_camera_shake_receive_stop_shake: std::ptr::null_mut(),
            u_legacy_camera_shake_receive_play_shake: std::ptr::null_mut(),
            u_legacy_camera_shake_receive_is_finished: std::ptr::null_mut(),
            u_legacy_camera_shake_blueprint_update_camera_shake: std::ptr::null_mut(),
            u_legacy_camera_shake_function_library_conv_legacy_camera_shake: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCameraAnimationCameraModifier::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopCameraAnimation"),
            &raw mut __FUNCTION_PTRS
                .u_camera_animation_camera_modifier_stop_camera_animation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAllCameraAnimationsOf"),
            &raw mut __FUNCTION_PTRS
                .u_camera_animation_camera_modifier_stop_all_camera_animations_of,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAllCameraAnimations"),
            &raw mut __FUNCTION_PTRS
                .u_camera_animation_camera_modifier_stop_all_camera_animations,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayCameraAnimation"),
            &raw mut __FUNCTION_PTRS
                .u_camera_animation_camera_modifier_play_camera_animation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCameraAnimationActive"),
            &raw mut __FUNCTION_PTRS
                .u_camera_animation_camera_modifier_is_camera_animation_active,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "GetCameraAnimationCameraModifierFromPlayerController",
            ),
            &raw mut __FUNCTION_PTRS
                .u_camera_animation_camera_modifier_get_camera_animation_camera_modifier_from_player_controller,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCameraAnimationCameraModifierFromID"),
            &raw mut __FUNCTION_PTRS
                .u_camera_animation_camera_modifier_get_camera_animation_camera_modifier_from_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCameraAnimationCameraModifier"),
            &raw mut __FUNCTION_PTRS
                .u_camera_animation_camera_modifier_get_camera_animation_camera_modifier,
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
            &raw mut __FUNCTION_PTRS
                .u_engine_camera_animation_function_library_conv_camera_shake_play_space,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_CameraAnimationPlaySpace"),
            &raw mut __FUNCTION_PTRS
                .u_engine_camera_animation_function_library_conv_camera_animation_play_space,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_CameraAnimationCameraModifier"),
            &raw mut __FUNCTION_PTRS
                .u_engine_camera_animation_function_library_conv_camera_animation_camera_modifier,
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
            &raw mut __FUNCTION_PTRS.u_engine_cameras_subsystem_stop_camera_animation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAllCameraAnimationsOf"),
            &raw mut __FUNCTION_PTRS
                .u_engine_cameras_subsystem_stop_all_camera_animations_of,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAllCameraAnimations"),
            &raw mut __FUNCTION_PTRS
                .u_engine_cameras_subsystem_stop_all_camera_animations,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayCameraAnimation"),
            &raw mut __FUNCTION_PTRS.u_engine_cameras_subsystem_play_camera_animation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCameraAnimationActive"),
            &raw mut __FUNCTION_PTRS
                .u_engine_cameras_subsystem_is_camera_animation_active,
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
            &raw mut __FUNCTION_PTRS
                .u_legacy_camera_shake_start_legacy_camera_shake_from_source,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartLegacyCameraShake"),
            &raw mut __FUNCTION_PTRS.u_legacy_camera_shake_start_legacy_camera_shake,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveStopShake"),
            &raw mut __FUNCTION_PTRS.u_legacy_camera_shake_receive_stop_shake,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceivePlayShake"),
            &raw mut __FUNCTION_PTRS.u_legacy_camera_shake_receive_play_shake,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveIsFinished"),
            &raw mut __FUNCTION_PTRS.u_legacy_camera_shake_receive_is_finished,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BlueprintUpdateCameraShake"),
            &raw mut __FUNCTION_PTRS.u_legacy_camera_shake_blueprint_update_camera_shake,
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
            &raw mut __FUNCTION_PTRS
                .u_legacy_camera_shake_function_library_conv_legacy_camera_shake,
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
    pub fn stop_camera_animation(
        &mut self,
        handle: &FCameraAnimationHandle,
        b_immediate: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_stop_camera_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(0).cast::<FCameraAnimationHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediate,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_stop_camera_animation,
                __buffer,
            )
        };
    }
    pub fn stop_all_camera_animations_of(
        &mut self,
        sequence: UPtr<crate::bindings::template_sequence::UCameraAnimationSequence>,
        b_immediate: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_stop_all_camera_animations_of,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::template_sequence::UCameraAnimationSequence,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediate,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_stop_all_camera_animations_of,
                __buffer,
            )
        };
    }
    pub fn stop_all_camera_animations(&mut self, b_immediate: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_stop_all_camera_animations,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediate,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_stop_all_camera_animations,
                __buffer,
            )
        };
    }
    pub fn play_camera_animation(
        &mut self,
        sequence: UPtr<crate::bindings::template_sequence::UCameraAnimationSequence>,
        params: FCameraAnimationParams,
    ) -> FCameraAnimationHandle {
        let mut __stack = crate::core_data::StackAlloc::<84>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_play_camera_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::template_sequence::UCameraAnimationSequence,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &params,
                __buffer.add(8).cast::<FCameraAnimationParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_play_camera_animation,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<FCameraAnimationHandle>().read() }
    }
    pub fn is_camera_animation_active(&self, handle: &FCameraAnimationHandle) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_is_camera_animation_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(0).cast::<FCameraAnimationHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_is_camera_animation_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_camera_animation_camera_modifier_from_player_controller(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UCameraAnimationCameraModifier> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_get_camera_animation_camera_modifier_from_player_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::engine_cameras::UCameraAnimationCameraModifier::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_get_camera_animation_camera_modifier_from_player_controller,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UCameraAnimationCameraModifier>>().read() }
    }
    pub fn get_camera_animation_camera_modifier_from_id(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        controller_id: i32,
    ) -> UPtr<UCameraAnimationCameraModifier> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_get_camera_animation_camera_modifier_from_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controller_id,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::engine_cameras::UCameraAnimationCameraModifier::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_get_camera_animation_camera_modifier_from_id,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UCameraAnimationCameraModifier>>().read() }
    }
    pub fn get_camera_animation_camera_modifier(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_index: i32,
    ) -> UPtr<UCameraAnimationCameraModifier> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_get_camera_animation_camera_modifier,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::engine_cameras::UCameraAnimationCameraModifier::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_camera_animation_camera_modifier_get_camera_animation_camera_modifier,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UCameraAnimationCameraModifier>>().read() }
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
    pub fn conv_camera_shake_play_space(
        camera_animation_play_space: ECameraAnimationPlaySpace,
    ) -> crate::bindings::engine::ECameraShakePlaySpace {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_camera_animation_function_library_conv_camera_shake_play_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_animation_play_space,
                __buffer.add(0).cast::<ECameraAnimationPlaySpace>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::engine_cameras::UEngineCameraAnimationFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_camera_animation_function_library_conv_camera_shake_play_space,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(1)
                .cast::<crate::bindings::engine::ECameraShakePlaySpace>()
                .read()
        }
    }
    pub fn conv_camera_animation_play_space(
        camera_shake_play_space: crate::bindings::engine::ECameraShakePlaySpace,
    ) -> ECameraAnimationPlaySpace {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_camera_animation_function_library_conv_camera_animation_play_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_shake_play_space,
                __buffer.add(0).cast::<crate::bindings::engine::ECameraShakePlaySpace>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::engine_cameras::UEngineCameraAnimationFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_camera_animation_function_library_conv_camera_animation_play_space,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<ECameraAnimationPlaySpace>().read() }
    }
    pub fn conv_camera_animation_camera_modifier(
        player_camera_manager: UPtr<crate::bindings::engine::APlayerCameraManager>,
    ) -> UPtr<UCameraAnimationCameraModifier> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_camera_animation_function_library_conv_camera_animation_camera_modifier,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_camera_manager,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerCameraManager>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::engine_cameras::UEngineCameraAnimationFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_camera_animation_function_library_conv_camera_animation_camera_modifier,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UCameraAnimationCameraModifier>>().read() }
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
    pub fn stop_camera_animation(
        &mut self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        handle: &FCameraAnimationHandle,
        b_immediate: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_cameras_subsystem_stop_camera_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(8).cast::<FCameraAnimationHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediate,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_cameras_subsystem_stop_camera_animation,
                __buffer,
            )
        };
    }
    pub fn stop_all_camera_animations_of(
        &mut self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        sequence: UPtr<crate::bindings::template_sequence::UCameraAnimationSequence>,
        b_immediate: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_cameras_subsystem_stop_all_camera_animations_of,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<
                            crate::bindings::template_sequence::UCameraAnimationSequence,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediate,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_cameras_subsystem_stop_all_camera_animations_of,
                __buffer,
            )
        };
    }
    pub fn stop_all_camera_animations(
        &mut self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        b_immediate: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_cameras_subsystem_stop_all_camera_animations,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediate,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_cameras_subsystem_stop_all_camera_animations,
                __buffer,
            )
        };
    }
    pub fn play_camera_animation(
        &mut self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        sequence: UPtr<crate::bindings::template_sequence::UCameraAnimationSequence>,
        params: FCameraAnimationParams,
    ) -> FCameraAnimationHandle {
        let mut __stack = crate::core_data::StackAlloc::<92>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_cameras_subsystem_play_camera_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<
                            crate::bindings::template_sequence::UCameraAnimationSequence,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &params,
                __buffer.add(16).cast::<FCameraAnimationParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_cameras_subsystem_play_camera_animation,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<FCameraAnimationHandle>().read() }
    }
    pub fn is_camera_animation_active(
        &self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        handle: &FCameraAnimationHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_cameras_subsystem_is_camera_animation_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(8).cast::<FCameraAnimationHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_engine_cameras_subsystem_is_camera_animation_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
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
    pub fn start_legacy_camera_shake_from_source(
        player_camera_manager: UPtr<crate::bindings::engine::APlayerCameraManager>,
        shake_class: TSubclassOf<ULegacyCameraShake>,
        source_component: UPtr<crate::bindings::engine::UCameraShakeSourceComponent>,
        scale: f32,
        play_space: crate::bindings::engine::ECameraShakePlaySpace,
        user_play_space_rot: crate::bindings::core_u_object::FRotator,
    ) -> UPtr<ULegacyCameraShake> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_start_legacy_camera_shake_from_source,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_camera_manager,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerCameraManager>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &shake_class,
                __buffer.add(8).cast::<TSubclassOf<ULegacyCameraShake>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_component,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<crate::bindings::engine::UCameraShakeSourceComponent>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(24).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &play_space,
                __buffer
                    .add(28)
                    .cast::<crate::bindings::engine::ECameraShakePlaySpace>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_play_space_rot,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::engine_cameras::ULegacyCameraShake::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_start_legacy_camera_shake_from_source,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<ULegacyCameraShake>>().read() }
    }
    pub fn start_legacy_camera_shake(
        player_camera_manager: UPtr<crate::bindings::engine::APlayerCameraManager>,
        shake_class: TSubclassOf<ULegacyCameraShake>,
        scale: f32,
        play_space: crate::bindings::engine::ECameraShakePlaySpace,
        user_play_space_rot: crate::bindings::core_u_object::FRotator,
    ) -> UPtr<ULegacyCameraShake> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_start_legacy_camera_shake,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_camera_manager,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerCameraManager>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &shake_class,
                __buffer.add(8).cast::<TSubclassOf<ULegacyCameraShake>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &play_space,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::engine::ECameraShakePlaySpace>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_play_space_rot,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::engine_cameras::ULegacyCameraShake::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_start_legacy_camera_shake,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<ULegacyCameraShake>>().read() }
    }
    pub fn receive_stop_shake(&mut self, b_immediately: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_receive_stop_shake,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediately,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_receive_stop_shake,
                __buffer,
            )
        };
    }
    pub fn receive_play_shake(&mut self, scale: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_receive_play_shake,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_receive_play_shake,
                __buffer,
            )
        };
    }
    pub fn receive_is_finished(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_receive_is_finished,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_receive_is_finished,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn blueprint_update_camera_shake(
        &mut self,
        delta_time: f32,
        alpha: f32,
        pov: &crate::bindings::engine::FMinimalViewInfo,
        modified_pov: &mut crate::bindings::engine::FMinimalViewInfo,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<4624>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_blueprint_update_camera_shake,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&delta_time, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&alpha, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                pov,
                __buffer.add(16).cast::<crate::bindings::engine::FMinimalViewInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                modified_pov,
                __buffer.add(2320).cast::<crate::bindings::engine::FMinimalViewInfo>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_blueprint_update_camera_shake,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(2320)
                .cast::<crate::bindings::engine::FMinimalViewInfo>()
                .swap(modified_pov);
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
    pub fn conv_legacy_camera_shake(
        camera_shake: UPtr<crate::bindings::engine::UCameraShakeBase>,
    ) -> UPtr<ULegacyCameraShake> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_function_library_conv_legacy_camera_shake,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_shake,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UCameraShakeBase>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::engine_cameras::ULegacyCameraShakeFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_cameras::__FUNCTION_PTRS
                    .u_legacy_camera_shake_function_library_conv_legacy_camera_shake,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<ULegacyCameraShake>>().read() }
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
