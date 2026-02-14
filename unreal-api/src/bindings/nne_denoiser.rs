#![allow(clippy::all)]
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
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(4))]
pub struct FTilingConfig {
    pub alignment: i32,
    pub overlap: i32,
    pub max_size: i32,
    pub min_size: i32,
}
impl FTilingConfig {}
#[repr(C, align(8))]
pub struct FNNEDenoiserBaseMappingData {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub tensor_index: i32,
    pub tensor_channel: i32,
    pub resource_channel: i32,
}
impl FNNEDenoiserBaseMappingData {}
#[repr(C, align(8))]
pub struct FNNEDenoiserInputMappingData {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub resource: EInputResourceName,
}
impl FNNEDenoiserInputMappingData {}
#[repr(C, align(8))]
pub struct FNNEDenoiserOutputMappingData {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub resource: EOutputResourceName,
}
impl FNNEDenoiserOutputMappingData {}
#[repr(C, align(8))]
pub struct FNNEDenoiserTemporalInputMappingData {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub resource: ETemporalInputResourceName,
    pub frame_index: i32,
}
impl FNNEDenoiserTemporalInputMappingData {}
#[repr(C, align(8))]
pub struct FNNEDenoiserTemporalOutputMappingData {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub resource: ETemporalOutputResourceName,
}
impl FNNEDenoiserTemporalOutputMappingData {}
#[repr(C, align(8))]
pub struct UNNEDenoiserAsset {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub model_data: TSoftObjectPtr<crate::bindings::nne::UNNEModelData>,
    pub input_mapping: TSoftObjectPtr<crate::bindings::engine::UDataTable>,
    pub output_mapping: TSoftObjectPtr<crate::bindings::engine::UDataTable>,
    pub tiling_config: FTilingConfig,
}
impl UNNEDenoiserAsset {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNNEDenoiserAsset")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNNEDenoiserAsset")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNNEDenoiserTemporalAsset {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub model_data: TSoftObjectPtr<crate::bindings::nne::UNNEModelData>,
    pub input_mapping: TSoftObjectPtr<crate::bindings::engine::UDataTable>,
    pub output_mapping: TSoftObjectPtr<crate::bindings::engine::UDataTable>,
    pub tiling_config: FTilingConfig,
}
impl UNNEDenoiserTemporalAsset {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNNEDenoiserTemporalAsset")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNNEDenoiserTemporalAsset")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNNEDenoiserSettings {
    __padding_end: [u8; 224],
}
impl UNNEDenoiserSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNNEDenoiserSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNNEDenoiserSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(transparent)]
pub struct EInputResourceName(pub u8);
impl EInputResourceName {
    pub const COLOR: EInputResourceName = EInputResourceName(0);
    pub const ALBEDO: EInputResourceName = EInputResourceName(1);
    pub const NORMAL: EInputResourceName = EInputResourceName(2);
    pub const OUTPUT: EInputResourceName = EInputResourceName(3);
}
#[repr(transparent)]
pub struct EOutputResourceName(pub u8);
impl EOutputResourceName {
    pub const OUTPUT: EOutputResourceName = EOutputResourceName(0);
}
#[repr(transparent)]
pub struct ETemporalInputResourceName(pub u8);
impl ETemporalInputResourceName {
    pub const COLOR: ETemporalInputResourceName = ETemporalInputResourceName(0);
    pub const ALBEDO: ETemporalInputResourceName = ETemporalInputResourceName(1);
    pub const NORMAL: ETemporalInputResourceName = ETemporalInputResourceName(2);
    pub const FLOW: ETemporalInputResourceName = ETemporalInputResourceName(3);
    pub const OUTPUT: ETemporalInputResourceName = ETemporalInputResourceName(4);
}
#[repr(transparent)]
pub struct ETemporalOutputResourceName(pub u8);
impl ETemporalOutputResourceName {
    pub const OUTPUT: ETemporalOutputResourceName = ETemporalOutputResourceName(0);
}
#[repr(transparent)]
pub struct EDenoiserRuntimeType(pub u8);
impl EDenoiserRuntimeType {
    pub const CPU: EDenoiserRuntimeType = EDenoiserRuntimeType(0);
    pub const GPU: EDenoiserRuntimeType = EDenoiserRuntimeType(1);
    pub const RDG: EDenoiserRuntimeType = EDenoiserRuntimeType(2);
}
