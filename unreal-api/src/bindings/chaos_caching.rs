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
    pub a_chaos_cache_manager_trigger_component_by_cache: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_trigger_component: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_trigger_all: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_stop: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_start: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_set_start_time: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_set_current_time: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_set_cache_collection: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_reset_single_transform: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_reset_all_component_transforms: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_remove_observed_component: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_is_recording: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_find_or_add_observed_component: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_enable_playback_by_cache: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_enable_playback: *mut crate::ffi::UFunctionOpague,
    pub a_chaos_cache_manager_clear_observed_components: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            a_chaos_cache_manager_trigger_component_by_cache: std::ptr::null_mut(),
            a_chaos_cache_manager_trigger_component: std::ptr::null_mut(),
            a_chaos_cache_manager_trigger_all: std::ptr::null_mut(),
            a_chaos_cache_manager_stop: std::ptr::null_mut(),
            a_chaos_cache_manager_start: std::ptr::null_mut(),
            a_chaos_cache_manager_set_start_time: std::ptr::null_mut(),
            a_chaos_cache_manager_set_current_time: std::ptr::null_mut(),
            a_chaos_cache_manager_set_cache_collection: std::ptr::null_mut(),
            a_chaos_cache_manager_reset_single_transform: std::ptr::null_mut(),
            a_chaos_cache_manager_reset_all_component_transforms: std::ptr::null_mut(),
            a_chaos_cache_manager_remove_observed_component: std::ptr::null_mut(),
            a_chaos_cache_manager_is_recording: std::ptr::null_mut(),
            a_chaos_cache_manager_find_or_add_observed_component: std::ptr::null_mut(),
            a_chaos_cache_manager_enable_playback_by_cache: std::ptr::null_mut(),
            a_chaos_cache_manager_enable_playback: std::ptr::null_mut(),
            a_chaos_cache_manager_clear_observed_components: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AChaosCacheManager::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TriggerComponentByCache"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_trigger_component_by_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TriggerComponent"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_trigger_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TriggerAll"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_trigger_all,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Stop"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_stop,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Start"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStartTime"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_set_start_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurrentTime"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_set_current_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCacheCollection"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_set_cache_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetSingleTransform"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_reset_single_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetAllComponentTransforms"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_reset_all_component_transforms,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveObservedComponent"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_remove_observed_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRecording"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_is_recording,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindOrAddObservedComponent"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_find_or_add_observed_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnablePlaybackByCache"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_enable_playback_by_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnablePlayback"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_enable_playback,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearObservedComponents"),
            &raw mut __FUNCTION_PTRS.a_chaos_cache_manager_clear_observed_components,
        );
    }
}
#[repr(C, align(16))]
pub struct FObservedComponent {
    pub cache_name: FName,
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 48],
    pub soft_component_ref: crate::bindings::engine::FSoftComponentReference,
    pub b_is_simulating: bool,
    pub b_playback_enabled: bool,
    pub usd_cache_directory: crate::bindings::core_u_object::FDirectoryPath,
    pub b_has_notify_breaks: bool,
    pub(crate) __padding_end: [u8; 255],
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
    pub(crate) __padding_1148: [u8; 1148],
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_trigger_component_by_cache,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_trigger_component_by_cache,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_trigger_component,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_trigger_component,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_trigger_all,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_trigger_all,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_stop,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_stop,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_start,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_start,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_set_current_time,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_set_current_time,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_set_cache_collection,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_set_cache_collection,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_reset_single_transform,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_reset_single_transform,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_reset_all_component_transforms,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_reset_all_component_transforms,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_remove_observed_component,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_remove_observed_component,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_is_recording,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_is_recording,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_find_or_add_observed_component,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_find_or_add_observed_component,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_enable_playback_by_cache,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_enable_playback_by_cache,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_enable_playback,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_enable_playback,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_clear_observed_components,
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
                crate::bindings::chaos_caching::__FUNCTION_PTRS
                    .a_chaos_cache_manager_clear_observed_components,
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
