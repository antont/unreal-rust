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
pub static mut A_CHAOS_CACHE_MANAGER_TRIGGER_COMPONENT_BY_CACHE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_TRIGGER_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_TRIGGER_ALL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_STOP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_START: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_SET_START_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_SET_CURRENT_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_SET_CACHE_COLLECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_RESET_SINGLE_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_RESET_ALL_COMPONENT_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_REMOVE_OBSERVED_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_IS_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_FIND_OR_ADD_OBSERVED_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_ENABLE_PLAYBACK_BY_CACHE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_ENABLE_PLAYBACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_CHAOS_CACHE_MANAGER_CLEAR_OBSERVED_COMPONENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AChaosCacheManager::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TriggerComponentByCache"),
            &raw mut A_CHAOS_CACHE_MANAGER_TRIGGER_COMPONENT_BY_CACHE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TriggerComponent"),
            &raw mut A_CHAOS_CACHE_MANAGER_TRIGGER_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TriggerAll"),
            &raw mut A_CHAOS_CACHE_MANAGER_TRIGGER_ALL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Stop"),
            &raw mut A_CHAOS_CACHE_MANAGER_STOP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Start"),
            &raw mut A_CHAOS_CACHE_MANAGER_START,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStartTime"),
            &raw mut A_CHAOS_CACHE_MANAGER_SET_START_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurrentTime"),
            &raw mut A_CHAOS_CACHE_MANAGER_SET_CURRENT_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCacheCollection"),
            &raw mut A_CHAOS_CACHE_MANAGER_SET_CACHE_COLLECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetSingleTransform"),
            &raw mut A_CHAOS_CACHE_MANAGER_RESET_SINGLE_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetAllComponentTransforms"),
            &raw mut A_CHAOS_CACHE_MANAGER_RESET_ALL_COMPONENT_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveObservedComponent"),
            &raw mut A_CHAOS_CACHE_MANAGER_REMOVE_OBSERVED_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRecording"),
            &raw mut A_CHAOS_CACHE_MANAGER_IS_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindOrAddObservedComponent"),
            &raw mut A_CHAOS_CACHE_MANAGER_FIND_OR_ADD_OBSERVED_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnablePlaybackByCache"),
            &raw mut A_CHAOS_CACHE_MANAGER_ENABLE_PLAYBACK_BY_CACHE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnablePlayback"),
            &raw mut A_CHAOS_CACHE_MANAGER_ENABLE_PLAYBACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearObservedComponents"),
            &raw mut A_CHAOS_CACHE_MANAGER_CLEAR_OBSERVED_COMPONENTS,
        );
    }
}
#[repr(C, align(16))]
pub struct FObservedComponent {
    pub cache_name: FName,
    #[doc(hidden)]
    __padding_64: [u8; 48],
    pub soft_component_ref: crate::bindings::engine::FSoftComponentReference,
    pub b_is_simulating: bool,
    pub b_playback_enabled: bool,
    pub usd_cache_directory: crate::bindings::core_u_object::FDirectoryPath,
    pub b_has_notify_breaks: bool,
    __padding_end: [u8; 255],
}
impl FObservedComponent {}
#[repr(C, align(8))]
pub struct UChaosCacheCollection {
    __padding_end: [u8; 72],
}
impl UChaosCacheCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCacheCollection")
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
pub struct AChaosCacheManager {
    #[doc(hidden)]
    __padding_1148: [u8; 1148],
    pub start_time: f32,
    __padding_end: [u8; 176],
}
impl AChaosCacheManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AChaosCacheManager")
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
pub struct AChaosCachePlayer {
    __padding_end: [u8; 1328],
}
impl AChaosCachePlayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AChaosCachePlayer")
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
pub struct IChaosCacheData {}
#[repr(C, align(8))]
pub struct UChaosCacheData {
    __padding_end: [u8; 48],
}
impl UChaosCacheData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCacheData")
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
pub struct UChaosCache {
    __padding_end: [u8; 912],
}
impl UChaosCache {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosCache")
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
pub struct UMovieSceneChaosCacheSection {
    __padding_end: [u8; 416],
}
impl UMovieSceneChaosCacheSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChaosCacheSection")
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
pub struct UMovieSceneChaosCacheTrack {
    __padding_end: [u8; 408],
}
impl UMovieSceneChaosCacheTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChaosCacheTrack")
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
pub struct UMovieSceneSpawnableChaosCacheBinding {
    __padding_end: [u8; 80],
}
impl UMovieSceneSpawnableChaosCacheBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnableChaosCacheBinding")
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
pub struct EChaosCacheInterpolationMode(pub u8);
impl EChaosCacheInterpolationMode {
    pub const QUAT_INTERP: EChaosCacheInterpolationMode = EChaosCacheInterpolationMode(
        0,
    );
    pub const EULER_INTERP: EChaosCacheInterpolationMode = EChaosCacheInterpolationMode(
        1,
    );
    pub const DUAL_QUAT_INTERP: EChaosCacheInterpolationMode = EChaosCacheInterpolationMode(
        2,
    );
}
#[repr(transparent)]
pub struct ECacheMode(pub u8);
impl ECacheMode {
    pub const NONE: ECacheMode = ECacheMode(0);
    pub const PLAY: ECacheMode = ECacheMode(1);
    pub const RECORD: ECacheMode = ECacheMode(2);
}
#[repr(transparent)]
pub struct EStartMode(pub u8);
impl EStartMode {
    pub const TIMED: EStartMode = EStartMode(0);
    pub const TRIGGERED: EStartMode = EStartMode(1);
}
