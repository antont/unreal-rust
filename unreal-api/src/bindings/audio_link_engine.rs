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
    pub u_audio_link_blueprint_interface_stop_link: *mut crate::ffi::UFunctionOpague,
    pub u_audio_link_blueprint_interface_set_link_sound: *mut crate::ffi::UFunctionOpague,
    pub u_audio_link_blueprint_interface_play_link: *mut crate::ffi::UFunctionOpague,
    pub u_audio_link_blueprint_interface_is_link_playing: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_audio_link_blueprint_interface_stop_link: std::ptr::null_mut(),
            u_audio_link_blueprint_interface_set_link_sound: std::ptr::null_mut(),
            u_audio_link_blueprint_interface_play_link: std::ptr::null_mut(),
            u_audio_link_blueprint_interface_is_link_playing: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioLinkBlueprintInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopLink"),
            &raw mut __FUNCTION_PTRS.u_audio_link_blueprint_interface_stop_link,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLinkSound"),
            &raw mut __FUNCTION_PTRS.u_audio_link_blueprint_interface_set_link_sound,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayLink"),
            &raw mut __FUNCTION_PTRS.u_audio_link_blueprint_interface_play_link,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLinkPlaying"),
            &raw mut __FUNCTION_PTRS.u_audio_link_blueprint_interface_is_link_playing,
        );
    }
}
pub struct IAudioLinkBlueprintInterface {}
#[repr(C, align(8))]
pub struct UAudioLinkBlueprintInterface {
    __padding_end: [u8; 48],
}
impl UAudioLinkBlueprintInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioLinkBlueprintInterface")
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
    pub fn stop_link(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_link_engine::__FUNCTION_PTRS
                    .u_audio_link_blueprint_interface_stop_link,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_link_engine::__FUNCTION_PTRS
                    .u_audio_link_blueprint_interface_stop_link,
                __buffer,
            )
        };
    }
    pub fn set_link_sound(
        &mut self,
        new_sound: UPtr<crate::bindings::engine::USoundBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_link_engine::__FUNCTION_PTRS
                    .u_audio_link_blueprint_interface_set_link_sound,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_sound,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USoundBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_link_engine::__FUNCTION_PTRS
                    .u_audio_link_blueprint_interface_set_link_sound,
                __buffer,
            )
        };
    }
    pub fn play_link(&mut self, start_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_link_engine::__FUNCTION_PTRS
                    .u_audio_link_blueprint_interface_play_link,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&start_time, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_link_engine::__FUNCTION_PTRS
                    .u_audio_link_blueprint_interface_play_link,
                __buffer,
            )
        };
    }
    pub fn is_link_playing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_link_engine::__FUNCTION_PTRS
                    .u_audio_link_blueprint_interface_is_link_playing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_link_engine::__FUNCTION_PTRS
                    .u_audio_link_blueprint_interface_is_link_playing,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
