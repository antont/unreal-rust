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
    __padding_end: [u8; 1],
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
    __padding_end: [u8; 7],
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
    __padding_end: [u8; 1],
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
