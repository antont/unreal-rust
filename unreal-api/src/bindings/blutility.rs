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
pub static mut U_EDITOR_UTILITY_OBJECT_RUN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ACTOR_ACTION_UTILITY_GET_SUPPORTED_CLASSES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ACTOR_ACTION_UTILITY_GET_SUPPORTED_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_ACTION_UTILITY_IS_ACTION_FOR_BLUEPRINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_ACTION_UTILITY_GET_SUPPORTED_CLASSES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_ACTION_UTILITY_GET_SUPPORTED_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASYNC_CAPTURE_SCENE_CAPTURE_SCENE_WITH_WARMUP_ASYNC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASYNC_CAPTURE_SCENE_CAPTURE_SCENE_ASYNC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASYNC_IMAGE_EXPORT_EXPORT_IMAGE_ASYNC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASYNC_REGISTER_AND_EXECUTE_TASK_REGISTER_AND_EXECUTE_TASK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_EDITOR_UTILITY_ACTOR_SET_RECEIVES_EDITOR_INPUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_EDITOR_UTILITY_ACTOR_RUN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_EDITOR_UTILITY_ACTOR_GET_RECEIVES_EDITOR_INPUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_EDITOR_UTILITY_ACTOR_GET_INPUT_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASYNC_EDITOR_DELAY_ASYNC_EDITOR_DELAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASYNC_EDITOR_WAIT_FOR_GAME_WORLD_ASYNC_WAIT_FOR_GAME_WORLD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASYNC_EDITOR_OPEN_MAP_AND_FOCUS_ACTOR_ASYNC_EDITOR_OPEN_MAP_AND_FOCUS_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_SYNC_BROWSER_TO_FOLDERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_RENAME_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTION_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTION_BOUNDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_PATH_VIEW_FOLDER_PATHS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_FOLDER_PATHS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_BLUEPRINT_CLASSES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_ASSETS_OF_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_GET_CURRENT_CONTENT_BROWSER_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_GET_CURRENT_CONTENT_BROWSER_ITEM_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_GET_ACTOR_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_FIND_SOURCE_WIDGET_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_CONVERT_TO_EDITOR_UTILITY_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_CAST_TO_WIDGET_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_LIBRARY_ADD_SOURCE_WIDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_UNREGISTER_TAB_BY_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_TRY_RUN_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_TRY_RUN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_REGISTERED_TAB_BY_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_WITH_ID_GENERATED_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_WITH_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_GENERATED_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_AND_GET_ID_GENERATED_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_AND_GET_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_RELEASE_INSTANCE_OF_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_REGISTER_TAB_AND_GET_ID_GENERATED_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_REGISTER_TAB_AND_GET_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_REGISTER_AND_EXECUTE_TASK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_FIND_UTILITY_WIDGET_FROM_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_DOES_TAB_EXIST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_CLOSE_TAB_BY_ID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_SUBSYSTEM_CAN_RUN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_TASK_WAS_CANCEL_REQUESTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_TASK_SET_TASK_NOTIFICATION_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_TASK_RUN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_TASK_RECEIVE_CANCEL_REQUESTED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_TASK_RECEIVE_BEGIN_EXECUTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_TASK_GET_TASK_TITLE_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_TASK_FINISH_EXECUTING_TASK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_WIDGET_RUN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_UTILITY_WIDGET_FIND_CHILD_WIDGET_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_SET_ACTOR_SELECTION_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_SELECT_NOTHING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_RENAME_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_ON_DEFAULT_ACTION_CLICKED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_SELECTION_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_SELECTION_BOUNDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_SELECTED_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_EDITOR_USER_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_ACTOR_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_FOR_EACH_SELECTED_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_FOR_EACH_SELECTED_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_CLEAR_ACTOR_SELECTION_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_SET_LEVEL_VIEWPORT_CAMERA_INFO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_SET_ACTOR_SELECTION_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_SELECT_NOTHING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_GET_SELECTION_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_GET_LEVEL_VIEWPORT_CAMERA_INFO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_GET_ACTOR_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_CLEAR_ACTOR_SELECTION_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorUtilityObject::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Run"),
            &raw mut U_EDITOR_UTILITY_OBJECT_RUN,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UActorActionUtility::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedClasses"),
            &raw mut U_ACTOR_ACTION_UTILITY_GET_SUPPORTED_CLASSES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedClass"),
            &raw mut U_ACTOR_ACTION_UTILITY_GET_SUPPORTED_CLASS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetActionUtility::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsActionForBlueprints"),
            &raw mut U_ASSET_ACTION_UTILITY_IS_ACTION_FOR_BLUEPRINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedClasses"),
            &raw mut U_ASSET_ACTION_UTILITY_GET_SUPPORTED_CLASSES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSupportedClass"),
            &raw mut U_ASSET_ACTION_UTILITY_GET_SUPPORTED_CLASS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAsyncCaptureScene::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CaptureSceneWithWarmupAsync"),
            &raw mut U_ASYNC_CAPTURE_SCENE_CAPTURE_SCENE_WITH_WARMUP_ASYNC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CaptureSceneAsync"),
            &raw mut U_ASYNC_CAPTURE_SCENE_CAPTURE_SCENE_ASYNC,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAsyncImageExport::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportImageAsync"),
            &raw mut U_ASYNC_IMAGE_EXPORT_EXPORT_IMAGE_ASYNC,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAsyncRegisterAndExecuteTask::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterAndExecuteTask"),
            &raw mut U_ASYNC_REGISTER_AND_EXECUTE_TASK_REGISTER_AND_EXECUTE_TASK,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AEditorUtilityActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReceivesEditorInput"),
            &raw mut A_EDITOR_UTILITY_ACTOR_SET_RECEIVES_EDITOR_INPUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Run"),
            &raw mut A_EDITOR_UTILITY_ACTOR_RUN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetReceivesEditorInput"),
            &raw mut A_EDITOR_UTILITY_ACTOR_GET_RECEIVES_EDITOR_INPUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputComponent"),
            &raw mut A_EDITOR_UTILITY_ACTOR_GET_INPUT_COMPONENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAsyncEditorDelay::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsyncEditorDelay"),
            &raw mut U_ASYNC_EDITOR_DELAY_ASYNC_EDITOR_DELAY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAsyncEditorWaitForGameWorld::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsyncWaitForGameWorld"),
            &raw mut U_ASYNC_EDITOR_WAIT_FOR_GAME_WORLD_ASYNC_WAIT_FOR_GAME_WORLD,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAsyncEditorOpenMapAndFocusActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsyncEditorOpenMapAndFocusActor"),
            &raw mut U_ASYNC_EDITOR_OPEN_MAP_AND_FOCUS_ACTOR_ASYNC_EDITOR_OPEN_MAP_AND_FOCUS_ACTOR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorUtilityLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SyncBrowserToFolders"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_SYNC_BROWSER_TO_FOLDERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameAsset"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_RENAME_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectionSet"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTION_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectionBounds"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTION_BOUNDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedPathViewFolderPaths"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_PATH_VIEW_FOLDER_PATHS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedFolderPaths"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_FOLDER_PATHS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedBlueprintClasses"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_BLUEPRINT_CLASSES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedAssetsOfClass"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_ASSETS_OF_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedAssets"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedAssetData"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentContentBrowserPath"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_GET_CURRENT_CONTENT_BROWSER_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentContentBrowserItemPath"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_GET_CURRENT_CONTENT_BROWSER_ITEM_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActorReference"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_GET_ACTOR_REFERENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindSourceWidgetByName"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_FIND_SOURCE_WIDGET_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToEditorUtilityWidget"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_CONVERT_TO_EDITOR_UTILITY_WIDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CastToWidgetBlueprint"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_CAST_TO_WIDGET_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSourceWidget"),
            &raw mut U_EDITOR_UTILITY_LIBRARY_ADD_SOURCE_WIDGET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorUtilitySubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterTabByID"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_UNREGISTER_TAB_BY_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TryRunClass"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_TRY_RUN_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TryRun"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_TRY_RUN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnRegisteredTabByID"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_REGISTERED_TAB_BY_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnAndRegisterTabWithIdGeneratedClass"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_WITH_ID_GENERATED_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnAndRegisterTabWithId"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_WITH_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnAndRegisterTabGeneratedClass"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_GENERATED_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnAndRegisterTabAndGetIDGeneratedClass"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_AND_GET_ID_GENERATED_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnAndRegisterTabAndGetID"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_AND_GET_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnAndRegisterTab"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReleaseInstanceOfAsset"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_RELEASE_INSTANCE_OF_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterTabAndGetIDGeneratedClass"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_REGISTER_TAB_AND_GET_ID_GENERATED_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterTabAndGetID"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_REGISTER_TAB_AND_GET_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterAndExecuteTask"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_REGISTER_AND_EXECUTE_TASK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindUtilityWidgetFromBlueprint"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_FIND_UTILITY_WIDGET_FROM_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesTabExist"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_DOES_TAB_EXIST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CloseTabByID"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_CLOSE_TAB_BY_ID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanRun"),
            &raw mut U_EDITOR_UTILITY_SUBSYSTEM_CAN_RUN,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorUtilityTask::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WasCancelRequested"),
            &raw mut U_EDITOR_UTILITY_TASK_WAS_CANCEL_REQUESTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTaskNotificationText"),
            &raw mut U_EDITOR_UTILITY_TASK_SET_TASK_NOTIFICATION_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Run"),
            &raw mut U_EDITOR_UTILITY_TASK_RUN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveCancelRequested"),
            &raw mut U_EDITOR_UTILITY_TASK_RECEIVE_CANCEL_REQUESTED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveBeginExecution"),
            &raw mut U_EDITOR_UTILITY_TASK_RECEIVE_BEGIN_EXECUTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTaskTitleOverride"),
            &raw mut U_EDITOR_UTILITY_TASK_GET_TASK_TITLE_OVERRIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FinishExecutingTask"),
            &raw mut U_EDITOR_UTILITY_TASK_FINISH_EXECUTING_TASK,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorUtilityWidget::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Run"),
            &raw mut U_EDITOR_UTILITY_WIDGET_RUN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindChildWidgetByName"),
            &raw mut U_EDITOR_UTILITY_WIDGET_FIND_CHILD_WIDGET_BY_NAME,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDEPRECATED_GlobalEditorUtilityBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetActorSelectionState"),
            &raw mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_SET_ACTOR_SELECTION_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectNothing"),
            &raw mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_SELECT_NOTHING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameAsset"),
            &raw mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_RENAME_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnDefaultActionClicked"),
            &raw mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_ON_DEFAULT_ACTION_CLICKED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectionSet"),
            &raw mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_SELECTION_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectionBounds"),
            &raw mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_SELECTION_BOUNDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedAssets"),
            &raw mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_SELECTED_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEditorUserSettings"),
            &raw mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_EDITOR_USER_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActorReference"),
            &raw mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_ACTOR_REFERENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ForEachSelectedAsset"),
            &raw mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_FOR_EACH_SELECTED_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ForEachSelectedActor"),
            &raw mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_FOR_EACH_SELECTED_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearActorSelectionSet"),
            &raw mut UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_CLEAR_ACTOR_SELECTION_SET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ADEPRECATED_PlacedEditorUtilityBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLevelViewportCameraInfo"),
            &raw mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_SET_LEVEL_VIEWPORT_CAMERA_INFO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetActorSelectionState"),
            &raw mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_SET_ACTOR_SELECTION_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectNothing"),
            &raw mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_SELECT_NOTHING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectionSet"),
            &raw mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_GET_SELECTION_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLevelViewportCameraInfo"),
            &raw mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_GET_LEVEL_VIEWPORT_CAMERA_INFO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActorReference"),
            &raw mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_GET_ACTOR_REFERENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearActorSelectionSet"),
            &raw mut ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_CLEAR_ACTOR_SELECTION_SET,
        );
    }
}
#[repr(C, align(8))]
pub struct UEditorFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UEditorFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorFunctionLibrary")
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
pub struct UEditorUtilityToolMenuEntry {
    __padding_end: [u8; 240],
}
impl UEditorUtilityToolMenuEntry {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityToolMenuEntry")
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
pub struct UEditorUtilityToolMenuSection {
    __padding_end: [u8; 64],
}
impl UEditorUtilityToolMenuSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityToolMenuSection")
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
pub struct IEditorUtilityExtension {}
#[repr(C, align(8))]
pub struct UEditorUtilityExtension {
    __padding_end: [u8; 48],
}
impl UEditorUtilityExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityExtension")
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
pub struct UEditorUtilityObject {
    __padding_end: [u8; 64],
}
impl UEditorUtilityObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityObject")
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
    pub fn run(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::U_EDITOR_UTILITY_OBJECT_RUN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_EDITOR_UTILITY_OBJECT_RUN,
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
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorActionUtility")
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
                crate::bindings::blutility::U_ACTOR_ACTION_UTILITY_GET_SUPPORTED_CLASSES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_ACTOR_ACTION_UTILITY_GET_SUPPORTED_CLASSES,
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
                crate::bindings::blutility::U_ACTOR_ACTION_UTILITY_GET_SUPPORTED_CLASS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_ACTOR_ACTION_UTILITY_GET_SUPPORTED_CLASS,
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
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetActionUtility")
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
    pub fn is_action_for_blueprints(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::U_ASSET_ACTION_UTILITY_IS_ACTION_FOR_BLUEPRINTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_ASSET_ACTION_UTILITY_IS_ACTION_FOR_BLUEPRINTS,
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
                crate::bindings::blutility::U_ASSET_ACTION_UTILITY_GET_SUPPORTED_CLASSES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_ASSET_ACTION_UTILITY_GET_SUPPORTED_CLASSES,
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
                crate::bindings::blutility::U_ASSET_ACTION_UTILITY_GET_SUPPORTED_CLASS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_ASSET_ACTION_UTILITY_GET_SUPPORTED_CLASS,
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
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncCaptureScene")
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
                crate::bindings::blutility::U_ASYNC_CAPTURE_SCENE_CAPTURE_SCENE_WITH_WARMUP_ASYNC,
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
                crate::bindings::blutility::U_ASYNC_CAPTURE_SCENE_CAPTURE_SCENE_WITH_WARMUP_ASYNC,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_ASYNC_CAPTURE_SCENE_CAPTURE_SCENE_ASYNC,
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
                crate::bindings::blutility::U_ASYNC_CAPTURE_SCENE_CAPTURE_SCENE_ASYNC,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UAsyncCaptureScene>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAsyncImageExport {
    __padding_end: [u8; 112],
}
impl UAsyncImageExport {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncImageExport")
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
                crate::bindings::blutility::U_ASYNC_IMAGE_EXPORT_EXPORT_IMAGE_ASYNC,
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
                crate::bindings::blutility::U_ASYNC_IMAGE_EXPORT_EXPORT_IMAGE_ASYNC,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UAsyncImageExport>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAsyncRegisterAndExecuteTask {
    __padding_end: [u8; 80],
}
impl UAsyncRegisterAndExecuteTask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncRegisterAndExecuteTask")
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
                crate::bindings::blutility::U_ASYNC_REGISTER_AND_EXECUTE_TASK_REGISTER_AND_EXECUTE_TASK,
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
                crate::bindings::blutility::U_ASYNC_REGISTER_AND_EXECUTE_TASK_REGISTER_AND_EXECUTE_TASK,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UAsyncRegisterAndExecuteTask>>().read() }
    }
}
#[repr(C, align(8))]
pub struct AEditorUtilityActor {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub b_receives_editor_input: bool,
}
impl AEditorUtilityActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AEditorUtilityActor")
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
    pub fn set_receives_editor_input(&mut self, b_in_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::A_EDITOR_UTILITY_ACTOR_SET_RECEIVES_EDITOR_INPUT,
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
                crate::bindings::blutility::A_EDITOR_UTILITY_ACTOR_SET_RECEIVES_EDITOR_INPUT,
                __buffer,
            )
        };
    }
    pub fn run(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::A_EDITOR_UTILITY_ACTOR_RUN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::A_EDITOR_UTILITY_ACTOR_RUN,
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
                crate::bindings::blutility::A_EDITOR_UTILITY_ACTOR_GET_RECEIVES_EDITOR_INPUT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::A_EDITOR_UTILITY_ACTOR_GET_RECEIVES_EDITOR_INPUT,
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
                crate::bindings::blutility::A_EDITOR_UTILITY_ACTOR_GET_INPUT_COMPONENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::A_EDITOR_UTILITY_ACTOR_GET_INPUT_COMPONENT,
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
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityActorComponent")
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
pub struct UEditorUtilityBlueprint {
    __padding_end: [u8; 1432],
}
impl UEditorUtilityBlueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityBlueprint")
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
pub struct UEditorUtilityBlueprintFactory {
    __padding_end: [u8; 160],
}
impl UEditorUtilityBlueprintFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityBlueprintFactory")
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
#[repr(C, align(16))]
pub struct AEditorUtilityCamera {
    __padding_end: [u8; 3136],
}
impl AEditorUtilityCamera {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AEditorUtilityCamera")
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
pub struct UEditorUtilityBlueprintAsyncActionBase {
    __padding_end: [u8; 56],
}
impl UEditorUtilityBlueprintAsyncActionBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityBlueprintAsyncActionBase")
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
pub struct UAsyncEditorDelay {
    __padding_end: [u8; 96],
}
impl UAsyncEditorDelay {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncEditorDelay")
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
                crate::bindings::blutility::U_ASYNC_EDITOR_DELAY_ASYNC_EDITOR_DELAY,
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
                crate::bindings::blutility::U_ASYNC_EDITOR_DELAY_ASYNC_EDITOR_DELAY,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UAsyncEditorDelay>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAsyncEditorWaitForGameWorld {
    __padding_end: [u8; 88],
}
impl UAsyncEditorWaitForGameWorld {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncEditorWaitForGameWorld")
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
                crate::bindings::blutility::U_ASYNC_EDITOR_WAIT_FOR_GAME_WORLD_ASYNC_WAIT_FOR_GAME_WORLD,
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
                crate::bindings::blutility::U_ASYNC_EDITOR_WAIT_FOR_GAME_WORLD_ASYNC_WAIT_FOR_GAME_WORLD,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UAsyncEditorWaitForGameWorld>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAsyncEditorOpenMapAndFocusActor {
    __padding_end: [u8; 136],
}
impl UAsyncEditorOpenMapAndFocusActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncEditorOpenMapAndFocusActor")
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
                crate::bindings::blutility::U_ASYNC_EDITOR_OPEN_MAP_AND_FOCUS_ACTOR_ASYNC_EDITOR_OPEN_MAP_AND_FOCUS_ACTOR,
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
                crate::bindings::blutility::U_ASYNC_EDITOR_OPEN_MAP_AND_FOCUS_ACTOR_ASYNC_EDITOR_OPEN_MAP_AND_FOCUS_ACTOR,
                __buffer,
            )
        };
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
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityLibrary")
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
    pub fn sync_browser_to_folders(folder_list: &TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_SYNC_BROWSER_TO_FOLDERS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_SYNC_BROWSER_TO_FOLDERS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_RENAME_ASSET,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_RENAME_ASSET,
                __buffer,
            )
        };
    }
    pub fn get_selection_set() -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTION_SET,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTION_SET,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTION_BOUNDS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTION_BOUNDS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_PATH_VIEW_FOLDER_PATHS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_PATH_VIEW_FOLDER_PATHS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_FOLDER_PATHS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_FOLDER_PATHS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_BLUEPRINT_CLASSES,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_BLUEPRINT_CLASSES,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_ASSETS_OF_CLASS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_ASSETS_OF_CLASS,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_ASSETS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_ASSETS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_ASSET_DATA,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_SELECTED_ASSET_DATA,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_CURRENT_CONTENT_BROWSER_PATH,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_CURRENT_CONTENT_BROWSER_PATH,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_CURRENT_CONTENT_BROWSER_ITEM_PATH,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blutility::UEditorUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_CURRENT_CONTENT_BROWSER_ITEM_PATH,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_ACTOR_REFERENCE,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_GET_ACTOR_REFERENCE,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_FIND_SOURCE_WIDGET_BY_NAME,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_FIND_SOURCE_WIDGET_BY_NAME,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_CONVERT_TO_EDITOR_UTILITY_WIDGET,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_CONVERT_TO_EDITOR_UTILITY_WIDGET,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_CAST_TO_WIDGET_BLUEPRINT,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_CAST_TO_WIDGET_BLUEPRINT,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_ADD_SOURCE_WIDGET,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_LIBRARY_ADD_SOURCE_WIDGET,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<crate::bindings::umg::UWidget>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UEditorUtilitySubsystem {
    __padding_end: [u8; 688],
}
impl UEditorUtilitySubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilitySubsystem")
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
    pub fn unregister_tab_by_id(&mut self, tab_id: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_UNREGISTER_TAB_BY_ID,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_UNREGISTER_TAB_BY_ID,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_TRY_RUN_CLASS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_TRY_RUN_CLASS,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_TRY_RUN,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_TRY_RUN,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_REGISTERED_TAB_BY_ID,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_REGISTERED_TAB_BY_ID,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_WITH_ID_GENERATED_CLASS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_WITH_ID_GENERATED_CLASS,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_WITH_ID,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_WITH_ID,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_GENERATED_CLASS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_GENERATED_CLASS,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_AND_GET_ID_GENERATED_CLASS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_AND_GET_ID_GENERATED_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FName>().swap(new_tab_id);
        }
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_AND_GET_ID,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB_AND_GET_ID,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FName>().swap(new_tab_id);
        }
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_SPAWN_AND_REGISTER_TAB,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_RELEASE_INSTANCE_OF_ASSET,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_RELEASE_INSTANCE_OF_ASSET,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_REGISTER_TAB_AND_GET_ID_GENERATED_CLASS,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_REGISTER_TAB_AND_GET_ID_GENERATED_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FName>().swap(new_tab_id);
        }
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_REGISTER_TAB_AND_GET_ID,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_REGISTER_TAB_AND_GET_ID,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FName>().swap(new_tab_id);
        }
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_REGISTER_AND_EXECUTE_TASK,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_REGISTER_AND_EXECUTE_TASK,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_FIND_UTILITY_WIDGET_FROM_BLUEPRINT,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_FIND_UTILITY_WIDGET_FROM_BLUEPRINT,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_DOES_TAB_EXIST,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_DOES_TAB_EXIST,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_CLOSE_TAB_BY_ID,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_CLOSE_TAB_BY_ID,
                __buffer,
            )
        };
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_CAN_RUN,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_SUBSYSTEM_CAN_RUN,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEditorUtilityTask {
    __padding_end: [u8; 128],
}
impl UEditorUtilityTask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityTask")
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
    pub fn was_cancel_requested(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::U_EDITOR_UTILITY_TASK_WAS_CANCEL_REQUESTED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_EDITOR_UTILITY_TASK_WAS_CANCEL_REQUESTED,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_TASK_SET_TASK_NOTIFICATION_TEXT,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_TASK_SET_TASK_NOTIFICATION_TEXT,
                __buffer,
            )
        };
    }
    pub fn finish_executing_task(&mut self, b_success: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::U_EDITOR_UTILITY_TASK_FINISH_EXECUTING_TASK,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_TASK_FINISH_EXECUTING_TASK,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UEditorUtilityWidget {
    #[doc(hidden)]
    __padding_1296: [u8; 1296],
    pub tab_display_name: FText,
    pub help_text: FString,
    #[doc(hidden)]
    __padding_1329: [u8; 1],
    pub b_auto_run_default_action: bool,
}
impl UEditorUtilityWidget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityWidget")
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
    pub fn run(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::U_EDITOR_UTILITY_WIDGET_RUN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::U_EDITOR_UTILITY_WIDGET_RUN,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_WIDGET_FIND_CHILD_WIDGET_BY_NAME,
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
                crate::bindings::blutility::U_EDITOR_UTILITY_WIDGET_FIND_CHILD_WIDGET_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<crate::bindings::umg::UWidget>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEditorUtilityWidgetBlueprint {
    __padding_end: [u8; 1672],
}
impl UEditorUtilityWidgetBlueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityWidgetBlueprint")
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
pub struct UEditorUtilityWidgetBlueprintFactory {
    __padding_end: [u8; 160],
}
impl UEditorUtilityWidgetBlueprintFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityWidgetBlueprintFactory")
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
#[repr(C, align(16))]
pub struct UEditorUtilityButton {
    __padding_end: [u8; 2160],
}
impl UEditorUtilityButton {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityButton")
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
#[repr(C, align(16))]
pub struct UEditorUtilityCheckBox {
    __padding_end: [u8; 3664],
}
impl UEditorUtilityCheckBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityCheckBox")
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
#[repr(C, align(16))]
pub struct UEditorUtilityCircularThrobber {
    __padding_end: [u8; 960],
}
impl UEditorUtilityCircularThrobber {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityCircularThrobber")
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
#[repr(C, align(16))]
pub struct UEditorUtilityComboBoxKey {
    __padding_end: [u8; 8000],
}
impl UEditorUtilityComboBoxKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityComboBoxKey")
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
#[repr(C, align(16))]
pub struct UEditorUtilityComboBoxString {
    __padding_end: [u8; 8128],
}
impl UEditorUtilityComboBoxString {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityComboBoxString")
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
#[repr(C, align(16))]
pub struct UEditorUtilityEditableText {
    __padding_end: [u8; 1696],
}
impl UEditorUtilityEditableText {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityEditableText")
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
#[repr(C, align(16))]
pub struct UEditorUtilityEditableTextBox {
    __padding_end: [u8; 4768],
}
impl UEditorUtilityEditableTextBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityEditableTextBox")
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
#[repr(C, align(16))]
pub struct UEditorUtilityExpandableArea {
    __padding_end: [u8; 1488],
}
impl UEditorUtilityExpandableArea {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityExpandableArea")
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
#[repr(C, align(16))]
pub struct UEditorUtilityInputKeySelector {
    __padding_end: [u8; 2816],
}
impl UEditorUtilityInputKeySelector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityInputKeySelector")
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
#[repr(C, align(16))]
pub struct UEditorUtilityListView {
    __padding_end: [u8; 5088],
}
impl UEditorUtilityListView {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityListView")
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
#[repr(C, align(16))]
pub struct UEditorUtilityMultiLineEditableText {
    __padding_end: [u8; 1744],
}
impl UEditorUtilityMultiLineEditableText {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityMultiLineEditableText")
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
#[repr(C, align(16))]
pub struct UEditorUtilityMultiLineEditableTextBox {
    __padding_end: [u8; 5584],
}
impl UEditorUtilityMultiLineEditableTextBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityMultiLineEditableTextBox")
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
#[repr(C, align(16))]
pub struct UEditorUtilityProgressBar {
    __padding_end: [u8; 1504],
}
impl UEditorUtilityProgressBar {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityProgressBar")
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
#[repr(C, align(16))]
pub struct UEditorUtilityScrollBar {
    __padding_end: [u8; 2672],
}
impl UEditorUtilityScrollBar {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityScrollBar")
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
#[repr(C, align(16))]
pub struct UEditorUtilityScrollBox {
    __padding_end: [u8; 3760],
}
impl UEditorUtilityScrollBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityScrollBox")
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
#[repr(C, align(16))]
pub struct UEditorUtilitySlider {
    __padding_end: [u8; 2224],
}
impl UEditorUtilitySlider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilitySlider")
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
#[repr(C, align(16))]
pub struct UEditorUtilitySpinBox {
    __padding_end: [u8; 2592],
}
impl UEditorUtilitySpinBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilitySpinBox")
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
#[repr(C, align(16))]
pub struct UEditorUtilityThrobber {
    __padding_end: [u8; 928],
}
impl UEditorUtilityThrobber {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityThrobber")
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
#[repr(C, align(16))]
pub struct UEditorUtilityTreeView {
    __padding_end: [u8; 5216],
}
impl UEditorUtilityTreeView {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityTreeView")
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
pub struct UEditorUtilityWidgetProjectSettings {
    __padding_end: [u8; 1176],
}
impl UEditorUtilityWidgetProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorUtilityWidgetProjectSettings")
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
pub struct UDEPRECATED_GlobalEditorUtilityBase {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub help_text: FString,
    #[doc(hidden)]
    __padding_65: [u8; 1],
    pub b_auto_run_default_action: bool,
    __padding_end: [u8; 54],
}
impl UDEPRECATED_GlobalEditorUtilityBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_GlobalEditorUtilityBase")
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_SET_ACTOR_SELECTION_STATE,
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_SET_ACTOR_SELECTION_STATE,
                __buffer,
            )
        };
    }
    pub fn select_nothing(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_SELECT_NOTHING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_SELECT_NOTHING,
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_RENAME_ASSET,
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_RENAME_ASSET,
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_SELECTION_SET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_SELECTION_SET,
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_SELECTION_BOUNDS,
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_SELECTION_BOUNDS,
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_SELECTED_ASSETS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_SELECTED_ASSETS,
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_EDITOR_USER_SETTINGS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_EDITOR_USER_SETTINGS,
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_ACTOR_REFERENCE,
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_GET_ACTOR_REFERENCE,
                __buffer,
            )
        };
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_FOR_EACH_SELECTED_ASSET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_FOR_EACH_SELECTED_ASSET,
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_FOR_EACH_SELECTED_ACTOR,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_FOR_EACH_SELECTED_ACTOR,
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
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_CLEAR_ACTOR_SELECTION_SET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::UDEPRECATED_GLOBAL_EDITOR_UTILITY_BASE_CLEAR_ACTOR_SELECTION_SET,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct ADEPRECATED_PlacedEditorUtilityBase {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub help_text: FString,
}
impl ADEPRECATED_PlacedEditorUtilityBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADEPRECATED_PlacedEditorUtilityBase")
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
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_SET_LEVEL_VIEWPORT_CAMERA_INFO,
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
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_SET_LEVEL_VIEWPORT_CAMERA_INFO,
                __buffer,
            )
        };
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
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_SET_ACTOR_SELECTION_STATE,
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
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_SET_ACTOR_SELECTION_STATE,
                __buffer,
            )
        };
    }
    pub fn select_nothing(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_SELECT_NOTHING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_SELECT_NOTHING,
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
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_GET_SELECTION_SET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_GET_SELECTION_SET,
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
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_GET_LEVEL_VIEWPORT_CAMERA_INFO,
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
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_GET_LEVEL_VIEWPORT_CAMERA_INFO,
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
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_GET_ACTOR_REFERENCE,
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
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_GET_ACTOR_REFERENCE,
                __buffer,
            )
        };
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
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_CLEAR_ACTOR_SELECTION_SET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blutility::ADEPRECATED_PLACED_EDITOR_UTILITY_BASE_CLEAR_ACTOR_SELECTION_SET,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UToolMenuWidget {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub menu_name: FString,
    pub menu_type: crate::bindings::slate::EMultiBoxType,
    #[doc(hidden)]
    __padding_736: [u8; 20],
    pub full_menu_name: FName,
}
impl UToolMenuWidget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuWidget")
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
