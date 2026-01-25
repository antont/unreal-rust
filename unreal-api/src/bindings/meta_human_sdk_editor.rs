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
    pub u_meta_human_asset_report_set_warnings_as_errors: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_set_verbose: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_set_subject: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_set_silent: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_has_warnings: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_get_report_result: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_generate_rich_text_report: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_generate_raw_report: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_generate_json_report: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_generate_html_report: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_add_warning: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_add_verbose: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_add_info: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_report_add_error: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_manager_update_asset_details: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_manager_update_asset_dependencies: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_manager_is_asset_of_type: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_manager_find_assets_for_packaging: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_asset_manager_create_archive: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_verification_rule_base_verify: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_verification_rule_collection_apply_all_rules: *mut crate::ffi::UFunctionOpague,
    pub u_meta_human_verification_rule_collection_add_verification_rule: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_meta_human_asset_report_set_warnings_as_errors: std::ptr::null_mut(),
            u_meta_human_asset_report_set_verbose: std::ptr::null_mut(),
            u_meta_human_asset_report_set_subject: std::ptr::null_mut(),
            u_meta_human_asset_report_set_silent: std::ptr::null_mut(),
            u_meta_human_asset_report_has_warnings: std::ptr::null_mut(),
            u_meta_human_asset_report_get_report_result: std::ptr::null_mut(),
            u_meta_human_asset_report_generate_rich_text_report: std::ptr::null_mut(),
            u_meta_human_asset_report_generate_raw_report: std::ptr::null_mut(),
            u_meta_human_asset_report_generate_json_report: std::ptr::null_mut(),
            u_meta_human_asset_report_generate_html_report: std::ptr::null_mut(),
            u_meta_human_asset_report_add_warning: std::ptr::null_mut(),
            u_meta_human_asset_report_add_verbose: std::ptr::null_mut(),
            u_meta_human_asset_report_add_info: std::ptr::null_mut(),
            u_meta_human_asset_report_add_error: std::ptr::null_mut(),
            u_meta_human_asset_manager_update_asset_details: std::ptr::null_mut(),
            u_meta_human_asset_manager_update_asset_dependencies: std::ptr::null_mut(),
            u_meta_human_asset_manager_is_asset_of_type: std::ptr::null_mut(),
            u_meta_human_asset_manager_find_assets_for_packaging: std::ptr::null_mut(),
            u_meta_human_asset_manager_create_archive: std::ptr::null_mut(),
            u_meta_human_verification_rule_base_verify: std::ptr::null_mut(),
            u_meta_human_verification_rule_collection_apply_all_rules: std::ptr::null_mut(),
            u_meta_human_verification_rule_collection_add_verification_rule: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaHumanAssetReport::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWarningsAsErrors"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_set_warnings_as_errors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVerbose"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_set_verbose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSubject"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_set_subject,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSilent"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_set_silent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasWarnings"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_has_warnings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetReportResult"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_get_report_result,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateRichTextReport"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_generate_rich_text_report,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateRawReport"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_generate_raw_report,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateJsonReport"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_generate_json_report,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateHtmlReport"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_generate_html_report,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddWarning"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_add_warning,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVerbose"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_add_verbose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInfo"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_add_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddError"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_report_add_error,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaHumanAssetManager::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateAssetDetails"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_manager_update_asset_details,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateAssetDependencies"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_manager_update_asset_dependencies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAssetOfType"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_manager_is_asset_of_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindAssetsForPackaging"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_manager_find_assets_for_packaging,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateArchive"),
            &raw mut __FUNCTION_PTRS.u_meta_human_asset_manager_create_archive,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaHumanVerificationRuleBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Verify"),
            &raw mut __FUNCTION_PTRS.u_meta_human_verification_rule_base_verify,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaHumanVerificationRuleCollection::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyAllRules"),
            &raw mut __FUNCTION_PTRS
                .u_meta_human_verification_rule_collection_apply_all_rules,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVerificationRule"),
            &raw mut __FUNCTION_PTRS
                .u_meta_human_verification_rule_collection_add_verification_rule,
        );
    }
}
#[repr(C, align(8))]
pub struct FMetaHumanAssetReportItem {
    pub message: FText,
    pub project_item: UPtr<crate::bindings::core_u_object::UObject>,
    pub source_item: FString,
}
impl FMetaHumanAssetReportItem {}
#[repr(C, align(1))]
pub struct FMetaHumanImportOptions {
    pub b_verbose: bool,
    pub b_force_update: bool,
}
impl FMetaHumanImportOptions {}
#[repr(C, align(8))]
pub struct FMetaHumanAggregateDetails {
    pub b_resizes_with_blendable_bodies: bool,
    pub b_has_clothing_mask: bool,
    pub included_lods: i32,
    pub lod0_vert_count: i32,
    pub num_unique_clothing_items: i32,
    pub b_contains_grooms: bool,
    pub b_contains_clothing: bool,
    pub b_is_editable_character: bool,
    pub platforms_included: TArray<
        crate::bindings::meta_human_sdk_runtime::EMetaHumanQualityLevel,
    >,
    pub num_unique_characters: i32,
    pub num_virtual_textures: i32,
    pub num_substrate_materials: i32,
    pub num_unique_grooms: i32,
    pub b_physics: bool,
    pub strands_count: i32,
    pub strands_point_count: i32,
    pub b_has_lods: bool,
    pub card_mesh_count: i32,
    pub card_mesh_vertices: i32,
    pub card_mesh_texture_resolution: crate::bindings::core_u_object::FIntVector2,
    pub volume_mesh_count: i32,
    pub volume_mesh_vertices: i32,
    pub volume_mesh_texture_resolution: crate::bindings::core_u_object::FIntVector2,
    pub num_materials: i32,
    pub engine_version: FString,
}
impl FMetaHumanAggregateDetails {}
#[repr(C, align(8))]
pub struct FMetaHumanAssetDescription {
    pub name: FName,
    pub asset_data: crate::bindings::core_u_object::FAssetData,
    pub dependent_packages: TArray<FName>,
    pub asset_type: EMetaHumanAssetType,
    pub details: FMetaHumanAggregateDetails,
    pub total_size: i32,
    pub verification_report: UPtr<UMetaHumanAssetReport>,
}
impl FMetaHumanAssetDescription {}
#[repr(C, align(8))]
pub struct FMetaHumanMultiArchiveDescription {
    pub contained_archives: TArray<FString>,
}
impl FMetaHumanMultiArchiveDescription {}
#[repr(C, align(8))]
pub struct FMetaHumanArchiveEntry {
    pub path: FString,
    pub version: FString,
}
impl FMetaHumanArchiveEntry {}
#[repr(C, align(8))]
pub struct FMetaHumanArchiveContents {
    pub files: TArray<FMetaHumanArchiveEntry>,
}
impl FMetaHumanArchiveContents {}
#[repr(C, align(1))]
pub struct FMetaHumanVerificationOptions {
    pub b_verbose: bool,
    pub b_treat_warnings_as_errors: bool,
    pub b_verify_packaging_rules: bool,
}
impl FMetaHumanVerificationOptions {}
#[repr(C, align(8))]
pub struct UMetaHumanPackageFactory {
    __padding_end: [u8; 136],
}
impl UMetaHumanPackageFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanPackageFactory")
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
pub struct UMetaHumanAssetReport {
    __padding_end: [u8; 120],
}
impl UMetaHumanAssetReport {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanAssetReport")
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
    pub fn set_warnings_as_errors(&mut self, value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_set_warnings_as_errors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_set_warnings_as_errors,
                __buffer,
            )
        };
    }
    pub fn set_verbose(&mut self, value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_set_verbose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_set_verbose,
                __buffer,
            )
        };
    }
    pub fn set_subject(&mut self, in_subject: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_set_subject,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_subject,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_set_subject,
                __buffer,
            )
        };
    }
    pub fn set_silent(&mut self, value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_set_silent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_set_silent,
                __buffer,
            )
        };
    }
    pub fn has_warnings(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_has_warnings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_has_warnings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_report_result(&self) -> EMetaHumanOperationResult {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_get_report_result,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_get_report_result,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EMetaHumanOperationResult>().read() }
    }
    pub fn generate_rich_text_report(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_generate_rich_text_report,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_generate_rich_text_report,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn generate_raw_report(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_generate_raw_report,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_generate_raw_report,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn generate_json_report(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_generate_json_report,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_generate_json_report,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn generate_html_report(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_generate_html_report,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_generate_html_report,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn add_warning(&mut self, message: &FMetaHumanAssetReportItem) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_add_warning,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                message,
                __buffer.add(0).cast::<FMetaHumanAssetReportItem>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_add_warning,
                __buffer,
            )
        };
    }
    pub fn add_verbose(&mut self, message: &FMetaHumanAssetReportItem) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_add_verbose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                message,
                __buffer.add(0).cast::<FMetaHumanAssetReportItem>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_add_verbose,
                __buffer,
            )
        };
    }
    pub fn add_info(&mut self, message: &FMetaHumanAssetReportItem) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_add_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                message,
                __buffer.add(0).cast::<FMetaHumanAssetReportItem>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_add_info,
                __buffer,
            )
        };
    }
    pub fn add_error(&mut self, message: &FMetaHumanAssetReportItem) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_add_error,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                message,
                __buffer.add(0).cast::<FMetaHumanAssetReportItem>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_report_add_error,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMetaHumanAssetManager {
    __padding_end: [u8; 48],
}
impl UMetaHumanAssetManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanAssetManager")
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
    pub fn update_asset_details(
        asset: &mut FMetaHumanAssetDescription,
    ) -> FMetaHumanAssetDescription {
        let mut __stack = crate::core_data::StackAlloc::<672>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_manager_update_asset_details,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset,
                __buffer.add(0).cast::<FMetaHumanAssetDescription>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::meta_human_sdk_editor::UMetaHumanAssetManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_manager_update_asset_details,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FMetaHumanAssetDescription>().swap(asset);
        }
        unsafe { __buffer.add(336).cast::<FMetaHumanAssetDescription>().read() }
    }
    pub fn update_asset_dependencies(
        asset: &mut FMetaHumanAssetDescription,
    ) -> FMetaHumanAssetDescription {
        let mut __stack = crate::core_data::StackAlloc::<672>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_manager_update_asset_dependencies,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset,
                __buffer.add(0).cast::<FMetaHumanAssetDescription>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::meta_human_sdk_editor::UMetaHumanAssetManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_manager_update_asset_dependencies,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FMetaHumanAssetDescription>().swap(asset);
        }
        unsafe { __buffer.add(336).cast::<FMetaHumanAssetDescription>().read() }
    }
    pub fn is_asset_of_type(
        root_package: &FName,
        asset_type: EMetaHumanAssetType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_manager_is_asset_of_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                root_package,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_type,
                __buffer.add(12).cast::<EMetaHumanAssetType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::meta_human_sdk_editor::UMetaHumanAssetManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_manager_is_asset_of_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn find_assets_for_packaging(
        asset_type: EMetaHumanAssetType,
    ) -> TArray<FMetaHumanAssetDescription> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_manager_find_assets_for_packaging,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_type,
                __buffer.add(0).cast::<EMetaHumanAssetType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::meta_human_sdk_editor::UMetaHumanAssetManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_manager_find_assets_for_packaging,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<FMetaHumanAssetDescription>>().read() }
    }
    pub fn create_archive(
        assets: &TArray<FMetaHumanAssetDescription>,
        archive_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_manager_create_archive,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets,
                __buffer.add(0).cast::<TArray<FMetaHumanAssetDescription>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &archive_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::meta_human_sdk_editor::UMetaHumanAssetManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_asset_manager_create_archive,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMetaHumanVerificationRuleBase {
    __padding_end: [u8; 48],
}
impl UMetaHumanVerificationRuleBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanVerificationRuleBase")
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
    pub fn verify(
        &mut self,
        to_verify: UPtr<crate::bindings::core_u_object::UObject>,
        report: UPtr<UMetaHumanAssetReport>,
        options: &FMetaHumanVerificationOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_verification_rule_base_verify,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &to_verify,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &report,
                __buffer.add(8).cast::<UPtr<UMetaHumanAssetReport>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(16).cast::<FMetaHumanVerificationOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_verification_rule_base_verify,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMetaHumanVerificationRuleCollection {
    __padding_end: [u8; 64],
}
impl UMetaHumanVerificationRuleCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanVerificationRuleCollection")
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
    pub fn apply_all_rules(
        &self,
        target: UPtr<crate::bindings::core_u_object::UObject>,
        report: UPtr<UMetaHumanAssetReport>,
        options: &FMetaHumanVerificationOptions,
    ) -> UPtr<UMetaHumanAssetReport> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_verification_rule_collection_apply_all_rules,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &report,
                __buffer.add(8).cast::<UPtr<UMetaHumanAssetReport>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(16).cast::<FMetaHumanVerificationOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_verification_rule_collection_apply_all_rules,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UMetaHumanAssetReport>>().read() }
    }
    pub fn add_verification_rule(&mut self, rule: UPtr<UMetaHumanVerificationRuleBase>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_verification_rule_collection_add_verification_rule,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rule,
                __buffer.add(0).cast::<UPtr<UMetaHumanVerificationRuleBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::meta_human_sdk_editor::__FUNCTION_PTRS
                    .u_meta_human_verification_rule_collection_add_verification_rule,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UVerifyMetaHumanCharacter {
    __padding_end: [u8; 48],
}
impl UVerifyMetaHumanCharacter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVerifyMetaHumanCharacter")
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
pub struct UVerifyMetaHumanGroom {
    __padding_end: [u8; 48],
}
impl UVerifyMetaHumanGroom {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVerifyMetaHumanGroom")
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
pub struct UVerifyMetaHumanOutfitClothing {
    __padding_end: [u8; 48],
}
impl UVerifyMetaHumanOutfitClothing {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVerifyMetaHumanOutfitClothing")
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
pub struct UVerifyMetaHumanPackageSource {
    __padding_end: [u8; 48],
}
impl UVerifyMetaHumanPackageSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVerifyMetaHumanPackageSource")
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
pub struct UVerifyMetaHumanSkeletalClothing {
    __padding_end: [u8; 48],
}
impl UVerifyMetaHumanSkeletalClothing {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVerifyMetaHumanSkeletalClothing")
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
pub struct UVerifyObjectValid {
    __padding_end: [u8; 48],
}
impl UVerifyObjectValid {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVerifyObjectValid")
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
pub struct UMetaHumanCharacterTypesVerificationExtensionBase {
    __padding_end: [u8; 48],
}
impl UMetaHumanCharacterTypesVerificationExtensionBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanCharacterTypesVerificationExtensionBase")
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
pub struct UMetaHumanCloudServicesSettings {
    __padding_end: [u8; 264],
}
impl UMetaHumanCloudServicesSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanCloudServicesSettings")
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
pub struct UMetaHumanSDKSettings {
    __padding_end: [u8; 176],
}
impl UMetaHumanSDKSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanSDKSettings")
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
pub struct EMetaHumanAssetType(pub u8);
impl EMetaHumanAssetType {
    pub const CHARACTER: EMetaHumanAssetType = EMetaHumanAssetType(0);
    pub const CHARACTER_ASSEMBLY: EMetaHumanAssetType = EMetaHumanAssetType(1);
    pub const SKELETAL_CLOTHING: EMetaHumanAssetType = EMetaHumanAssetType(2);
    pub const OUTFIT_CLOTHING: EMetaHumanAssetType = EMetaHumanAssetType(3);
    pub const GROOM: EMetaHumanAssetType = EMetaHumanAssetType(4);
}
#[repr(transparent)]
pub struct EMetaHumanOperationResult(pub u8);
impl EMetaHumanOperationResult {
    pub const SUCCESS: EMetaHumanOperationResult = EMetaHumanOperationResult(0);
    pub const FAILURE: EMetaHumanOperationResult = EMetaHumanOperationResult(1);
}
#[repr(transparent)]
pub struct EMetaHumanCloudServiceEnvironment(pub i32);
impl EMetaHumanCloudServiceEnvironment {
    pub const PRODUCTION: EMetaHumanCloudServiceEnvironment = EMetaHumanCloudServiceEnvironment(
        0,
    );
    pub const GAME_DEV: EMetaHumanCloudServiceEnvironment = EMetaHumanCloudServiceEnvironment(
        1,
    );
}
