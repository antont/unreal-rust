#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FComparisonToleranceAmount {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
    pub min_brightness: u8,
    pub max_brightness: u8,
}
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
#[repr(C, align(1))]
pub struct FAutomationWaitForLoadingOptions {
    pub wait_for_replication_to_settle: bool,
}
#[repr(C, align(8))]
pub struct FAITestSpawnInfoBase {
    pub spawn_location: UPtr<crate::bindings::engine::AActor>,
    pub number_to_spawn: i32,
    pub spawn_delay: f32,
    pub pre_spawn_delay: f32,
}
#[repr(C, align(8))]
pub struct FAITestSpawnInfo {
    pub pawn_class: TSubclassOf<crate::bindings::engine::APawn>,
    pub controller_class: TSubclassOf<crate::bindings::ai_module::AAIController>,
    pub team_id: crate::bindings::ai_module::FGenericTeamId,
    pub behavior_tree: UPtr<crate::bindings::ai_module::UBehaviorTree>,
}
#[repr(C, align(4))]
pub struct FPendingDelayedSpawn {}
#[repr(C, align(8))]
pub struct FAITestSpawnSetBase {
    pub name: FName,
    pub flags_20: u8,
    pub fallback_spawn_location: UPtr<crate::bindings::engine::AActor>,
}
#[repr(C, align(8))]
pub struct FAITestSpawnSet {
    pub spawn_info_container: TArray<FAITestSpawnInfo>,
}
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
#[repr(C, align(4))]
pub struct FTraceQueryTestNames {
    pub component_name: FName,
    pub physical_material_name: FName,
    pub actor_name: FName,
}
#[repr(C, align(8))]
pub struct FTraceQueryTestResultsInnerMost {
    pub single_hit: crate::bindings::engine::FHitResult,
    pub single_names: FTraceQueryTestNames,
    pub b_single_result: bool,
    pub multi_hits: TArray<crate::bindings::engine::FHitResult>,
    pub multi_names: TArray<FTraceQueryTestNames>,
    pub b_multi_result: bool,
}
#[repr(C, align(8))]
pub struct FTraceQueryTestResultsInner {
    pub line_results: FTraceQueryTestResultsInnerMost,
    pub sphere_results: FTraceQueryTestResultsInnerMost,
    pub capsule_results: FTraceQueryTestResultsInnerMost,
    pub box_results: FTraceQueryTestResultsInnerMost,
}
pub struct UAutomationViewSettings {
    pub anti_aliasing: bool,
    pub motion_blur: bool,
    pub temporal_aa: bool,
    pub screen_space_reflections: bool,
    pub screen_space_ao: bool,
    pub distance_field_ao: bool,
    pub contact_shadows: bool,
    pub eye_adaptation: bool,
    pub bloom: bool,
}
pub struct UAutomationEditorTask {}
pub struct UAutomationBlueprintFunctionLibrary {}
pub struct UFuncTestRenderingComponent {}
pub struct AFunctionalTest {
    pub test_label: FString,
    pub author: FString,
    pub description: FString,
    pub test_tags: FString,
    pub sprite_component: UPtr<crate::bindings::engine::UBillboardComponent>,
    pub flags_1224: u8,
    pub persistent_level_name: FName,
    pub log_error_handling: EFunctionalTestLogHandling,
    pub log_warning_handling: EFunctionalTestLogHandling,
    pub observation_point: UPtr<crate::bindings::engine::AActor>,
    pub flags_1256: u8,
    pub random_numbers_stream: crate::bindings::core_u_object::FRandomStream,
    pub result: EFunctionalTestResult,
    pub preparation_time_limit: f32,
    pub time_limit: f32,
    pub times_up_message: FText,
    pub times_up_result: EFunctionalTestResult,
    pub on_test_prepare: FFunctionalTest_OnTestPrepare,
    pub on_test_start: FFunctionalTest_OnTestStart,
    pub on_test_finished: FFunctionalTest_OnTestFinished,
    pub auto_destroy_actors: TArray<UPtr<crate::bindings::engine::AActor>>,
    pub render_comp: UPtr<UFuncTestRenderingComponent>,
    pub test_name: UPtr<crate::bindings::engine::UTextRenderComponent>,
    pub b_is_running: bool,
    pub total_time: f32,
}
pub struct AFunctionalAITestBase {
    pub spawn_location_randomization_range: f32,
    pub spawned_pawns: TArray<UPtr<crate::bindings::engine::APawn>>,
    pub pending_delayed_spawns: TArray<FPendingDelayedSpawn>,
    pub current_spawn_set_index: i32,
    pub current_spawn_set_name: FString,
    pub on_ai_spawned: FFunctionalAITestBase_OnAISpawned,
    pub on_all_ais_pawned: FFunctionalAITestBase_OnAllAISPawned,
    pub nav_mesh_debug_origin: crate::bindings::core_u_object::FVector,
    pub nav_mesh_debug_extent: crate::bindings::core_u_object::FVector,
    pub flags_1704: u8,
}
pub struct AFunctionalAITest {
    pub spawn_sets: TArray<FAITestSpawnSet>,
}
pub struct UAutomationPerformaceHelper {}
pub struct AFunctionalTestGameMode {}
pub struct UFunctionalTestingManager {
    pub tests_left: TArray<UPtr<AFunctionalTest>>,
    pub all_tests: TArray<UPtr<AFunctionalTest>>,
    pub on_setup_tests: FFunctionalTestingManager_OnSetupTests,
    pub on_tests_complete: FFunctionalTestingManager_OnTestsComplete,
    pub on_tests_begin: FFunctionalTestingManager_OnTestsBegin,
}
pub struct APhasedAutomationActorBase {}
pub struct AFunctionalTestLevelScript {}
pub struct UFunctionalTestUtilityLibrary {}
pub struct AScreenshotFunctionalTestBase {
    pub notes: FString,
    pub screenshot_camera: UPtr<crate::bindings::engine::UCameraComponent>,
    pub screenshot_options: FAutomationScreenshotOptions,
}
pub struct AFunctionalUIScreenshotTest {
    pub widget_class: TSubclassOf<crate::bindings::umg::UUserWidget>,
    pub spawned_widget: UPtr<crate::bindings::umg::UUserWidget>,
    pub widget_location: EWidgetTestAppearLocation,
    pub screenshot_rt: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    pub b_hide_debug_canvas: bool,
}
pub struct UGroundTruthData {
    pub b_reset_ground_truth: bool,
    pub object_data: UPtr<crate::bindings::core_u_object::UObject>,
}
pub struct AScreenshotFunctionalTest {
    pub b_camera_cut_on_screenshot_prep: bool,
}
pub struct UTestPhaseComponent {}
pub struct UTraceQueryTestResults {
    pub channel_results: FTraceQueryTestResultsInner,
    pub object_results: FTraceQueryTestResultsInner,
    pub profile_results: FTraceQueryTestResultsInner,
    pub batch_options: FTraceChannelTestBatchOptions,
}
pub struct FFunctionalTest_OnTestPrepare;
pub struct FFunctionalTest_OnTestStart;
pub struct FFunctionalTest_OnTestFinished;
pub struct FFunctionalAITestBase_OnAISpawned;
pub struct FFunctionalAITestBase_OnAllAISPawned;
pub struct FFunctionalTestingManager_OnSetupTests;
pub struct FFunctionalTestingManager_OnTestsComplete;
pub struct FFunctionalTestingManager_OnTestsBegin;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EComparisonTolerance(pub u8);
impl EComparisonTolerance {
    pub const ZERO: EComparisonTolerance = EComparisonTolerance(0);
    pub const LOW: EComparisonTolerance = EComparisonTolerance(1);
    pub const MEDIUM: EComparisonTolerance = EComparisonTolerance(2);
    pub const HIGH: EComparisonTolerance = EComparisonTolerance(3);
    pub const CUSTOM: EComparisonTolerance = EComparisonTolerance(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWidgetTestAppearLocation(pub i32);
impl EWidgetTestAppearLocation {
    pub const VIEWPORT: EWidgetTestAppearLocation = EWidgetTestAppearLocation(0);
    pub const PLAYER_SCREEN: EWidgetTestAppearLocation = EWidgetTestAppearLocation(1);
}
