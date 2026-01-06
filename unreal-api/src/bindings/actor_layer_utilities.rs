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
pub static mut U_LAYERS_BLUEPRINT_LIBRARY_REMOVE_ACTOR_FROM_LAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LAYERS_BLUEPRINT_LIBRARY_GET_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LAYERS_BLUEPRINT_LIBRARY_ADD_ACTOR_TO_LAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULayersBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveActorFromLayer"),
            &raw mut U_LAYERS_BLUEPRINT_LIBRARY_REMOVE_ACTOR_FROM_LAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActors"),
            &raw mut U_LAYERS_BLUEPRINT_LIBRARY_GET_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddActorToLayer"),
            &raw mut U_LAYERS_BLUEPRINT_LIBRARY_ADD_ACTOR_TO_LAYER,
        );
    }
}
#[repr(C, align(4))]
pub struct FActorLayer {
    pub name: FName,
}
impl FActorLayer {}
#[repr(C, align(8))]
pub struct ULayersBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl ULayersBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULayersBlueprintLibrary")
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
