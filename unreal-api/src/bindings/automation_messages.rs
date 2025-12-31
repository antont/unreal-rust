#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FAutomationWorkerMessageBase {
    pub instance_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerFindWorkers {
    pub changelist: i32,
    pub game_name: FString,
    pub process_name: FString,
    pub session_id: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerFindWorkersResponse {
    pub device_name: FString,
    pub instance_name: FString,
    pub platform: FString,
    pub os_version_name: FString,
    pub model_name: FString,
    pub gpu_name: FString,
    pub cpu_model_name: FString,
    pub ram_in_gb: u32,
    pub render_mode_name: FString,
    pub session_id: crate::bindings::core_u_object::FGuid,
    pub rhi_name: FString,
}
#[repr(C, align(4))]
pub struct FAutomationWorkerWorkerOffline {}
#[repr(C, align(4))]
pub struct FAutomationWorkerPing {}
#[repr(C, align(4))]
pub struct FAutomationWorkerStartTestSession {}
#[repr(C, align(4))]
pub struct FAutomationWorkerStopTestSession {}
#[repr(C, align(4))]
pub struct FAutomationWorkerStopTests {}
#[repr(C, align(4))]
pub struct FAutomationWorkerPong {}
#[repr(C, align(4))]
pub struct FAutomationWorkerRequestTests {
    pub developer_directory_included: bool,
    pub requested_test_flags: u32,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerSingleTestReply {
    pub display_name: FString,
    pub full_test_path: FString,
    pub test_name: FString,
    pub test_parameter: FString,
    pub source_file: FString,
    pub source_file_line: i32,
    pub asset_path: FString,
    pub open_command: FString,
    pub test_flags: u32,
    pub num_participants_required: u32,
    pub test_tags: FString,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerRequestTestsReplyComplete {
    pub tests: TArray<FAutomationWorkerSingleTestReply>,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerRunTests {
    pub execution_count: u32,
    pub role_index: i32,
    pub test_name: FString,
    pub beautified_test_name: FString,
    pub full_test_path: FString,
    pub b_send_analytics: bool,
    pub b_prune_logs_on_success: bool,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerRunTestsReply {
    pub test_name: FString,
    pub entries: TArray<crate::bindings::core_u_object::FAutomationExecutionEntry>,
    pub warning_total: i32,
    pub error_total: i32,
    pub duration: f32,
    pub execution_count: u32,
    pub state: crate::bindings::automation_test::EAutomationState,
}
#[repr(C, align(4))]
pub struct FAutomationWorkerRequestNextNetworkCommand {
    pub execution_count: u32,
}
#[repr(C, align(4))]
pub struct FAutomationWorkerNextNetworkCommandReply {}
#[repr(C, align(8))]
pub struct FAutomationScreenshotMetadata {
    pub screen_shot_name: FString,
    pub variant_name: FString,
    pub context: FString,
    pub test_name: FString,
    pub notes: FString,
    pub id: crate::bindings::core_u_object::FGuid,
    pub commit: FString,
    pub width: i32,
    pub height: i32,
    pub rhi: FString,
    pub platform: FString,
    pub feature_level: FString,
    pub b_is_stereo: bool,
    pub b_is_substrate: bool,
    pub vendor: FString,
    pub adapter_name: FString,
    pub adapter_internal_driver_version: FString,
    pub adapter_user_driver_version: FString,
    pub unique_device_id: FString,
    pub resolution_quality: f32,
    pub view_distance_quality: i32,
    pub anti_aliasing_quality: i32,
    pub shadow_quality: i32,
    pub global_illumination_quality: i32,
    pub reflection_quality: i32,
    pub post_process_quality: i32,
    pub texture_quality: i32,
    pub effects_quality: i32,
    pub foliage_quality: i32,
    pub shading_quality: i32,
    pub b_has_comparison_rules: bool,
    pub tolerance_red: u8,
    pub tolerance_green: u8,
    pub tolerance_blue: u8,
    pub tolerance_alpha: u8,
    pub tolerance_min_brightness: u8,
    pub tolerance_max_brightness: u8,
    pub maximum_local_error: f32,
    pub maximum_global_error: f32,
    pub b_ignore_anti_aliasing: bool,
    pub b_ignore_colors: bool,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerScreenImage {
    pub screen_image: TArray<u8>,
    pub frame_trace: TArray<u8>,
    pub screen_shot_name: FString,
    pub metadata: FAutomationScreenshotMetadata,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerImageComparisonResults {
    pub unique_id: crate::bindings::core_u_object::FGuid,
    pub screenshot_path: FString,
    pub b_new: bool,
    pub b_similar: bool,
    pub max_local_difference: f64,
    pub global_difference: f64,
    pub error_message: FString,
    pub incoming_file_path: FString,
    pub report_comparison_file_path: FString,
    pub report_approved_file_path: FString,
    pub report_incoming_file_path: FString,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerTestDataRequest {
    pub data_type: FString,
    pub data_platform: FString,
    pub data_test_name: FString,
    pub data_name: FString,
    pub json_data: FString,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerTestDataResponse {
    pub json_data: FString,
    pub b_is_new: bool,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerPerformanceDataRequest {
    pub platform: FString,
    pub hardware: FString,
    pub test_name: FString,
    pub data_points: TArray<f64>,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerPerformanceDataResponse {
    pub b_success: bool,
    pub error_message: FString,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerTelemetryItem {
    pub data_point: FString,
    pub measurement: f64,
    pub context: FString,
}
#[repr(C, align(8))]
pub struct FAutomationWorkerTelemetryData {
    pub storage: FString,
    pub configuration: FString,
    pub platform: FString,
    pub test_name: FString,
    pub items: TArray<FAutomationWorkerTelemetryItem>,
}
