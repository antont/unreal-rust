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
    pub u_editor_validator_base_k2_validate_loaded_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_base_k2_can_validate_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_base_k2_can_validate: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_base_get_validation_result: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_base_asset_warning: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_base_asset_passes: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_base_asset_fails: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_subsystem_validate_changelists: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_subsystem_validate_changelist: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_subsystem_validate_assets_with_settings: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_subsystem_remove_validator: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_subsystem_is_object_valid: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_subsystem_is_asset_valid: *mut crate::ffi::UFunctionOpague,
    pub u_editor_validator_subsystem_add_validator: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_editor_validator_base_k2_validate_loaded_asset: std::ptr::null_mut(),
            u_editor_validator_base_k2_can_validate_asset: std::ptr::null_mut(),
            u_editor_validator_base_k2_can_validate: std::ptr::null_mut(),
            u_editor_validator_base_get_validation_result: std::ptr::null_mut(),
            u_editor_validator_base_asset_warning: std::ptr::null_mut(),
            u_editor_validator_base_asset_passes: std::ptr::null_mut(),
            u_editor_validator_base_asset_fails: std::ptr::null_mut(),
            u_editor_validator_subsystem_validate_changelists: std::ptr::null_mut(),
            u_editor_validator_subsystem_validate_changelist: std::ptr::null_mut(),
            u_editor_validator_subsystem_validate_assets_with_settings: std::ptr::null_mut(),
            u_editor_validator_subsystem_remove_validator: std::ptr::null_mut(),
            u_editor_validator_subsystem_is_object_valid: std::ptr::null_mut(),
            u_editor_validator_subsystem_is_asset_valid: std::ptr::null_mut(),
            u_editor_validator_subsystem_add_validator: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorValidatorBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_ValidateLoadedAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_validator_base_k2_validate_loaded_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_CanValidateAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_validator_base_k2_can_validate_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_CanValidate"),
            &raw mut __FUNCTION_PTRS.u_editor_validator_base_k2_can_validate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValidationResult"),
            &raw mut __FUNCTION_PTRS.u_editor_validator_base_get_validation_result,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssetWarning"),
            &raw mut __FUNCTION_PTRS.u_editor_validator_base_asset_warning,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssetPasses"),
            &raw mut __FUNCTION_PTRS.u_editor_validator_base_asset_passes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssetFails"),
            &raw mut __FUNCTION_PTRS.u_editor_validator_base_asset_fails,
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
            &raw mut __FUNCTION_PTRS.u_editor_validator_subsystem_validate_changelists,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ValidateChangelist"),
            &raw mut __FUNCTION_PTRS.u_editor_validator_subsystem_validate_changelist,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ValidateAssetsWithSettings"),
            &raw mut __FUNCTION_PTRS
                .u_editor_validator_subsystem_validate_assets_with_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveValidator"),
            &raw mut __FUNCTION_PTRS.u_editor_validator_subsystem_remove_validator,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsObjectValid"),
            &raw mut __FUNCTION_PTRS.u_editor_validator_subsystem_is_object_valid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAssetValid"),
            &raw mut __FUNCTION_PTRS.u_editor_validator_subsystem_is_asset_valid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddValidator"),
            &raw mut __FUNCTION_PTRS.u_editor_validator_subsystem_add_validator,
        );
    }
}
#[repr(C, align(8))]
pub struct FValidateAssetsDetails {
    pub(crate) __padding_end: [u8; 96],
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
    pub(crate) __padding_end: [u8; 47],
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
    pub(crate) __padding_1136: [u8; 1136],
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
    pub fn k2_validate_loaded_asset(
        &mut self,
        in_asset: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> crate::bindings::core_u_object::EDataValidationResult {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_k2_validate_loaded_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_k2_validate_loaded_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::core_u_object::EDataValidationResult>()
                .read()
        }
    }
    pub fn k2_can_validate_asset(
        &self,
        in_asset: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_k2_can_validate_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_k2_can_validate_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn k2_can_validate(
        &self,
        in_usecase: crate::bindings::core_u_object::EDataValidationUsecase,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_k2_can_validate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_usecase,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::core_u_object::EDataValidationUsecase>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_k2_can_validate,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_validation_result(
        &self,
    ) -> crate::bindings::core_u_object::EDataValidationResult {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_get_validation_result,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_get_validation_result,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::EDataValidationResult>()
                .read()
        }
    }
    pub fn asset_warning(
        &mut self,
        in_asset: UPtr<crate::bindings::core_u_object::UObject>,
        in_message: &FText,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_asset_warning,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_message,
                __buffer.add(8).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_asset_warning,
                __buffer,
            )
        };
    }
    pub fn asset_passes(
        &mut self,
        in_asset: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_asset_passes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_asset_passes,
                __buffer,
            )
        };
    }
    pub fn asset_fails(
        &mut self,
        in_asset: UPtr<crate::bindings::core_u_object::UObject>,
        in_message: &FText,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_asset_fails,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_message,
                __buffer.add(8).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_base_asset_fails,
                __buffer,
            )
        };
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
    pub fn validate_changelists(
        &self,
        in_changelists: TArray<UPtr<UDataValidationChangelist>>,
        in_settings: &FValidateAssetsSettings,
        out_results: &mut FValidateAssetsResults,
    ) -> crate::bindings::core_u_object::EDataValidationResult {
        let mut __stack = crate::core_data::StackAlloc::<281>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_validate_changelists,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_changelists,
                __buffer.add(0).cast::<TArray<UPtr<UDataValidationChangelist>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(16).cast::<FValidateAssetsSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_results,
                __buffer.add(80).cast::<FValidateAssetsResults>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_validate_changelists,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(80).cast::<FValidateAssetsResults>().swap(out_results);
        }
        unsafe {
            __buffer
                .add(280)
                .cast::<crate::bindings::core_u_object::EDataValidationResult>()
                .read()
        }
    }
    pub fn validate_changelist(
        &self,
        in_changelist: UPtr<UDataValidationChangelist>,
        in_settings: &FValidateAssetsSettings,
        out_results: &mut FValidateAssetsResults,
    ) -> crate::bindings::core_u_object::EDataValidationResult {
        let mut __stack = crate::core_data::StackAlloc::<273>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_validate_changelist,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_changelist,
                __buffer.add(0).cast::<UPtr<UDataValidationChangelist>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(8).cast::<FValidateAssetsSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_results,
                __buffer.add(72).cast::<FValidateAssetsResults>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_validate_changelist,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<FValidateAssetsResults>().swap(out_results);
        }
        unsafe {
            __buffer
                .add(272)
                .cast::<crate::bindings::core_u_object::EDataValidationResult>()
                .read()
        }
    }
    pub fn validate_assets_with_settings(
        &self,
        asset_data_list: &TArray<crate::bindings::core_u_object::FAssetData>,
        in_settings: &FValidateAssetsSettings,
        out_results: &mut FValidateAssetsResults,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<284>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_validate_assets_with_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_data_list,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(16).cast::<FValidateAssetsSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_results,
                __buffer.add(80).cast::<FValidateAssetsResults>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_validate_assets_with_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(80).cast::<FValidateAssetsResults>().swap(out_results);
        }
        unsafe { __buffer.add(280).cast::<i32>().read() }
    }
    pub fn remove_validator(&mut self, in_validator: UPtr<UEditorValidatorBase>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_remove_validator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_validator,
                __buffer.add(0).cast::<UPtr<UEditorValidatorBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_remove_validator,
                __buffer,
            )
        };
    }
    pub fn is_object_valid(
        &self,
        in_object: UPtr<crate::bindings::core_u_object::UObject>,
        validation_errors: &mut TArray<FText>,
        validation_warnings: &mut TArray<FText>,
        in_validation_usecase: crate::bindings::core_u_object::EDataValidationUsecase,
    ) -> crate::bindings::core_u_object::EDataValidationResult {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_is_object_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                validation_errors,
                __buffer.add(8).cast::<TArray<FText>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                validation_warnings,
                __buffer.add(24).cast::<TArray<FText>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_validation_usecase,
                __buffer
                    .add(40)
                    .cast::<crate::bindings::core_u_object::EDataValidationUsecase>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_is_object_valid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FText>>().swap(validation_errors);
        }
        unsafe {
            __buffer.add(24).cast::<TArray<FText>>().swap(validation_warnings);
        }
        unsafe {
            __buffer
                .add(41)
                .cast::<crate::bindings::core_u_object::EDataValidationResult>()
                .read()
        }
    }
    pub fn is_asset_valid(
        &self,
        asset_data: &crate::bindings::core_u_object::FAssetData,
        validation_errors: &mut TArray<FText>,
        validation_warnings: &mut TArray<FText>,
        in_validation_usecase: crate::bindings::core_u_object::EDataValidationUsecase,
    ) -> crate::bindings::core_u_object::EDataValidationResult {
        let mut __stack = crate::core_data::StackAlloc::<186>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_is_asset_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                validation_errors,
                __buffer.add(152).cast::<TArray<FText>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                validation_warnings,
                __buffer.add(168).cast::<TArray<FText>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_validation_usecase,
                __buffer
                    .add(184)
                    .cast::<crate::bindings::core_u_object::EDataValidationUsecase>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_is_asset_valid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(152).cast::<TArray<FText>>().swap(validation_errors);
        }
        unsafe {
            __buffer.add(168).cast::<TArray<FText>>().swap(validation_warnings);
        }
        unsafe {
            __buffer
                .add(185)
                .cast::<crate::bindings::core_u_object::EDataValidationResult>()
                .read()
        }
    }
    pub fn add_validator(&mut self, in_validator: UPtr<UEditorValidatorBase>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_add_validator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_validator,
                __buffer.add(0).cast::<UPtr<UEditorValidatorBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_validation::__FUNCTION_PTRS
                    .u_editor_validator_subsystem_add_validator,
                __buffer,
            )
        };
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
