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
pub static mut U_EDITOR_VALIDATOR_BASE_K2_VALIDATE_LOADED_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_BASE_K2_CAN_VALIDATE_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_BASE_K2_CAN_VALIDATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_BASE_GET_VALIDATION_RESULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_BASE_ASSET_WARNING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_BASE_ASSET_PASSES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_BASE_ASSET_FAILS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_SUBSYSTEM_VALIDATE_CHANGELISTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_SUBSYSTEM_VALIDATE_CHANGELIST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_SUBSYSTEM_VALIDATE_ASSETS_WITH_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_SUBSYSTEM_REMOVE_VALIDATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_SUBSYSTEM_IS_OBJECT_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_SUBSYSTEM_IS_ASSET_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_VALIDATOR_SUBSYSTEM_ADD_VALIDATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorValidatorBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_ValidateLoadedAsset"),
            &raw mut U_EDITOR_VALIDATOR_BASE_K2_VALIDATE_LOADED_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_CanValidateAsset"),
            &raw mut U_EDITOR_VALIDATOR_BASE_K2_CAN_VALIDATE_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_CanValidate"),
            &raw mut U_EDITOR_VALIDATOR_BASE_K2_CAN_VALIDATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValidationResult"),
            &raw mut U_EDITOR_VALIDATOR_BASE_GET_VALIDATION_RESULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssetWarning"),
            &raw mut U_EDITOR_VALIDATOR_BASE_ASSET_WARNING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssetPasses"),
            &raw mut U_EDITOR_VALIDATOR_BASE_ASSET_PASSES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssetFails"),
            &raw mut U_EDITOR_VALIDATOR_BASE_ASSET_FAILS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorValidatorSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ValidateChangelists"),
            &raw mut U_EDITOR_VALIDATOR_SUBSYSTEM_VALIDATE_CHANGELISTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ValidateChangelist"),
            &raw mut U_EDITOR_VALIDATOR_SUBSYSTEM_VALIDATE_CHANGELIST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ValidateAssetsWithSettings"),
            &raw mut U_EDITOR_VALIDATOR_SUBSYSTEM_VALIDATE_ASSETS_WITH_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveValidator"),
            &raw mut U_EDITOR_VALIDATOR_SUBSYSTEM_REMOVE_VALIDATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsObjectValid"),
            &raw mut U_EDITOR_VALIDATOR_SUBSYSTEM_IS_OBJECT_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAssetValid"),
            &raw mut U_EDITOR_VALIDATOR_SUBSYSTEM_IS_ASSET_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddValidator"),
            &raw mut U_EDITOR_VALIDATOR_SUBSYSTEM_ADD_VALIDATOR,
        );
    }
}
#[repr(C, align(8))]
pub struct FValidateAssetsDetails {
    __padding_end: [u8; 96],
}
impl FValidateAssetsDetails {}
#[repr(C, align(4))]
pub struct FValidatorStatistics {
    pub assets_validated: i32,
    pub assets_added_for_validation: i32,
}
impl FValidatorStatistics {}
#[repr(C, align(8))]
pub struct FValidateAssetsResults {
    pub num_requested: i32,
    pub num_external_objects: i32,
    pub num_checked: i32,
    pub num_valid: i32,
    pub num_invalid: i32,
    pub num_skipped: i32,
    pub num_warnings: i32,
    pub num_unable_to_validate: i32,
    pub b_asset_limit_reached: bool,
    pub validator_statistics: TMap<
        crate::bindings::core_u_object::FTopLevelAssetPath,
        FValidatorStatistics,
    >,
    pub assets_details: TMap<FString, FValidateAssetsDetails>,
}
impl FValidateAssetsResults {}
#[repr(C, align(8))]
pub struct FValidateAssetsSettings {
    pub b_skip_excluded_directories: bool,
    pub b_show_if_no_failures: bool,
    pub b_collect_per_asset_details: bool,
    pub validation_usecase: crate::bindings::core_u_object::EDataValidationUsecase,
    pub b_load_assets_for_validation: bool,
    pub b_unload_assets_loaded_for_validation: bool,
    pub b_load_external_objects_for_validation: bool,
    pub b_capture_asset_load_logs: bool,
    pub b_capture_logs_during_validation: bool,
    pub b_capture_warnings_during_validation_as_errors: bool,
    pub max_assets_to_validate: i32,
    pub b_validate_referencers_of_deleted_assets: bool,
    __padding_end: [u8; 47],
}
impl FValidateAssetsSettings {}
#[repr(C, align(8))]
pub struct UDataValidationSettings {
    __padding_end: [u8; 128],
}
impl UDataValidationSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataValidationSettings")
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
pub struct UDataValidationChangelist {
    __padding_end: [u8; 144],
}
impl UDataValidationChangelist {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataValidationChangelist")
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
pub struct UDataValidationCommandlet {
    __padding_end: [u8; 136],
}
impl UDataValidationCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataValidationCommandlet")
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
pub struct ADataValidationTestActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub b_pass_validation: bool,
    __padding_end: [u8; 15],
}
impl ADataValidationTestActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADataValidationTestActor")
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
pub struct UEditorValidatorBase {
    __padding_end: [u8; 120],
}
impl UEditorValidatorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorValidatorBase")
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
pub struct UDirtyFilesChangelistValidator {
    __padding_end: [u8; 120],
}
impl UDirtyFilesChangelistValidator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDirtyFilesChangelistValidator")
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
pub struct UEditorValidatorSubsystem {
    __padding_end: [u8; 424],
}
impl UEditorValidatorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorValidatorSubsystem")
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
pub struct UEditorValidator_Localization {
    __padding_end: [u8; 200],
}
impl UEditorValidator_Localization {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorValidator_Localization")
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
pub struct UEditorValidator_Material {
    __padding_end: [u8; 136],
}
impl UEditorValidator_Material {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorValidator_Material")
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
pub struct UValidationMaterial {
    __padding_end: [u8; 3400],
}
impl UValidationMaterial {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UValidationMaterial")
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
pub struct UPackageFileValidator {
    __padding_end: [u8; 128],
}
impl UPackageFileValidator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPackageFileValidator")
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
pub struct UWorldPartitionChangelistValidator {
    __padding_end: [u8; 408],
}
impl UWorldPartitionChangelistValidator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionChangelistValidator")
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
pub struct EMaterialEditorValidationFeatureLevel(pub i32);
impl EMaterialEditorValidationFeatureLevel {
    pub const CURRENT_MAX_FEATURE_LEVEL: EMaterialEditorValidationFeatureLevel = EMaterialEditorValidationFeatureLevel(
        6,
    );
    pub const ES3_1: EMaterialEditorValidationFeatureLevel = EMaterialEditorValidationFeatureLevel(
        1,
    );
    pub const SM5: EMaterialEditorValidationFeatureLevel = EMaterialEditorValidationFeatureLevel(
        3,
    );
    pub const SM6: EMaterialEditorValidationFeatureLevel = EMaterialEditorValidationFeatureLevel(
        4,
    );
}
#[repr(transparent)]
pub struct EMaterialEditorValidationQualityLevel(pub u8);
impl EMaterialEditorValidationQualityLevel {
    pub const LOW: EMaterialEditorValidationQualityLevel = EMaterialEditorValidationQualityLevel(
        0,
    );
    pub const MEDIUM: EMaterialEditorValidationQualityLevel = EMaterialEditorValidationQualityLevel(
        2,
    );
    pub const HIGH: EMaterialEditorValidationQualityLevel = EMaterialEditorValidationQualityLevel(
        1,
    );
    pub const EPIC: EMaterialEditorValidationQualityLevel = EMaterialEditorValidationQualityLevel(
        3,
    );
}
