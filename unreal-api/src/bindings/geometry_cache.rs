#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub a_geometry_cache_actor_get_geometry_cache_component: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_tick_at_this_time: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_stop: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_set_wireframe_override_color: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_set_start_time_offset: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_set_playback_speed: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_set_override_wireframe_color: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_set_motion_vector_scale: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_set_looping: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_set_geometry_cache: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_set_extrapolate_frames: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_play_reversed_from_end: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_play_reversed: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_play_from_start: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_play: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_pause: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_is_playing_reversed: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_is_playing: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_is_looping: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_is_extrapolating_frames: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_get_wireframe_override_color: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_get_start_time_offset: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_get_playback_speed: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_get_playback_direction: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_get_override_wireframe_color: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_get_number_of_tracks: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_get_number_of_frames: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_get_motion_vector_scale: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_get_elapsed_time: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_get_duration: *mut crate::ffi::UFunctionOpague,
    pub u_geometry_cache_component_get_animation_time: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_geometry_cache_track_flipbook_animation_add_mesh_sample: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_geometry_cache_track_transform_animation_set_mesh: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_geometry_cache_track_transform_group_animation_set_mesh: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            a_geometry_cache_actor_get_geometry_cache_component: std::ptr::null_mut(),
            u_geometry_cache_component_tick_at_this_time: std::ptr::null_mut(),
            u_geometry_cache_component_stop: std::ptr::null_mut(),
            u_geometry_cache_component_set_wireframe_override_color: std::ptr::null_mut(),
            u_geometry_cache_component_set_start_time_offset: std::ptr::null_mut(),
            u_geometry_cache_component_set_playback_speed: std::ptr::null_mut(),
            u_geometry_cache_component_set_override_wireframe_color: std::ptr::null_mut(),
            u_geometry_cache_component_set_motion_vector_scale: std::ptr::null_mut(),
            u_geometry_cache_component_set_looping: std::ptr::null_mut(),
            u_geometry_cache_component_set_geometry_cache: std::ptr::null_mut(),
            u_geometry_cache_component_set_extrapolate_frames: std::ptr::null_mut(),
            u_geometry_cache_component_play_reversed_from_end: std::ptr::null_mut(),
            u_geometry_cache_component_play_reversed: std::ptr::null_mut(),
            u_geometry_cache_component_play_from_start: std::ptr::null_mut(),
            u_geometry_cache_component_play: std::ptr::null_mut(),
            u_geometry_cache_component_pause: std::ptr::null_mut(),
            u_geometry_cache_component_is_playing_reversed: std::ptr::null_mut(),
            u_geometry_cache_component_is_playing: std::ptr::null_mut(),
            u_geometry_cache_component_is_looping: std::ptr::null_mut(),
            u_geometry_cache_component_is_extrapolating_frames: std::ptr::null_mut(),
            u_geometry_cache_component_get_wireframe_override_color: std::ptr::null_mut(),
            u_geometry_cache_component_get_start_time_offset: std::ptr::null_mut(),
            u_geometry_cache_component_get_playback_speed: std::ptr::null_mut(),
            u_geometry_cache_component_get_playback_direction: std::ptr::null_mut(),
            u_geometry_cache_component_get_override_wireframe_color: std::ptr::null_mut(),
            u_geometry_cache_component_get_number_of_tracks: std::ptr::null_mut(),
            u_geometry_cache_component_get_number_of_frames: std::ptr::null_mut(),
            u_geometry_cache_component_get_motion_vector_scale: std::ptr::null_mut(),
            u_geometry_cache_component_get_elapsed_time: std::ptr::null_mut(),
            u_geometry_cache_component_get_duration: std::ptr::null_mut(),
            u_geometry_cache_component_get_animation_time: std::ptr::null_mut(),
            udeprecated_geometry_cache_track_flipbook_animation_add_mesh_sample: std::ptr::null_mut(),
            udeprecated_geometry_cache_track_transform_animation_set_mesh: std::ptr::null_mut(),
            udeprecated_geometry_cache_track_transform_group_animation_set_mesh: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AGeometryCacheActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGeometryCacheComponent"),
            &raw mut __FUNCTION_PTRS.a_geometry_cache_actor_get_geometry_cache_component,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGeometryCacheComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TickAtThisTime"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_tick_at_this_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Stop"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_stop,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWireframeOverrideColor"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_cache_component_set_wireframe_override_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStartTimeOffset"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_set_start_time_offset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaybackSpeed"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_set_playback_speed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOverrideWireframeColor"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_cache_component_set_override_wireframe_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMotionVectorScale"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_set_motion_vector_scale,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLooping"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_set_looping,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGeometryCache"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_set_geometry_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetExtrapolateFrames"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_set_extrapolate_frames,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayReversedFromEnd"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_play_reversed_from_end,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayReversed"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_play_reversed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayFromStart"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_play_from_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Play"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_play,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Pause"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_pause,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPlayingReversed"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_is_playing_reversed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPlaying"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_is_playing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLooping"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_is_looping,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsExtrapolatingFrames"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_is_extrapolating_frames,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWireframeOverrideColor"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_cache_component_get_wireframe_override_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStartTimeOffset"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_get_start_time_offset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackSpeed"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_get_playback_speed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackDirection"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_get_playback_direction,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOverrideWireframeColor"),
            &raw mut __FUNCTION_PTRS
                .u_geometry_cache_component_get_override_wireframe_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumberOfTracks"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_get_number_of_tracks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumberOfFrames"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_get_number_of_frames,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMotionVectorScale"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_get_motion_vector_scale,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetElapsedTime"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_get_elapsed_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDuration"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_get_duration,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationTime"),
            &raw mut __FUNCTION_PTRS.u_geometry_cache_component_get_animation_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDEPRECATED_GeometryCacheTrack_FlipbookAnimation::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddMeshSample"),
            &raw mut __FUNCTION_PTRS
                .udeprecated_geometry_cache_track_flipbook_animation_add_mesh_sample,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDEPRECATED_GeometryCacheTrack_TransformAnimation::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMesh"),
            &raw mut __FUNCTION_PTRS
                .udeprecated_geometry_cache_track_transform_animation_set_mesh,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDEPRECATED_GeometryCacheTrack_TransformGroupAnimation::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMesh"),
            &raw mut __FUNCTION_PTRS
                .udeprecated_geometry_cache_track_transform_group_animation_set_mesh,
        );
    }
}
#[repr(C, align(8))]
pub struct UGeometryCache {
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 72],
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub material_slot_names: TArray<FName>,
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 64],
    pub start_frame: i32,
    pub end_frame: i32,
    __padding_end: [u8; 8],
}
impl UGeometryCache {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCache")
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
pub struct AGeometryCacheActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub geometry_cache_component: UPtr<UGeometryCacheComponent>,
}
impl AGeometryCacheActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGeometryCacheActor")
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
    pub fn get_geometry_cache_component(&self) -> UPtr<UGeometryCacheComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .a_geometry_cache_actor_get_geometry_cache_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .a_geometry_cache_actor_get_geometry_cache_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UGeometryCacheComponent>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UGeometryCacheCodecBase {
    __padding_end: [u8; 72],
}
impl UGeometryCacheCodecBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCacheCodecBase")
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
pub struct UGeometryCacheCodecRaw {
    __padding_end: [u8; 96],
}
impl UGeometryCacheCodecRaw {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCacheCodecRaw")
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
pub struct UGeometryCacheCodecV1 {
    __padding_end: [u8; 112],
}
impl UGeometryCacheCodecV1 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCacheCodecV1")
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
pub struct UGeometryCacheComponent {
    #[doc(hidden)]
    pub(crate) __padding_1576: [u8; 1576],
    pub geometry_cache: UPtr<UGeometryCache>,
    pub b_running: bool,
    pub b_looping: bool,
    pub start_time_offset: f32,
    pub playback_speed: f32,
    pub motion_vector_scale: f32,
    #[doc(hidden)]
    pub(crate) __padding_1684: [u8; 84],
    pub duration: f32,
    __padding_end: [u8; 24],
}
impl UGeometryCacheComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCacheComponent")
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
    pub fn tick_at_this_time(
        &mut self,
        time: f32,
        b_in_is_running: bool,
        b_in_backwards: bool,
        b_in_is_looping: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<7>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_tick_at_this_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_running,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_backwards,
                __buffer.add(5).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_looping,
                __buffer.add(6).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_tick_at_this_time,
                __buffer,
            )
        };
    }
    pub fn stop(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_stop,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_stop,
                __buffer,
            )
        };
    }
    pub fn set_wireframe_override_color(
        &mut self,
        color: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_wireframe_override_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &color,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_wireframe_override_color,
                __buffer,
            )
        };
    }
    pub fn set_start_time_offset(&mut self, new_start_time_offset: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_start_time_offset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_start_time_offset,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_start_time_offset,
                __buffer,
            )
        };
    }
    pub fn set_playback_speed(&mut self, new_playback_speed: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_playback_speed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_playback_speed,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_playback_speed,
                __buffer,
            )
        };
    }
    pub fn set_override_wireframe_color(&mut self, b_override: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_override_wireframe_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_override,
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
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_override_wireframe_color,
                __buffer,
            )
        };
    }
    pub fn set_motion_vector_scale(&mut self, new_motion_vector_scale: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_motion_vector_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_motion_vector_scale,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_motion_vector_scale,
                __buffer,
            )
        };
    }
    pub fn set_looping(&mut self, b_new_looping: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_looping,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_looping,
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
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_looping,
                __buffer,
            )
        };
    }
    pub fn set_geometry_cache(&mut self, new_geom_cache: UPtr<UGeometryCache>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_geometry_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_geom_cache,
                __buffer.add(0).cast::<UPtr<UGeometryCache>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_geometry_cache,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_extrapolate_frames(&mut self, b_new_extrapolating: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_extrapolate_frames,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_extrapolating,
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
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_set_extrapolate_frames,
                __buffer,
            )
        };
    }
    pub fn play_reversed_from_end(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_play_reversed_from_end,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_play_reversed_from_end,
                __buffer,
            )
        };
    }
    pub fn play_reversed(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_play_reversed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_play_reversed,
                __buffer,
            )
        };
    }
    pub fn play_from_start(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_play_from_start,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_play_from_start,
                __buffer,
            )
        };
    }
    pub fn play(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_play,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_play,
                __buffer,
            )
        };
    }
    pub fn pause(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_pause,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_pause,
                __buffer,
            )
        };
    }
    pub fn is_playing_reversed(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_is_playing_reversed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_is_playing_reversed,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_playing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_is_playing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_is_playing,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_looping(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_is_looping,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_is_looping,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_extrapolating_frames(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_is_extrapolating_frames,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_is_extrapolating_frames,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_wireframe_override_color(
        &self,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_wireframe_override_color,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_wireframe_override_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>().read()
        }
    }
    pub fn get_start_time_offset(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_start_time_offset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_start_time_offset,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_playback_speed(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_playback_speed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_playback_speed,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_playback_direction(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_playback_direction,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_playback_direction,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_override_wireframe_color(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_override_wireframe_color,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_override_wireframe_color,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_number_of_tracks(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_number_of_tracks,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_number_of_tracks,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_number_of_frames(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_number_of_frames,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_number_of_frames,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_motion_vector_scale(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_motion_vector_scale,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_motion_vector_scale,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_elapsed_time(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_elapsed_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_elapsed_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_duration(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_duration,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_duration,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_animation_time(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_animation_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_cache::__FUNCTION_PTRS
                    .u_geometry_cache_component_get_animation_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UGeometryCacheTrack {
    __padding_end: [u8; 96],
}
impl UGeometryCacheTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCacheTrack")
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
pub struct UDEPRECATED_GeometryCacheTrack_FlipbookAnimation {
    __padding_end: [u8; 136],
}
impl UDEPRECATED_GeometryCacheTrack_FlipbookAnimation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_GeometryCacheTrack_FlipbookAnimation")
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
pub struct UGeometryCacheTrackStreamable {
    __padding_end: [u8; 256],
}
impl UGeometryCacheTrackStreamable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometryCacheTrackStreamable")
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
pub struct UDEPRECATED_GeometryCacheTrack_TransformAnimation {
    __padding_end: [u8; 312],
}
impl UDEPRECATED_GeometryCacheTrack_TransformAnimation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_GeometryCacheTrack_TransformAnimation")
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
pub struct UDEPRECATED_GeometryCacheTrack_TransformGroupAnimation {
    __padding_end: [u8; 312],
}
impl UDEPRECATED_GeometryCacheTrack_TransformGroupAnimation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_GeometryCacheTrack_TransformGroupAnimation")
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
pub struct UNiagaraGeometryCacheRendererProperties {
    __padding_end: [u8; 3576],
}
impl UNiagaraGeometryCacheRendererProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraGeometryCacheRendererProperties")
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
