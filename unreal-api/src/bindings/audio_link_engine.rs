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
pub static mut U_AUDIO_LINK_BLUEPRINT_INTERFACE_STOP_LINK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_LINK_BLUEPRINT_INTERFACE_SET_LINK_SOUND: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_LINK_BLUEPRINT_INTERFACE_PLAY_LINK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUDIO_LINK_BLUEPRINT_INTERFACE_IS_LINK_PLAYING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioLinkBlueprintInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopLink"),
            &raw mut U_AUDIO_LINK_BLUEPRINT_INTERFACE_STOP_LINK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLinkSound"),
            &raw mut U_AUDIO_LINK_BLUEPRINT_INTERFACE_SET_LINK_SOUND,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayLink"),
            &raw mut U_AUDIO_LINK_BLUEPRINT_INTERFACE_PLAY_LINK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLinkPlaying"),
            &raw mut U_AUDIO_LINK_BLUEPRINT_INTERFACE_IS_LINK_PLAYING,
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
}
