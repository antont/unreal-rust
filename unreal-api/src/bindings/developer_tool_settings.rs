#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
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
#[repr(C, align(8))]
pub struct UCookerSettings {
    __padding_end: [u8; 232],
}
impl UCookerSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCookerSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCookerSettings")
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
pub struct UPlatformsMenuSettings {
    __padding_end: [u8; 984],
}
impl UPlatformsMenuSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlatformsMenuSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlatformsMenuSettings")
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
pub struct UProjectPackagingSettings {
    __padding_end: [u8; 680],
}
impl UProjectPackagingSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProjectPackagingSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProjectPackagingSettings")
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
pub struct ECookProgressDisplayMode(pub i32);
impl ECookProgressDisplayMode {
    pub const NOTHING: ECookProgressDisplayMode = ECookProgressDisplayMode(0);
    pub const REMAINING_PACKAGES: ECookProgressDisplayMode = ECookProgressDisplayMode(1);
    pub const PACKAGE_NAMES: ECookProgressDisplayMode = ECookProgressDisplayMode(2);
    pub const NAMES_AND_REMAINING_PACKAGES: ECookProgressDisplayMode = ECookProgressDisplayMode(
        3,
    );
    pub const INSTIGATORS: ECookProgressDisplayMode = ECookProgressDisplayMode(4);
    pub const INSTIGATORS_AND_COUNT: ECookProgressDisplayMode = ECookProgressDisplayMode(
        5,
    );
    pub const INSTIGATORS_AND_NAMES: ECookProgressDisplayMode = ECookProgressDisplayMode(
        6,
    );
    pub const INSTIGATORS_AND_NAMES_AND_COUNT: ECookProgressDisplayMode = ECookProgressDisplayMode(
        7,
    );
    pub const MAX: ECookProgressDisplayMode = ECookProgressDisplayMode(8);
}
#[repr(transparent)]
pub struct EBlueprintComponentDataCookingMethod(pub i32);
impl EBlueprintComponentDataCookingMethod {
    pub const DISABLED: EBlueprintComponentDataCookingMethod = EBlueprintComponentDataCookingMethod(
        0,
    );
    pub const ALL_BLUEPRINTS: EBlueprintComponentDataCookingMethod = EBlueprintComponentDataCookingMethod(
        1,
    );
    pub const ENABLED_BLUEPRINTS_ONLY: EBlueprintComponentDataCookingMethod = EBlueprintComponentDataCookingMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EBlueprintPropertyGuidsCookingMethod(pub i32);
impl EBlueprintPropertyGuidsCookingMethod {
    pub const DISABLED: EBlueprintPropertyGuidsCookingMethod = EBlueprintPropertyGuidsCookingMethod(
        0,
    );
    pub const ALL_BLUEPRINTS: EBlueprintPropertyGuidsCookingMethod = EBlueprintPropertyGuidsCookingMethod(
        1,
    );
    pub const ENABLED_BLUEPRINTS_ONLY: EBlueprintPropertyGuidsCookingMethod = EBlueprintPropertyGuidsCookingMethod(
        2,
    );
}
#[repr(transparent)]
pub struct ETextureFormatASTCCompressor(pub i32);
impl ETextureFormatASTCCompressor {
    pub const INTEL_ISPC: ETextureFormatASTCCompressor = ETextureFormatASTCCompressor(0);
    pub const ARM: ETextureFormatASTCCompressor = ETextureFormatASTCCompressor(1);
    pub const MAX: ETextureFormatASTCCompressor = ETextureFormatASTCCompressor(2);
}
#[repr(transparent)]
pub struct EProjectPackagingBuildConfigurations(pub u8);
impl EProjectPackagingBuildConfigurations {
    pub const PPBC_DEBUG: EProjectPackagingBuildConfigurations = EProjectPackagingBuildConfigurations(
        0,
    );
    pub const PPBC_DEBUG_GAME: EProjectPackagingBuildConfigurations = EProjectPackagingBuildConfigurations(
        1,
    );
    pub const PPBC_DEVELOPMENT: EProjectPackagingBuildConfigurations = EProjectPackagingBuildConfigurations(
        2,
    );
    pub const PPBC_TEST: EProjectPackagingBuildConfigurations = EProjectPackagingBuildConfigurations(
        3,
    );
    pub const PPBC_SHIPPING: EProjectPackagingBuildConfigurations = EProjectPackagingBuildConfigurations(
        4,
    );
}
#[repr(transparent)]
pub struct EProjectPackagingBuild(pub i32);
impl EProjectPackagingBuild {
    pub const ALWAYS: EProjectPackagingBuild = EProjectPackagingBuild(0);
    pub const NEVER: EProjectPackagingBuild = EProjectPackagingBuild(1);
    pub const IF_PROJECT_HAS_CODE: EProjectPackagingBuild = EProjectPackagingBuild(2);
    pub const IF_EDITOR_WAS_BUILT_LOCALLY: EProjectPackagingBuild = EProjectPackagingBuild(
        3,
    );
}
#[repr(transparent)]
pub struct EProjectPackagingBlueprintNativizationMethod(pub u8);
impl EProjectPackagingBlueprintNativizationMethod {
    pub const DISABLED: EProjectPackagingBlueprintNativizationMethod = EProjectPackagingBlueprintNativizationMethod(
        0,
    );
    pub const INCLUSIVE: EProjectPackagingBlueprintNativizationMethod = EProjectPackagingBlueprintNativizationMethod(
        1,
    );
    pub const EXCLUSIVE: EProjectPackagingBlueprintNativizationMethod = EProjectPackagingBlueprintNativizationMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EAssetRegistryWritebackMethod(pub u8);
impl EAssetRegistryWritebackMethod {
    pub const DISABLED: EAssetRegistryWritebackMethod = EAssetRegistryWritebackMethod(0);
    pub const ORIGINAL_FILE: EAssetRegistryWritebackMethod = EAssetRegistryWritebackMethod(
        1,
    );
    pub const ADJACENT_FILE: EAssetRegistryWritebackMethod = EAssetRegistryWritebackMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EProjectPackagingInternationalizationPresets(pub u8);
impl EProjectPackagingInternationalizationPresets {
    pub const ENGLISH: EProjectPackagingInternationalizationPresets = EProjectPackagingInternationalizationPresets(
        0,
    );
    pub const EFIGS: EProjectPackagingInternationalizationPresets = EProjectPackagingInternationalizationPresets(
        1,
    );
    pub const EFIGSCJK: EProjectPackagingInternationalizationPresets = EProjectPackagingInternationalizationPresets(
        2,
    );
    pub const CJK: EProjectPackagingInternationalizationPresets = EProjectPackagingInternationalizationPresets(
        3,
    );
    pub const ALL: EProjectPackagingInternationalizationPresets = EProjectPackagingInternationalizationPresets(
        4,
    );
}
