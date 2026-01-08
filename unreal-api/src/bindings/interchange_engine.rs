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
pub struct FunctionPtrs {
    pub u_interchange_file_picker_base_scripted_file_picker_for_translator_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_file_picker_base_scripted_file_picker_for_translator_asset_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_configuration_base_scripted_show_test_plan_configuration_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_configuration_base_scripted_show_scene_pipeline_configuration_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_configuration_base_scripted_show_reimport_pipeline_configuration_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_configuration_base_scripted_show_pipeline_configuration_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_asset_import_data_set_translator_settings: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_asset_import_data_set_pipelines: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_asset_import_data_set_node_container: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_asset_import_data_script_get_first_filename: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_asset_import_data_script_extract_filenames: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_asset_import_data_script_extract_display_labels: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_asset_import_data_get_translator_settings: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_asset_import_data_get_stored_node: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_asset_import_data_get_stored_factory_node: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_asset_import_data_get_pipelines: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_asset_import_data_get_number_of_pipelines: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_asset_import_data_get_node_container: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_stack_override_add_python_pipeline: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_stack_override_add_pipeline: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_stack_override_add_blueprint_pipeline: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_warn_if_interchange_is_active: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_wait_until_all_tasks_done: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_scripted_reimport_asset_async: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_scripted_import_scene_async: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_scripted_import_asset_async: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_reimport_asset: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_is_object_being_imported: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_is_interchange_active: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_import_scene: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_import_asset: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_get_translator_for_source_data: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_get_supported_formats_for_object: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_get_supported_formats: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_get_supported_asset_type_formats: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_get_registered_factory_class: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_get_interchange_manager_scripted: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_get_asset_import_data: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_export_scene: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_export_asset: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_create_source_data: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_can_translate_source_data: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_can_reimport: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_manager_cancel_all_tasks: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_utilities_scripted_import_morph_target: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_project_settings_script_get_pipeline_stack_from_source_data: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_project_settings_script_get_pipeline_array_from_translator_pipelines: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_project_settings_script_get_pipeline_array_from_pipeline_stack: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_editor_settings_set_used_group_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_editor_settings_get_used_group_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_editor_settings_get_selectable_items: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_interchange_file_picker_base_scripted_file_picker_for_translator_type: std::ptr::null_mut(),
            u_interchange_file_picker_base_scripted_file_picker_for_translator_asset_type: std::ptr::null_mut(),
            u_interchange_pipeline_configuration_base_scripted_show_test_plan_configuration_dialog: std::ptr::null_mut(),
            u_interchange_pipeline_configuration_base_scripted_show_scene_pipeline_configuration_dialog: std::ptr::null_mut(),
            u_interchange_pipeline_configuration_base_scripted_show_reimport_pipeline_configuration_dialog: std::ptr::null_mut(),
            u_interchange_pipeline_configuration_base_scripted_show_pipeline_configuration_dialog: std::ptr::null_mut(),
            u_interchange_asset_import_data_set_translator_settings: std::ptr::null_mut(),
            u_interchange_asset_import_data_set_pipelines: std::ptr::null_mut(),
            u_interchange_asset_import_data_set_node_container: std::ptr::null_mut(),
            u_interchange_asset_import_data_script_get_first_filename: std::ptr::null_mut(),
            u_interchange_asset_import_data_script_extract_filenames: std::ptr::null_mut(),
            u_interchange_asset_import_data_script_extract_display_labels: std::ptr::null_mut(),
            u_interchange_asset_import_data_get_translator_settings: std::ptr::null_mut(),
            u_interchange_asset_import_data_get_stored_node: std::ptr::null_mut(),
            u_interchange_asset_import_data_get_stored_factory_node: std::ptr::null_mut(),
            u_interchange_asset_import_data_get_pipelines: std::ptr::null_mut(),
            u_interchange_asset_import_data_get_number_of_pipelines: std::ptr::null_mut(),
            u_interchange_asset_import_data_get_node_container: std::ptr::null_mut(),
            u_interchange_pipeline_stack_override_add_python_pipeline: std::ptr::null_mut(),
            u_interchange_pipeline_stack_override_add_pipeline: std::ptr::null_mut(),
            u_interchange_pipeline_stack_override_add_blueprint_pipeline: std::ptr::null_mut(),
            u_interchange_manager_warn_if_interchange_is_active: std::ptr::null_mut(),
            u_interchange_manager_wait_until_all_tasks_done: std::ptr::null_mut(),
            u_interchange_manager_scripted_reimport_asset_async: std::ptr::null_mut(),
            u_interchange_manager_scripted_import_scene_async: std::ptr::null_mut(),
            u_interchange_manager_scripted_import_asset_async: std::ptr::null_mut(),
            u_interchange_manager_reimport_asset: std::ptr::null_mut(),
            u_interchange_manager_is_object_being_imported: std::ptr::null_mut(),
            u_interchange_manager_is_interchange_active: std::ptr::null_mut(),
            u_interchange_manager_import_scene: std::ptr::null_mut(),
            u_interchange_manager_import_asset: std::ptr::null_mut(),
            u_interchange_manager_get_translator_for_source_data: std::ptr::null_mut(),
            u_interchange_manager_get_supported_formats_for_object: std::ptr::null_mut(),
            u_interchange_manager_get_supported_formats: std::ptr::null_mut(),
            u_interchange_manager_get_supported_asset_type_formats: std::ptr::null_mut(),
            u_interchange_manager_get_registered_factory_class: std::ptr::null_mut(),
            u_interchange_manager_get_interchange_manager_scripted: std::ptr::null_mut(),
            u_interchange_manager_get_asset_import_data: std::ptr::null_mut(),
            u_interchange_manager_export_scene: std::ptr::null_mut(),
            u_interchange_manager_export_asset: std::ptr::null_mut(),
            u_interchange_manager_create_source_data: std::ptr::null_mut(),
            u_interchange_manager_can_translate_source_data: std::ptr::null_mut(),
            u_interchange_manager_can_reimport: std::ptr::null_mut(),
            u_interchange_manager_cancel_all_tasks: std::ptr::null_mut(),
            u_interchange_mesh_utilities_scripted_import_morph_target: std::ptr::null_mut(),
            u_interchange_project_settings_script_get_pipeline_stack_from_source_data: std::ptr::null_mut(),
            u_interchange_project_settings_script_get_pipeline_array_from_translator_pipelines: std::ptr::null_mut(),
            u_interchange_project_settings_script_get_pipeline_array_from_pipeline_stack: std::ptr::null_mut(),
            u_interchange_editor_settings_set_used_group_name: std::ptr::null_mut(),
            u_interchange_editor_settings_get_used_group_name: std::ptr::null_mut(),
            u_interchange_editor_settings_get_selectable_items: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeFilePickerBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedFilePickerForTranslatorType"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_file_picker_base_scripted_file_picker_for_translator_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedFilePickerForTranslatorAssetType"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_file_picker_base_scripted_file_picker_for_translator_asset_type,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangePipelineConfigurationBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedShowTestPlanConfigurationDialog"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_pipeline_configuration_base_scripted_show_test_plan_configuration_dialog,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedShowScenePipelineConfigurationDialog"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_pipeline_configuration_base_scripted_show_scene_pipeline_configuration_dialog,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedShowReimportPipelineConfigurationDialog"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_pipeline_configuration_base_scripted_show_reimport_pipeline_configuration_dialog,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedShowPipelineConfigurationDialog"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_pipeline_configuration_base_scripted_show_pipeline_configuration_dialog,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeAssetImportData::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTranslatorSettings"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_asset_import_data_set_translator_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPipelines"),
            &raw mut __FUNCTION_PTRS.u_interchange_asset_import_data_set_pipelines,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeContainer"),
            &raw mut __FUNCTION_PTRS.u_interchange_asset_import_data_set_node_container,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptGetFirstFilename"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_asset_import_data_script_get_first_filename,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptExtractFilenames"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_asset_import_data_script_extract_filenames,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptExtractDisplayLabels"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_asset_import_data_script_extract_display_labels,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTranslatorSettings"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_asset_import_data_get_translator_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStoredNode"),
            &raw mut __FUNCTION_PTRS.u_interchange_asset_import_data_get_stored_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStoredFactoryNode"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_asset_import_data_get_stored_factory_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPipelines"),
            &raw mut __FUNCTION_PTRS.u_interchange_asset_import_data_get_pipelines,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumberOfPipelines"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_asset_import_data_get_number_of_pipelines,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeContainer"),
            &raw mut __FUNCTION_PTRS.u_interchange_asset_import_data_get_node_container,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangePipelineStackOverride::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddPythonPipeline"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_pipeline_stack_override_add_python_pipeline,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddPipeline"),
            &raw mut __FUNCTION_PTRS.u_interchange_pipeline_stack_override_add_pipeline,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBlueprintPipeline"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_pipeline_stack_override_add_blueprint_pipeline,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeManager::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WarnIfInterchangeIsActive"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_warn_if_interchange_is_active,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WaitUntilAllTasksDone"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_wait_until_all_tasks_done,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedReimportAssetAsync"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_scripted_reimport_asset_async,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedImportSceneAsync"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_scripted_import_scene_async,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedImportAssetAsync"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_scripted_import_asset_async,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReimportAsset"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_reimport_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsObjectBeingImported"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_is_object_being_imported,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInterchangeActive"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_is_interchange_active,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportScene"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_import_scene,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportAsset"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_import_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTranslatorForSourceData"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_get_translator_for_source_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedFormatsForObject"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_manager_get_supported_formats_for_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedFormats"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_get_supported_formats,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedAssetTypeFormats"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_manager_get_supported_asset_type_formats,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRegisteredFactoryClass"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_get_registered_factory_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInterchangeManagerScripted"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_manager_get_interchange_manager_scripted,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetImportData"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_get_asset_import_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportScene"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_export_scene,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportAsset"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_export_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateSourceData"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_create_source_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanTranslateSourceData"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_can_translate_source_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanReimport"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_can_reimport,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CancelAllTasks"),
            &raw mut __FUNCTION_PTRS.u_interchange_manager_cancel_all_tasks,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeMeshUtilities::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptedImportMorphTarget"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_utilities_scripted_import_morph_target,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeProjectSettingsScript::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPipelineStackFromSourceData"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_project_settings_script_get_pipeline_stack_from_source_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPipelineArrayFromTranslatorPipelines"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_project_settings_script_get_pipeline_array_from_translator_pipelines,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPipelineArrayFromPipelineStack"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_project_settings_script_get_pipeline_array_from_pipeline_stack,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeEditorSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUsedGroupName"),
            &raw mut __FUNCTION_PTRS.u_interchange_editor_settings_set_used_group_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUsedGroupName"),
            &raw mut __FUNCTION_PTRS.u_interchange_editor_settings_get_used_group_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectableItems"),
            &raw mut __FUNCTION_PTRS.u_interchange_editor_settings_get_selectable_items,
        );
    }
}
#[repr(C, align(8))]
pub struct FInterchangeFilePickerParameters {
    __padding_end: [u8; 64],
}
impl FInterchangeFilePickerParameters {}
#[repr(C, align(8))]
pub struct FInterchangeStackInfo {
    pub stack_name: FName,
    pub pipelines: TArray<
        UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
    >,
}
impl FInterchangeStackInfo {}
#[repr(C, align(8))]
pub struct FImportAssetParameters {
    pub reimport_asset: UPtr<crate::bindings::core_u_object::UObject>,
    pub reimport_source_index: i32,
    pub b_is_automated: bool,
    pub b_follow_redirectors: bool,
    pub override_pipelines: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    pub import_level: UPtr<crate::bindings::engine::ULevel>,
    pub destination_name: FString,
    pub b_replace_existing: bool,
    pub b_force_show_dialog: bool,
    pub on_asset_done: FImportAssetParameters_OnAssetDone,
    #[doc(hidden)]
    __padding_120: [u8; 24],
    pub on_assets_import_done: FImportAssetParameters_OnAssetsImportDone,
    #[doc(hidden)]
    __padding_176: [u8; 24],
    pub on_scene_object_done: FImportAssetParameters_OnSceneObjectDone,
    #[doc(hidden)]
    __padding_232: [u8; 24],
    pub on_scene_import_done: FImportAssetParameters_OnSceneImportDone,
    __padding_end: [u8; 32],
}
impl FImportAssetParameters {}
#[repr(C, align(8))]
pub struct FInterchangeTranslatorPipelines {
    pub translator: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub pipelines: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
}
impl FInterchangeTranslatorPipelines {}
#[repr(C, align(8))]
pub struct FInterchangePipelineStack {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub per_translator_pipelines: TArray<FInterchangeTranslatorPipelines>,
}
impl FInterchangePipelineStack {}
#[repr(C, align(8))]
pub struct FInterchangePerTranslatorDialogOverride {
    pub translator: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
}
impl FInterchangePerTranslatorDialogOverride {}
#[repr(C, align(8))]
pub struct FInterchangeDialogOverride {
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
    pub per_translator_import_dialog_override: TArray<
        FInterchangePerTranslatorDialogOverride,
    >,
}
impl FInterchangeDialogOverride {}
#[repr(C, align(8))]
pub struct FInterchangeImportSettings {
    pub pipeline_stacks: TMap<FName, FInterchangePipelineStack>,
    pub default_pipeline_stack: FName,
    pub import_dialog_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
}
impl FInterchangeImportSettings {}
#[repr(C, align(8))]
pub struct FInterchangeSceneImportSettings {
    #[doc(hidden)]
    __padding_152: [u8; 152],
    pub per_translator_dialog_override: TArray<FInterchangePerTranslatorDialogOverride>,
}
impl FInterchangeSceneImportSettings {}
#[repr(C, align(8))]
pub struct FInterchangeContentImportSettings {
    #[doc(hidden)]
    __padding_152: [u8; 152],
    pub default_pipeline_stack_override: TMap<
        crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
        FName,
    >,
    pub show_import_dialog_override: TMap<
        crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
        FInterchangeDialogOverride,
    >,
}
impl FInterchangeContentImportSettings {}
#[repr(C, align(8))]
pub struct FInterchangeGroup {
    pub display_name: FName,
    pub unique_id: crate::bindings::core_u_object::FGuid,
    pub default_pipeline_stack: FName,
    pub default_pipeline_stack_override: TMap<
        crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
        FName,
    >,
    pub b_show_import_dialog: bool,
    pub b_show_reimport_dialog: bool,
    pub show_import_dialog_override: TMap<
        crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
        FInterchangeDialogOverride,
    >,
}
impl FInterchangeGroup {}
#[repr(C, align(8))]
pub struct UInterchangeBlueprintPipelineBase {
    __padding_end: [u8; 1432],
}
impl UInterchangeBlueprintPipelineBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeBlueprintPipelineBase")
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
pub struct UInterchangeEditorUtilitiesBase {
    __padding_end: [u8; 48],
}
impl UInterchangeEditorUtilitiesBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeEditorUtilitiesBase")
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
pub struct UInterchangeFilePickerBase {
    __padding_end: [u8; 48],
}
impl UInterchangeFilePickerBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeFilePickerBase")
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
    pub fn scripted_file_picker_for_translator_type(
        &mut self,
        translator_type: crate::bindings::interchange_core::EInterchangeTranslatorType,
        parameters: &mut FInterchangeFilePickerParameters,
        out_filenames: &mut TArray<FString>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_file_picker_base_scripted_file_picker_for_translator_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &translator_type,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::interchange_core::EInterchangeTranslatorType,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FInterchangeFilePickerParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_filenames,
                __buffer.add(72).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_file_picker_base_scripted_file_picker_for_translator_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FInterchangeFilePickerParameters>().swap(parameters);
        }
        unsafe {
            __buffer.add(72).cast::<TArray<FString>>().swap(out_filenames);
        }
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn scripted_file_picker_for_translator_asset_type(
        &mut self,
        translator_asset_type: crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
        parameters: &mut FInterchangeFilePickerParameters,
        out_filenames: &mut TArray<FString>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_file_picker_base_scripted_file_picker_for_translator_asset_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &translator_asset_type,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FInterchangeFilePickerParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_filenames,
                __buffer.add(72).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_file_picker_base_scripted_file_picker_for_translator_asset_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FInterchangeFilePickerParameters>().swap(parameters);
        }
        unsafe {
            __buffer.add(72).cast::<TArray<FString>>().swap(out_filenames);
        }
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangePipelineConfigurationBase {
    __padding_end: [u8; 48],
}
impl UInterchangePipelineConfigurationBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangePipelineConfigurationBase")
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
    pub fn scripted_show_test_plan_configuration_dialog(
        &mut self,
        pipeline_stacks: &mut TArray<FInterchangeStackInfo>,
        out_pipelines: &mut TArray<
            UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
        >,
        source_data: UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
        translator: UPtr<crate::bindings::interchange_core::UInterchangeTranslatorBase>,
        base_node_container: UPtr<
            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
        >,
        reimport_asset: UPtr<crate::bindings::core_u_object::UObject>,
        b_scene_import: bool,
        b_reimport: bool,
    ) -> EInterchangePipelineConfigurationDialogResult {
        let mut __stack = crate::core_data::StackAlloc::<67>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_configuration_base_scripted_show_test_plan_configuration_dialog,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pipeline_stacks,
                __buffer.add(0).cast::<TArray<FInterchangeStackInfo>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_pipelines,
                __buffer
                    .add(16)
                    .cast::<
                        TArray<
                            UPtr<
                                crate::bindings::interchange_core::UInterchangePipelineBase,
                            >,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_data,
                __buffer
                    .add(32)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &translator,
                __buffer
                    .add(40)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeTranslatorBase,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer
                    .add(48)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reimport_asset,
                __buffer.add(56).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_scene_import,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reimport,
                __buffer.add(65).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_configuration_base_scripted_show_test_plan_configuration_dialog,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<FInterchangeStackInfo>>()
                .swap(pipeline_stacks);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    TArray<
                        UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
                    >,
                >()
                .swap(out_pipelines);
        }
        unsafe {
            __buffer
                .add(66)
                .cast::<EInterchangePipelineConfigurationDialogResult>()
                .read()
        }
    }
    pub fn scripted_show_scene_pipeline_configuration_dialog(
        &mut self,
        pipeline_stacks: &mut TArray<FInterchangeStackInfo>,
        out_pipelines: &mut TArray<
            UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
        >,
        source_data: UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
        translator: UPtr<crate::bindings::interchange_core::UInterchangeTranslatorBase>,
        base_node_container: UPtr<
            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
        >,
    ) -> EInterchangePipelineConfigurationDialogResult {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_configuration_base_scripted_show_scene_pipeline_configuration_dialog,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pipeline_stacks,
                __buffer.add(0).cast::<TArray<FInterchangeStackInfo>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_pipelines,
                __buffer
                    .add(16)
                    .cast::<
                        TArray<
                            UPtr<
                                crate::bindings::interchange_core::UInterchangePipelineBase,
                            >,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_data,
                __buffer
                    .add(32)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &translator,
                __buffer
                    .add(40)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeTranslatorBase,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer
                    .add(48)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_configuration_base_scripted_show_scene_pipeline_configuration_dialog,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<FInterchangeStackInfo>>()
                .swap(pipeline_stacks);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    TArray<
                        UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
                    >,
                >()
                .swap(out_pipelines);
        }
        unsafe {
            __buffer
                .add(56)
                .cast::<EInterchangePipelineConfigurationDialogResult>()
                .read()
        }
    }
    pub fn scripted_show_reimport_pipeline_configuration_dialog(
        &mut self,
        pipeline_stacks: &mut TArray<FInterchangeStackInfo>,
        out_pipelines: &mut TArray<
            UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
        >,
        source_data: UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
        translator: UPtr<crate::bindings::interchange_core::UInterchangeTranslatorBase>,
        base_node_container: UPtr<
            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
        >,
        reimport_asset: UPtr<crate::bindings::core_u_object::UObject>,
        b_scene_import: bool,
    ) -> EInterchangePipelineConfigurationDialogResult {
        let mut __stack = crate::core_data::StackAlloc::<66>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_configuration_base_scripted_show_reimport_pipeline_configuration_dialog,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pipeline_stacks,
                __buffer.add(0).cast::<TArray<FInterchangeStackInfo>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_pipelines,
                __buffer
                    .add(16)
                    .cast::<
                        TArray<
                            UPtr<
                                crate::bindings::interchange_core::UInterchangePipelineBase,
                            >,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_data,
                __buffer
                    .add(32)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &translator,
                __buffer
                    .add(40)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeTranslatorBase,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer
                    .add(48)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reimport_asset,
                __buffer.add(56).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_scene_import,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_configuration_base_scripted_show_reimport_pipeline_configuration_dialog,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<FInterchangeStackInfo>>()
                .swap(pipeline_stacks);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    TArray<
                        UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
                    >,
                >()
                .swap(out_pipelines);
        }
        unsafe {
            __buffer
                .add(65)
                .cast::<EInterchangePipelineConfigurationDialogResult>()
                .read()
        }
    }
    pub fn scripted_show_pipeline_configuration_dialog(
        &mut self,
        pipeline_stacks: &mut TArray<FInterchangeStackInfo>,
        out_pipelines: &mut TArray<
            UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
        >,
        source_data: UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
        translator: UPtr<crate::bindings::interchange_core::UInterchangeTranslatorBase>,
        base_node_container: UPtr<
            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
        >,
    ) -> EInterchangePipelineConfigurationDialogResult {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_configuration_base_scripted_show_pipeline_configuration_dialog,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pipeline_stacks,
                __buffer.add(0).cast::<TArray<FInterchangeStackInfo>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_pipelines,
                __buffer
                    .add(16)
                    .cast::<
                        TArray<
                            UPtr<
                                crate::bindings::interchange_core::UInterchangePipelineBase,
                            >,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_data,
                __buffer
                    .add(32)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &translator,
                __buffer
                    .add(40)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeTranslatorBase,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer
                    .add(48)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_configuration_base_scripted_show_pipeline_configuration_dialog,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<FInterchangeStackInfo>>()
                .swap(pipeline_stacks);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    TArray<
                        UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
                    >,
                >()
                .swap(out_pipelines);
        }
        unsafe {
            __buffer
                .add(56)
                .cast::<EInterchangePipelineConfigurationDialogResult>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeAssetImportData {
    __padding_end: [u8; 312],
}
impl UInterchangeAssetImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeAssetImportData")
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
    pub fn set_translator_settings(
        &self,
        translator_settings: UPtr<
            crate::bindings::interchange_core::UInterchangeTranslatorSettings,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_set_translator_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &translator_settings,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeTranslatorSettings,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_set_translator_settings,
                __buffer,
            )
        };
    }
    pub fn set_pipelines(
        &mut self,
        in_pipelines: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_set_pipelines,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pipelines,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_set_pipelines,
                __buffer,
            )
        };
    }
    pub fn set_node_container(
        &self,
        in_node_container: UPtr<
            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_set_node_container,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_container,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_set_node_container,
                __buffer,
            )
        };
    }
    pub fn script_get_first_filename(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_script_get_first_filename,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_script_get_first_filename,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn script_extract_filenames(&self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_script_extract_filenames,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_script_extract_filenames,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn script_extract_display_labels(&self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_script_extract_display_labels,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_script_extract_display_labels,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_translator_settings(
        &self,
    ) -> UPtr<crate::bindings::interchange_core::UInterchangeTranslatorSettings> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_get_translator_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_get_translator_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    UPtr<
                        crate::bindings::interchange_core::UInterchangeTranslatorSettings,
                    >,
                >()
                .read()
        }
    }
    pub fn get_stored_node(
        &self,
        in_node_unique_id: FString,
    ) -> UPtr<crate::bindings::interchange_core::UInterchangeBaseNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_get_stored_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_unique_id,
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
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_get_stored_node,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>>()
                .read()
        }
    }
    pub fn get_stored_factory_node(
        &self,
        in_node_unique_id: FString,
    ) -> UPtr<crate::bindings::interchange_core::UInterchangeFactoryBaseNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_get_stored_factory_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_unique_id,
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
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_get_stored_factory_node,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    UPtr<crate::bindings::interchange_core::UInterchangeFactoryBaseNode>,
                >()
                .read()
        }
    }
    pub fn get_pipelines(
        &self,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_get_pipelines,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_get_pipelines,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn get_number_of_pipelines(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_get_number_of_pipelines,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_get_number_of_pipelines,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_node_container(
        &self,
    ) -> UPtr<crate::bindings::interchange_core::UInterchangeBaseNodeContainer> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_get_node_container,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_asset_import_data_get_node_container,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    UPtr<
                        crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                    >,
                >()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeAssetImportDataConverterBase {
    __padding_end: [u8; 48],
}
impl UInterchangeAssetImportDataConverterBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeAssetImportDataConverterBase")
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
pub struct UInterchangePipelineStackOverride {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub override_pipelines: TArray<crate::bindings::core_u_object::FSoftObjectPath>,
}
impl UInterchangePipelineStackOverride {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangePipelineStackOverride")
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
    pub fn add_python_pipeline(
        &mut self,
        pipeline_base: UPtr<UInterchangePythonPipelineBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_stack_override_add_python_pipeline,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pipeline_base,
                __buffer.add(0).cast::<UPtr<UInterchangePythonPipelineBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_stack_override_add_python_pipeline,
                __buffer,
            )
        };
    }
    pub fn add_pipeline(
        &mut self,
        pipeline_base: UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_stack_override_add_pipeline,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pipeline_base,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_stack_override_add_pipeline,
                __buffer,
            )
        };
    }
    pub fn add_blueprint_pipeline(
        &mut self,
        pipeline_base: UPtr<UInterchangeBlueprintPipelineBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_stack_override_add_blueprint_pipeline,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pipeline_base,
                __buffer.add(0).cast::<UPtr<UInterchangeBlueprintPipelineBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_pipeline_stack_override_add_blueprint_pipeline,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UInterchangeManager {
    __padding_end: [u8; 1072],
}
impl UInterchangeManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeManager")
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
    pub fn warn_if_interchange_is_active(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_warn_if_interchange_is_active,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_warn_if_interchange_is_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn wait_until_all_tasks_done(&mut self, b_cancel: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_wait_until_all_tasks_done,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_cancel, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_wait_until_all_tasks_done,
                __buffer,
            )
        };
    }
    pub fn scripted_reimport_asset_async(
        &mut self,
        object_to_reimport: UPtr<crate::bindings::core_u_object::UObject>,
        import_asset_parameters: &FImportAssetParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<305>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_scripted_reimport_asset_async,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_to_reimport,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                import_asset_parameters,
                __buffer.add(8).cast::<FImportAssetParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_scripted_reimport_asset_async,
                __buffer,
            )
        };
        unsafe { __buffer.add(304).cast::<bool>().read() }
    }
    pub fn scripted_import_scene_async(
        &mut self,
        content_path: FString,
        source_data: UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
        import_asset_parameters: &FImportAssetParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<321>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_scripted_import_scene_async,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &content_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_data,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                import_asset_parameters,
                __buffer.add(24).cast::<FImportAssetParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_scripted_import_scene_async,
                __buffer,
            )
        };
        unsafe { __buffer.add(320).cast::<bool>().read() }
    }
    pub fn scripted_import_asset_async(
        &mut self,
        content_path: FString,
        source_data: UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
        import_asset_parameters: &FImportAssetParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<321>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_scripted_import_asset_async,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &content_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_data,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                import_asset_parameters,
                __buffer.add(24).cast::<FImportAssetParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_scripted_import_asset_async,
                __buffer,
            )
        };
        unsafe { __buffer.add(320).cast::<bool>().read() }
    }
    pub fn reimport_asset(
        &mut self,
        object_to_reimport: UPtr<crate::bindings::core_u_object::UObject>,
        import_asset_parameters: &FImportAssetParameters,
        out_imported_objects: &mut TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<321>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_reimport_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_to_reimport,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                import_asset_parameters,
                __buffer.add(8).cast::<FImportAssetParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_imported_objects,
                __buffer
                    .add(304)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_reimport_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(304)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .swap(out_imported_objects);
        }
        unsafe { __buffer.add(320).cast::<bool>().read() }
    }
    pub fn is_object_being_imported(
        &self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_is_object_being_imported,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
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
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_is_object_being_imported,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_interchange_active(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_is_interchange_active,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_is_interchange_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn import_scene(
        &mut self,
        content_path: FString,
        source_data: UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
        import_asset_parameters: &FImportAssetParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<321>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_import_scene,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &content_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_data,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                import_asset_parameters,
                __buffer.add(24).cast::<FImportAssetParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_import_scene,
                __buffer,
            )
        };
        unsafe { __buffer.add(320).cast::<bool>().read() }
    }
    pub fn import_asset(
        &mut self,
        content_path: FString,
        source_data: UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
        import_asset_parameters: &FImportAssetParameters,
        out_imported_objects: &mut TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<337>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_import_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &content_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_data,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                import_asset_parameters,
                __buffer.add(24).cast::<FImportAssetParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_imported_objects,
                __buffer
                    .add(320)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_import_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(320)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .swap(out_imported_objects);
        }
        unsafe { __buffer.add(336).cast::<bool>().read() }
    }
    pub fn get_translator_for_source_data(
        &self,
        source_data: UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
    ) -> UPtr<crate::bindings::interchange_core::UInterchangeTranslatorBase> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_translator_for_source_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_data,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_translator_for_source_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    UPtr<crate::bindings::interchange_core::UInterchangeTranslatorBase>,
                >()
                .read()
        }
    }
    pub fn get_supported_formats_for_object(
        &self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
        source_file_index: i32,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_supported_formats_for_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_file_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_supported_formats_for_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<FString>>().read() }
    }
    pub fn get_supported_formats(
        &self,
        for_translator_type: crate::bindings::interchange_core::EInterchangeTranslatorType,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_supported_formats,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &for_translator_type,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::interchange_core::EInterchangeTranslatorType,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_supported_formats,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<FString>>().read() }
    }
    pub fn get_supported_asset_type_formats(
        &self,
        for_translator_asset_type: crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
        for_translator_type: crate::bindings::interchange_core::EInterchangeTranslatorType,
        b_strict_match_translator_type: bool,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_supported_asset_type_formats,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &for_translator_asset_type,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::interchange_core::EInterchangeTranslatorAssetType,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &for_translator_type,
                __buffer
                    .add(1)
                    .cast::<
                        crate::bindings::interchange_core::EInterchangeTranslatorType,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_strict_match_translator_type,
                __buffer.add(2).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_supported_asset_type_formats,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<FString>>().read() }
    }
    pub fn get_registered_factory_class(
        &self,
        class_to_make: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_registered_factory_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class_to_make,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_registered_factory_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_interchange_manager_scripted() -> UPtr<UInterchangeManager> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_interchange_manager_scripted,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::interchange_engine::UInterchangeManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_interchange_manager_scripted,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UInterchangeManager>>().read() }
    }
    pub fn get_asset_import_data(
        &self,
        asset: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<UInterchangeAssetImportData> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_asset_import_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
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
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_get_asset_import_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UInterchangeAssetImportData>>().read() }
    }
    pub fn export_scene(
        &mut self,
        world: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_automated: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_export_scene,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_automated,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_export_scene,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn export_asset(
        &mut self,
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_automated: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_export_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_automated,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_export_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn create_source_data(
        in_file_name: FString,
    ) -> UPtr<crate::bindings::interchange_core::UInterchangeSourceData> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_create_source_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_file_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_engine::UInterchangeManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_create_source_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                >()
                .read()
        }
    }
    pub fn can_translate_source_data(
        &self,
        source_data: UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
        b_scene_import_only: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_can_translate_source_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_data,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_scene_import_only,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_can_translate_source_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn can_reimport(
        &self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
        out_filenames: &mut TArray<FString>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_can_reimport,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_filenames,
                __buffer.add(8).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_can_reimport,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FString>>().swap(out_filenames);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn cancel_all_tasks(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_cancel_all_tasks,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_manager_cancel_all_tasks,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UInterchangeMeshUtilities {
    __padding_end: [u8; 48],
}
impl UInterchangeMeshUtilities {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeMeshUtilities")
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
    pub fn scripted_import_morph_target(
        &self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        source_data: UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
        morph_target_name: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_mesh_utilities_scripted_import_morph_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_data,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &morph_target_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_mesh_utilities_scripted_import_morph_target,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeProjectSettings {
    #[doc(hidden)]
    __padding_104: [u8; 104],
    pub content_import_settings: FInterchangeContentImportSettings,
    pub scene_import_settings: FInterchangeSceneImportSettings,
    pub file_picker_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub b_static_mesh_use_smooth_edges_if_smoothing_information_is_missing: bool,
    pub generic_pipeline_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub converter_default_pipeline: crate::bindings::core_u_object::FSoftObjectPath,
    pub interchange_groups: TArray<FInterchangeGroup>,
}
impl UInterchangeProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeProjectSettings")
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
pub struct UInterchangeProjectSettingsScript {
    __padding_end: [u8; 48],
}
impl UInterchangeProjectSettingsScript {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeProjectSettingsScript")
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
    pub fn get_pipeline_stack_from_source_data(
        b_is_scene_import: bool,
        source_data: UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
    ) -> TArray<UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_project_settings_script_get_pipeline_stack_from_source_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_scene_import,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_data,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_engine::UInterchangeProjectSettingsScript::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_project_settings_script_get_pipeline_stack_from_source_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    TArray<
                        UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
                    >,
                >()
                .read()
        }
    }
    pub fn get_pipeline_array_from_translator_pipelines(
        interchange_translator_pipeline: &FInterchangeTranslatorPipelines,
    ) -> TArray<UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_project_settings_script_get_pipeline_array_from_translator_pipelines,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                interchange_translator_pipeline,
                __buffer.add(0).cast::<FInterchangeTranslatorPipelines>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_engine::UInterchangeProjectSettingsScript::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_project_settings_script_get_pipeline_array_from_translator_pipelines,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(64)
                .cast::<
                    TArray<
                        UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
                    >,
                >()
                .read()
        }
    }
    pub fn get_pipeline_array_from_pipeline_stack(
        interchange_pipeline_stack: &FInterchangePipelineStack,
    ) -> TArray<UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_project_settings_script_get_pipeline_array_from_pipeline_stack,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                interchange_pipeline_stack,
                __buffer.add(0).cast::<FInterchangePipelineStack>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_engine::UInterchangeProjectSettingsScript::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_project_settings_script_get_pipeline_array_from_pipeline_stack,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<
                    TArray<
                        UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
                    >,
                >()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeEditorSettings {
    #[doc(hidden)]
    __padding_104: [u8; 104],
    pub b_show_import_dialog_at_reimport: bool,
    __padding_end: [u8; 31],
}
impl UInterchangeEditorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeEditorSettings")
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
    pub fn set_used_group_name(&mut self, in_used_group_name: &FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_editor_settings_set_used_group_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_used_group_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_editor_settings_set_used_group_name,
                __buffer,
            )
        };
    }
    pub fn get_used_group_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_editor_settings_get_used_group_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_engine::__FUNCTION_PTRS
                    .u_interchange_editor_settings_get_used_group_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangePythonPipelineBase {
    __padding_end: [u8; 344],
}
impl UInterchangePythonPipelineBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangePythonPipelineBase")
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
pub struct UInterchangePythonPipelineAsset {
    __padding_end: [u8; 120],
}
impl UInterchangePythonPipelineAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangePythonPipelineAsset")
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
pub struct UInterchangeSceneImportAsset {
    __padding_end: [u8; 216],
}
impl UInterchangeSceneImportAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSceneImportAsset")
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
pub struct FImportAssetParameters_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImportAssetParameters_OnAssetsImportDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImportAssetParameters_OnSceneObjectDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImportAssetParameters_OnSceneImportDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImportAsset_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImportScene_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FReimportAsset_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FScriptedImportAssetAsync_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FScriptedImportSceneAsync_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FScriptedReimportAssetAsync_OnAssetDone {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EInterchangePipelineConfigurationDialogResult(pub u8);
impl EInterchangePipelineConfigurationDialogResult {
    pub const CANCEL: EInterchangePipelineConfigurationDialogResult = EInterchangePipelineConfigurationDialogResult(
        0,
    );
    pub const IMPORT: EInterchangePipelineConfigurationDialogResult = EInterchangePipelineConfigurationDialogResult(
        1,
    );
    pub const IMPORT_ALL: EInterchangePipelineConfigurationDialogResult = EInterchangePipelineConfigurationDialogResult(
        2,
    );
    pub const SAVE_CONFIG: EInterchangePipelineConfigurationDialogResult = EInterchangePipelineConfigurationDialogResult(
        3,
    );
}
