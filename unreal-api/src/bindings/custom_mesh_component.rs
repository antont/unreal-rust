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
pub static mut U_CUSTOM_MESH_COMPONENT_SET_CUSTOM_MESH_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CUSTOM_MESH_COMPONENT_CLEAR_CUSTOM_MESH_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CUSTOM_MESH_COMPONENT_ADD_CUSTOM_MESH_TRIANGLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCustomMeshComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomMeshTriangles"),
            &raw mut U_CUSTOM_MESH_COMPONENT_SET_CUSTOM_MESH_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearCustomMeshTriangles"),
            &raw mut U_CUSTOM_MESH_COMPONENT_CLEAR_CUSTOM_MESH_TRIANGLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCustomMeshTriangles"),
            &raw mut U_CUSTOM_MESH_COMPONENT_ADD_CUSTOM_MESH_TRIANGLES,
        );
    }
}
#[repr(C, align(8))]
pub struct FCustomMeshTriangle {
    pub vertex0: crate::bindings::core_u_object::FVector,
    pub vertex1: crate::bindings::core_u_object::FVector,
    pub vertex2: crate::bindings::core_u_object::FVector,
}
impl FCustomMeshTriangle {}
#[repr(C, align(16))]
pub struct UCustomMeshComponent {
    __padding_end: [u8; 1600],
}
impl UCustomMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCustomMeshComponent")
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
