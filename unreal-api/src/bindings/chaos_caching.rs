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
    pub fn trigger_component_by_cache(&mut self, in_cache_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_TRIGGER_COMPONENT_BY_CACHE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cache_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_TRIGGER_COMPONENT_BY_CACHE,
                __buffer,
            )
        };
    }
    pub fn trigger_component(
        &mut self,
        in_component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_TRIGGER_COMPONENT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_TRIGGER_COMPONENT,
                __buffer,
            )
        };
    }
    pub fn trigger_all(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_TRIGGER_ALL,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_TRIGGER_ALL,
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
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_STOP,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_STOP,
                __buffer,
            )
        };
    }
    pub fn start(&mut self, in_start_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_START,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_start_time,
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
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_START,
                __buffer,
            )
        };
    }
    pub fn set_current_time(&mut self, current_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_SET_CURRENT_TIME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &current_time,
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
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_SET_CURRENT_TIME,
                __buffer,
            )
        };
    }
    pub fn set_cache_collection(
        &mut self,
        in_cache_collection: UPtr<UChaosCacheCollection>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_SET_CACHE_COLLECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cache_collection,
                __buffer.add(0).cast::<UPtr<UChaosCacheCollection>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_SET_CACHE_COLLECTION,
                __buffer,
            )
        };
    }
    pub fn reset_single_transform(&mut self, in_index: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_RESET_SINGLE_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_RESET_SINGLE_TRANSFORM,
                __buffer,
            )
        };
    }
    pub fn reset_all_component_transforms(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_RESET_ALL_COMPONENT_TRANSFORMS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_RESET_ALL_COMPONENT_TRANSFORMS,
                __buffer,
            )
        };
    }
    pub fn remove_observed_component(
        &mut self,
        in_component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_REMOVE_OBSERVED_COMPONENT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_REMOVE_OBSERVED_COMPONENT,
                __buffer,
            )
        };
    }
    pub fn is_recording(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_IS_RECORDING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_IS_RECORDING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn find_or_add_observed_component(
        &mut self,
        in_component: UPtr<crate::bindings::engine::UPrimitiveComponent>,
        cache_name: &FName,
        b_transfer_simulation_flag: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_FIND_OR_ADD_OBSERVED_COMPONENT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                cache_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_transfer_simulation_flag,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_FIND_OR_ADD_OBSERVED_COMPONENT,
                __buffer,
            )
        };
    }
    pub fn enable_playback_by_cache(&mut self, in_cache_name: FName, b_enable: bool) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_ENABLE_PLAYBACK_BY_CACHE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cache_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enable, __buffer.add(12).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_ENABLE_PLAYBACK_BY_CACHE,
                __buffer,
            )
        };
    }
    pub fn enable_playback(&mut self, index: i32, b_enable: bool) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_ENABLE_PLAYBACK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enable, __buffer.add(4).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_ENABLE_PLAYBACK,
                __buffer,
            )
        };
    }
    pub fn clear_observed_components(&mut self, b_destroy_components: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_CLEAR_OBSERVED_COMPONENTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_destroy_components,
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
                crate::bindings::chaos_caching::A_CHAOS_CACHE_MANAGER_CLEAR_OBSERVED_COMPONENTS,
                __buffer,
            )
        };
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
