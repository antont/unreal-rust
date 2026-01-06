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
pub static mut UDNA_MESH_VERTEX_COLOR_DATA_ASSET_GET_COLOR_BY_MESH_AND_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDNAMeshVertexColorDataAsset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetColorByMeshAndIndex"),
            &raw mut UDNA_MESH_VERTEX_COLOR_DATA_ASSET_GET_COLOR_BY_MESH_AND_INDEX,
        );
    }
}
#[repr(C, align(8))]
pub struct FMeshVertexColorData {
    pub mesh_name: FString,
    pub colors: TArray<crate::bindings::core_u_object::FLinearColor>,
}
impl FMeshVertexColorData {}
#[repr(C, align(8))]
pub struct UDNAMeshVertexColorDataAsset {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub mesh_color_entries: TArray<FMeshVertexColorData>,
}
impl UDNAMeshVertexColorDataAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDNAMeshVertexColorDataAsset")
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
pub struct UMetaHumanInterchangeDnaTranslator {
    __padding_end: [u8; 184],
}
impl UMetaHumanInterchangeDnaTranslator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanInterchangeDnaTranslator")
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
