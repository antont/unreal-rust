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
pub static mut U_STATIC_MESH_DESCRIPTION_SET_VERTEX_INSTANCE_UV: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATIC_MESH_DESCRIPTION_SET_POLYGON_GROUP_MATERIAL_SLOT_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATIC_MESH_DESCRIPTION_GET_VERTEX_INSTANCE_UV: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_STATIC_MESH_DESCRIPTION_CREATE_CUBE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UStaticMeshDescription::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVertexInstanceUV"),
            &raw mut U_STATIC_MESH_DESCRIPTION_SET_VERTEX_INSTANCE_UV,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPolygonGroupMaterialSlotName"),
            &raw mut U_STATIC_MESH_DESCRIPTION_SET_POLYGON_GROUP_MATERIAL_SLOT_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexInstanceUV"),
            &raw mut U_STATIC_MESH_DESCRIPTION_GET_VERTEX_INSTANCE_UV,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateCube"),
            &raw mut U_STATIC_MESH_DESCRIPTION_CREATE_CUBE,
        );
    }
}
#[repr(C, align(8))]
pub struct FUVMapSettings {
    pub size: crate::bindings::core_u_object::FVector,
    pub uv_tile: crate::bindings::core_u_object::FVector2D,
    pub position: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
}
impl FUVMapSettings {}
#[repr(C, align(8))]
pub struct UStaticMeshDescription {
    __padding_end: [u8; 760],
}
impl UStaticMeshDescription {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshDescription")
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
