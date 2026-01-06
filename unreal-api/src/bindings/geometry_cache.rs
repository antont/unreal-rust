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
pub static mut A_GEOMETRY_CACHE_ACTOR_GET_GEOMETRY_CACHE_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_TICK_AT_THIS_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_STOP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_SET_WIREFRAME_OVERRIDE_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_SET_START_TIME_OFFSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_SET_PLAYBACK_SPEED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_SET_OVERRIDE_WIREFRAME_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_SET_MOTION_VECTOR_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_SET_LOOPING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_SET_GEOMETRY_CACHE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_SET_EXTRAPOLATE_FRAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_PLAY_REVERSED_FROM_END: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_PLAY_REVERSED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_PLAY_FROM_START: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_PLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_PAUSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_IS_PLAYING_REVERSED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_IS_PLAYING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_IS_LOOPING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_IS_EXTRAPOLATING_FRAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_GET_WIREFRAME_OVERRIDE_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_GET_START_TIME_OFFSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_GET_PLAYBACK_SPEED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_GET_PLAYBACK_DIRECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_GET_OVERRIDE_WIREFRAME_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_GET_NUMBER_OF_TRACKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_GET_NUMBER_OF_FRAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_GET_MOTION_VECTOR_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_GET_ELAPSED_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_GET_DURATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GEOMETRY_CACHE_COMPONENT_GET_ANIMATION_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GEOMETRY_CACHE_TRACK_FLIPBOOK_ANIMATION_ADD_MESH_SAMPLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GEOMETRY_CACHE_TRACK_TRANSFORM_ANIMATION_SET_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GEOMETRY_CACHE_TRACK_TRANSFORM_GROUP_ANIMATION_SET_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AGeometryCacheActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGeometryCacheComponent"),
            &raw mut A_GEOMETRY_CACHE_ACTOR_GET_GEOMETRY_CACHE_COMPONENT,
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
            &raw mut U_GEOMETRY_CACHE_COMPONENT_TICK_AT_THIS_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Stop"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_STOP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWireframeOverrideColor"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_SET_WIREFRAME_OVERRIDE_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStartTimeOffset"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_SET_START_TIME_OFFSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaybackSpeed"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_SET_PLAYBACK_SPEED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOverrideWireframeColor"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_SET_OVERRIDE_WIREFRAME_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMotionVectorScale"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_SET_MOTION_VECTOR_SCALE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLooping"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_SET_LOOPING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGeometryCache"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_SET_GEOMETRY_CACHE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetExtrapolateFrames"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_SET_EXTRAPOLATE_FRAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayReversedFromEnd"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_PLAY_REVERSED_FROM_END,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayReversed"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_PLAY_REVERSED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayFromStart"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_PLAY_FROM_START,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Play"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_PLAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Pause"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_PAUSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPlayingReversed"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_IS_PLAYING_REVERSED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPlaying"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_IS_PLAYING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLooping"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_IS_LOOPING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsExtrapolatingFrames"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_IS_EXTRAPOLATING_FRAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWireframeOverrideColor"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_GET_WIREFRAME_OVERRIDE_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStartTimeOffset"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_GET_START_TIME_OFFSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackSpeed"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_GET_PLAYBACK_SPEED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackDirection"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_GET_PLAYBACK_DIRECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOverrideWireframeColor"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_GET_OVERRIDE_WIREFRAME_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumberOfTracks"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_GET_NUMBER_OF_TRACKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumberOfFrames"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_GET_NUMBER_OF_FRAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMotionVectorScale"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_GET_MOTION_VECTOR_SCALE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetElapsedTime"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_GET_ELAPSED_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDuration"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_GET_DURATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationTime"),
            &raw mut U_GEOMETRY_CACHE_COMPONENT_GET_ANIMATION_TIME,
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
            &raw mut UDEPRECATED_GEOMETRY_CACHE_TRACK_FLIPBOOK_ANIMATION_ADD_MESH_SAMPLE,
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
            &raw mut UDEPRECATED_GEOMETRY_CACHE_TRACK_TRANSFORM_ANIMATION_SET_MESH,
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
            &raw mut UDEPRECATED_GEOMETRY_CACHE_TRACK_TRANSFORM_GROUP_ANIMATION_SET_MESH,
        );
    }
}
#[repr(C, align(8))]
pub struct UGeometryCache {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub material_slot_names: TArray<FName>,
    #[doc(hidden)]
    __padding_168: [u8; 64],
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
    __padding_1136: [u8; 1136],
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
    __padding_1576: [u8; 1576],
    pub geometry_cache: UPtr<UGeometryCache>,
    pub b_running: bool,
    pub b_looping: bool,
    pub start_time_offset: f32,
    pub playback_speed: f32,
    pub motion_vector_scale: f32,
    #[doc(hidden)]
    __padding_1684: [u8; 84],
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
