#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAutomationArtifact {
    pub id: crate::bindings::core_u_object::FGuid,
    pub name: FString,
    pub ty: EAutomationArtifactType,
    pub files: TMap<FString, FString>,
}
#[repr(C, align(8))]
pub struct FAutomatedTestResult {
    pub test_display_name: FString,
    pub full_test_path: FString,
    pub tags: TArray<FString>,
    pub state: crate::bindings::automation_test::EAutomationState,
    pub device_instance: TArray<FString>,
    pub duration: f32,
    pub date_time: crate::bindings::core_u_object::FDateTime,
    pub entries: TArray<crate::bindings::core_u_object::FAutomationExecutionEntry>,
    pub warnings: i32,
    pub errors: i32,
    pub artifacts: TArray<FAutomationArtifact>,
}
#[repr(C, align(8))]
pub struct FAutomatedTestPassResults {
    pub devices: TArray<FAutomationDeviceInfo>,
    pub report_created_on: crate::bindings::core_u_object::FDateTime,
    pub succeeded: i32,
    pub succeeded_with_warnings: i32,
    pub failed: i32,
    pub not_run: i32,
    pub in_process: i32,
    pub total_duration: f32,
    pub comparison_exported: bool,
    pub comparison_export_directory: FString,
    pub tests: TArray<FAutomatedTestResult>,
}
#[repr(C, align(8))]
pub struct FAutomationDeviceInfo {
    pub device_name: FString,
    pub instance: crate::bindings::core_u_object::FGuid,
    pub instance_name: FString,
    pub platform: FString,
    pub os_version: FString,
    pub model: FString,
    pub gpu: FString,
    pub cpu_model: FString,
    pub ram_in_gb: u32,
    pub render_mode: FString,
    pub rhi: FString,
    pub app_instance_log: FString,
}
#[repr(C, align(8))]
pub struct FAutomatedTestFilterBase {
    pub contains: FString,
    pub match_from_start: bool,
    pub match_from_end: bool,
}
#[repr(C, align(8))]
pub struct FAutomatedTestTagFilter {}
#[repr(C, align(8))]
pub struct FAutomatedTestFilter {
    pub exclude: TArray<FAutomatedTestFilterBase>,
    pub tags: TArray<FAutomatedTestTagFilter>,
}
#[repr(C, align(8))]
pub struct FAutomatedTestGroup {
    pub name: FString,
    pub filters: TArray<FAutomatedTestFilter>,
}
pub struct UAutomationControllerSettings {
    pub groups: TArray<FAutomatedTestGroup>,
    pub b_suppress_log_errors: bool,
    pub b_suppress_log_warnings: bool,
    pub b_elevate_log_warnings_to_errors: bool,
    pub suppressed_log_categories: TArray<FString>,
    pub b_keep_pie_open: bool,
    pub b_auto_expand_single_item_subgroups: bool,
    pub b_sort_tests_by_failure: bool,
    pub b_prune_logs_on_success: bool,
    pub b_treat_log_warnings_as_test_errors: bool,
    pub check_test_interval_seconds: f32,
    pub game_instance_lost_timer_seconds: f32,
    pub telemetry_directory: FString,
    pub b_reset_telemetry_storage_on_new_session: bool,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAutomationArtifactType(pub u8);
impl EAutomationArtifactType {
    pub const NONE: EAutomationArtifactType = EAutomationArtifactType(0);
    pub const IMAGE: EAutomationArtifactType = EAutomationArtifactType(1);
    pub const COMPARISON: EAutomationArtifactType = EAutomationArtifactType(2);
}
