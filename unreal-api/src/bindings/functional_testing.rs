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
pub static mut U_AUTOMATION_EDITOR_TASK_IS_VALID_TASK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_EDITOR_TASK_IS_TASK_DONE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_HIGH_RES_SCREENSHOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_AUTOMATION_SCREENSHOT_OF_UI: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_AUTOMATION_SCREENSHOT_AT_CAMERA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_AUTOMATION_SCREENSHOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_TEST_TELEMETRY_STORAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_SCALABILITY_QUALITY_TO_LOW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_SCALABILITY_QUALITY_TO_EPIC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_SCALABILITY_QUALITY_LEVEL_RELATIVE_TO_MAX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_VIEWPORT_VISUALIZE_BUFFER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_VIEWPORT_VIEW_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_ACTIVE_VIEWPORT_WIREFRAME_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_ACTIVE_VIEWPORT_VIEW_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_INC_MAX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_INC_AVERAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_EXC_MAX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_EXC_AVERAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_CALL_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_EDITOR_ACTIVE_VIEWPORT_WIREFRAME_OPACITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_EDITOR_ACTIVE_VIEWPORT_VIEW_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_DEFAULT_SCREENSHOT_OPTIONS_FOR_RENDERING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_DEFAULT_SCREENSHOT_OPTIONS_FOR_GAMEPLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_FINISH_LOADING_BEFORE_SCREENSHOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ENABLE_STAT_GROUP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_DISABLE_STAT_GROUP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_COMPARE_IMAGE_AGAINST_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_AUTOMATION_WAIT_FOR_LOADING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ARE_AUTOMATED_TESTS_RUNNING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_WARNING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_TELEMETRY_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_INFO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_ERROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_PLAIN_LOG_MESSAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_PLAIN_LOG_ERROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_LOG_MESSAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_LOG_ERROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_SET_TIME_LIMIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE_FROM_INTEGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE_FROM_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE_FROM_BOOLEAN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_REGISTER_AUTO_DESTROY_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_RECEIVE_TEST_FINISHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_RECEIVE_START_TEST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_RECEIVE_PREPARE_TEST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ON_WANTS_RE_RUN_CHECK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ON_ADDITIONAL_TEST_FINISHED_MESSAGE_REQUEST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_LOG_MESSAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_IS_RUNNING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_IS_READY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_IS_ENABLED_IN_WORLD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_IS_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_GET_CURRENT_RERUN_REASON: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_FINISH_TEST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_EDIT_TAGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_DEBUG_GATHER_RELEVANT_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_VALUE_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_VALUE_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_VALUE_DOUBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_VALUE_DATE_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_TRUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_VECTOR4: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_VECTOR2_D: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_ROTATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_QUAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_PLANE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_MATRIX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_BOX2_D: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_IS_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_FALSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_VECTOR4: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_VECTOR2_D: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_TRACE_QUERY_RESULTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_ROTATOR_ORIENTATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_ROTATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_QUAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_PLANE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_MATRIX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_DOUBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_BOX2_D: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ADD_WARNING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ADD_RERUN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ADD_INFO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_TEST_ADD_ERROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_FUNCTIONAL_AI_TEST_BASE_IS_ONE_OF_SPAWNED_PAWNS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_WRITE_LOG_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_TRIGGER_GPU_TRACE_IF_RECORD_FALLS_BELOW_BUDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_STOP_CPU_PROFILING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_START_CPU_PROFILING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_SAMPLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_ON_BEGIN_TESTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_ON_ALL_TESTS_COMPLETE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_IS_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_IS_CURRENT_RECORD_WITHIN_RENDER_THREAD_BUDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_IS_CURRENT_RECORD_WITHIN_GPU_BUDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_IS_CURRENT_RECORD_WITHIN_GAME_THREAD_BUDGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_END_STATS_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_END_RECORDING_BASELINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_END_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_BEGIN_STATS_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_BEGIN_RECORDING_BASELINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_AUTOMATION_PERFORMACE_HELPER_BEGIN_RECORDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FUNCTIONAL_TESTING_MANAGER_RUN_ALL_FUNCTIONAL_TESTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_PHASED_AUTOMATION_ACTOR_BASE_ON_FUNCTIONAL_TESTING_COMPLETE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_PHASED_AUTOMATION_ACTOR_BASE_ON_FUNCTIONAL_TESTING_BEGIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FUNCTIONAL_TEST_UTILITY_LIBRARY_TRACE_CHANNEL_TEST_UTIL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GROUND_TRUTH_DATA_SAVE_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GROUND_TRUTH_DATA_RESET_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GROUND_TRUTH_DATA_LOAD_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GROUND_TRUTH_DATA_CAN_MODIFY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TRACE_QUERY_TEST_RESULTS_TO_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAutomationEditorTask::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidTask"),
            &raw mut U_AUTOMATION_EDITOR_TASK_IS_VALID_TASK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTaskDone"),
            &raw mut U_AUTOMATION_EDITOR_TASK_IS_TASK_DONE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAutomationBlueprintFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TakeHighResScreenshot"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_HIGH_RES_SCREENSHOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TakeAutomationScreenshotOfUI"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_AUTOMATION_SCREENSHOT_OF_UI,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TakeAutomationScreenshotAtCamera"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_AUTOMATION_SCREENSHOT_AT_CAMERA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TakeAutomationScreenshot"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_AUTOMATION_SCREENSHOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTestTelemetryStorage"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_TEST_TELEMETRY_STORAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScalabilityQualityToLow"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_SCALABILITY_QUALITY_TO_LOW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScalabilityQualityToEpic"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_SCALABILITY_QUALITY_TO_EPIC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScalabilityQualityLevelRelativeToMax"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_SCALABILITY_QUALITY_LEVEL_RELATIVE_TO_MAX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEditorViewportVisualizeBuffer"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_VIEWPORT_VISUALIZE_BUFFER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEditorViewportViewMode"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_VIEWPORT_VIEW_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEditorActiveViewportWireframeOpacity"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_ACTIVE_VIEWPORT_WIREFRAME_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEditorActiveViewportViewMode"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_ACTIVE_VIEWPORT_VIEW_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatIncMax"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_INC_MAX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatIncAverage"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_INC_AVERAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatExcMax"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_EXC_MAX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatExcAverage"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_EXC_AVERAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatCallCount"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_CALL_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEditorActiveViewportWireframeOpacity"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_EDITOR_ACTIVE_VIEWPORT_WIREFRAME_OPACITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEditorActiveViewportViewMode"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_EDITOR_ACTIVE_VIEWPORT_VIEW_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultScreenshotOptionsForRendering"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_DEFAULT_SCREENSHOT_OPTIONS_FOR_RENDERING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultScreenshotOptionsForGameplay"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_DEFAULT_SCREENSHOT_OPTIONS_FOR_GAMEPLAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FinishLoadingBeforeScreenshot"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_FINISH_LOADING_BEFORE_SCREENSHOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnableStatGroup"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ENABLE_STAT_GROUP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisableStatGroup"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_DISABLE_STAT_GROUP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CompareImageAgainstReference"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_COMPARE_IMAGE_AGAINST_REFERENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AutomationWaitForLoading"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_AUTOMATION_WAIT_FOR_LOADING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AreAutomatedTestsRunning"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ARE_AUTOMATED_TESTS_RUNNING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTestWarning"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_WARNING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTestTelemetryData"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_TELEMETRY_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTestInfo"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_INFO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTestError"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_ERROR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddExpectedPlainLogMessage"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_PLAIN_LOG_MESSAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddExpectedPlainLogError"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_PLAIN_LOG_ERROR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddExpectedLogMessage"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_LOG_MESSAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddExpectedLogError"),
            &raw mut U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_LOG_ERROR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AFunctionalTest::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTimeLimit"),
            &raw mut A_FUNCTIONAL_TEST_SET_TIME_LIMIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConsoleVariableFromInteger"),
            &raw mut A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE_FROM_INTEGER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConsoleVariableFromFloat"),
            &raw mut A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE_FROM_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConsoleVariableFromBoolean"),
            &raw mut A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE_FROM_BOOLEAN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConsoleVariable"),
            &raw mut A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterAutoDestroyActor"),
            &raw mut A_FUNCTIONAL_TEST_REGISTER_AUTO_DESTROY_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTestFinished"),
            &raw mut A_FUNCTIONAL_TEST_RECEIVE_TEST_FINISHED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveStartTest"),
            &raw mut A_FUNCTIONAL_TEST_RECEIVE_START_TEST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceivePrepareTest"),
            &raw mut A_FUNCTIONAL_TEST_RECEIVE_PREPARE_TEST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnWantsReRunCheck"),
            &raw mut A_FUNCTIONAL_TEST_ON_WANTS_RE_RUN_CHECK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAdditionalTestFinishedMessageRequest"),
            &raw mut A_FUNCTIONAL_TEST_ON_ADDITIONAL_TEST_FINISHED_MESSAGE_REQUEST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LogMessage"),
            &raw mut A_FUNCTIONAL_TEST_LOG_MESSAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRunning"),
            &raw mut A_FUNCTIONAL_TEST_IS_RUNNING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsReady"),
            &raw mut A_FUNCTIONAL_TEST_IS_READY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEnabledInWorld"),
            &raw mut A_FUNCTIONAL_TEST_IS_ENABLED_IN_WORLD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEnabled"),
            &raw mut A_FUNCTIONAL_TEST_IS_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentRerunReason"),
            &raw mut A_FUNCTIONAL_TEST_GET_CURRENT_RERUN_REASON,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FinishTest"),
            &raw mut A_FUNCTIONAL_TEST_FINISH_TEST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditTags"),
            &raw mut A_FUNCTIONAL_TEST_EDIT_TAGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DebugGatherRelevantActors"),
            &raw mut A_FUNCTIONAL_TEST_DEBUG_GATHER_RELEVANT_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertValue_Int"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_VALUE_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertValue_Float"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_VALUE_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertValue_Double"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_VALUE_DOUBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertValue_DateTime"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_VALUE_DATE_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertTrue"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_TRUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Vector4"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_VECTOR4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Vector2D"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_VECTOR2_D,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Vector"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_VECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Transform"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_String"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Rotator"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_ROTATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Quat"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_QUAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Plane"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_PLANE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Matrix"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_MATRIX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Box2D"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_BOX2_D,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertIsValid"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_IS_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertFalse"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_FALSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Vector4"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_VECTOR4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Vector2D"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_VECTOR2_D,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Vector"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_VECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Transform"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_TraceQueryResults"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_TRACE_QUERY_RESULTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_String"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_RotatorOrientation"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_ROTATOR_ORIENTATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Rotator"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_ROTATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Quat"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_QUAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Plane"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_PLANE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Object"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Name"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Matrix"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_MATRIX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Int"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Float"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Double"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_DOUBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Box2D"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_BOX2_D,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Bool"),
            &raw mut A_FUNCTIONAL_TEST_ASSERT_EQUAL_BOOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddWarning"),
            &raw mut A_FUNCTIONAL_TEST_ADD_WARNING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddRerun"),
            &raw mut A_FUNCTIONAL_TEST_ADD_RERUN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInfo"),
            &raw mut A_FUNCTIONAL_TEST_ADD_INFO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddError"),
            &raw mut A_FUNCTIONAL_TEST_ADD_ERROR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AFunctionalAITestBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsOneOfSpawnedPawns"),
            &raw mut A_FUNCTIONAL_AI_TEST_BASE_IS_ONE_OF_SPAWNED_PAWNS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAutomationPerformaceHelper::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteLogFile"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_WRITE_LOG_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TriggerGPUTraceIfRecordFallsBelowBudget"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_TRIGGER_GPU_TRACE_IF_RECORD_FALLS_BELOW_BUDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Tick"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_TICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopCPUProfiling"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_STOP_CPU_PROFILING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartCPUProfiling"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_START_CPU_PROFILING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Sample"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_SAMPLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnBeginTests"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_ON_BEGIN_TESTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAllTestsComplete"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_ON_ALL_TESTS_COMPLETE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRecording"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_IS_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCurrentRecordWithinRenderThreadBudget"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_IS_CURRENT_RECORD_WITHIN_RENDER_THREAD_BUDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCurrentRecordWithinGPUBudget"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_IS_CURRENT_RECORD_WITHIN_GPU_BUDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCurrentRecordWithinGameThreadBudget"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_IS_CURRENT_RECORD_WITHIN_GAME_THREAD_BUDGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EndStatsFile"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_END_STATS_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EndRecordingBaseline"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_END_RECORDING_BASELINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EndRecording"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_END_RECORDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BeginStatsFile"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_BEGIN_STATS_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BeginRecordingBaseline"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_BEGIN_RECORDING_BASELINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BeginRecording"),
            &raw mut U_AUTOMATION_PERFORMACE_HELPER_BEGIN_RECORDING,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFunctionalTestingManager::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RunAllFunctionalTests"),
            &raw mut U_FUNCTIONAL_TESTING_MANAGER_RUN_ALL_FUNCTIONAL_TESTS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = APhasedAutomationActorBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnFunctionalTestingComplete"),
            &raw mut A_PHASED_AUTOMATION_ACTOR_BASE_ON_FUNCTIONAL_TESTING_COMPLETE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnFunctionalTestingBegin"),
            &raw mut A_PHASED_AUTOMATION_ACTOR_BASE_ON_FUNCTIONAL_TESTING_BEGIN,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFunctionalTestUtilityLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TraceChannelTestUtil"),
            &raw mut U_FUNCTIONAL_TEST_UTILITY_LIBRARY_TRACE_CHANNEL_TEST_UTIL,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGroundTruthData::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveObject"),
            &raw mut U_GROUND_TRUTH_DATA_SAVE_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetObject"),
            &raw mut U_GROUND_TRUTH_DATA_RESET_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadObject"),
            &raw mut U_GROUND_TRUTH_DATA_LOAD_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanModify"),
            &raw mut U_GROUND_TRUTH_DATA_CAN_MODIFY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTraceQueryTestResults::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ToString"),
            &raw mut U_TRACE_QUERY_TEST_RESULTS_TO_STRING,
        );
    }
}
#[repr(C, align(1))]
pub struct FComparisonToleranceAmount {
    __padding_end: [u8; 6],
}
impl FComparisonToleranceAmount {}
#[repr(C, align(8))]
pub struct FAutomationScreenshotOptions {
    pub resolution: crate::bindings::core_u_object::FVector2D,
    pub delay: f32,
    pub frame_delay: i32,
    pub b_override_override_time_to: bool,
    pub override_time_to: f64,
    pub b_disable_noisy_rendering_features: bool,
    pub b_disable_tonemapping: bool,
    pub view_settings: UPtr<UAutomationViewSettings>,
    pub visualize_buffer: FName,
    pub tolerance: EComparisonTolerance,
    pub tolerance_amount: FComparisonToleranceAmount,
    pub maximum_local_error: f32,
    pub maximum_global_error: f32,
    pub b_ignore_anti_aliasing: bool,
    pub b_ignore_colors: bool,
    pub b_allow_emulating_splitscreen: bool,
}
impl FAutomationScreenshotOptions {}
#[repr(C, align(1))]
pub struct FAutomationWaitForLoadingOptions {
    pub wait_for_replication_to_settle: bool,
}
impl FAutomationWaitForLoadingOptions {}
#[repr(C, align(8))]
pub struct FAITestSpawnInfoBase {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub spawn_location: UPtr<crate::bindings::engine::AActor>,
    pub number_to_spawn: i32,
    pub spawn_delay: f32,
    pub pre_spawn_delay: f32,
    __padding_end: [u8; 12],
}
impl FAITestSpawnInfoBase {}
#[repr(C, align(8))]
pub struct FAITestSpawnInfo {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub pawn_class: TSubclassOf<crate::bindings::engine::APawn>,
    pub controller_class: TSubclassOf<crate::bindings::ai_module::AAIController>,
    pub team_id: crate::bindings::ai_module::FGenericTeamId,
    pub behavior_tree: UPtr<crate::bindings::ai_module::UBehaviorTree>,
}
impl FAITestSpawnInfo {}
#[repr(C, align(4))]
pub struct FPendingDelayedSpawn {
    __padding_end: [u8; 20],
}
impl FPendingDelayedSpawn {}
#[repr(C, align(8))]
pub struct FAITestSpawnSetBase {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub name: FName,
    pub flags_20: u8,
    pub fallback_spawn_location: UPtr<crate::bindings::engine::AActor>,
}
impl FAITestSpawnSetBase {}
#[repr(C, align(8))]
pub struct FAITestSpawnSet {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub spawn_info_container: TArray<FAITestSpawnInfo>,
}
impl FAITestSpawnSet {}
#[repr(C, align(1))]
pub struct FTraceChannelTestBatchOptions {
    pub b_line_trace: bool,
    pub b_sphere_trace: bool,
    pub b_capsule_trace: bool,
    pub b_box_trace: bool,
    pub b_channel_trace: bool,
    pub b_objects_trace: bool,
    pub b_profile_trace: bool,
}
impl FTraceChannelTestBatchOptions {}
#[repr(C, align(4))]
pub struct FTraceQueryTestNames {
    pub component_name: FName,
    pub physical_material_name: FName,
    pub actor_name: FName,
}
impl FTraceQueryTestNames {}
#[repr(C, align(8))]
pub struct FTraceQueryTestResultsInnerMost {
    pub single_hit: crate::bindings::engine::FHitResult,
    pub single_names: FTraceQueryTestNames,
    pub b_single_result: bool,
    pub multi_hits: TArray<crate::bindings::engine::FHitResult>,
    pub multi_names: TArray<FTraceQueryTestNames>,
    pub b_multi_result: bool,
}
impl FTraceQueryTestResultsInnerMost {}
#[repr(C, align(8))]
pub struct FTraceQueryTestResultsInner {
    pub line_results: FTraceQueryTestResultsInnerMost,
    pub sphere_results: FTraceQueryTestResultsInnerMost,
    pub capsule_results: FTraceQueryTestResultsInnerMost,
    pub box_results: FTraceQueryTestResultsInnerMost,
}
impl FTraceQueryTestResultsInner {}
#[repr(C, align(8))]
pub struct UAutomationViewSettings {
    __padding_end: [u8; 72],
}
impl UAutomationViewSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutomationViewSettings")
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
pub struct UAutomationEditorTask {
    __padding_end: [u8; 56],
}
impl UAutomationEditorTask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutomationEditorTask")
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
    pub fn is_valid_task(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_EDITOR_TASK_IS_VALID_TASK,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_EDITOR_TASK_IS_VALID_TASK,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_task_done(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_EDITOR_TASK_IS_TASK_DONE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_EDITOR_TASK_IS_TASK_DONE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAutomationBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UAutomationBlueprintFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutomationBlueprintFunctionLibrary")
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
    pub fn take_high_res_screenshot(
        res_x: i32,
        res_y: i32,
        filename: FString,
        camera: UPtr<crate::bindings::engine::ACameraActor>,
        b_mask_enabled: bool,
        b_capture_hdr: bool,
        comparison_tolerance: EComparisonTolerance,
        comparison_notes: FString,
        delay: f32,
        b_force_game_view: bool,
    ) -> UPtr<UAutomationEditorTask> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_HIGH_RES_SCREENSHOT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&res_x, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&res_y, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filename,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::ACameraActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_mask_enabled,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_capture_hdr,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &comparison_tolerance,
                __buffer.add(34).cast::<EComparisonTolerance>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &comparison_notes,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&delay, __buffer.add(56).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_game_view,
                __buffer.add(60).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_HIGH_RES_SCREENSHOT,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<UPtr<UAutomationEditorTask>>().read() }
    }
    pub fn take_automation_screenshot_of_ui(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        latent_info: crate::bindings::engine::FLatentActionInfo,
        name: FString,
        options: &FAutomationScreenshotOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_AUTOMATION_SCREENSHOT_OF_UI,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &latent_info,
                __buffer.add(8).cast::<crate::bindings::engine::FLatentActionInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(40).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(56).cast::<FAutomationScreenshotOptions>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_AUTOMATION_SCREENSHOT_OF_UI,
                __buffer,
            )
        };
    }
    pub fn take_automation_screenshot_at_camera(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        latent_info: crate::bindings::engine::FLatentActionInfo,
        camera: UPtr<crate::bindings::engine::ACameraActor>,
        name_override: FString,
        notes: FString,
        options: &FAutomationScreenshotOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_AUTOMATION_SCREENSHOT_AT_CAMERA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &latent_info,
                __buffer.add(8).cast::<crate::bindings::engine::FLatentActionInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera,
                __buffer.add(40).cast::<UPtr<crate::bindings::engine::ACameraActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &name_override,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&notes, __buffer.add(64).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(80).cast::<FAutomationScreenshotOptions>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_AUTOMATION_SCREENSHOT_AT_CAMERA,
                __buffer,
            )
        };
    }
    pub fn take_automation_screenshot(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        latent_info: crate::bindings::engine::FLatentActionInfo,
        name: FString,
        notes: FString,
        options: &FAutomationScreenshotOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_AUTOMATION_SCREENSHOT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &latent_info,
                __buffer.add(8).cast::<crate::bindings::engine::FLatentActionInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(40).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&notes, __buffer.add(56).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(72).cast::<FAutomationScreenshotOptions>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_TAKE_AUTOMATION_SCREENSHOT,
                __buffer,
            )
        };
    }
    pub fn set_test_telemetry_storage(storage_name: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_TEST_TELEMETRY_STORAGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &storage_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_TEST_TELEMETRY_STORAGE,
                __buffer,
            )
        };
    }
    pub fn set_scalability_quality_to_low(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_SCALABILITY_QUALITY_TO_LOW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_SCALABILITY_QUALITY_TO_LOW,
                __buffer,
            )
        };
    }
    pub fn set_scalability_quality_to_epic(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_SCALABILITY_QUALITY_TO_EPIC,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_SCALABILITY_QUALITY_TO_EPIC,
                __buffer,
            )
        };
    }
    pub fn set_scalability_quality_level_relative_to_max(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        value: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_SCALABILITY_QUALITY_LEVEL_RELATIVE_TO_MAX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_SCALABILITY_QUALITY_LEVEL_RELATIVE_TO_MAX,
                __buffer,
            )
        };
    }
    pub fn set_editor_viewport_visualize_buffer(buffer_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_VIEWPORT_VISUALIZE_BUFFER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &buffer_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_VIEWPORT_VISUALIZE_BUFFER,
                __buffer,
            )
        };
    }
    pub fn set_editor_viewport_view_mode(
        index: crate::bindings::engine::EViewModeIndex,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_VIEWPORT_VIEW_MODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &index,
                __buffer.add(0).cast::<crate::bindings::engine::EViewModeIndex>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_VIEWPORT_VIEW_MODE,
                __buffer,
            )
        };
    }
    pub fn set_editor_active_viewport_wireframe_opacity(opacity: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_ACTIVE_VIEWPORT_WIREFRAME_OPACITY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&opacity, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_ACTIVE_VIEWPORT_WIREFRAME_OPACITY,
                __buffer,
            )
        };
    }
    pub fn set_editor_active_viewport_view_mode(
        index: crate::bindings::engine::EViewModeIndex,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_ACTIVE_VIEWPORT_VIEW_MODE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &index,
                __buffer.add(0).cast::<crate::bindings::engine::EViewModeIndex>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_SET_EDITOR_ACTIVE_VIEWPORT_VIEW_MODE,
                __buffer,
            )
        };
    }
    pub fn get_stat_inc_max(stat_name: FName) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_INC_MAX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stat_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_INC_MAX,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<f32>().read() }
    }
    pub fn get_stat_inc_average(stat_name: FName) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_INC_AVERAGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stat_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_INC_AVERAGE,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<f32>().read() }
    }
    pub fn get_stat_exc_max(stat_name: FName) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_EXC_MAX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stat_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_EXC_MAX,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<f32>().read() }
    }
    pub fn get_stat_exc_average(stat_name: FName) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_EXC_AVERAGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stat_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_EXC_AVERAGE,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<f32>().read() }
    }
    pub fn get_stat_call_count(stat_name: FName) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_CALL_COUNT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stat_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_STAT_CALL_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<f32>().read() }
    }
    pub fn get_editor_active_viewport_wireframe_opacity() -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_EDITOR_ACTIVE_VIEWPORT_WIREFRAME_OPACITY,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_EDITOR_ACTIVE_VIEWPORT_WIREFRAME_OPACITY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_editor_active_viewport_view_mode() -> crate::bindings::engine::EViewModeIndex {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_EDITOR_ACTIVE_VIEWPORT_VIEW_MODE,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_EDITOR_ACTIVE_VIEWPORT_VIEW_MODE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::engine::EViewModeIndex>().read()
        }
    }
    pub fn get_default_screenshot_options_for_rendering(
        tolerance: EComparisonTolerance,
        delay: f32,
    ) -> FAutomationScreenshotOptions {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_DEFAULT_SCREENSHOT_OPTIONS_FOR_RENDERING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tolerance,
                __buffer.add(0).cast::<EComparisonTolerance>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&delay, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_DEFAULT_SCREENSHOT_OPTIONS_FOR_RENDERING,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FAutomationScreenshotOptions>().read() }
    }
    pub fn get_default_screenshot_options_for_gameplay(
        tolerance: EComparisonTolerance,
        delay: f32,
    ) -> FAutomationScreenshotOptions {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_DEFAULT_SCREENSHOT_OPTIONS_FOR_GAMEPLAY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tolerance,
                __buffer.add(0).cast::<EComparisonTolerance>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&delay, __buffer.add(4).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_GET_DEFAULT_SCREENSHOT_OPTIONS_FOR_GAMEPLAY,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FAutomationScreenshotOptions>().read() }
    }
    pub fn finish_loading_before_screenshot() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_FINISH_LOADING_BEFORE_SCREENSHOT,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_FINISH_LOADING_BEFORE_SCREENSHOT,
                __buffer,
            )
        };
    }
    pub fn enable_stat_group(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        group_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ENABLE_STAT_GROUP,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &group_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ENABLE_STAT_GROUP,
                __buffer,
            )
        };
    }
    pub fn disable_stat_group(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        group_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_DISABLE_STAT_GROUP,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &group_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_DISABLE_STAT_GROUP,
                __buffer,
            )
        };
    }
    pub fn compare_image_against_reference(
        image_file_path: FString,
        comparison_name: FString,
        comparison_tolerance: EComparisonTolerance,
        comparison_notes: FString,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_COMPARE_IMAGE_AGAINST_REFERENCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &image_file_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &comparison_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &comparison_tolerance,
                __buffer.add(32).cast::<EComparisonTolerance>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &comparison_notes,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(56).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_COMPARE_IMAGE_AGAINST_REFERENCE,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn automation_wait_for_loading(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        latent_info: crate::bindings::engine::FLatentActionInfo,
        options: FAutomationWaitForLoadingOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_AUTOMATION_WAIT_FOR_LOADING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &latent_info,
                __buffer.add(8).cast::<crate::bindings::engine::FLatentActionInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &options,
                __buffer.add(40).cast::<FAutomationWaitForLoadingOptions>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_AUTOMATION_WAIT_FOR_LOADING,
                __buffer,
            )
        };
    }
    pub fn are_automated_tests_running() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ARE_AUTOMATED_TESTS_RUNNING,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ARE_AUTOMATED_TESTS_RUNNING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn add_test_warning(in_log_item: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_WARNING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_log_item,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_WARNING,
                __buffer,
            )
        };
    }
    pub fn add_test_telemetry_data(
        data_point: FString,
        measurement: f32,
        context: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_TELEMETRY_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_point,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &measurement,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_TELEMETRY_DATA,
                __buffer,
            )
        };
    }
    pub fn add_test_info(in_log_item: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_INFO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_log_item,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_INFO,
                __buffer,
            )
        };
    }
    pub fn add_test_error(in_log_item: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_ERROR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_log_item,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_TEST_ERROR,
                __buffer,
            )
        };
    }
    pub fn add_expected_plain_log_message(
        expected_string: FString,
        occurrences: i32,
        exact_match: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_PLAIN_LOG_MESSAGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected_string,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &occurrences,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &exact_match,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_PLAIN_LOG_MESSAGE,
                __buffer,
            )
        };
    }
    pub fn add_expected_plain_log_error(
        expected_string: FString,
        occurrences: i32,
        exact_match: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_PLAIN_LOG_ERROR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected_string,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &occurrences,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &exact_match,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_PLAIN_LOG_ERROR,
                __buffer,
            )
        };
    }
    pub fn add_expected_log_message(
        expected_pattern_string: FString,
        occurrences: i32,
        exact_match: bool,
        is_regex: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_LOG_MESSAGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected_pattern_string,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &occurrences,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &exact_match,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&is_regex, __buffer.add(21).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_LOG_MESSAGE,
                __buffer,
            )
        };
    }
    pub fn add_expected_log_error(
        expected_pattern_string: FString,
        occurrences: i32,
        exact_match: bool,
        is_regex: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_LOG_ERROR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected_pattern_string,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &occurrences,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &exact_match,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&is_regex, __buffer.add(21).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_BLUEPRINT_FUNCTION_LIBRARY_ADD_EXPECTED_LOG_ERROR,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UFuncTestRenderingComponent {
    __padding_end: [u8; 1504],
}
impl UFuncTestRenderingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFuncTestRenderingComponent")
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
pub struct AFunctionalTest {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub test_label: FString,
    pub author: FString,
    pub description: FString,
    pub test_tags: FString,
    #[doc(hidden)]
    __padding_1224: [u8; 24],
    pub flags_1224: u8,
    #[doc(hidden)]
    __padding_1240: [u8; 15],
    pub log_error_handling: EFunctionalTestLogHandling,
    pub log_warning_handling: EFunctionalTestLogHandling,
    pub observation_point: UPtr<crate::bindings::engine::AActor>,
    pub flags_1256: u8,
    pub random_numbers_stream: crate::bindings::core_u_object::FRandomStream,
    pub result: EFunctionalTestResult,
    pub preparation_time_limit: f32,
    pub time_limit: f32,
    pub times_up_message: FText,
    #[doc(hidden)]
    __padding_1504: [u8; 208],
    pub total_time: f32,
    __padding_end: [u8; 36],
}
impl AFunctionalTest {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AFunctionalTest")
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
    pub fn set_time_limit(
        &mut self,
        new_time_limit: f32,
        result_when_time_runs_out: EFunctionalTestResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_SET_TIME_LIMIT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_time_limit,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &result_when_time_runs_out,
                __buffer.add(4).cast::<EFunctionalTestResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_SET_TIME_LIMIT,
                __buffer,
            )
        };
    }
    pub fn set_console_variable_from_integer(&mut self, name: FString, in_value: i32) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE_FROM_INTEGER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE_FROM_INTEGER,
                __buffer,
            )
        };
    }
    pub fn set_console_variable_from_float(&mut self, name: FString, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE_FROM_FLOAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE_FROM_FLOAT,
                __buffer,
            )
        };
    }
    pub fn set_console_variable_from_boolean(&mut self, name: FString, in_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE_FROM_BOOLEAN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE_FROM_BOOLEAN,
                __buffer,
            )
        };
    }
    pub fn set_console_variable(&mut self, name: FString, in_value: FString) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_SET_CONSOLE_VARIABLE,
                __buffer,
            )
        };
    }
    pub fn register_auto_destroy_actor(
        &mut self,
        actor_to_auto_destroy: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_REGISTER_AUTO_DESTROY_ACTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_to_auto_destroy,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_REGISTER_AUTO_DESTROY_ACTOR,
                __buffer,
            )
        };
    }
    pub fn log_message(&mut self, message: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_LOG_MESSAGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message,
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
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_LOG_MESSAGE,
                __buffer,
            )
        };
    }
    pub fn is_running(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_IS_RUNNING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_IS_RUNNING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_enabled_in_world(
        &self,
        world: UPtr<crate::bindings::engine::UWorld>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_IS_ENABLED_IN_WORLD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_IS_ENABLED_IN_WORLD,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_IS_ENABLED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_IS_ENABLED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_current_rerun_reason(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_GET_CURRENT_RERUN_REASON,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_GET_CURRENT_RERUN_REASON,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn finish_test(&mut self, test_result: EFunctionalTestResult, message: FString) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_FINISH_TEST,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &test_result,
                __buffer.add(0).cast::<EFunctionalTestResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message,
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
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_FINISH_TEST,
                __buffer,
            )
        };
    }
    pub fn assert_value_int(
        &mut self,
        actual: i32,
        should_be: EComparisonMethod,
        expected: i32,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_VALUE_INT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&actual, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &should_be,
                __buffer.add(4).cast::<EComparisonMethod>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&expected, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(16).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_VALUE_INT,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn assert_value_float(
        &mut self,
        actual: f32,
        should_be: EComparisonMethod,
        expected: f32,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_VALUE_FLOAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&actual, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &should_be,
                __buffer.add(4).cast::<EComparisonMethod>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&expected, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(16).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_VALUE_FLOAT,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn assert_value_double(
        &mut self,
        actual: f64,
        should_be: EComparisonMethod,
        expected: f64,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_VALUE_DOUBLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&actual, __buffer.add(0).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &should_be,
                __buffer.add(8).cast::<EComparisonMethod>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&expected, __buffer.add(16).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(24).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(40).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_VALUE_DOUBLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn assert_value_date_time(
        &mut self,
        actual: crate::bindings::core_u_object::FDateTime,
        should_be: EComparisonMethod,
        expected: crate::bindings::core_u_object::FDateTime,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_VALUE_DATE_TIME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FDateTime>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &should_be,
                __buffer.add(8).cast::<EComparisonMethod>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FDateTime>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(24).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(40).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_VALUE_DATE_TIME,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn assert_true(
        &mut self,
        condition: bool,
        message: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_TRUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&condition, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_TRUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn assert_not_equal_vector4(
        &mut self,
        actual: crate::bindings::core_u_object::FVector4,
        not_expected: crate::bindings::core_u_object::FVector4,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_VECTOR4,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &not_expected,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(64).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(80).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_VECTOR4,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn assert_not_equal_vector2_d(
        &mut self,
        actual: crate::bindings::core_u_object::FVector2D,
        not_expected: crate::bindings::core_u_object::FVector2D,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_VECTOR2_D,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &not_expected,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(32).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(48).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_VECTOR2_D,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn assert_not_equal_vector(
        &mut self,
        actual: crate::bindings::core_u_object::FVector,
        not_expected: crate::bindings::core_u_object::FVector,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_VECTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &not_expected,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(48).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(64).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_VECTOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn assert_not_equal_transform(
        &mut self,
        actual: &crate::bindings::core_u_object::FTransform,
        not_expected: &crate::bindings::core_u_object::FTransform,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<217>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                not_expected,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(192).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer
                    .add(208)
                    .cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_TRANSFORM,
                __buffer,
            )
        };
        unsafe { __buffer.add(216).cast::<bool>().read() }
    }
    pub fn assert_not_equal_string(
        &mut self,
        actual: FString,
        not_expected: FString,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_STRING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&actual, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &not_expected,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(32).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(48).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_STRING,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn assert_not_equal_rotator(
        &mut self,
        actual: crate::bindings::core_u_object::FRotator,
        not_expected: crate::bindings::core_u_object::FRotator,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_ROTATOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &not_expected,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(48).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(64).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_ROTATOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn assert_not_equal_quat(
        &mut self,
        actual: crate::bindings::core_u_object::FQuat,
        not_expected: crate::bindings::core_u_object::FQuat,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_QUAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &not_expected,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(64).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(80).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_QUAT,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn assert_not_equal_plane(
        &mut self,
        actual: crate::bindings::core_u_object::FPlane,
        not_expected: crate::bindings::core_u_object::FPlane,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_PLANE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FPlane>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &not_expected,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FPlane>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(64).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(80).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_PLANE,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn assert_not_equal_matrix(
        &mut self,
        actual: crate::bindings::core_u_object::FMatrix,
        not_expected: crate::bindings::core_u_object::FMatrix,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<281>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_MATRIX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FMatrix>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &not_expected,
                __buffer.add(128).cast::<crate::bindings::core_u_object::FMatrix>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(256).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer
                    .add(272)
                    .cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_MATRIX,
                __buffer,
            )
        };
        unsafe { __buffer.add(280).cast::<bool>().read() }
    }
    pub fn assert_not_equal_box2_d(
        &mut self,
        actual: crate::bindings::core_u_object::FBox2D,
        not_expected: crate::bindings::core_u_object::FBox2D,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_BOX2_D,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FBox2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &not_expected,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FBox2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(80).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(96).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_NOT_EQUAL_BOX2_D,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<bool>().read() }
    }
    pub fn assert_is_valid(
        &mut self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
        message: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_IS_VALID,
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
                &message,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_IS_VALID,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn assert_false(
        &mut self,
        condition: bool,
        message: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_FALSE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&condition, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_FALSE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn assert_equal_vector4(
        &mut self,
        actual: crate::bindings::core_u_object::FVector4,
        expected: crate::bindings::core_u_object::FVector4,
        what: FString,
        tolerance: f32,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_VECTOR4,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(64).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(80).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(88).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_VECTOR4,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn assert_equal_vector2_d(
        &mut self,
        actual: crate::bindings::core_u_object::FVector2D,
        expected: crate::bindings::core_u_object::FVector2D,
        what: FString,
        tolerance: f32,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_VECTOR2_D,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(32).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(48).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(56).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_VECTOR2_D,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn assert_equal_vector(
        &mut self,
        actual: crate::bindings::core_u_object::FVector,
        expected: crate::bindings::core_u_object::FVector,
        what: FString,
        tolerance: f32,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_VECTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(48).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(64).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(72).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_VECTOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn assert_equal_transform(
        &mut self,
        actual: &crate::bindings::core_u_object::FTransform,
        expected: &crate::bindings::core_u_object::FTransform,
        what: FString,
        tolerance: f32,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<225>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_TRANSFORM,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                expected,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(192).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tolerance,
                __buffer.add(208).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer
                    .add(216)
                    .cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_TRANSFORM,
                __buffer,
            )
        };
        unsafe { __buffer.add(224).cast::<bool>().read() }
    }
    pub fn assert_equal_trace_query_results(
        &mut self,
        actual: UPtr<UTraceQueryTestResults>,
        expected: UPtr<UTraceQueryTestResults>,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_TRACE_QUERY_RESULTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<UPtr<UTraceQueryTestResults>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(8).cast::<UPtr<UTraceQueryTestResults>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(16).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_TRACE_QUERY_RESULTS,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn assert_equal_string(
        &mut self,
        actual: FString,
        expected: FString,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_STRING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&actual, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(32).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(48).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_STRING,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn assert_equal_rotator_orientation(
        &mut self,
        actual: crate::bindings::core_u_object::FRotator,
        expected: crate::bindings::core_u_object::FRotator,
        what: FString,
        tolerance: f32,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_ROTATOR_ORIENTATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(48).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(64).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(72).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_ROTATOR_ORIENTATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn assert_equal_rotator(
        &mut self,
        actual: crate::bindings::core_u_object::FRotator,
        expected: crate::bindings::core_u_object::FRotator,
        what: FString,
        tolerance: f32,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_ROTATOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(48).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(64).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(72).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_ROTATOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn assert_equal_quat(
        &mut self,
        actual: crate::bindings::core_u_object::FQuat,
        expected: crate::bindings::core_u_object::FQuat,
        what: FString,
        tolerance: f32,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_QUAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(64).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(80).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(88).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_QUAT,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn assert_equal_plane(
        &mut self,
        actual: crate::bindings::core_u_object::FPlane,
        expected: crate::bindings::core_u_object::FPlane,
        what: FString,
        tolerance: f32,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_PLANE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FPlane>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FPlane>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(64).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(80).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(88).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_PLANE,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn assert_equal_object(
        &mut self,
        actual: UPtr<crate::bindings::core_u_object::UObject>,
        expected: UPtr<crate::bindings::core_u_object::UObject>,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_OBJECT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(16).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_OBJECT,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn assert_equal_name(
        &mut self,
        actual: FName,
        expected: FName,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&actual, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(24).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(40).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn assert_equal_matrix(
        &mut self,
        actual: crate::bindings::core_u_object::FMatrix,
        expected: crate::bindings::core_u_object::FMatrix,
        what: FString,
        tolerance: f32,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<289>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_MATRIX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FMatrix>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(128).cast::<crate::bindings::core_u_object::FMatrix>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(256).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tolerance,
                __buffer.add(272).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer
                    .add(280)
                    .cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_MATRIX,
                __buffer,
            )
        };
        unsafe { __buffer.add(288).cast::<bool>().read() }
    }
    pub fn assert_equal_int(
        &mut self,
        actual: i32,
        expected: i32,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_INT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&actual, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&expected, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(8).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_INT,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn assert_equal_float(
        &mut self,
        actual: f32,
        expected: f32,
        what: FString,
        tolerance: f32,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_FLOAT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&actual, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&expected, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(8).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(24).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_FLOAT,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn assert_equal_double(
        &mut self,
        actual: f64,
        expected: f64,
        what: FString,
        tolerance: f64,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_DOUBLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&actual, __buffer.add(0).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&expected, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(16).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(32).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(40).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_DOUBLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn assert_equal_box2_d(
        &mut self,
        actual: crate::bindings::core_u_object::FBox2D,
        expected: crate::bindings::core_u_object::FBox2D,
        what: FString,
        tolerance: f32,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<113>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_BOX2_D,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actual,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FBox2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expected,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FBox2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(80).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(96).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer
                    .add(104)
                    .cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_BOX2_D,
                __buffer,
            )
        };
        unsafe { __buffer.add(112).cast::<bool>().read() }
    }
    pub fn assert_equal_bool(
        &mut self,
        actual: bool,
        expected: bool,
        what: FString,
        context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_BOOL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&actual, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&expected, __buffer.add(1).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&what, __buffer.add(8).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_object,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ASSERT_EQUAL_BOOL,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_warning(&mut self, message: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ADD_WARNING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message,
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
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ADD_WARNING,
                __buffer,
            )
        };
    }
    pub fn add_rerun(&mut self, reason: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ADD_RERUN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&reason, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ADD_RERUN,
                __buffer,
            )
        };
    }
    pub fn add_info(&mut self, message: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ADD_INFO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message,
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
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ADD_INFO,
                __buffer,
            )
        };
    }
    pub fn add_error(&mut self, message: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ADD_ERROR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message,
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
                crate::bindings::functional_testing::A_FUNCTIONAL_TEST_ADD_ERROR,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct AFunctionalAITestBase {
    #[doc(hidden)]
    __padding_1544: [u8; 1544],
    pub spawn_location_randomization_range: f32,
    pub spawned_pawns: TArray<UPtr<crate::bindings::engine::APawn>>,
    pub pending_delayed_spawns: TArray<FPendingDelayedSpawn>,
    pub current_spawn_set_index: i32,
    pub current_spawn_set_name: FString,
    #[doc(hidden)]
    __padding_1704: [u8; 96],
    pub flags_1704: u8,
    __padding_end: [u8; 15],
}
impl AFunctionalAITestBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AFunctionalAITestBase")
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
    pub fn is_one_of_spawned_pawns(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::A_FUNCTIONAL_AI_TEST_BASE_IS_ONE_OF_SPAWNED_PAWNS,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::A_FUNCTIONAL_AI_TEST_BASE_IS_ONE_OF_SPAWNED_PAWNS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct AFunctionalAITest {
    #[doc(hidden)]
    __padding_1720: [u8; 1720],
    pub spawn_sets: TArray<FAITestSpawnSet>,
}
impl AFunctionalAITest {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AFunctionalAITest")
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
pub struct UAutomationPerformaceHelper {
    __padding_end: [u8; 104],
}
impl UAutomationPerformaceHelper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutomationPerformaceHelper")
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
    pub fn write_log_file(&mut self, capture_dir: FString, capture_extension: FString) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_WRITE_LOG_FILE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &capture_dir,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &capture_extension,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_WRITE_LOG_FILE,
                __buffer,
            )
        };
    }
    pub fn trigger_gpu_trace_if_record_falls_below_budget(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_TRIGGER_GPU_TRACE_IF_RECORD_FALLS_BELOW_BUDGET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_TRIGGER_GPU_TRACE_IF_RECORD_FALLS_BELOW_BUDGET,
                __buffer,
            )
        };
    }
    pub fn tick(&mut self, delta_seconds: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_TICK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_TICK,
                __buffer,
            )
        };
    }
    pub fn stop_cpu_profiling(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_STOP_CPU_PROFILING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_STOP_CPU_PROFILING,
                __buffer,
            )
        };
    }
    pub fn start_cpu_profiling(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_START_CPU_PROFILING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_START_CPU_PROFILING,
                __buffer,
            )
        };
    }
    pub fn sample(&mut self, delta_seconds: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_SAMPLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_SAMPLE,
                __buffer,
            )
        };
    }
    pub fn on_begin_tests(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_ON_BEGIN_TESTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_ON_BEGIN_TESTS,
                __buffer,
            )
        };
    }
    pub fn on_all_tests_complete(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_ON_ALL_TESTS_COMPLETE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_ON_ALL_TESTS_COMPLETE,
                __buffer,
            )
        };
    }
    pub fn is_recording(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_IS_RECORDING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_IS_RECORDING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_current_record_within_render_thread_budget(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_IS_CURRENT_RECORD_WITHIN_RENDER_THREAD_BUDGET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_IS_CURRENT_RECORD_WITHIN_RENDER_THREAD_BUDGET,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_current_record_within_gpu_budget(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_IS_CURRENT_RECORD_WITHIN_GPU_BUDGET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_IS_CURRENT_RECORD_WITHIN_GPU_BUDGET,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_current_record_within_game_thread_budget(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_IS_CURRENT_RECORD_WITHIN_GAME_THREAD_BUDGET,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_IS_CURRENT_RECORD_WITHIN_GAME_THREAD_BUDGET,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn end_stats_file(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_END_STATS_FILE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_END_STATS_FILE,
                __buffer,
            )
        };
    }
    pub fn end_recording_baseline(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_END_RECORDING_BASELINE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_END_RECORDING_BASELINE,
                __buffer,
            )
        };
    }
    pub fn end_recording(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_END_RECORDING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_END_RECORDING,
                __buffer,
            )
        };
    }
    pub fn begin_stats_file(&mut self, record_name: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_BEGIN_STATS_FILE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &record_name,
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
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_BEGIN_STATS_FILE,
                __buffer,
            )
        };
    }
    pub fn begin_recording_baseline(&mut self, record_name: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_BEGIN_RECORDING_BASELINE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &record_name,
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
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_BEGIN_RECORDING_BASELINE,
                __buffer,
            )
        };
    }
    pub fn begin_recording(
        &mut self,
        record_name: FString,
        in_gpu_budget: f32,
        in_render_thread_budget: f32,
        in_game_thread_budget: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_BEGIN_RECORDING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &record_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_gpu_budget,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_render_thread_budget,
                __buffer.add(20).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_game_thread_budget,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_AUTOMATION_PERFORMACE_HELPER_BEGIN_RECORDING,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct AFunctionalTestGameMode {
    __padding_end: [u8; 1296],
}
impl AFunctionalTestGameMode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AFunctionalTestGameMode")
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
pub struct UFunctionalTestingManager {
    __padding_end: [u8; 240],
}
impl UFunctionalTestingManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFunctionalTestingManager")
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
    pub fn run_all_functional_tests(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_new_log: bool,
        b_run_looped: bool,
        failed_tests_repro_string: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_FUNCTIONAL_TESTING_MANAGER_RUN_ALL_FUNCTIONAL_TESTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_new_log, __buffer.add(8).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_run_looped,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &failed_tests_repro_string,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UFunctionalTestingManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_FUNCTIONAL_TESTING_MANAGER_RUN_ALL_FUNCTIONAL_TESTS,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct APhasedAutomationActorBase {
    __padding_end: [u8; 1136],
}
impl APhasedAutomationActorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APhasedAutomationActorBase")
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
pub struct AFunctionalTestLevelScript {
    __padding_end: [u8; 1144],
}
impl AFunctionalTestLevelScript {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AFunctionalTestLevelScript")
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
pub struct UFunctionalTestUtilityLibrary {
    __padding_end: [u8; 48],
}
impl UFunctionalTestUtilityLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFunctionalTestUtilityLibrary")
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
    pub fn trace_channel_test_util(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        batch_options: &FTraceChannelTestBatchOptions,
        start: crate::bindings::core_u_object::FVector,
        end: crate::bindings::core_u_object::FVector,
        sphere_capsule_radius: f32,
        capsule_half_height: f32,
        box_half_size: crate::bindings::core_u_object::FVector,
        orientation: crate::bindings::core_u_object::FRotator,
        trace_channel: crate::bindings::engine::ETraceTypeQuery,
        object_types: TArray<crate::bindings::engine::EObjectTypeQuery>,
        profile_name: FName,
        b_trace_complex: bool,
        actors_to_ignore: &TArray<UPtr<crate::bindings::engine::AActor>>,
        b_ignore_self: bool,
        draw_debug_type: crate::bindings::engine::EDrawDebugTrace,
        trace_color: crate::bindings::core_u_object::FLinearColor,
        trace_hit_color: crate::bindings::core_u_object::FLinearColor,
        draw_time: f32,
    ) -> UPtr<UTraceQueryTestResults> {
        let mut __stack = crate::core_data::StackAlloc::<224>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_FUNCTIONAL_TEST_UTILITY_LIBRARY_TRACE_CHANNEL_TEST_UTIL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                batch_options,
                __buffer.add(8).cast::<FTraceChannelTestBatchOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &end,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sphere_capsule_radius,
                __buffer.add(64).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &capsule_half_height,
                __buffer.add(68).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &box_half_size,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &orientation,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trace_channel,
                __buffer.add(120).cast::<crate::bindings::engine::ETraceTypeQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_types,
                __buffer
                    .add(128)
                    .cast::<TArray<crate::bindings::engine::EObjectTypeQuery>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &profile_name,
                __buffer.add(144).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_trace_complex,
                __buffer.add(156).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_ignore,
                __buffer
                    .add(160)
                    .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_ignore_self,
                __buffer.add(176).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &draw_debug_type,
                __buffer.add(177).cast::<crate::bindings::engine::EDrawDebugTrace>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trace_color,
                __buffer.add(180).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trace_hit_color,
                __buffer.add(196).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &draw_time,
                __buffer.add(212).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::functional_testing::UFunctionalTestUtilityLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_FUNCTIONAL_TEST_UTILITY_LIBRARY_TRACE_CHANNEL_TEST_UTIL,
                __buffer,
            )
        };
        unsafe { __buffer.add(216).cast::<UPtr<UTraceQueryTestResults>>().read() }
    }
}
#[repr(C, align(8))]
pub struct AScreenshotFunctionalTestBase {
    #[doc(hidden)]
    __padding_1544: [u8; 1544],
    pub notes: FString,
    pub screenshot_camera: UPtr<crate::bindings::engine::UCameraComponent>,
    pub screenshot_options: FAutomationScreenshotOptions,
    __padding_end: [u8; 32],
}
impl AScreenshotFunctionalTestBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AScreenshotFunctionalTestBase")
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
pub struct AFunctionalUIScreenshotTest {
    __padding_end: [u8; 1752],
}
impl AFunctionalUIScreenshotTest {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AFunctionalUIScreenshotTest")
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
pub struct UGroundTruthData {
    __padding_end: [u8; 64],
}
impl UGroundTruthData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroundTruthData")
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
    pub fn save_object(
        &mut self,
        ground_truth: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_GROUND_TRUTH_DATA_SAVE_OBJECT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ground_truth,
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
                crate::bindings::functional_testing::U_GROUND_TRUTH_DATA_SAVE_OBJECT,
                __buffer,
            )
        };
    }
    pub fn reset_object(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_GROUND_TRUTH_DATA_RESET_OBJECT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_GROUND_TRUTH_DATA_RESET_OBJECT,
                __buffer,
            )
        };
    }
    pub fn load_object(&mut self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_GROUND_TRUTH_DATA_LOAD_OBJECT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_GROUND_TRUTH_DATA_LOAD_OBJECT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn can_modify(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_GROUND_TRUTH_DATA_CAN_MODIFY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_GROUND_TRUTH_DATA_CAN_MODIFY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct AScreenshotFunctionalTest {
    #[doc(hidden)]
    __padding_1688: [u8; 1688],
    pub b_camera_cut_on_screenshot_prep: bool,
    __padding_end: [u8; 71],
}
impl AScreenshotFunctionalTest {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AScreenshotFunctionalTest")
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
pub struct UTestPhaseComponent {
    __padding_end: [u8; 656],
}
impl UTestPhaseComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestPhaseComponent")
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
pub struct UTraceQueryTestResults {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub channel_results: FTraceQueryTestResultsInner,
    pub object_results: FTraceQueryTestResultsInner,
    pub profile_results: FTraceQueryTestResultsInner,
    pub batch_options: FTraceChannelTestBatchOptions,
}
impl UTraceQueryTestResults {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTraceQueryTestResults")
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
    pub fn to_string(&mut self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::U_TRACE_QUERY_TEST_RESULTS_TO_STRING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::U_TRACE_QUERY_TEST_RESULTS_TO_STRING,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct FFunctionalTest_OnTestPrepare {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFunctionalTest_OnTestStart {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFunctionalTest_OnTestFinished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFunctionalAITestBase_OnAISpawned {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFunctionalAITestBase_OnAllAISPawned {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFunctionalTestingManager_OnSetupTests {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFunctionalTestingManager_OnTestsComplete {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFunctionalTestingManager_OnTestsBegin {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EComparisonTolerance(pub u8);
impl EComparisonTolerance {
    pub const ZERO: EComparisonTolerance = EComparisonTolerance(0);
    pub const LOW: EComparisonTolerance = EComparisonTolerance(1);
    pub const MEDIUM: EComparisonTolerance = EComparisonTolerance(2);
    pub const HIGH: EComparisonTolerance = EComparisonTolerance(3);
    pub const CUSTOM: EComparisonTolerance = EComparisonTolerance(4);
}
#[repr(transparent)]
pub struct EComparisonMethod(pub u8);
impl EComparisonMethod {
    pub const EQUAL_TO: EComparisonMethod = EComparisonMethod(0);
    pub const NOT_EQUAL_TO: EComparisonMethod = EComparisonMethod(1);
    pub const GREATER_THAN_OR_EQUAL_TO: EComparisonMethod = EComparisonMethod(2);
    pub const LESS_THAN_OR_EQUAL_TO: EComparisonMethod = EComparisonMethod(3);
    pub const GREATER_THAN: EComparisonMethod = EComparisonMethod(4);
    pub const LESS_THAN: EComparisonMethod = EComparisonMethod(5);
}
#[repr(transparent)]
pub struct EFunctionalTestResult(pub u8);
impl EFunctionalTestResult {
    pub const DEFAULT: EFunctionalTestResult = EFunctionalTestResult(0);
    pub const INVALID: EFunctionalTestResult = EFunctionalTestResult(1);
    pub const ERROR: EFunctionalTestResult = EFunctionalTestResult(2);
    pub const RUNNING: EFunctionalTestResult = EFunctionalTestResult(3);
    pub const FAILED: EFunctionalTestResult = EFunctionalTestResult(4);
    pub const SUCCEEDED: EFunctionalTestResult = EFunctionalTestResult(5);
}
#[repr(transparent)]
pub struct EFunctionalTestLogHandling(pub u8);
impl EFunctionalTestLogHandling {
    pub const PROJECT_DEFAULT: EFunctionalTestLogHandling = EFunctionalTestLogHandling(
        0,
    );
    pub const OUTPUT_IS_ERROR: EFunctionalTestLogHandling = EFunctionalTestLogHandling(
        1,
    );
    pub const OUTPUT_IGNORED: EFunctionalTestLogHandling = EFunctionalTestLogHandling(2);
}
#[repr(transparent)]
pub struct EWidgetTestAppearLocation(pub i32);
impl EWidgetTestAppearLocation {
    pub const VIEWPORT: EWidgetTestAppearLocation = EWidgetTestAppearLocation(0);
    pub const PLAYER_SCREEN: EWidgetTestAppearLocation = EWidgetTestAppearLocation(1);
}
