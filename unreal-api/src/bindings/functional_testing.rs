#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
impl UAutomationViewSettings {}
#[repr(C, align(8))]
pub struct UAutomationEditorTask {
    __padding_end: [u8; 56],
}
impl UAutomationEditorTask {}
#[repr(C, align(8))]
pub struct UAutomationBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UAutomationBlueprintFunctionLibrary {}
#[repr(C, align(16))]
pub struct UFuncTestRenderingComponent {
    __padding_end: [u8; 1504],
}
impl UFuncTestRenderingComponent {}
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
impl AFunctionalTest {}
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
impl AFunctionalAITestBase {}
#[repr(C, align(8))]
pub struct AFunctionalAITest {
    #[doc(hidden)]
    __padding_1720: [u8; 1720],
    pub spawn_sets: TArray<FAITestSpawnSet>,
}
impl AFunctionalAITest {}
#[repr(C, align(8))]
pub struct UAutomationPerformaceHelper {
    __padding_end: [u8; 104],
}
impl UAutomationPerformaceHelper {}
#[repr(C, align(8))]
pub struct AFunctionalTestGameMode {
    __padding_end: [u8; 1296],
}
impl AFunctionalTestGameMode {}
#[repr(C, align(8))]
pub struct UFunctionalTestingManager {
    __padding_end: [u8; 240],
}
impl UFunctionalTestingManager {}
#[repr(C, align(8))]
pub struct APhasedAutomationActorBase {
    __padding_end: [u8; 1136],
}
impl APhasedAutomationActorBase {}
#[repr(C, align(8))]
pub struct AFunctionalTestLevelScript {
    __padding_end: [u8; 1144],
}
impl AFunctionalTestLevelScript {}
#[repr(C, align(8))]
pub struct UFunctionalTestUtilityLibrary {
    __padding_end: [u8; 48],
}
impl UFunctionalTestUtilityLibrary {}
#[repr(C, align(8))]
pub struct AScreenshotFunctionalTestBase {
    #[doc(hidden)]
    __padding_1544: [u8; 1544],
    pub notes: FString,
    pub screenshot_camera: UPtr<crate::bindings::engine::UCameraComponent>,
    pub screenshot_options: FAutomationScreenshotOptions,
    __padding_end: [u8; 32],
}
impl AScreenshotFunctionalTestBase {}
#[repr(C, align(8))]
pub struct AFunctionalUIScreenshotTest {
    __padding_end: [u8; 1752],
}
impl AFunctionalUIScreenshotTest {}
#[repr(C, align(8))]
pub struct UGroundTruthData {
    __padding_end: [u8; 64],
}
impl UGroundTruthData {}
#[repr(C, align(8))]
pub struct AScreenshotFunctionalTest {
    #[doc(hidden)]
    __padding_1688: [u8; 1688],
    pub b_camera_cut_on_screenshot_prep: bool,
    __padding_end: [u8; 71],
}
impl AScreenshotFunctionalTest {}
#[repr(C, align(16))]
pub struct UTestPhaseComponent {
    __padding_end: [u8; 656],
}
impl UTestPhaseComponent {}
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
impl UTraceQueryTestResults {}
#[repr(transparent)]
pub struct FFunctionalTest_OnTestPrepare {
    _opague: u8,
}
#[repr(transparent)]
pub struct FFunctionalTest_OnTestStart {
    _opague: u8,
}
#[repr(transparent)]
pub struct FFunctionalTest_OnTestFinished {
    _opague: u8,
}
#[repr(transparent)]
pub struct FFunctionalAITestBase_OnAISpawned {
    _opague: u8,
}
#[repr(transparent)]
pub struct FFunctionalAITestBase_OnAllAISPawned {
    _opague: u8,
}
#[repr(transparent)]
pub struct FFunctionalTestingManager_OnSetupTests {
    _opague: u8,
}
#[repr(transparent)]
pub struct FFunctionalTestingManager_OnTestsComplete {
    _opague: u8,
}
#[repr(transparent)]
pub struct FFunctionalTestingManager_OnTestsBegin {
    _opague: u8,
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
