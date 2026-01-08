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
    pub u_automation_editor_task_is_valid_task: *mut crate::ffi::UFunctionOpague,
    pub u_automation_editor_task_is_task_done: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_take_high_res_screenshot: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_take_automation_screenshot_of_ui: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_take_automation_screenshot_at_camera: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_take_automation_screenshot: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_set_test_telemetry_storage: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_set_scalability_quality_to_low: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_set_scalability_quality_to_epic: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_set_scalability_quality_level_relative_to_max: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_set_editor_viewport_visualize_buffer: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_set_editor_viewport_view_mode: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_set_editor_active_viewport_wireframe_opacity: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_set_editor_active_viewport_view_mode: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_get_stat_inc_max: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_get_stat_inc_average: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_get_stat_exc_max: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_get_stat_exc_average: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_get_stat_call_count: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_get_editor_active_viewport_wireframe_opacity: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_get_editor_active_viewport_view_mode: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_get_default_screenshot_options_for_rendering: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_get_default_screenshot_options_for_gameplay: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_finish_loading_before_screenshot: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_enable_stat_group: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_disable_stat_group: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_compare_image_against_reference: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_automation_wait_for_loading: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_are_automated_tests_running: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_add_test_warning: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_add_test_telemetry_data: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_add_test_info: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_add_test_error: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_add_expected_plain_log_message: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_add_expected_plain_log_error: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_add_expected_log_message: *mut crate::ffi::UFunctionOpague,
    pub u_automation_blueprint_function_library_add_expected_log_error: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_set_time_limit: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_set_console_variable_from_integer: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_set_console_variable_from_float: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_set_console_variable_from_boolean: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_set_console_variable: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_register_auto_destroy_actor: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_receive_test_finished: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_receive_start_test: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_receive_prepare_test: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_on_wants_re_run_check: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_on_additional_test_finished_message_request: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_log_message: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_is_running: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_is_ready: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_is_enabled_in_world: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_is_enabled: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_get_current_rerun_reason: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_finish_test: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_edit_tags: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_debug_gather_relevant_actors: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_value_int: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_value_float: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_value_double: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_value_date_time: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_true: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_not_equal_vector4: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_not_equal_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_not_equal_vector: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_not_equal_transform: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_not_equal_string: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_not_equal_rotator: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_not_equal_quat: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_not_equal_plane: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_not_equal_matrix: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_not_equal_box2_d: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_is_valid: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_false: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_vector4: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_vector: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_transform: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_trace_query_results: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_string: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_rotator_orientation: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_rotator: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_quat: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_plane: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_object: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_name: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_matrix: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_int: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_float: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_double: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_box2_d: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_assert_equal_bool: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_add_warning: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_add_rerun: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_add_info: *mut crate::ffi::UFunctionOpague,
    pub a_functional_test_add_error: *mut crate::ffi::UFunctionOpague,
    pub a_functional_ai_test_base_is_one_of_spawned_pawns: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_write_log_file: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_trigger_gpu_trace_if_record_falls_below_budget: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_tick: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_stop_cpu_profiling: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_start_cpu_profiling: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_sample: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_on_begin_tests: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_on_all_tests_complete: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_is_recording: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_is_current_record_within_render_thread_budget: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_is_current_record_within_gpu_budget: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_is_current_record_within_game_thread_budget: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_end_stats_file: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_end_recording_baseline: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_end_recording: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_begin_stats_file: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_begin_recording_baseline: *mut crate::ffi::UFunctionOpague,
    pub u_automation_performace_helper_begin_recording: *mut crate::ffi::UFunctionOpague,
    pub u_functional_testing_manager_run_all_functional_tests: *mut crate::ffi::UFunctionOpague,
    pub a_phased_automation_actor_base_on_functional_testing_complete: *mut crate::ffi::UFunctionOpague,
    pub a_phased_automation_actor_base_on_functional_testing_begin: *mut crate::ffi::UFunctionOpague,
    pub u_functional_test_utility_library_trace_channel_test_util: *mut crate::ffi::UFunctionOpague,
    pub u_ground_truth_data_save_object: *mut crate::ffi::UFunctionOpague,
    pub u_ground_truth_data_reset_object: *mut crate::ffi::UFunctionOpague,
    pub u_ground_truth_data_load_object: *mut crate::ffi::UFunctionOpague,
    pub u_ground_truth_data_can_modify: *mut crate::ffi::UFunctionOpague,
    pub u_trace_query_test_results_to_string: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_automation_editor_task_is_valid_task: std::ptr::null_mut(),
            u_automation_editor_task_is_task_done: std::ptr::null_mut(),
            u_automation_blueprint_function_library_take_high_res_screenshot: std::ptr::null_mut(),
            u_automation_blueprint_function_library_take_automation_screenshot_of_ui: std::ptr::null_mut(),
            u_automation_blueprint_function_library_take_automation_screenshot_at_camera: std::ptr::null_mut(),
            u_automation_blueprint_function_library_take_automation_screenshot: std::ptr::null_mut(),
            u_automation_blueprint_function_library_set_test_telemetry_storage: std::ptr::null_mut(),
            u_automation_blueprint_function_library_set_scalability_quality_to_low: std::ptr::null_mut(),
            u_automation_blueprint_function_library_set_scalability_quality_to_epic: std::ptr::null_mut(),
            u_automation_blueprint_function_library_set_scalability_quality_level_relative_to_max: std::ptr::null_mut(),
            u_automation_blueprint_function_library_set_editor_viewport_visualize_buffer: std::ptr::null_mut(),
            u_automation_blueprint_function_library_set_editor_viewport_view_mode: std::ptr::null_mut(),
            u_automation_blueprint_function_library_set_editor_active_viewport_wireframe_opacity: std::ptr::null_mut(),
            u_automation_blueprint_function_library_set_editor_active_viewport_view_mode: std::ptr::null_mut(),
            u_automation_blueprint_function_library_get_stat_inc_max: std::ptr::null_mut(),
            u_automation_blueprint_function_library_get_stat_inc_average: std::ptr::null_mut(),
            u_automation_blueprint_function_library_get_stat_exc_max: std::ptr::null_mut(),
            u_automation_blueprint_function_library_get_stat_exc_average: std::ptr::null_mut(),
            u_automation_blueprint_function_library_get_stat_call_count: std::ptr::null_mut(),
            u_automation_blueprint_function_library_get_editor_active_viewport_wireframe_opacity: std::ptr::null_mut(),
            u_automation_blueprint_function_library_get_editor_active_viewport_view_mode: std::ptr::null_mut(),
            u_automation_blueprint_function_library_get_default_screenshot_options_for_rendering: std::ptr::null_mut(),
            u_automation_blueprint_function_library_get_default_screenshot_options_for_gameplay: std::ptr::null_mut(),
            u_automation_blueprint_function_library_finish_loading_before_screenshot: std::ptr::null_mut(),
            u_automation_blueprint_function_library_enable_stat_group: std::ptr::null_mut(),
            u_automation_blueprint_function_library_disable_stat_group: std::ptr::null_mut(),
            u_automation_blueprint_function_library_compare_image_against_reference: std::ptr::null_mut(),
            u_automation_blueprint_function_library_automation_wait_for_loading: std::ptr::null_mut(),
            u_automation_blueprint_function_library_are_automated_tests_running: std::ptr::null_mut(),
            u_automation_blueprint_function_library_add_test_warning: std::ptr::null_mut(),
            u_automation_blueprint_function_library_add_test_telemetry_data: std::ptr::null_mut(),
            u_automation_blueprint_function_library_add_test_info: std::ptr::null_mut(),
            u_automation_blueprint_function_library_add_test_error: std::ptr::null_mut(),
            u_automation_blueprint_function_library_add_expected_plain_log_message: std::ptr::null_mut(),
            u_automation_blueprint_function_library_add_expected_plain_log_error: std::ptr::null_mut(),
            u_automation_blueprint_function_library_add_expected_log_message: std::ptr::null_mut(),
            u_automation_blueprint_function_library_add_expected_log_error: std::ptr::null_mut(),
            a_functional_test_set_time_limit: std::ptr::null_mut(),
            a_functional_test_set_console_variable_from_integer: std::ptr::null_mut(),
            a_functional_test_set_console_variable_from_float: std::ptr::null_mut(),
            a_functional_test_set_console_variable_from_boolean: std::ptr::null_mut(),
            a_functional_test_set_console_variable: std::ptr::null_mut(),
            a_functional_test_register_auto_destroy_actor: std::ptr::null_mut(),
            a_functional_test_receive_test_finished: std::ptr::null_mut(),
            a_functional_test_receive_start_test: std::ptr::null_mut(),
            a_functional_test_receive_prepare_test: std::ptr::null_mut(),
            a_functional_test_on_wants_re_run_check: std::ptr::null_mut(),
            a_functional_test_on_additional_test_finished_message_request: std::ptr::null_mut(),
            a_functional_test_log_message: std::ptr::null_mut(),
            a_functional_test_is_running: std::ptr::null_mut(),
            a_functional_test_is_ready: std::ptr::null_mut(),
            a_functional_test_is_enabled_in_world: std::ptr::null_mut(),
            a_functional_test_is_enabled: std::ptr::null_mut(),
            a_functional_test_get_current_rerun_reason: std::ptr::null_mut(),
            a_functional_test_finish_test: std::ptr::null_mut(),
            a_functional_test_edit_tags: std::ptr::null_mut(),
            a_functional_test_debug_gather_relevant_actors: std::ptr::null_mut(),
            a_functional_test_assert_value_int: std::ptr::null_mut(),
            a_functional_test_assert_value_float: std::ptr::null_mut(),
            a_functional_test_assert_value_double: std::ptr::null_mut(),
            a_functional_test_assert_value_date_time: std::ptr::null_mut(),
            a_functional_test_assert_true: std::ptr::null_mut(),
            a_functional_test_assert_not_equal_vector4: std::ptr::null_mut(),
            a_functional_test_assert_not_equal_vector2_d: std::ptr::null_mut(),
            a_functional_test_assert_not_equal_vector: std::ptr::null_mut(),
            a_functional_test_assert_not_equal_transform: std::ptr::null_mut(),
            a_functional_test_assert_not_equal_string: std::ptr::null_mut(),
            a_functional_test_assert_not_equal_rotator: std::ptr::null_mut(),
            a_functional_test_assert_not_equal_quat: std::ptr::null_mut(),
            a_functional_test_assert_not_equal_plane: std::ptr::null_mut(),
            a_functional_test_assert_not_equal_matrix: std::ptr::null_mut(),
            a_functional_test_assert_not_equal_box2_d: std::ptr::null_mut(),
            a_functional_test_assert_is_valid: std::ptr::null_mut(),
            a_functional_test_assert_false: std::ptr::null_mut(),
            a_functional_test_assert_equal_vector4: std::ptr::null_mut(),
            a_functional_test_assert_equal_vector2_d: std::ptr::null_mut(),
            a_functional_test_assert_equal_vector: std::ptr::null_mut(),
            a_functional_test_assert_equal_transform: std::ptr::null_mut(),
            a_functional_test_assert_equal_trace_query_results: std::ptr::null_mut(),
            a_functional_test_assert_equal_string: std::ptr::null_mut(),
            a_functional_test_assert_equal_rotator_orientation: std::ptr::null_mut(),
            a_functional_test_assert_equal_rotator: std::ptr::null_mut(),
            a_functional_test_assert_equal_quat: std::ptr::null_mut(),
            a_functional_test_assert_equal_plane: std::ptr::null_mut(),
            a_functional_test_assert_equal_object: std::ptr::null_mut(),
            a_functional_test_assert_equal_name: std::ptr::null_mut(),
            a_functional_test_assert_equal_matrix: std::ptr::null_mut(),
            a_functional_test_assert_equal_int: std::ptr::null_mut(),
            a_functional_test_assert_equal_float: std::ptr::null_mut(),
            a_functional_test_assert_equal_double: std::ptr::null_mut(),
            a_functional_test_assert_equal_box2_d: std::ptr::null_mut(),
            a_functional_test_assert_equal_bool: std::ptr::null_mut(),
            a_functional_test_add_warning: std::ptr::null_mut(),
            a_functional_test_add_rerun: std::ptr::null_mut(),
            a_functional_test_add_info: std::ptr::null_mut(),
            a_functional_test_add_error: std::ptr::null_mut(),
            a_functional_ai_test_base_is_one_of_spawned_pawns: std::ptr::null_mut(),
            u_automation_performace_helper_write_log_file: std::ptr::null_mut(),
            u_automation_performace_helper_trigger_gpu_trace_if_record_falls_below_budget: std::ptr::null_mut(),
            u_automation_performace_helper_tick: std::ptr::null_mut(),
            u_automation_performace_helper_stop_cpu_profiling: std::ptr::null_mut(),
            u_automation_performace_helper_start_cpu_profiling: std::ptr::null_mut(),
            u_automation_performace_helper_sample: std::ptr::null_mut(),
            u_automation_performace_helper_on_begin_tests: std::ptr::null_mut(),
            u_automation_performace_helper_on_all_tests_complete: std::ptr::null_mut(),
            u_automation_performace_helper_is_recording: std::ptr::null_mut(),
            u_automation_performace_helper_is_current_record_within_render_thread_budget: std::ptr::null_mut(),
            u_automation_performace_helper_is_current_record_within_gpu_budget: std::ptr::null_mut(),
            u_automation_performace_helper_is_current_record_within_game_thread_budget: std::ptr::null_mut(),
            u_automation_performace_helper_end_stats_file: std::ptr::null_mut(),
            u_automation_performace_helper_end_recording_baseline: std::ptr::null_mut(),
            u_automation_performace_helper_end_recording: std::ptr::null_mut(),
            u_automation_performace_helper_begin_stats_file: std::ptr::null_mut(),
            u_automation_performace_helper_begin_recording_baseline: std::ptr::null_mut(),
            u_automation_performace_helper_begin_recording: std::ptr::null_mut(),
            u_functional_testing_manager_run_all_functional_tests: std::ptr::null_mut(),
            a_phased_automation_actor_base_on_functional_testing_complete: std::ptr::null_mut(),
            a_phased_automation_actor_base_on_functional_testing_begin: std::ptr::null_mut(),
            u_functional_test_utility_library_trace_channel_test_util: std::ptr::null_mut(),
            u_ground_truth_data_save_object: std::ptr::null_mut(),
            u_ground_truth_data_reset_object: std::ptr::null_mut(),
            u_ground_truth_data_load_object: std::ptr::null_mut(),
            u_ground_truth_data_can_modify: std::ptr::null_mut(),
            u_trace_query_test_results_to_string: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAutomationEditorTask::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidTask"),
            &raw mut __FUNCTION_PTRS.u_automation_editor_task_is_valid_task,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTaskDone"),
            &raw mut __FUNCTION_PTRS.u_automation_editor_task_is_task_done,
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
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_take_high_res_screenshot,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TakeAutomationScreenshotOfUI"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_take_automation_screenshot_of_ui,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TakeAutomationScreenshotAtCamera"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_take_automation_screenshot_at_camera,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TakeAutomationScreenshot"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_take_automation_screenshot,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTestTelemetryStorage"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_set_test_telemetry_storage,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScalabilityQualityToLow"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_set_scalability_quality_to_low,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScalabilityQualityToEpic"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_set_scalability_quality_to_epic,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetScalabilityQualityLevelRelativeToMax"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_set_scalability_quality_level_relative_to_max,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEditorViewportVisualizeBuffer"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_set_editor_viewport_visualize_buffer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEditorViewportViewMode"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_set_editor_viewport_view_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEditorActiveViewportWireframeOpacity"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_set_editor_active_viewport_wireframe_opacity,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEditorActiveViewportViewMode"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_set_editor_active_viewport_view_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatIncMax"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_get_stat_inc_max,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatIncAverage"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_get_stat_inc_average,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatExcMax"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_get_stat_exc_max,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatExcAverage"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_get_stat_exc_average,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStatCallCount"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_get_stat_call_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEditorActiveViewportWireframeOpacity"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_get_editor_active_viewport_wireframe_opacity,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEditorActiveViewportViewMode"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_get_editor_active_viewport_view_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultScreenshotOptionsForRendering"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_get_default_screenshot_options_for_rendering,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefaultScreenshotOptionsForGameplay"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_get_default_screenshot_options_for_gameplay,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FinishLoadingBeforeScreenshot"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_finish_loading_before_screenshot,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnableStatGroup"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_enable_stat_group,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisableStatGroup"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_disable_stat_group,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CompareImageAgainstReference"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_compare_image_against_reference,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AutomationWaitForLoading"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_automation_wait_for_loading,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AreAutomatedTestsRunning"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_are_automated_tests_running,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTestWarning"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_add_test_warning,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTestTelemetryData"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_add_test_telemetry_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTestInfo"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_add_test_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTestError"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_add_test_error,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddExpectedPlainLogMessage"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_add_expected_plain_log_message,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddExpectedPlainLogError"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_add_expected_plain_log_error,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddExpectedLogMessage"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_add_expected_log_message,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddExpectedLogError"),
            &raw mut __FUNCTION_PTRS
                .u_automation_blueprint_function_library_add_expected_log_error,
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
            &raw mut __FUNCTION_PTRS.a_functional_test_set_time_limit,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConsoleVariableFromInteger"),
            &raw mut __FUNCTION_PTRS.a_functional_test_set_console_variable_from_integer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConsoleVariableFromFloat"),
            &raw mut __FUNCTION_PTRS.a_functional_test_set_console_variable_from_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConsoleVariableFromBoolean"),
            &raw mut __FUNCTION_PTRS.a_functional_test_set_console_variable_from_boolean,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConsoleVariable"),
            &raw mut __FUNCTION_PTRS.a_functional_test_set_console_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterAutoDestroyActor"),
            &raw mut __FUNCTION_PTRS.a_functional_test_register_auto_destroy_actor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveTestFinished"),
            &raw mut __FUNCTION_PTRS.a_functional_test_receive_test_finished,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveStartTest"),
            &raw mut __FUNCTION_PTRS.a_functional_test_receive_start_test,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceivePrepareTest"),
            &raw mut __FUNCTION_PTRS.a_functional_test_receive_prepare_test,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnWantsReRunCheck"),
            &raw mut __FUNCTION_PTRS.a_functional_test_on_wants_re_run_check,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAdditionalTestFinishedMessageRequest"),
            &raw mut __FUNCTION_PTRS
                .a_functional_test_on_additional_test_finished_message_request,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LogMessage"),
            &raw mut __FUNCTION_PTRS.a_functional_test_log_message,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRunning"),
            &raw mut __FUNCTION_PTRS.a_functional_test_is_running,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsReady"),
            &raw mut __FUNCTION_PTRS.a_functional_test_is_ready,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEnabledInWorld"),
            &raw mut __FUNCTION_PTRS.a_functional_test_is_enabled_in_world,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEnabled"),
            &raw mut __FUNCTION_PTRS.a_functional_test_is_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentRerunReason"),
            &raw mut __FUNCTION_PTRS.a_functional_test_get_current_rerun_reason,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FinishTest"),
            &raw mut __FUNCTION_PTRS.a_functional_test_finish_test,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditTags"),
            &raw mut __FUNCTION_PTRS.a_functional_test_edit_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DebugGatherRelevantActors"),
            &raw mut __FUNCTION_PTRS.a_functional_test_debug_gather_relevant_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertValue_Int"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_value_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertValue_Float"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_value_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertValue_Double"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_value_double,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertValue_DateTime"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_value_date_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertTrue"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_true,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Vector4"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_not_equal_vector4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Vector2D"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_not_equal_vector2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Vector"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_not_equal_vector,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Transform"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_not_equal_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_String"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_not_equal_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Rotator"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_not_equal_rotator,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Quat"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_not_equal_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Plane"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_not_equal_plane,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Matrix"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_not_equal_matrix,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertNotEqual_Box2D"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_not_equal_box2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertIsValid"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_is_valid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertFalse"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_false,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Vector4"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_vector4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Vector2D"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_vector2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Vector"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_vector,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Transform"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_TraceQueryResults"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_trace_query_results,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_String"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_RotatorOrientation"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_rotator_orientation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Rotator"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_rotator,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Quat"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Plane"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_plane,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Object"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Name"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Matrix"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_matrix,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Int"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Float"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Double"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_double,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Box2D"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_box2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssertEqual_Bool"),
            &raw mut __FUNCTION_PTRS.a_functional_test_assert_equal_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddWarning"),
            &raw mut __FUNCTION_PTRS.a_functional_test_add_warning,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddRerun"),
            &raw mut __FUNCTION_PTRS.a_functional_test_add_rerun,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInfo"),
            &raw mut __FUNCTION_PTRS.a_functional_test_add_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddError"),
            &raw mut __FUNCTION_PTRS.a_functional_test_add_error,
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
            &raw mut __FUNCTION_PTRS.a_functional_ai_test_base_is_one_of_spawned_pawns,
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
            &raw mut __FUNCTION_PTRS.u_automation_performace_helper_write_log_file,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TriggerGPUTraceIfRecordFallsBelowBudget"),
            &raw mut __FUNCTION_PTRS
                .u_automation_performace_helper_trigger_gpu_trace_if_record_falls_below_budget,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Tick"),
            &raw mut __FUNCTION_PTRS.u_automation_performace_helper_tick,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopCPUProfiling"),
            &raw mut __FUNCTION_PTRS.u_automation_performace_helper_stop_cpu_profiling,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartCPUProfiling"),
            &raw mut __FUNCTION_PTRS.u_automation_performace_helper_start_cpu_profiling,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Sample"),
            &raw mut __FUNCTION_PTRS.u_automation_performace_helper_sample,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnBeginTests"),
            &raw mut __FUNCTION_PTRS.u_automation_performace_helper_on_begin_tests,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAllTestsComplete"),
            &raw mut __FUNCTION_PTRS.u_automation_performace_helper_on_all_tests_complete,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRecording"),
            &raw mut __FUNCTION_PTRS.u_automation_performace_helper_is_recording,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCurrentRecordWithinRenderThreadBudget"),
            &raw mut __FUNCTION_PTRS
                .u_automation_performace_helper_is_current_record_within_render_thread_budget,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCurrentRecordWithinGPUBudget"),
            &raw mut __FUNCTION_PTRS
                .u_automation_performace_helper_is_current_record_within_gpu_budget,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCurrentRecordWithinGameThreadBudget"),
            &raw mut __FUNCTION_PTRS
                .u_automation_performace_helper_is_current_record_within_game_thread_budget,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EndStatsFile"),
            &raw mut __FUNCTION_PTRS.u_automation_performace_helper_end_stats_file,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EndRecordingBaseline"),
            &raw mut __FUNCTION_PTRS
                .u_automation_performace_helper_end_recording_baseline,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EndRecording"),
            &raw mut __FUNCTION_PTRS.u_automation_performace_helper_end_recording,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BeginStatsFile"),
            &raw mut __FUNCTION_PTRS.u_automation_performace_helper_begin_stats_file,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BeginRecordingBaseline"),
            &raw mut __FUNCTION_PTRS
                .u_automation_performace_helper_begin_recording_baseline,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BeginRecording"),
            &raw mut __FUNCTION_PTRS.u_automation_performace_helper_begin_recording,
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
            &raw mut __FUNCTION_PTRS
                .u_functional_testing_manager_run_all_functional_tests,
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
            &raw mut __FUNCTION_PTRS
                .a_phased_automation_actor_base_on_functional_testing_complete,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnFunctionalTestingBegin"),
            &raw mut __FUNCTION_PTRS
                .a_phased_automation_actor_base_on_functional_testing_begin,
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
            &raw mut __FUNCTION_PTRS
                .u_functional_test_utility_library_trace_channel_test_util,
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
            &raw mut __FUNCTION_PTRS.u_ground_truth_data_save_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetObject"),
            &raw mut __FUNCTION_PTRS.u_ground_truth_data_reset_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadObject"),
            &raw mut __FUNCTION_PTRS.u_ground_truth_data_load_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanModify"),
            &raw mut __FUNCTION_PTRS.u_ground_truth_data_can_modify,
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
            &raw mut __FUNCTION_PTRS.u_trace_query_test_results_to_string,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_editor_task_is_valid_task,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_editor_task_is_valid_task,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_editor_task_is_task_done,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_editor_task_is_task_done,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_take_high_res_screenshot,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_take_high_res_screenshot,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_take_automation_screenshot_of_ui,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_take_automation_screenshot_of_ui,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_take_automation_screenshot_at_camera,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_take_automation_screenshot_at_camera,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_take_automation_screenshot,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_take_automation_screenshot,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_test_telemetry_storage,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_test_telemetry_storage,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_scalability_quality_to_low,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_scalability_quality_to_low,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_scalability_quality_to_epic,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_scalability_quality_to_epic,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_scalability_quality_level_relative_to_max,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_scalability_quality_level_relative_to_max,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_editor_viewport_visualize_buffer,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_editor_viewport_visualize_buffer,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_editor_viewport_view_mode,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_editor_viewport_view_mode,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_editor_active_viewport_wireframe_opacity,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_editor_active_viewport_wireframe_opacity,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_editor_active_viewport_view_mode,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_set_editor_active_viewport_view_mode,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_stat_inc_max,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_stat_inc_max,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_stat_inc_average,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_stat_inc_average,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_stat_exc_max,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_stat_exc_max,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_stat_exc_average,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_stat_exc_average,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_stat_call_count,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_stat_call_count,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_editor_active_viewport_wireframe_opacity,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_editor_active_viewport_wireframe_opacity,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_editor_active_viewport_view_mode,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_editor_active_viewport_view_mode,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_default_screenshot_options_for_rendering,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_default_screenshot_options_for_rendering,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_default_screenshot_options_for_gameplay,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_get_default_screenshot_options_for_gameplay,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_finish_loading_before_screenshot,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_finish_loading_before_screenshot,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_enable_stat_group,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_enable_stat_group,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_disable_stat_group,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_disable_stat_group,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_compare_image_against_reference,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_compare_image_against_reference,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_automation_wait_for_loading,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_automation_wait_for_loading,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_are_automated_tests_running,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::functional_testing::UAutomationBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_are_automated_tests_running,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_test_warning,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_test_warning,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_test_telemetry_data,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_test_telemetry_data,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_test_info,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_test_info,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_test_error,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_test_error,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_expected_plain_log_message,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_expected_plain_log_message,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_expected_plain_log_error,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_expected_plain_log_error,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_expected_log_message,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_expected_log_message,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_expected_log_error,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_blueprint_function_library_add_expected_log_error,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_set_time_limit,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_set_time_limit,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_set_console_variable_from_integer,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_set_console_variable_from_integer,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_set_console_variable_from_float,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_set_console_variable_from_float,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_set_console_variable_from_boolean,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_set_console_variable_from_boolean,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_set_console_variable,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_set_console_variable,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_register_auto_destroy_actor,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_register_auto_destroy_actor,
                __buffer,
            )
        };
    }
    pub fn receive_test_finished(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_receive_test_finished,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_receive_test_finished,
                __buffer,
            )
        };
    }
    pub fn receive_start_test(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_receive_start_test,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_receive_start_test,
                __buffer,
            )
        };
    }
    pub fn receive_prepare_test(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_receive_prepare_test,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_receive_prepare_test,
                __buffer,
            )
        };
    }
    pub fn on_wants_re_run_check(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_on_wants_re_run_check,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_on_wants_re_run_check,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn on_additional_test_finished_message_request(
        &self,
        test_result: EFunctionalTestResult,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_on_additional_test_finished_message_request,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_on_additional_test_finished_message_request,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn log_message(&mut self, message: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_log_message,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_log_message,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_is_running,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_is_running,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_ready(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_is_ready,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_is_ready,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_is_enabled_in_world,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_is_enabled_in_world,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_is_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_is_enabled,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_get_current_rerun_reason,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_get_current_rerun_reason,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_finish_test,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_finish_test,
                __buffer,
            )
        };
    }
    pub fn debug_gather_relevant_actors(
        &self,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_debug_gather_relevant_actors,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_debug_gather_relevant_actors,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_value_int,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_value_int,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_value_float,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_value_float,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_value_double,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_value_double,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_value_date_time,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_value_date_time,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_true,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_true,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_vector4,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_vector4,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_vector2_d,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_vector2_d,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_vector,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_vector,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_transform,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_transform,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_string,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_string,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_rotator,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_rotator,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_quat,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_quat,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_plane,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_plane,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_matrix,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_matrix,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_box2_d,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_not_equal_box2_d,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_is_valid,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_is_valid,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_false,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_false,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_vector4,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_vector4,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_vector2_d,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_vector2_d,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_vector,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_vector,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_transform,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_transform,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_trace_query_results,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_trace_query_results,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_string,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_string,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_rotator_orientation,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_rotator_orientation,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_rotator,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_rotator,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_quat,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_quat,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_plane,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_plane,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_object,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_object,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_name,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_name,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_matrix,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_matrix,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_int,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_int,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_float,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_float,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_double,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_double,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_box2_d,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_box2_d,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_bool,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_assert_equal_bool,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_add_warning,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_add_warning,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_add_rerun,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_add_rerun,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_add_info,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_add_info,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_add_error,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_test_add_error,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_ai_test_base_is_one_of_spawned_pawns,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_functional_ai_test_base_is_one_of_spawned_pawns,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_write_log_file,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_write_log_file,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_trigger_gpu_trace_if_record_falls_below_budget,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_trigger_gpu_trace_if_record_falls_below_budget,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_tick,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_tick,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_stop_cpu_profiling,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_stop_cpu_profiling,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_start_cpu_profiling,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_start_cpu_profiling,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_sample,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_sample,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_on_begin_tests,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_on_begin_tests,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_on_all_tests_complete,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_on_all_tests_complete,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_is_recording,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_is_recording,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_is_current_record_within_render_thread_budget,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_is_current_record_within_render_thread_budget,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_is_current_record_within_gpu_budget,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_is_current_record_within_gpu_budget,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_is_current_record_within_game_thread_budget,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_is_current_record_within_game_thread_budget,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_end_stats_file,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_end_stats_file,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_end_recording_baseline,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_end_recording_baseline,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_end_recording,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_end_recording,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_begin_stats_file,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_begin_stats_file,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_begin_recording_baseline,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_begin_recording_baseline,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_begin_recording,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_automation_performace_helper_begin_recording,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_functional_testing_manager_run_all_functional_tests,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_functional_testing_manager_run_all_functional_tests,
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
    pub fn on_functional_testing_complete(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_phased_automation_actor_base_on_functional_testing_complete,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_phased_automation_actor_base_on_functional_testing_complete,
                __buffer,
            )
        };
    }
    pub fn on_functional_testing_begin(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_phased_automation_actor_base_on_functional_testing_begin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .a_phased_automation_actor_base_on_functional_testing_begin,
                __buffer,
            )
        };
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_functional_test_utility_library_trace_channel_test_util,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_functional_test_utility_library_trace_channel_test_util,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_ground_truth_data_save_object,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_ground_truth_data_save_object,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_ground_truth_data_reset_object,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_ground_truth_data_reset_object,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_ground_truth_data_load_object,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_ground_truth_data_load_object,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_ground_truth_data_can_modify,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_ground_truth_data_can_modify,
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
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_trace_query_test_results_to_string,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::functional_testing::__FUNCTION_PTRS
                    .u_trace_query_test_results_to_string,
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
