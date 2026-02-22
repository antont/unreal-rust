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
pub struct FunctionPtrs {
    pub u_editor_utility_object_run: *mut crate::ffi::UFunctionOpague,
    pub u_actor_action_utility_get_supported_classes: *mut crate::ffi::UFunctionOpague,
    pub u_actor_action_utility_get_supported_class: *mut crate::ffi::UFunctionOpague,
    pub u_asset_action_utility_is_action_for_blueprints: *mut crate::ffi::UFunctionOpague,
    pub u_asset_action_utility_get_supported_classes: *mut crate::ffi::UFunctionOpague,
    pub u_asset_action_utility_get_supported_class: *mut crate::ffi::UFunctionOpague,
    pub u_async_capture_scene_capture_scene_with_warmup_async: *mut crate::ffi::UFunctionOpague,
    pub u_async_capture_scene_capture_scene_async: *mut crate::ffi::UFunctionOpague,
    pub u_async_image_export_export_image_async: *mut crate::ffi::UFunctionOpague,
    pub u_async_register_and_execute_task_register_and_execute_task: *mut crate::ffi::UFunctionOpague,
    pub a_editor_utility_actor_set_receives_editor_input: *mut crate::ffi::UFunctionOpague,
    pub a_editor_utility_actor_run: *mut crate::ffi::UFunctionOpague,
    pub a_editor_utility_actor_get_receives_editor_input: *mut crate::ffi::UFunctionOpague,
    pub a_editor_utility_actor_get_input_component: *mut crate::ffi::UFunctionOpague,
    pub u_async_editor_delay_async_editor_delay: *mut crate::ffi::UFunctionOpague,
    pub u_async_editor_wait_for_game_world_async_wait_for_game_world: *mut crate::ffi::UFunctionOpague,
    pub u_async_editor_open_map_and_focus_actor_async_editor_open_map_and_focus_actor: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_sync_browser_to_folders: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_rename_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_get_selection_set: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_get_selection_bounds: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_get_selected_path_view_folder_paths: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_get_selected_folder_paths: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_get_selected_blueprint_classes: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_get_selected_assets_of_class: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_get_selected_assets: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_get_selected_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_get_current_content_browser_path: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_get_current_content_browser_item_path: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_get_actor_reference: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_find_source_widget_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_convert_to_editor_utility_widget: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_cast_to_widget_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_library_add_source_widget: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_unregister_tab_by_id: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_try_run_class: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_try_run: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_spawn_registered_tab_by_id: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_spawn_and_register_tab_with_id_generated_class: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_spawn_and_register_tab_with_id: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_spawn_and_register_tab_generated_class: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_spawn_and_register_tab_and_get_id_generated_class: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_spawn_and_register_tab_and_get_id: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_spawn_and_register_tab: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_release_instance_of_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_register_tab_and_get_id_generated_class: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_register_tab_and_get_id: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_register_and_execute_task: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_find_utility_widget_from_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_does_tab_exist: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_close_tab_by_id: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_subsystem_can_run: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_task_was_cancel_requested: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_task_set_task_notification_text: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_task_run: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_task_receive_cancel_requested: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_task_receive_begin_execution: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_task_get_task_title_override: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_task_finish_executing_task: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_widget_run: *mut crate::ffi::UFunctionOpague,
    pub u_editor_utility_widget_find_child_widget_by_name: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_global_editor_utility_base_set_actor_selection_state: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_global_editor_utility_base_select_nothing: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_global_editor_utility_base_rename_asset: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_global_editor_utility_base_on_default_action_clicked: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_global_editor_utility_base_get_selection_set: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_global_editor_utility_base_get_selection_bounds: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_global_editor_utility_base_get_selected_assets: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_global_editor_utility_base_get_editor_user_settings: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_global_editor_utility_base_get_actor_reference: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_global_editor_utility_base_for_each_selected_asset: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_global_editor_utility_base_for_each_selected_actor: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_global_editor_utility_base_clear_actor_selection_set: *mut crate::ffi::UFunctionOpague,
    pub adeprecated_placed_editor_utility_base_set_level_viewport_camera_info: *mut crate::ffi::UFunctionOpague,
    pub adeprecated_placed_editor_utility_base_set_actor_selection_state: *mut crate::ffi::UFunctionOpague,
    pub adeprecated_placed_editor_utility_base_select_nothing: *mut crate::ffi::UFunctionOpague,
    pub adeprecated_placed_editor_utility_base_get_selection_set: *mut crate::ffi::UFunctionOpague,
    pub adeprecated_placed_editor_utility_base_get_level_viewport_camera_info: *mut crate::ffi::UFunctionOpague,
    pub adeprecated_placed_editor_utility_base_get_actor_reference: *mut crate::ffi::UFunctionOpague,
    pub adeprecated_placed_editor_utility_base_clear_actor_selection_set: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_editor_utility_object_run: std::ptr::null_mut(),
            u_actor_action_utility_get_supported_classes: std::ptr::null_mut(),
            u_actor_action_utility_get_supported_class: std::ptr::null_mut(),
            u_asset_action_utility_is_action_for_blueprints: std::ptr::null_mut(),
            u_asset_action_utility_get_supported_classes: std::ptr::null_mut(),
            u_asset_action_utility_get_supported_class: std::ptr::null_mut(),
            u_async_capture_scene_capture_scene_with_warmup_async: std::ptr::null_mut(),
            u_async_capture_scene_capture_scene_async: std::ptr::null_mut(),
            u_async_image_export_export_image_async: std::ptr::null_mut(),
            u_async_register_and_execute_task_register_and_execute_task: std::ptr::null_mut(),
            a_editor_utility_actor_set_receives_editor_input: std::ptr::null_mut(),
            a_editor_utility_actor_run: std::ptr::null_mut(),
            a_editor_utility_actor_get_receives_editor_input: std::ptr::null_mut(),
            a_editor_utility_actor_get_input_component: std::ptr::null_mut(),
            u_async_editor_delay_async_editor_delay: std::ptr::null_mut(),
            u_async_editor_wait_for_game_world_async_wait_for_game_world: std::ptr::null_mut(),
            u_async_editor_open_map_and_focus_actor_async_editor_open_map_and_focus_actor: std::ptr::null_mut(),
            u_editor_utility_library_sync_browser_to_folders: std::ptr::null_mut(),
            u_editor_utility_library_rename_asset: std::ptr::null_mut(),
            u_editor_utility_library_get_selection_set: std::ptr::null_mut(),
            u_editor_utility_library_get_selection_bounds: std::ptr::null_mut(),
            u_editor_utility_library_get_selected_path_view_folder_paths: std::ptr::null_mut(),
            u_editor_utility_library_get_selected_folder_paths: std::ptr::null_mut(),
            u_editor_utility_library_get_selected_blueprint_classes: std::ptr::null_mut(),
            u_editor_utility_library_get_selected_assets_of_class: std::ptr::null_mut(),
            u_editor_utility_library_get_selected_assets: std::ptr::null_mut(),
            u_editor_utility_library_get_selected_asset_data: std::ptr::null_mut(),
            u_editor_utility_library_get_current_content_browser_path: std::ptr::null_mut(),
            u_editor_utility_library_get_current_content_browser_item_path: std::ptr::null_mut(),
            u_editor_utility_library_get_actor_reference: std::ptr::null_mut(),
            u_editor_utility_library_find_source_widget_by_name: std::ptr::null_mut(),
            u_editor_utility_library_convert_to_editor_utility_widget: std::ptr::null_mut(),
            u_editor_utility_library_cast_to_widget_blueprint: std::ptr::null_mut(),
            u_editor_utility_library_add_source_widget: std::ptr::null_mut(),
            u_editor_utility_subsystem_unregister_tab_by_id: std::ptr::null_mut(),
            u_editor_utility_subsystem_try_run_class: std::ptr::null_mut(),
            u_editor_utility_subsystem_try_run: std::ptr::null_mut(),
            u_editor_utility_subsystem_spawn_registered_tab_by_id: std::ptr::null_mut(),
            u_editor_utility_subsystem_spawn_and_register_tab_with_id_generated_class: std::ptr::null_mut(),
            u_editor_utility_subsystem_spawn_and_register_tab_with_id: std::ptr::null_mut(),
            u_editor_utility_subsystem_spawn_and_register_tab_generated_class: std::ptr::null_mut(),
            u_editor_utility_subsystem_spawn_and_register_tab_and_get_id_generated_class: std::ptr::null_mut(),
            u_editor_utility_subsystem_spawn_and_register_tab_and_get_id: std::ptr::null_mut(),
            u_editor_utility_subsystem_spawn_and_register_tab: std::ptr::null_mut(),
            u_editor_utility_subsystem_release_instance_of_asset: std::ptr::null_mut(),
            u_editor_utility_subsystem_register_tab_and_get_id_generated_class: std::ptr::null_mut(),
            u_editor_utility_subsystem_register_tab_and_get_id: std::ptr::null_mut(),
            u_editor_utility_subsystem_register_and_execute_task: std::ptr::null_mut(),
            u_editor_utility_subsystem_find_utility_widget_from_blueprint: std::ptr::null_mut(),
            u_editor_utility_subsystem_does_tab_exist: std::ptr::null_mut(),
            u_editor_utility_subsystem_close_tab_by_id: std::ptr::null_mut(),
            u_editor_utility_subsystem_can_run: std::ptr::null_mut(),
            u_editor_utility_task_was_cancel_requested: std::ptr::null_mut(),
            u_editor_utility_task_set_task_notification_text: std::ptr::null_mut(),
            u_editor_utility_task_run: std::ptr::null_mut(),
            u_editor_utility_task_receive_cancel_requested: std::ptr::null_mut(),
            u_editor_utility_task_receive_begin_execution: std::ptr::null_mut(),
            u_editor_utility_task_get_task_title_override: std::ptr::null_mut(),
            u_editor_utility_task_finish_executing_task: std::ptr::null_mut(),
            u_editor_utility_widget_run: std::ptr::null_mut(),
            u_editor_utility_widget_find_child_widget_by_name: std::ptr::null_mut(),
            udeprecated_global_editor_utility_base_set_actor_selection_state: std::ptr::null_mut(),
            udeprecated_global_editor_utility_base_select_nothing: std::ptr::null_mut(),
            udeprecated_global_editor_utility_base_rename_asset: std::ptr::null_mut(),
            udeprecated_global_editor_utility_base_on_default_action_clicked: std::ptr::null_mut(),
            udeprecated_global_editor_utility_base_get_selection_set: std::ptr::null_mut(),
            udeprecated_global_editor_utility_base_get_selection_bounds: std::ptr::null_mut(),
            udeprecated_global_editor_utility_base_get_selected_assets: std::ptr::null_mut(),
            udeprecated_global_editor_utility_base_get_editor_user_settings: std::ptr::null_mut(),
            udeprecated_global_editor_utility_base_get_actor_reference: std::ptr::null_mut(),
            udeprecated_global_editor_utility_base_for_each_selected_asset: std::ptr::null_mut(),
            udeprecated_global_editor_utility_base_for_each_selected_actor: std::ptr::null_mut(),
            udeprecated_global_editor_utility_base_clear_actor_selection_set: std::ptr::null_mut(),
            adeprecated_placed_editor_utility_base_set_level_viewport_camera_info: std::ptr::null_mut(),
            adeprecated_placed_editor_utility_base_set_actor_selection_state: std::ptr::null_mut(),
            adeprecated_placed_editor_utility_base_select_nothing: std::ptr::null_mut(),
            adeprecated_placed_editor_utility_base_get_selection_set: std::ptr::null_mut(),
            adeprecated_placed_editor_utility_base_get_level_viewport_camera_info: std::ptr::null_mut(),
            adeprecated_placed_editor_utility_base_get_actor_reference: std::ptr::null_mut(),
            adeprecated_placed_editor_utility_base_clear_actor_selection_set: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEditorUtilityObject::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Run"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_object_run,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UActorActionUtility::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSupportedClasses"),
                &raw mut __FUNCTION_PTRS.u_actor_action_utility_get_supported_classes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSupportedClass"),
                &raw mut __FUNCTION_PTRS.u_actor_action_utility_get_supported_class,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAssetActionUtility::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsActionForBlueprints"),
                &raw mut __FUNCTION_PTRS.u_asset_action_utility_is_action_for_blueprints,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSupportedClasses"),
                &raw mut __FUNCTION_PTRS.u_asset_action_utility_get_supported_classes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSupportedClass"),
                &raw mut __FUNCTION_PTRS.u_asset_action_utility_get_supported_class,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAsyncCaptureScene::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CaptureSceneWithWarmupAsync"),
                &raw mut __FUNCTION_PTRS
                    .u_async_capture_scene_capture_scene_with_warmup_async,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CaptureSceneAsync"),
                &raw mut __FUNCTION_PTRS.u_async_capture_scene_capture_scene_async,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAsyncImageExport::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportImageAsync"),
                &raw mut __FUNCTION_PTRS.u_async_image_export_export_image_async,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAsyncRegisterAndExecuteTask::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterAndExecuteTask"),
                &raw mut __FUNCTION_PTRS
                    .u_async_register_and_execute_task_register_and_execute_task,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AEditorUtilityActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetReceivesEditorInput"),
                &raw mut __FUNCTION_PTRS.a_editor_utility_actor_set_receives_editor_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Run"),
                &raw mut __FUNCTION_PTRS.a_editor_utility_actor_run,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetReceivesEditorInput"),
                &raw mut __FUNCTION_PTRS.a_editor_utility_actor_get_receives_editor_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInputComponent"),
                &raw mut __FUNCTION_PTRS.a_editor_utility_actor_get_input_component,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAsyncEditorDelay::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AsyncEditorDelay"),
                &raw mut __FUNCTION_PTRS.u_async_editor_delay_async_editor_delay,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAsyncEditorWaitForGameWorld::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AsyncWaitForGameWorld"),
                &raw mut __FUNCTION_PTRS
                    .u_async_editor_wait_for_game_world_async_wait_for_game_world,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAsyncEditorOpenMapAndFocusActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AsyncEditorOpenMapAndFocusActor"),
                &raw mut __FUNCTION_PTRS
                    .u_async_editor_open_map_and_focus_actor_async_editor_open_map_and_focus_actor,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEditorUtilityLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SyncBrowserToFolders"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_library_sync_browser_to_folders,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameAsset"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_library_rename_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectionSet"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_library_get_selection_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectionBounds"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_library_get_selection_bounds,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedPathViewFolderPaths"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_path_view_folder_paths,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedFolderPaths"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_folder_paths,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedBlueprintClasses"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_blueprint_classes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedAssetsOfClass"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_assets_of_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedAssets"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_library_get_selected_assets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedAssetData"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_library_get_selected_asset_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentContentBrowserPath"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_library_get_current_content_browser_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentContentBrowserItemPath"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_library_get_current_content_browser_item_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorReference"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_library_get_actor_reference,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindSourceWidgetByName"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_library_find_source_widget_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToEditorUtilityWidget"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_library_convert_to_editor_utility_widget,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CastToWidgetBlueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_library_cast_to_widget_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSourceWidget"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_library_add_source_widget,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEditorUtilitySubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnregisterTabByID"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_subsystem_unregister_tab_by_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TryRunClass"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_subsystem_try_run_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TryRun"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_subsystem_try_run,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SpawnRegisteredTabByID"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_registered_tab_by_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SpawnAndRegisterTabWithIdGeneratedClass"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_with_id_generated_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SpawnAndRegisterTabWithId"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_with_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SpawnAndRegisterTabGeneratedClass"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_generated_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SpawnAndRegisterTabAndGetIDGeneratedClass"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_and_get_id_generated_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SpawnAndRegisterTabAndGetID"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_and_get_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SpawnAndRegisterTab"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReleaseInstanceOfAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_subsystem_release_instance_of_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterTabAndGetIDGeneratedClass"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_subsystem_register_tab_and_get_id_generated_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterTabAndGetID"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_subsystem_register_tab_and_get_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterAndExecuteTask"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_subsystem_register_and_execute_task,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindUtilityWidgetFromBlueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_subsystem_find_utility_widget_from_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DoesTabExist"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_subsystem_does_tab_exist,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CloseTabByID"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_subsystem_close_tab_by_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanRun"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_subsystem_can_run,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEditorUtilityTask::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WasCancelRequested"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_task_was_cancel_requested,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTaskNotificationText"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_task_set_task_notification_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Run"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_task_run,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveCancelRequested"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_task_receive_cancel_requested,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveBeginExecution"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_task_receive_begin_execution,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTaskTitleOverride"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_task_get_task_title_override,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FinishExecutingTask"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_task_finish_executing_task,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEditorUtilityWidget::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Run"),
                &raw mut __FUNCTION_PTRS.u_editor_utility_widget_run,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindChildWidgetByName"),
                &raw mut __FUNCTION_PTRS
                    .u_editor_utility_widget_find_child_widget_by_name,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDEPRECATED_GlobalEditorUtilityBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetActorSelectionState"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_set_actor_selection_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectNothing"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_select_nothing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameAsset"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_rename_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnDefaultActionClicked"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_on_default_action_clicked,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectionSet"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_selection_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectionBounds"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_selection_bounds,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedAssets"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_selected_assets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEditorUserSettings"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_editor_user_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorReference"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_actor_reference,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ForEachSelectedAsset"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_for_each_selected_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ForEachSelectedActor"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_for_each_selected_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearActorSelectionSet"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_clear_actor_selection_set,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ADEPRECATED_PlacedEditorUtilityBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLevelViewportCameraInfo"),
                &raw mut __FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_set_level_viewport_camera_info,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetActorSelectionState"),
                &raw mut __FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_set_actor_selection_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectNothing"),
                &raw mut __FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_select_nothing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectionSet"),
                &raw mut __FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_get_selection_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLevelViewportCameraInfo"),
                &raw mut __FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_get_level_viewport_camera_info,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorReference"),
                &raw mut __FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_get_actor_reference,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearActorSelectionSet"),
                &raw mut __FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_clear_actor_selection_set,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UEditorFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UEditorFunctionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorFunctionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorFunctionLibrary")
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
pub struct UEditorUtilityToolMenuEntry {
    __padding_end: [u8; 240],
}
impl UEditorUtilityToolMenuEntry {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityToolMenuEntry")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityToolMenuEntry")
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
pub struct UEditorUtilityToolMenuSection {
    __padding_end: [u8; 64],
}
impl UEditorUtilityToolMenuSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityToolMenuSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityToolMenuSection")
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
pub struct IEditorUtilityExtension {}
#[repr(C, align(8))]
pub struct UEditorUtilityExtension {
    __padding_end: [u8; 48],
}
impl UEditorUtilityExtension {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityExtension")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityExtension")
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
pub struct UEditorUtilityObject {
    __padding_end: [u8; 64],
}
impl UEditorUtilityObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityObject")
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
    pub fn run(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS.u_editor_utility_object_run,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS.u_editor_utility_object_run,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UActorActionUtility {
    __padding_end: [u8; 88],
}
impl UActorActionUtility {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorActionUtility")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorActionUtility")
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
    pub fn get_supported_classes(
        &self,
    ) -> TArray<TSoftObjectPtr<crate::bindings::core_u_object::UClass>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_actor_action_utility_get_supported_classes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_actor_action_utility_get_supported_classes,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<TSoftObjectPtr<crate::bindings::core_u_object::UClass>>>()
                .read()
        }
    }
    pub fn get_supported_class(
        &self,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_actor_action_utility_get_supported_class,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_actor_action_utility_get_supported_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAssetActionUtility {
    __padding_end: [u8; 112],
}
impl UAssetActionUtility {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetActionUtility")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetActionUtility")
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
    pub fn is_action_for_blueprints(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_asset_action_utility_is_action_for_blueprints,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_asset_action_utility_is_action_for_blueprints,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_supported_classes(
        &self,
    ) -> TArray<TSoftObjectPtr<crate::bindings::core_u_object::UClass>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_asset_action_utility_get_supported_classes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_asset_action_utility_get_supported_classes,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<TSoftObjectPtr<crate::bindings::core_u_object::UClass>>>()
                .read()
        }
    }
    pub fn get_supported_class(
        &self,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_asset_action_utility_get_supported_class,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_asset_action_utility_get_supported_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAsyncCaptureScene {
    __padding_end: [u8; 104],
}
impl UAsyncCaptureScene {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncCaptureScene")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncCaptureScene")
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
    pub fn capture_scene_with_warmup_async(
        view_camera: UPtr<crate::bindings::engine::UCameraComponent>,
        scene_capture_class: TSubclassOf<crate::bindings::engine::ASceneCapture2D>,
        res_x: i32,
        res_y: i32,
        warm_up_frames: i32,
    ) -> UPtr<UAsyncCaptureScene> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_capture_scene_capture_scene_with_warmup_async,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &view_camera,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UCameraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scene_capture_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::engine::ASceneCapture2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&res_x, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&res_y, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &warm_up_frames,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UAsyncCaptureScene::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_capture_scene_capture_scene_with_warmup_async,
                __buffer,
            )
        };
        std::mem::forget(view_camera);
        std::mem::forget(scene_capture_class);
        std::mem::forget(res_x);
        std::mem::forget(res_y);
        std::mem::forget(warm_up_frames);
        unsafe { __buffer.add(32).cast::<UPtr<UAsyncCaptureScene>>().read() }
    }
    pub fn capture_scene_async(
        view_camera: UPtr<crate::bindings::engine::UCameraComponent>,
        scene_capture_class: TSubclassOf<crate::bindings::engine::ASceneCapture2D>,
        res_x: i32,
        res_y: i32,
    ) -> UPtr<UAsyncCaptureScene> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_capture_scene_capture_scene_async,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &view_camera,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UCameraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scene_capture_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::engine::ASceneCapture2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&res_x, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&res_y, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::blutility::UAsyncCaptureScene::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_capture_scene_capture_scene_async,
                __buffer,
            )
        };
        std::mem::forget(view_camera);
        std::mem::forget(scene_capture_class);
        std::mem::forget(res_x);
        std::mem::forget(res_y);
        unsafe { __buffer.add(24).cast::<UPtr<UAsyncCaptureScene>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAsyncImageExport {
    __padding_end: [u8; 112],
}
impl UAsyncImageExport {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncImageExport")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncImageExport")
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
    pub fn export_image_async(
        texture: UPtr<crate::bindings::engine::UTexture>,
        output_file: FString,
        quality: i32,
    ) -> UPtr<UAsyncImageExport> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_image_export_export_image_async,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_file,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&quality, __buffer.add(24).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::blutility::UAsyncImageExport::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_image_export_export_image_async,
                __buffer,
            )
        };
        std::mem::forget(texture);
        std::mem::forget(output_file);
        std::mem::forget(quality);
        unsafe { __buffer.add(32).cast::<UPtr<UAsyncImageExport>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAsyncRegisterAndExecuteTask {
    __padding_end: [u8; 80],
}
impl UAsyncRegisterAndExecuteTask {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncRegisterAndExecuteTask")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncRegisterAndExecuteTask")
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
    pub fn register_and_execute_task(
        task: UPtr<UEditorUtilityTask>,
        optional_parent_task: UPtr<UEditorUtilityTask>,
    ) -> UPtr<UAsyncRegisterAndExecuteTask> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_register_and_execute_task_register_and_execute_task,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task,
                __buffer.add(0).cast::<UPtr<UEditorUtilityTask>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_parent_task,
                __buffer.add(8).cast::<UPtr<UEditorUtilityTask>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UAsyncRegisterAndExecuteTask::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_register_and_execute_task_register_and_execute_task,
                __buffer,
            )
        };
        std::mem::forget(task);
        std::mem::forget(optional_parent_task);
        unsafe { __buffer.add(16).cast::<UPtr<UAsyncRegisterAndExecuteTask>>().read() }
    }
}
#[repr(C, align(8))]
pub struct AEditorUtilityActor {
    #[doc(hidden)]
    pub(crate) __padding_1144: [u8; 1144],
    pub b_receives_editor_input: bool,
}
impl AEditorUtilityActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AEditorUtilityActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AEditorUtilityActor")
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
    pub fn set_receives_editor_input(&mut self, b_in_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .a_editor_utility_actor_set_receives_editor_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_value,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .a_editor_utility_actor_set_receives_editor_input,
                __buffer,
            )
        };
        std::mem::forget(b_in_value);
    }
    pub fn run(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS.a_editor_utility_actor_run,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS.a_editor_utility_actor_run,
                __buffer,
            )
        };
    }
    pub fn get_receives_editor_input(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .a_editor_utility_actor_get_receives_editor_input,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .a_editor_utility_actor_get_receives_editor_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_input_component(&self) -> UPtr<crate::bindings::engine::UInputComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .a_editor_utility_actor_get_input_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .a_editor_utility_actor_get_input_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UInputComponent>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UEditorUtilityActorComponent {
    __padding_end: [u8; 240],
}
impl UEditorUtilityActorComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityActorComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityActorComponent")
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
pub struct UEditorUtilityBlueprint {
    __padding_end: [u8; 1432],
}
impl UEditorUtilityBlueprint {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityBlueprint")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityBlueprint")
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
pub struct UEditorUtilityBlueprintFactory {
    __padding_end: [u8; 160],
}
impl UEditorUtilityBlueprintFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityBlueprintFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityBlueprintFactory")
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
#[repr(C, align(16))]
pub struct AEditorUtilityCamera {
    __padding_end: [u8; 3136],
}
impl AEditorUtilityCamera {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AEditorUtilityCamera")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AEditorUtilityCamera")
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
pub struct UEditorUtilityBlueprintAsyncActionBase {
    __padding_end: [u8; 56],
}
impl UEditorUtilityBlueprintAsyncActionBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityBlueprintAsyncActionBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityBlueprintAsyncActionBase")
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
pub struct UAsyncEditorDelay {
    __padding_end: [u8; 96],
}
impl UAsyncEditorDelay {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncEditorDelay")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncEditorDelay")
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
    pub fn async_editor_delay(
        seconds: f32,
        minimum_frames: i32,
    ) -> UPtr<UAsyncEditorDelay> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_editor_delay_async_editor_delay,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &minimum_frames,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UAsyncEditorDelay::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_editor_delay_async_editor_delay,
                __buffer,
            )
        };
        std::mem::forget(seconds);
        std::mem::forget(minimum_frames);
        unsafe { __buffer.add(8).cast::<UPtr<UAsyncEditorDelay>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAsyncEditorWaitForGameWorld {
    __padding_end: [u8; 88],
}
impl UAsyncEditorWaitForGameWorld {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncEditorWaitForGameWorld")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncEditorWaitForGameWorld")
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
    pub fn async_wait_for_game_world(
        index: i32,
        server: bool,
    ) -> UPtr<UAsyncEditorWaitForGameWorld> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_editor_wait_for_game_world_async_wait_for_game_world,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&server, __buffer.add(4).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::blutility::UAsyncEditorWaitForGameWorld::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_editor_wait_for_game_world_async_wait_for_game_world,
                __buffer,
            )
        };
        std::mem::forget(index);
        std::mem::forget(server);
        unsafe { __buffer.add(8).cast::<UPtr<UAsyncEditorWaitForGameWorld>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAsyncEditorOpenMapAndFocusActor {
    __padding_end: [u8; 136],
}
impl UAsyncEditorOpenMapAndFocusActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncEditorOpenMapAndFocusActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncEditorOpenMapAndFocusActor")
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
    pub fn async_editor_open_map_and_focus_actor(
        map: crate::bindings::core_u_object::FSoftObjectPath,
        focus_actor_name: FString,
    ) -> UPtr<UAsyncEditorOpenMapAndFocusActor> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_editor_open_map_and_focus_actor_async_editor_open_map_and_focus_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &map,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &focus_actor_name,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UAsyncEditorOpenMapAndFocusActor::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_async_editor_open_map_and_focus_actor_async_editor_open_map_and_focus_actor,
                __buffer,
            )
        };
        std::mem::forget(map);
        std::mem::forget(focus_actor_name);
        unsafe {
            __buffer.add(56).cast::<UPtr<UAsyncEditorOpenMapAndFocusActor>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UEditorUtilityLibrary {
    __padding_end: [u8; 48],
}
impl UEditorUtilityLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityLibrary")
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
    pub fn sync_browser_to_folders(folder_list: &TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_sync_browser_to_folders,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                folder_list,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_sync_browser_to_folders,
                __buffer,
            )
        };
    }
    pub fn rename_asset(
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        new_name: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_rename_asset,
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
                &new_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_rename_asset,
                __buffer,
            )
        };
        std::mem::forget(asset);
        std::mem::forget(new_name);
    }
    pub fn get_selection_set() -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selection_set,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selection_set,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_selection_bounds(
        origin: &mut crate::bindings::core_u_object::FVector,
        box_extent: &mut crate::bindings::core_u_object::FVector,
        sphere_radius: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selection_bounds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                origin,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                box_extent,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                sphere_radius,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selection_bounds,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(origin);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(box_extent);
        }
        unsafe {
            __buffer.add(48).cast::<f32>().swap(sphere_radius);
        }
    }
    pub fn get_selected_path_view_folder_paths() -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_path_view_folder_paths,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_path_view_folder_paths,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_selected_folder_paths() -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_folder_paths,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_folder_paths,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_selected_blueprint_classes() -> TArray<
        TSubclassOf<crate::bindings::core_u_object::UObject>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_blueprint_classes,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_blueprint_classes,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<TSubclassOf<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn get_selected_assets_of_class(
        asset_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_assets_of_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_assets_of_class,
                __buffer,
            )
        };
        std::mem::forget(asset_class);
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn get_selected_assets() -> TArray<
        UPtr<crate::bindings::core_u_object::UObject>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_assets,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_assets,
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
    pub fn get_selected_asset_data() -> TArray<
        crate::bindings::core_u_object::FAssetData,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_asset_data,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_selected_asset_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .read()
        }
    }
    pub fn get_current_content_browser_path(out_path: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_current_content_browser_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_current_content_browser_path,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(out_path);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_current_content_browser_item_path() -> crate::bindings::content_browser_data::FContentBrowserItemPath {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_current_content_browser_item_path,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_current_content_browser_item_path,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::content_browser_data::FContentBrowserItemPath>()
                .read()
        }
    }
    pub fn get_actor_reference(
        &mut self,
        path_to_actor: FString,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_actor_reference,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_to_actor,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_get_actor_reference,
                __buffer,
            )
        };
        std::mem::forget(path_to_actor);
        unsafe {
            __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn find_source_widget_by_name(
        widget_blueprint: UPtr<crate::bindings::umg_editor::UWidgetBlueprint>,
        widget_name: FName,
    ) -> UPtr<crate::bindings::umg::UWidget> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_find_source_widget_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &widget_blueprint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::umg_editor::UWidgetBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &widget_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_find_source_widget_by_name,
                __buffer,
            )
        };
        std::mem::forget(widget_blueprint);
        std::mem::forget(widget_name);
        unsafe { __buffer.add(24).cast::<UPtr<crate::bindings::umg::UWidget>>().read() }
    }
    pub fn convert_to_editor_utility_widget(
        widget_bp: UPtr<crate::bindings::umg_editor::UWidgetBlueprint>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_convert_to_editor_utility_widget,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &widget_bp,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::umg_editor::UWidgetBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_convert_to_editor_utility_widget,
                __buffer,
            )
        };
        std::mem::forget(widget_bp);
    }
    pub fn cast_to_widget_blueprint(
        object: UPtr<crate::bindings::core_u_object::UObject>,
        branches: &mut ECastToWidgetBlueprintCases,
        as_widget_blueprint: &mut UPtr<crate::bindings::umg_editor::UWidgetBlueprint>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_cast_to_widget_blueprint,
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
                branches,
                __buffer.add(8).cast::<ECastToWidgetBlueprintCases>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                as_widget_blueprint,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::umg_editor::UWidgetBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_cast_to_widget_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<ECastToWidgetBlueprintCases>().swap(branches);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::umg_editor::UWidgetBlueprint>>()
                .swap(as_widget_blueprint);
        }
        std::mem::forget(object);
    }
    pub fn add_source_widget(
        widget_blueprint: UPtr<crate::bindings::umg_editor::UWidgetBlueprint>,
        widget_class: TSubclassOf<crate::bindings::umg::UWidget>,
        widget_name: FName,
        widget_parent_name: FName,
    ) -> UPtr<crate::bindings::umg::UWidget> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_add_source_widget,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &widget_blueprint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::umg_editor::UWidgetBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &widget_class,
                __buffer.add(8).cast::<TSubclassOf<crate::bindings::umg::UWidget>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &widget_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &widget_parent_name,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_library_add_source_widget,
                __buffer,
            )
        };
        std::mem::forget(widget_blueprint);
        std::mem::forget(widget_class);
        std::mem::forget(widget_name);
        std::mem::forget(widget_parent_name);
        unsafe { __buffer.add(40).cast::<UPtr<crate::bindings::umg::UWidget>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UEditorUtilitySubsystem {
    __padding_end: [u8; 688],
}
impl UEditorUtilitySubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilitySubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilitySubsystem")
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
    pub fn unregister_tab_by_id(&mut self, tab_id: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_unregister_tab_by_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tab_id, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_unregister_tab_by_id,
                __buffer,
            )
        };
        std::mem::forget(tab_id);
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn try_run_class(
        &mut self,
        object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_try_run_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_class,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_try_run_class,
                __buffer,
            )
        };
        std::mem::forget(object_class);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn try_run(
        &mut self,
        asset: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_try_run,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_try_run,
                __buffer,
            )
        };
        std::mem::forget(asset);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn spawn_registered_tab_by_id(&mut self, new_tab_id: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_registered_tab_by_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_tab_id,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_registered_tab_by_id,
                __buffer,
            )
        };
        std::mem::forget(new_tab_id);
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn spawn_and_register_tab_with_id_generated_class(
        &mut self,
        in_generated_widget_blueprint: TSubclassOf<
            crate::bindings::core_u_object::UObject,
        >,
        in_tab_id: FName,
    ) -> UPtr<UEditorUtilityWidget> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_with_id_generated_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_generated_widget_blueprint,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_tab_id,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_with_id_generated_class,
                __buffer,
            )
        };
        std::mem::forget(in_generated_widget_blueprint);
        std::mem::forget(in_tab_id);
        unsafe { __buffer.add(24).cast::<UPtr<UEditorUtilityWidget>>().read() }
    }
    pub fn spawn_and_register_tab_with_id(
        &mut self,
        in_blueprint: UPtr<UEditorUtilityWidgetBlueprint>,
        in_tab_id: FName,
    ) -> UPtr<UEditorUtilityWidget> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_with_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint,
                __buffer.add(0).cast::<UPtr<UEditorUtilityWidgetBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_tab_id,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_with_id,
                __buffer,
            )
        };
        std::mem::forget(in_blueprint);
        std::mem::forget(in_tab_id);
        unsafe { __buffer.add(24).cast::<UPtr<UEditorUtilityWidget>>().read() }
    }
    pub fn spawn_and_register_tab_generated_class(
        &mut self,
        in_generated_widget_blueprint: TSubclassOf<
            crate::bindings::core_u_object::UObject,
        >,
    ) -> UPtr<UEditorUtilityWidget> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_generated_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_generated_widget_blueprint,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_generated_class,
                __buffer,
            )
        };
        std::mem::forget(in_generated_widget_blueprint);
        unsafe { __buffer.add(8).cast::<UPtr<UEditorUtilityWidget>>().read() }
    }
    pub fn spawn_and_register_tab_and_get_id_generated_class(
        &mut self,
        in_generated_widget_blueprint: TSubclassOf<
            crate::bindings::core_u_object::UObject,
        >,
        new_tab_id: &mut FName,
    ) -> UPtr<UEditorUtilityWidget> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_and_get_id_generated_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_generated_widget_blueprint,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_tab_id,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_and_get_id_generated_class,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FName>().swap(new_tab_id);
        }
        std::mem::forget(in_generated_widget_blueprint);
        unsafe { __buffer.add(24).cast::<UPtr<UEditorUtilityWidget>>().read() }
    }
    pub fn spawn_and_register_tab_and_get_id(
        &mut self,
        in_blueprint: UPtr<UEditorUtilityWidgetBlueprint>,
        new_tab_id: &mut FName,
    ) -> UPtr<UEditorUtilityWidget> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_and_get_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint,
                __buffer.add(0).cast::<UPtr<UEditorUtilityWidgetBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_tab_id,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab_and_get_id,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FName>().swap(new_tab_id);
        }
        std::mem::forget(in_blueprint);
        unsafe { __buffer.add(24).cast::<UPtr<UEditorUtilityWidget>>().read() }
    }
    pub fn spawn_and_register_tab(
        &mut self,
        in_blueprint: UPtr<UEditorUtilityWidgetBlueprint>,
    ) -> UPtr<UEditorUtilityWidget> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint,
                __buffer.add(0).cast::<UPtr<UEditorUtilityWidgetBlueprint>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_spawn_and_register_tab,
                __buffer,
            )
        };
        std::mem::forget(in_blueprint);
        unsafe { __buffer.add(8).cast::<UPtr<UEditorUtilityWidget>>().read() }
    }
    pub fn release_instance_of_asset(
        &mut self,
        asset: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_release_instance_of_asset,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_release_instance_of_asset,
                __buffer,
            )
        };
        std::mem::forget(asset);
    }
    pub fn register_tab_and_get_id_generated_class(
        &mut self,
        in_generated_widget_blueprint: TSubclassOf<
            crate::bindings::core_u_object::UObject,
        >,
        new_tab_id: &mut FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_register_tab_and_get_id_generated_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_generated_widget_blueprint,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_tab_id,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_register_tab_and_get_id_generated_class,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FName>().swap(new_tab_id);
        }
        std::mem::forget(in_generated_widget_blueprint);
    }
    pub fn register_tab_and_get_id(
        &mut self,
        in_blueprint: UPtr<UEditorUtilityWidgetBlueprint>,
        new_tab_id: &mut FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_register_tab_and_get_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint,
                __buffer.add(0).cast::<UPtr<UEditorUtilityWidgetBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_tab_id,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_register_tab_and_get_id,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FName>().swap(new_tab_id);
        }
        std::mem::forget(in_blueprint);
    }
    pub fn register_and_execute_task(
        &mut self,
        new_task: UPtr<UEditorUtilityTask>,
        optional_parent_task: UPtr<UEditorUtilityTask>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_register_and_execute_task,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_task,
                __buffer.add(0).cast::<UPtr<UEditorUtilityTask>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_parent_task,
                __buffer.add(8).cast::<UPtr<UEditorUtilityTask>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_register_and_execute_task,
                __buffer,
            )
        };
        std::mem::forget(new_task);
        std::mem::forget(optional_parent_task);
    }
    pub fn find_utility_widget_from_blueprint(
        &mut self,
        in_blueprint: UPtr<UEditorUtilityWidgetBlueprint>,
    ) -> UPtr<UEditorUtilityWidget> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_find_utility_widget_from_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint,
                __buffer.add(0).cast::<UPtr<UEditorUtilityWidgetBlueprint>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_find_utility_widget_from_blueprint,
                __buffer,
            )
        };
        std::mem::forget(in_blueprint);
        unsafe { __buffer.add(8).cast::<UPtr<UEditorUtilityWidget>>().read() }
    }
    pub fn does_tab_exist(&mut self, new_tab_id: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_does_tab_exist,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_tab_id,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_does_tab_exist,
                __buffer,
            )
        };
        std::mem::forget(new_tab_id);
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn close_tab_by_id(&mut self, new_tab_id: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_close_tab_by_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_tab_id,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_close_tab_by_id,
                __buffer,
            )
        };
        std::mem::forget(new_tab_id);
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn can_run(&self, asset: UPtr<crate::bindings::core_u_object::UObject>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_can_run,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_subsystem_can_run,
                __buffer,
            )
        };
        std::mem::forget(asset);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEditorUtilityTask {
    __padding_end: [u8; 128],
}
impl UEditorUtilityTask {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityTask")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityTask")
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
    pub fn was_cancel_requested(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_task_was_cancel_requested,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_task_was_cancel_requested,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_task_notification_text(&mut self, text: &FText) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_task_set_task_notification_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(text, __buffer.add(0).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_task_set_task_notification_text,
                __buffer,
            )
        };
    }
    pub fn receive_cancel_requested(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_task_receive_cancel_requested,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_task_receive_cancel_requested,
                __buffer,
            )
        };
    }
    pub fn receive_begin_execution(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_task_receive_begin_execution,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_task_receive_begin_execution,
                __buffer,
            )
        };
    }
    pub fn get_task_title_override(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_task_get_task_title_override,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_task_get_task_title_override,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn finish_executing_task(&mut self, b_success: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_task_finish_executing_task,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_success, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_task_finish_executing_task,
                __buffer,
            )
        };
        std::mem::forget(b_success);
    }
}
#[repr(C, align(8))]
pub struct UEditorUtilityWidget {
    #[doc(hidden)]
    pub(crate) __padding_1296: [u8; 1296],
    pub tab_display_name: FText,
    pub help_text: FString,
    #[doc(hidden)]
    pub(crate) __padding_1329: [u8; 1],
    pub b_auto_run_default_action: bool,
}
impl UEditorUtilityWidget {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityWidget")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityWidget")
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
    pub fn run(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS.u_editor_utility_widget_run,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS.u_editor_utility_widget_run,
                __buffer,
            )
        };
    }
    pub fn find_child_widget_by_name(
        &self,
        widget_name: FName,
    ) -> UPtr<crate::bindings::umg::UWidget> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_widget_find_child_widget_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &widget_name,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .u_editor_utility_widget_find_child_widget_by_name,
                __buffer,
            )
        };
        std::mem::forget(widget_name);
        unsafe { __buffer.add(16).cast::<UPtr<crate::bindings::umg::UWidget>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEditorUtilityWidgetBlueprint {
    __padding_end: [u8; 1672],
}
impl UEditorUtilityWidgetBlueprint {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityWidgetBlueprint")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityWidgetBlueprint")
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
pub struct UEditorUtilityWidgetBlueprintFactory {
    __padding_end: [u8; 160],
}
impl UEditorUtilityWidgetBlueprintFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityWidgetBlueprintFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityWidgetBlueprintFactory")
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
#[repr(C, align(16))]
pub struct UEditorUtilityButton {
    __padding_end: [u8; 2160],
}
impl UEditorUtilityButton {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityButton")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityButton")
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
#[repr(C, align(16))]
pub struct UEditorUtilityCheckBox {
    __padding_end: [u8; 3664],
}
impl UEditorUtilityCheckBox {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityCheckBox")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityCheckBox")
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
#[repr(C, align(16))]
pub struct UEditorUtilityCircularThrobber {
    __padding_end: [u8; 960],
}
impl UEditorUtilityCircularThrobber {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityCircularThrobber")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityCircularThrobber")
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
#[repr(C, align(16))]
pub struct UEditorUtilityComboBoxKey {
    __padding_end: [u8; 8000],
}
impl UEditorUtilityComboBoxKey {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityComboBoxKey")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityComboBoxKey")
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
#[repr(C, align(16))]
pub struct UEditorUtilityComboBoxString {
    __padding_end: [u8; 8128],
}
impl UEditorUtilityComboBoxString {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityComboBoxString")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityComboBoxString")
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
#[repr(C, align(16))]
pub struct UEditorUtilityEditableText {
    __padding_end: [u8; 1696],
}
impl UEditorUtilityEditableText {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityEditableText")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityEditableText")
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
#[repr(C, align(16))]
pub struct UEditorUtilityEditableTextBox {
    __padding_end: [u8; 4768],
}
impl UEditorUtilityEditableTextBox {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityEditableTextBox")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityEditableTextBox")
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
#[repr(C, align(16))]
pub struct UEditorUtilityExpandableArea {
    __padding_end: [u8; 1488],
}
impl UEditorUtilityExpandableArea {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityExpandableArea")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityExpandableArea")
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
#[repr(C, align(16))]
pub struct UEditorUtilityInputKeySelector {
    __padding_end: [u8; 2816],
}
impl UEditorUtilityInputKeySelector {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityInputKeySelector")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityInputKeySelector")
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
#[repr(C, align(16))]
pub struct UEditorUtilityListView {
    __padding_end: [u8; 5088],
}
impl UEditorUtilityListView {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityListView")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityListView")
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
#[repr(C, align(16))]
pub struct UEditorUtilityMultiLineEditableText {
    __padding_end: [u8; 1744],
}
impl UEditorUtilityMultiLineEditableText {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityMultiLineEditableText")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityMultiLineEditableText")
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
#[repr(C, align(16))]
pub struct UEditorUtilityMultiLineEditableTextBox {
    __padding_end: [u8; 5584],
}
impl UEditorUtilityMultiLineEditableTextBox {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityMultiLineEditableTextBox")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityMultiLineEditableTextBox")
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
#[repr(C, align(16))]
pub struct UEditorUtilityProgressBar {
    __padding_end: [u8; 1504],
}
impl UEditorUtilityProgressBar {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityProgressBar")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityProgressBar")
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
#[repr(C, align(16))]
pub struct UEditorUtilityScrollBar {
    __padding_end: [u8; 2672],
}
impl UEditorUtilityScrollBar {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityScrollBar")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityScrollBar")
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
#[repr(C, align(16))]
pub struct UEditorUtilityScrollBox {
    __padding_end: [u8; 3760],
}
impl UEditorUtilityScrollBox {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityScrollBox")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityScrollBox")
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
#[repr(C, align(16))]
pub struct UEditorUtilitySlider {
    __padding_end: [u8; 2224],
}
impl UEditorUtilitySlider {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilitySlider")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilitySlider")
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
#[repr(C, align(16))]
pub struct UEditorUtilitySpinBox {
    __padding_end: [u8; 2592],
}
impl UEditorUtilitySpinBox {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilitySpinBox")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilitySpinBox")
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
#[repr(C, align(16))]
pub struct UEditorUtilityThrobber {
    __padding_end: [u8; 928],
}
impl UEditorUtilityThrobber {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityThrobber")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityThrobber")
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
#[repr(C, align(16))]
pub struct UEditorUtilityTreeView {
    __padding_end: [u8; 5216],
}
impl UEditorUtilityTreeView {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityTreeView")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityTreeView")
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
pub struct UEditorUtilityWidgetProjectSettings {
    __padding_end: [u8; 1176],
}
impl UEditorUtilityWidgetProjectSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityWidgetProjectSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityWidgetProjectSettings")
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
pub struct UDEPRECATED_GlobalEditorUtilityBase {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub help_text: FString,
    #[doc(hidden)]
    pub(crate) __padding_65: [u8; 1],
    pub b_auto_run_default_action: bool,
    __padding_end: [u8; 54],
}
impl UDEPRECATED_GlobalEditorUtilityBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_GlobalEditorUtilityBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_GlobalEditorUtilityBase")
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
    pub fn set_actor_selection_state(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        b_should_be_selected: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_set_actor_selection_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_be_selected,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_set_actor_selection_state,
                __buffer,
            )
        };
        std::mem::forget(actor);
        std::mem::forget(b_should_be_selected);
    }
    pub fn select_nothing(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_select_nothing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_select_nothing,
                __buffer,
            )
        };
    }
    pub fn rename_asset(
        &mut self,
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        new_name: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_rename_asset,
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
                &new_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_rename_asset,
                __buffer,
            )
        };
        std::mem::forget(asset);
        std::mem::forget(new_name);
    }
    pub fn on_default_action_clicked(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_on_default_action_clicked,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_on_default_action_clicked,
                __buffer,
            )
        };
    }
    pub fn get_selection_set(
        &mut self,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_selection_set,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_selection_set,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_selection_bounds(
        &mut self,
        origin: &mut crate::bindings::core_u_object::FVector,
        box_extent: &mut crate::bindings::core_u_object::FVector,
        sphere_radius: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_selection_bounds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                origin,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                box_extent,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                sphere_radius,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_selection_bounds,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(origin);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(box_extent);
        }
        unsafe {
            __buffer.add(48).cast::<f32>().swap(sphere_radius);
        }
    }
    pub fn get_selected_assets(
        &mut self,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_selected_assets,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_selected_assets,
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
    pub fn get_editor_user_settings(
        &mut self,
    ) -> UPtr<crate::bindings::unreal_ed::UEditorPerProjectUserSettings> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_editor_user_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_editor_user_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    UPtr<crate::bindings::unreal_ed::UEditorPerProjectUserSettings>,
                >()
                .read()
        }
    }
    pub fn get_actor_reference(
        &mut self,
        path_to_actor: FString,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_actor_reference,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_to_actor,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_get_actor_reference,
                __buffer,
            )
        };
        std::mem::forget(path_to_actor);
        unsafe {
            __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn for_each_selected_asset(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_for_each_selected_asset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_for_each_selected_asset,
                __buffer,
            )
        };
    }
    pub fn for_each_selected_actor(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_for_each_selected_actor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_for_each_selected_actor,
                __buffer,
            )
        };
    }
    pub fn clear_actor_selection_set(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_clear_actor_selection_set,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .udeprecated_global_editor_utility_base_clear_actor_selection_set,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct ADEPRECATED_PlacedEditorUtilityBase {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub help_text: FString,
}
impl ADEPRECATED_PlacedEditorUtilityBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADEPRECATED_PlacedEditorUtilityBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADEPRECATED_PlacedEditorUtilityBase")
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
    pub fn set_level_viewport_camera_info(
        &mut self,
        camera_location: crate::bindings::core_u_object::FVector,
        camera_rotation: crate::bindings::core_u_object::FRotator,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_set_level_viewport_camera_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_location,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rotation,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_set_level_viewport_camera_info,
                __buffer,
            )
        };
        std::mem::forget(camera_location);
        std::mem::forget(camera_rotation);
    }
    pub fn set_actor_selection_state(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        b_should_be_selected: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_set_actor_selection_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_be_selected,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_set_actor_selection_state,
                __buffer,
            )
        };
        std::mem::forget(actor);
        std::mem::forget(b_should_be_selected);
    }
    pub fn select_nothing(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_select_nothing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_select_nothing,
                __buffer,
            )
        };
    }
    pub fn get_selection_set(
        &mut self,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_get_selection_set,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_get_selection_set,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_level_viewport_camera_info(
        &mut self,
        camera_location: &mut crate::bindings::core_u_object::FVector,
        camera_rotation: &mut crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_get_level_viewport_camera_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_location,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_rotation,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_get_level_viewport_camera_info,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(camera_location);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FRotator>()
                .swap(camera_rotation);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn get_actor_reference(
        &mut self,
        path_to_actor: FString,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_get_actor_reference,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_to_actor,
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
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_get_actor_reference,
                __buffer,
            )
        };
        std::mem::forget(path_to_actor);
        unsafe {
            __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn clear_actor_selection_set(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_clear_actor_selection_set,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::__FUNCTION_PTRS
                    .adeprecated_placed_editor_utility_base_clear_actor_selection_set,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UToolMenuWidget {
    #[doc(hidden)]
    pub(crate) __padding_696: [u8; 696],
    pub menu_name: FString,
    pub menu_type: crate::bindings::slate::EMultiBoxType,
    #[doc(hidden)]
    pub(crate) __padding_736: [u8; 20],
    pub full_menu_name: FName,
}
impl UToolMenuWidget {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuWidget")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuWidget")
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
pub struct FAsyncCaptureScene_Complete {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncImageExport_Complete {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncRegisterAndExecuteTask_OnFinished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncEditorDelay_Complete {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncEditorWaitForGameWorld_Complete {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncEditorOpenMapAndFocusActor_Complete {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorUtilitySubsystem_OnBeginPIE {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorUtilitySubsystem_OnEndPIE {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGlobalEditorUtilityBase_OnEachSelectedActor {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGlobalEditorUtilityBase_OnEachSelectedAsset {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ECastToWidgetBlueprintCases(pub u8);
impl ECastToWidgetBlueprintCases {
    pub const CAST_SUCCEEDED: ECastToWidgetBlueprintCases = ECastToWidgetBlueprintCases(
        0,
    );
    pub const CAST_FAILED: ECastToWidgetBlueprintCases = ECastToWidgetBlueprintCases(1);
}
