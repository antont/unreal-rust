#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(4))]
pub struct FPropertyEntry {
    pub property: crate::bindings::engine::EMaterialProperty,
    pub b_use_custom_size: bool,
    pub custom_size: crate::bindings::core_u_object::FIntPoint,
    pub b_use_constant_value: bool,
    pub constant_value: f32,
}
impl FPropertyEntry {}
#[repr(C, align(8))]
pub struct UMaterialOptions {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub properties: TArray<FPropertyEntry>,
    pub texture_size: crate::bindings::core_u_object::FIntPoint,
    pub lod_indices: TArray<i32>,
    pub b_use_mesh_data: bool,
    pub b_use_specific_uv_index: bool,
    pub texture_coordinate_index: i32,
}
impl UMaterialOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialOptions")
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
pub struct UAssetBakeOptions {
    __padding_end: [u8; 48],
}
impl UAssetBakeOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetBakeOptions")
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
pub struct UMaterialMergeOptions {
    #[doc(hidden)]
    pub(crate) __padding_49: [u8; 49],
    pub blend_mode: crate::bindings::engine::EBlendMode,
}
impl UMaterialMergeOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialMergeOptions")
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
pub struct EMaterialBakeMethod(pub u8);
impl EMaterialBakeMethod {
    pub const INDIVIDUAL_MATERIAL: EMaterialBakeMethod = EMaterialBakeMethod(0);
    pub const ATLAS_MATERIAL: EMaterialBakeMethod = EMaterialBakeMethod(1);
    pub const BINNED_MATERIAL: EMaterialBakeMethod = EMaterialBakeMethod(2);
}
