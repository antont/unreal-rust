#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FActorPerceptionUpdateInfo {
    pub target_id: i32,
    pub target: TWeakObjectPtr<AActor>,
    pub stimulus: FAIStimulus,
}
#[repr(C, align(8))]
pub struct FAIStimulus {
    pub age: f32,
    pub expiration_age: f32,
    pub strength: f32,
    pub stimulus_location: FVector,
    pub receiver_location: FVector,
    pub tag: FName,
    pub flags_92: u8,
}
#[repr(C, align(4))]
pub struct FAIRequestID {
    pub request_id: u32,
}
#[repr(C, align(1))]
pub struct FGenericTeamId {
    pub team_id: u8,
}
#[repr(C, align(4))]
pub struct FSimpleIndexedHandleBase {}
#[repr(C, align(4))]
pub struct FIndexedHandleBase {}
#[repr(C, align(4))]
pub struct FCompactIndexedHandleBase {}
#[repr(C, align(4))]
pub struct FSequentialIDBase {
    pub value: u32,
}
#[repr(C, align(8))]
pub struct FAIMoveRequest {
    pub goal_actor: TWeakObjectPtr<AActor>,
}
#[repr(C, align(4))]
pub struct FIntervalCountdown {
    pub interval: f32,
}
#[repr(C, align(8))]
pub struct FBehaviorTreeTemplateInfo {
    pub asset: UPtr<UBehaviorTree>,
    pub template: UPtr<UBTCompositeNode>,
}
#[repr(C, align(8))]
pub struct FBlackboardKeySelector {
    pub allowed_types: TArray<UPtr<UBlackboardKeyType>>,
    pub selected_key_name: FName,
    pub selected_key_type: TSubclassOf<UBlackboardKeyType>,
    pub selected_key_id: i32,
    pub flags_44: u8,
}
#[repr(C, align(8))]
pub struct FBlackboardEntry {
    pub entry_name: FName,
    pub entry_description: FString,
    pub entry_category: FName,
    pub key_type: UPtr<UBlackboardKeyType>,
    pub flags_56: u8,
}
#[repr(C, align(2))]
pub struct FBTDecoratorLogic {
    pub operation: EBTDecoratorLogic,
    pub number: u16,
}
#[repr(C, align(8))]
pub struct FBTCompositeChild {
    pub child_composite: UPtr<UBTCompositeNode>,
    pub child_task: UPtr<UBTTaskNode>,
    pub decorators: TArray<UPtr<UBTDecorator>>,
    pub decorator_ops: TArray<FBTDecoratorLogic>,
}
#[repr(C, align(8))]
pub struct FValueOrBlackboardKeyBase {
    pub key: FName,
}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Bool {
    pub default_value: bool,
}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Class {
    pub default_value: TSubclassOf<UObject>,
    pub base_class: TSubclassOf<UObject>,
}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Enum {
    pub default_value: u8,
    pub enum_type: UPtr<UEnum>,
    pub native_enum_type_name: FString,
}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Float {
    pub default_value: f32,
}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Int32 {
    pub default_value: i32,
}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Name {
    pub default_value: FName,
}
#[repr(C, align(8))]
pub struct FValueOrBBKey_String {
    pub default_value: FString,
}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Object {
    pub default_value: UPtr<UObject>,
    pub base_class: TSubclassOf<UObject>,
}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Rotator {
    pub default_value: FRotator,
}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Vector {
    pub default_value: FVector,
}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Struct {
    pub default_value: FInstancedStruct,
    pub b_can_edit_default_value_type: bool,
}
#[repr(C, align(8))]
pub struct FAIDataProviderValue {
    pub data_binding: UPtr<UAIDataProvider>,
    pub data_field: FName,
}
#[repr(C, align(8))]
pub struct FAIDataProviderTypedValue {
    pub property_type_deprecated: TSubclassOf<UObject>,
}
#[repr(C, align(8))]
pub struct FAIDataProviderStructValue {}
#[repr(C, align(8))]
pub struct FAIDataProviderIntValue {
    pub default_value: i32,
}
#[repr(C, align(8))]
pub struct FAIDataProviderFloatValue {
    pub default_value: f32,
}
#[repr(C, align(8))]
pub struct FAIDataProviderBoolValue {
    pub default_value: bool,
}
#[repr(C, align(8))]
pub struct FEnvQueryManagerConfig {
    pub max_allowed_testing_time: f32,
    pub b_test_queries_using_breadth: bool,
    pub query_count_warning_threshold: i32,
    pub query_count_warning_interval: f64,
    pub execution_time_warning_seconds: f64,
    pub handling_result_time_warning_seconds: f64,
    pub generation_time_warning_seconds: f64,
}
#[repr(C, align(8))]
pub struct FEnvQueryRequest {
    pub query_template: UPtr<UEnvQuery>,
    pub owner: UPtr<UObject>,
    pub world: UPtr<UWorld>,
}
#[repr(C, align(8))]
pub struct FEnvQueryInstanceCache {
    pub template: UPtr<UEnvQuery>,
    pub instance: FEnvQueryInstance,
}
#[repr(C, align(8))]
pub struct FEnvQueryResult {
    pub item_type: TSubclassOf<UEnvQueryItemType>,
    pub option_index: i32,
    pub query_id: i32,
}
#[repr(C, align(8))]
pub struct FEnvQueryInstance {
    pub world: UPtr<UWorld>,
    pub context_cache: TArray<FEnvQueryInstanceContextCacheItem>,
    pub options: TArray<FEnvQueryOptionInstance>,
    pub item_type_vector_cdo: UPtr<UEnvQueryItemType_VectorBase>,
    pub item_type_actor_cdo: UPtr<UEnvQueryItemType_ActorBase>,
}
#[repr(C, align(8))]
pub struct FEnvQueryOptionInstance {
    pub generator: UPtr<UEnvQueryGenerator>,
    pub tests: TArray<UPtr<UEnvQueryTest>>,
    pub item_type: TSubclassOf<UEnvQueryItemType>,
}
#[repr(C, align(8))]
pub struct FEnvQueryInstanceContextCacheItem {
    pub class: TSubclassOf<UObject>,
    pub context_data: FEnvQueryContextData,
}
#[repr(C, align(8))]
pub struct FEnvQueryContextData {
    pub value_type: TSubclassOf<UEnvQueryItemType>,
}
#[repr(C, align(4))]
pub struct FEnvNamedValue {
    pub param_name: FName,
    pub param_type: EAIParamType,
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FEnvDirection {
    pub line_from: TSubclassOf<UEnvQueryContext>,
    pub line_to: TSubclassOf<UEnvQueryContext>,
    pub rotation: TSubclassOf<UEnvQueryContext>,
    pub dir_mode: EEnvDirection,
}
#[repr(C, align(8))]
pub struct FEnvTraceData {
    pub version_num: i32,
    pub navigation_filter: TSubclassOf<UNavigationQueryFilter>,
    pub project_down: f32,
    pub project_up: f32,
    pub extent_x: f32,
    pub extent_y: f32,
    pub extent_z: f32,
    pub post_projection_vertical_offset: f32,
    pub trace_channel: ETraceTypeQuery,
    pub serialized_channel: ECollisionChannel,
    pub trace_profile_name: FName,
    pub trace_shape: EEnvTraceShape,
    pub trace_mode: EEnvQueryTrace,
    pub flags_60: u8,
}
#[repr(C, align(8))]
pub struct FEnvOverlapData {
    pub extent_x: f32,
    pub extent_y: f32,
    pub extent_z: f32,
    pub shape_offset: FVector,
    pub overlap_channel: ECollisionChannel,
    pub overlap_shape: EEnvOverlapShape,
    pub flags_44: u8,
}
#[repr(C, align(8))]
pub struct FAIDynamicParam {
    pub param_name: FName,
    pub param_type: EAIParamType,
    pub flags_13: u8,
    pub value: f32,
    pub bb_key: FBlackboardKeySelector,
}
#[repr(C, align(8))]
pub struct FEQSParametrizedQueryExecutionRequest {
    pub query_template: UPtr<UEnvQuery>,
    pub query_config: TArray<FAIDynamicParam>,
    pub eqs_query_blackboard_key: FBlackboardKeySelector,
    pub run_mode: EEnvQueryRunMode,
    pub flags_76: u8,
}
#[repr(C, align(4))]
pub struct FCrowdAvoidanceConfig {
    pub velocity_bias: f32,
    pub desired_velocity_weight: f32,
    pub current_velocity_weight: f32,
    pub side_bias_weight: f32,
    pub impact_time_weight: f32,
    pub impact_time_range: f32,
    pub custom_pattern_idx: u8,
    pub adaptive_divisions: u8,
    pub adaptive_rings: u8,
    pub adaptive_depth: u8,
}
#[repr(C, align(8))]
pub struct FCrowdAvoidanceSamplingPattern {
    pub angles: TArray<f32>,
    pub radii: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FRecastGraphWrapper {
    pub recast_nav_mesh_actor: UPtr<ARecastNavMesh>,
}
#[repr(C, align(8))]
pub struct FActorPerceptionBlueprintInfo {
    pub target: UPtr<AActor>,
    pub last_sensed_stimuli: TArray<FAIStimulus>,
    pub flags_24: u8,
}
#[repr(C, align(4))]
pub struct FAISenseAffiliationFilter {
    pub flags_0: u8,
}
#[repr(C, align(8))]
pub struct FAIDamageEvent {
    pub amount: f32,
    pub location: FVector,
    pub hit_location: FVector,
    pub damaged_actor: UPtr<AActor>,
    pub instigator: UPtr<AActor>,
    pub tag: FName,
}
#[repr(C, align(8))]
pub struct FAINoiseEvent {
    pub noise_location: FVector,
    pub loudness: f32,
    pub max_range: f32,
    pub instigator: UPtr<AActor>,
    pub tag: FName,
}
#[repr(C, align(8))]
pub struct FAIPredictionEvent {
    pub requestor: UPtr<AActor>,
    pub predicted_actor: UPtr<AActor>,
}
#[repr(C, align(8))]
pub struct FAISightEvent {
    pub seen_actor: UPtr<AActor>,
    pub observer: UPtr<AActor>,
}
#[repr(C, align(8))]
pub struct FAITeamStimulusEvent {
    pub broadcaster: UPtr<AActor>,
    pub enemy: UPtr<AActor>,
}
#[repr(C, align(8))]
pub struct FAITouchEvent {
    pub touch_receiver: UPtr<AActor>,
    pub other_actor: UPtr<AActor>,
}
pub struct UAIAsyncTaskBlueprintProxy {
    pub on_success: FAIAsyncTaskBlueprintProxy_OnSuccess,
    pub on_fail: FAIAsyncTaskBlueprintProxy_OnFail,
}
pub struct UAIResourceInterface {}
pub struct IAIResourceInterface {}
pub struct UAISenseBlueprintListener {}
pub struct UAISenseConfig {
    pub debug_color: FColor,
    pub max_age: f32,
    pub flags_56: u8,
}
pub struct UAISenseConfig_Blueprint {
    pub implementation: TSubclassOf<UAISense_Blueprint>,
}
pub struct UAISenseConfig_Hearing {
    pub implementation: TSubclassOf<UAISense_Hearing>,
    pub hearing_range: f32,
    pub lo_s_hearing_range: f32,
    pub flags_96: u8,
    pub detection_by_affiliation: FAISenseAffiliationFilter,
}
pub struct UAISenseConfig_Prediction {}
pub struct UAISenseConfig_Sight {
    pub implementation: TSubclassOf<UAISense_Sight>,
    pub sight_radius: f32,
    pub lose_sight_radius: f32,
    pub peripheral_vision_angle_degrees: f32,
    pub detection_by_affiliation: FAISenseAffiliationFilter,
    pub auto_success_range_from_last_seen_location: f32,
    pub point_of_view_backward_offset: f32,
    pub near_clipping_radius: f32,
}
pub struct UAISenseConfig_Team {}
pub struct UAISenseConfig_Touch {
    pub detection_by_affiliation: FAISenseAffiliationFilter,
}
pub struct UAISenseEvent {}
pub struct UAISenseEvent_Damage {
    pub event: FAIDamageEvent,
}
pub struct UAISenseEvent_Hearing {
    pub event: FAINoiseEvent,
}
pub struct UCrowdAgentInterface {}
pub struct ICrowdAgentInterface {}
pub struct UEQSQueryResultSourceInterface {}
pub struct IEQSQueryResultSourceInterface {}
pub struct UGenericTeamAgentInterface {}
pub struct IGenericTeamAgentInterface {}
pub struct AAIController {
    pub flags_1376: u8,
    pub path_following_component: UPtr<UPathFollowingComponent>,
    pub brain_component: UPtr<UBrainComponent>,
    pub perception_component: UPtr<UAIPerceptionComponent>,
    pub blackboard: UPtr<UBlackboardComponent>,
    pub cached_gameplay_tasks_component: UPtr<UGameplayTasksComponent>,
    pub default_navigation_filter_class: TSubclassOf<UNavigationQueryFilter>,
    pub receive_move_completed: FAIController_ReceiveMoveCompleted,
}
pub struct UAIResource_Movement {}
pub struct UAIResource_Logic {}
pub struct UAISubsystem {
    pub ai_system: UPtr<UAISystem>,
}
pub struct UAISystem {
    pub perception_system_class_name: FSoftClassPath,
    pub hot_spot_manager_class_name: FSoftClassPath,
    pub env_query_manager_class_name: FSoftClassPath,
    pub acceptance_radius: f32,
    pub pathfollowing_regular_path_point_acceptance_radius: f32,
    pub pathfollowing_nav_link_acceptance_radius: f32,
    pub b_finish_move_on_goal_overlap: bool,
    pub b_accept_partial_paths: bool,
    pub b_allow_strafing: bool,
    pub b_allow_controllers_as_eqs_querier: bool,
    pub b_enable_debugger_plugin: bool,
    pub b_forget_stale_actors: bool,
    pub b_add_blackboard_self_key: bool,
    pub b_clear_bb_entry_on_bteqs_fail: bool,
    pub b_blackboard_key_decorator_allows_none_as_value: bool,
    pub default_blackboard: TSoftObjectPtr<UBlackboardData>,
    pub default_sight_collision_channel: ECollisionChannel,
    pub behavior_tree_manager: UPtr<UBehaviorTreeManager>,
    pub environment_query_manager: UPtr<UEnvQueryManager>,
    pub perception_system: UPtr<UAIPerceptionSystem>,
    pub all_proxy_objects: TArray<UPtr<UAIAsyncTaskBlueprintProxy>>,
    pub hot_spot_manager: UPtr<UAIHotSpotManager>,
    pub nav_local_grids: UPtr<UNavLocalGridManager>,
}
pub struct UBehaviorTree {
    pub root_node: UPtr<UBTCompositeNode>,
    pub bt_graph: UPtr<UEdGraph>,
    pub last_edited_documents: TArray<FEditedDocumentInfo>,
    pub blackboard_asset: UPtr<UBlackboardData>,
    pub root_decorators: TArray<UPtr<UBTDecorator>>,
    pub root_decorator_ops: TArray<FBTDecoratorLogic>,
}
pub struct UBrainComponent {
    pub blackboard_comp: UPtr<UBlackboardComponent>,
    pub ai_owner: UPtr<AAIController>,
}
pub struct UBehaviorTreeComponent {
    pub node_instances: TArray<UPtr<UBTNode>>,
    pub default_behavior_tree_asset: UPtr<UBehaviorTree>,
}
pub struct UBehaviorTreeManager {
    pub max_debugger_steps: i32,
    pub loaded_templates: TArray<FBehaviorTreeTemplateInfo>,
    pub active_components: TArray<UPtr<UBehaviorTreeComponent>>,
}
pub struct UBehaviorTreeTypes {}
pub struct UBlackboardAssetProvider {}
pub struct IBlackboardAssetProvider {}
pub struct UBlackboardComponent {
    pub brain_comp: UPtr<UBrainComponent>,
    pub default_blackboard_asset: UPtr<UBlackboardData>,
    pub blackboard_asset: UPtr<UBlackboardData>,
    pub key_instances: TArray<UPtr<UBlackboardKeyType>>,
}
pub struct UBlackboardData {
    pub parent: UPtr<UBlackboardData>,
    pub parent_keys: TArray<FBlackboardEntry>,
    pub keys: TArray<FBlackboardEntry>,
    pub flags_96: u8,
}
pub struct UBlackboardKeyType {}
pub struct UBlackboardKeyType_Bool {
    pub b_default_value: bool,
}
pub struct UBlackboardKeyType_Class {
    pub base_class: TSubclassOf<UObject>,
    pub default_value: TSubclassOf<UObject>,
}
pub struct UBlackboardKeyType_Enum {
    pub enum_type: UPtr<UEnum>,
    pub enum_name: FString,
    pub default_value: u8,
    pub flags_84: u8,
}
pub struct UBlackboardKeyType_Float {
    pub default_value: f32,
}
pub struct UBlackboardKeyType_Int {
    pub default_value: i32,
}
pub struct UBlackboardKeyType_Name {
    pub default_value: FName,
}
pub struct UBlackboardKeyType_NativeEnum {
    pub enum_name: FString,
    pub enum_type: UPtr<UEnum>,
}
pub struct UBlackboardKeyType_Object {
    pub base_class: TSubclassOf<UObject>,
    pub default_value: UPtr<UObject>,
}
pub struct UBlackboardKeyType_Rotator {
    pub default_value: FRotator,
    pub b_use_default_value: bool,
}
pub struct UBlackboardKeyType_String {
    pub string_value: FString,
    pub default_value: FString,
}
pub struct UBlackboardKeyType_Struct {
    pub default_value: FInstancedStruct,
    pub value: FInstancedStruct,
}
pub struct UBlackboardKeyType_Vector {
    pub default_value: FVector,
    pub b_use_default_value: bool,
}
pub struct UBTNode {
    pub node_name: FString,
    pub tree_asset: UPtr<UBehaviorTree>,
    pub parent_node: UPtr<UBTCompositeNode>,
    pub execution_index: u16,
    pub memory_offset: u16,
    pub tree_depth: u8,
    pub flags_101: u8,
}
pub struct UBTAuxiliaryNode {}
pub struct UBTCompositeNode {
    pub children: TArray<FBTCompositeChild>,
    pub services: TArray<UPtr<UBTService>>,
    pub flags_136: u8,
}
pub struct UBTDecorator {
    pub flags_112: u8,
    pub flow_abort_mode: EBTFlowAbortMode,
}
pub struct UBTFunctionLibrary {}
pub struct UBTService {
    pub interval: f32,
    pub random_deviation: f32,
    pub flags_120: u8,
}
pub struct UBTTaskNode {
    pub services: TArray<UPtr<UBTService>>,
    pub flags_120: u8,
}
pub struct UBTComposite_Selector {}
pub struct UBTComposite_Sequence {}
pub struct UBTComposite_SimpleParallel {
    pub finish_mode: EBTParallelMode,
}
pub struct UBTDecorator_BlackboardBase {
    pub blackboard_key: FBlackboardKeySelector,
}
pub struct UBTDecorator_Blackboard {
    pub int_value: i32,
    pub float_value: f32,
    pub string_value: FString,
    pub cached_description: FString,
    pub operation_type: u8,
    pub notify_observer: EBTBlackboardRestart,
    pub basic_operation: EBasicKeyOperation,
    pub arithmetic_operation: EArithmeticKeyOperation,
    pub text_operation: ETextKeyOperation,
}
pub struct UBTDecorator_BlueprintBase {
    pub ai_owner: UPtr<AAIController>,
    pub actor_owner: UPtr<AActor>,
    pub observed_key_names: TArray<FName>,
    pub custom_description: FString,
    pub flags_184: u8,
}
pub struct UBTDecorator_CheckGameplayTagsOnActor {
    pub actor_to_check: FBlackboardKeySelector,
    pub tags_to_match: EGameplayContainerMatchType,
    pub gameplay_tags: FGameplayTagContainer,
    pub cached_description: FString,
}
pub struct UBTDecorator_CompareBBEntries {
    pub operator: EBlackBoardEntryComparison,
    pub blackboard_key_a: FBlackboardKeySelector,
    pub blackboard_key_b: FBlackboardKeySelector,
}
pub struct UBTDecorator_ConditionalLoop {}
pub struct UBTDecorator_ConeCheck {
    pub cone_half_angle: FValueOrBBKey_Float,
    pub cone_origin: FBlackboardKeySelector,
    pub cone_direction: FBlackboardKeySelector,
    pub observed: FBlackboardKeySelector,
}
pub struct UBTDecorator_Cooldown {
    pub cool_down_time: FValueOrBBKey_Float,
}
pub struct UBTDecorator_DoesPathExist {
    pub blackboard_key_a: FBlackboardKeySelector,
    pub blackboard_key_b: FBlackboardKeySelector,
    pub flags_216: u8,
    pub path_query_type: FValueOrBBKey_Enum,
    pub filter_class: FValueOrBBKey_Class,
}
pub struct UBTDecorator_ForceSuccess {}
pub struct UBTDecorator_IsAtLocation {
    pub acceptable_radius: f32,
    pub parametrized_acceptable_radius: FAIDataProviderFloatValue,
    pub geometric_distance_type: FAIDistanceType,
    pub flags_244: u8,
    pub b_use_nav_agent_goal_location: FValueOrBBKey_Bool,
    pub b_path_finding_based_test: FValueOrBBKey_Bool,
}
pub struct UBTDecorator_IsBBEntryOfClass {
    pub test_class: FValueOrBBKey_Class,
}
pub struct UBTDecorator_KeepInCone {
    pub cone_half_angle: FValueOrBBKey_Float,
    pub cone_origin: FBlackboardKeySelector,
    pub observed: FBlackboardKeySelector,
    pub flags_248: u8,
}
pub struct UBTDecorator_Loop {
    pub num_loops: FValueOrBBKey_Int32,
    pub b_infinite_loop: bool,
    pub infinite_loop_timeout_time: FValueOrBBKey_Float,
}
pub struct UBTDecorator_LoopUntil {
    pub required_result: FValueOrBBKey_Enum,
}
pub struct UBTDecorator_ReachedMoveGoal {}
pub struct UBTDecorator_SetTagCooldown {
    pub cooldown_tag: FGameplayTag,
    pub cooldown_duration: FValueOrBBKey_Float,
    pub b_add_to_existing_duration: FValueOrBBKey_Bool,
}
pub struct UBTDecorator_TagCooldown {
    pub cooldown_tag: FGameplayTag,
    pub cooldown_duration: FValueOrBBKey_Float,
    pub b_add_to_existing_duration: FValueOrBBKey_Bool,
    pub b_activates_cooldown: FValueOrBBKey_Bool,
}
pub struct UBTDecorator_TimeLimit {
    pub time_limit: FValueOrBBKey_Float,
}
pub struct UBTService_BlackboardBase {
    pub blackboard_key: FBlackboardKeySelector,
}
pub struct UBTService_BlueprintBase {
    pub ai_owner: UPtr<AAIController>,
    pub actor_owner: UPtr<AActor>,
    pub custom_description: FString,
    pub flags_176: u8,
}
pub struct UBTService_DefaultFocus {
    pub focus_priority: u8,
}
pub struct UBTService_RunEQS {
    pub eqs_request: FEQSParametrizedQueryExecutionRequest,
    pub b_update_bb_on_fail: bool,
}
pub struct UBTTask_BlackboardBase {
    pub blackboard_key: FBlackboardKeySelector,
}
pub struct UBTTask_BlueprintBase {
    pub ai_owner: UPtr<AAIController>,
    pub actor_owner: UPtr<AActor>,
    pub tick_interval: FIntervalCountdown,
    pub custom_description: FString,
    pub flags_192: u8,
}
pub struct UBTTask_FinishWithResult {
    pub result: FValueOrBBKey_Enum,
}
pub struct UBTTask_GameplayTaskBase {
    pub b_wait_for_gameplay_task: FValueOrBBKey_Bool,
}
pub struct UBTTask_MakeNoise {
    pub loudnes: FValueOrBBKey_Float,
}
pub struct UBTTask_MoveTo {
    pub acceptable_radius: FValueOrBBKey_Float,
    pub filter_class: FValueOrBBKey_Class,
    pub observed_blackboard_value_tolerance: FValueOrBBKey_Float,
    pub b_allow_strafe: FValueOrBBKey_Bool,
    pub b_allow_partial_path: FValueOrBBKey_Bool,
    pub b_track_moving_goal: FValueOrBBKey_Bool,
    pub b_require_navigable_end_location: FValueOrBBKey_Bool,
    pub b_project_goal_location: FValueOrBBKey_Bool,
    pub b_reach_test_includes_agent_radius: FValueOrBBKey_Bool,
    pub b_reach_test_includes_goal_radius: FValueOrBBKey_Bool,
    pub b_start_from_previous_path: FValueOrBBKey_Bool,
    pub flags_536: u8,
}
pub struct UBTTask_MoveDirectlyToward {}
pub struct UBTTask_PlayAnimation {
    pub animation_to_play: FValueOrBBKey_Object,
    pub b_looping: FValueOrBBKey_Bool,
    pub b_non_blocking: FValueOrBBKey_Bool,
    pub my_owner_comp: UPtr<UBehaviorTreeComponent>,
    pub cached_skel_mesh: UPtr<USkeletalMeshComponent>,
}
pub struct UBTTask_PlaySound {
    pub sound_to_play: FValueOrBBKey_Object,
}
pub struct UBTTask_RotateToFaceBBEntry {
    pub precision: FValueOrBBKey_Float,
}
pub struct UBTTask_RunBehavior {
    pub behavior_asset: UPtr<UBehaviorTree>,
}
pub struct UBTTask_RunBehaviorDynamic {
    pub injection_tag: FGameplayTag,
    pub default_behavior_asset: UPtr<UBehaviorTree>,
    pub behavior_asset: UPtr<UBehaviorTree>,
}
pub struct UBTTask_RunEQSQuery {
    pub b_use_bb_key: bool,
    pub eqs_request: FEQSParametrizedQueryExecutionRequest,
    pub b_update_bb_on_fail: bool,
}
pub struct UBTTask_SetKeyValueBool {
    pub value: FValueOrBBKey_Bool,
}
pub struct UBTTask_SetKeyValueClass {
    pub base_class: TSubclassOf<UObject>,
    pub value: FValueOrBBKey_Class,
}
pub struct UBTTask_SetKeyValueEnum {
    pub enum_type: UPtr<UEnum>,
    pub value: FValueOrBBKey_Enum,
}
pub struct UBTTask_SetKeyValueInt32 {
    pub value: FValueOrBBKey_Int32,
}
pub struct UBTTask_SetKeyValueFloat {
    pub value: FValueOrBBKey_Float,
}
pub struct UBTTask_SetKeyValueName {
    pub value: FValueOrBBKey_Name,
}
pub struct UBTTask_SetKeyValueString {
    pub value: FValueOrBBKey_String,
}
pub struct UBTTask_SetKeyValueObject {
    pub base_class: TSubclassOf<UObject>,
    pub value: FValueOrBBKey_Object,
}
pub struct UBTTask_SetKeyValueRotator {
    pub value: FValueOrBBKey_Rotator,
}
pub struct UBTTask_SetKeyValueStruct {
    pub struct_type: UPtr<UScriptStruct>,
    pub value: FValueOrBBKey_Struct,
}
pub struct UBTTask_SetKeyValueVector {
    pub value: FValueOrBBKey_Vector,
}
pub struct UBTTask_SetTagCooldown {
    pub cooldown_tag: FGameplayTag,
    pub b_add_to_existing_duration: FValueOrBBKey_Bool,
    pub cooldown_duration: FValueOrBBKey_Float,
}
pub struct UBTTask_Wait {
    pub wait_time: FValueOrBBKey_Float,
    pub random_deviation: FValueOrBBKey_Float,
}
pub struct UBTTask_WaitBlackboardTime {
    pub blackboard_key: FBlackboardKeySelector,
}
pub struct UValueOrBBKeyBlueprintUtility {}
pub struct UAIBlueprintHelperLibrary {}
pub struct UAIDataProvider {}
pub struct UAIDataProvider_QueryParams {
    pub param_name: FName,
    pub float_value: f32,
    pub int_value: i32,
    pub bool_value: bool,
}
pub struct UAIDataProvider_Random {
    pub min: f32,
    pub max: f32,
    pub flags_80: u8,
}
pub struct ADetourCrowdAIController {}
pub struct UEnvQueryContext {}
pub struct UEnvQueryContext_BlueprintBase {}
pub struct UEnvQueryContext_Item {}
pub struct UEnvQueryContext_NavigationData {
    pub nav_agent_properties: FNavAgentProperties,
}
pub struct UEnvQueryContext_Querier {}
pub struct UEnvQuery {
    pub ed_graph: UPtr<UEdGraph>,
    pub query_name: FName,
    pub options: TArray<UPtr<UEnvQueryOption>>,
}
pub struct UEnvQueryDebugHelpers {}
pub struct UEnvQueryNode {
    pub ver_num: i32,
}
pub struct UEnvQueryGenerator {
    pub option_name: FString,
    pub item_type: TSubclassOf<UEnvQueryItemType>,
    pub flags_80: u8,
    pub env_query_result_normalization_option: EEnvQueryResultNormalizationOption,
}
pub struct UEnvQueryInstanceBlueprintWrapper {
    pub query_id: i32,
    pub item_type: TSubclassOf<UEnvQueryItemType>,
    pub option_index: i32,
    pub on_query_finished_event: FEnvQueryInstanceBlueprintWrapper_OnQueryFinishedEvent,
}
pub struct UEnvQueryManager {
    pub instance_cache: TArray<FEnvQueryInstanceCache>,
    pub local_contexts: TArray<UPtr<UEnvQueryContext>>,
    pub gc_shielded_wrappers: TArray<UPtr<UEnvQueryInstanceBlueprintWrapper>>,
    pub max_allowed_testing_time: f32,
    pub b_test_queries_using_breadth: bool,
    pub query_count_warning_threshold: i32,
    pub query_count_warning_interval: f64,
    pub execution_time_warning_seconds: f64,
    pub handling_result_time_warning_seconds: f64,
    pub generation_time_warning_seconds: f64,
}
pub struct UEnvQueryOption {
    pub generator: UPtr<UEnvQueryGenerator>,
    pub tests: TArray<UPtr<UEnvQueryTest>>,
}
pub struct UEnvQueryTest {
    pub test_order: i32,
    pub test_purpose: EEnvTestPurpose,
    pub test_comment: FString,
    pub multiple_context_filter_op: EEnvTestFilterOperator,
    pub multiple_context_score_op: EEnvTestScoreOperator,
    pub filter_type: EEnvTestFilterType,
    pub bool_value: FAIDataProviderBoolValue,
    pub float_value_min: FAIDataProviderFloatValue,
    pub float_value_max: FAIDataProviderFloatValue,
    pub scoring_equation: EEnvTestScoreEquation,
    pub clamp_min_type: EEnvQueryTestClamping,
    pub clamp_max_type: EEnvQueryTestClamping,
    pub normalization_type: EEQSNormalizationType,
    pub score_clamp_min: FAIDataProviderFloatValue,
    pub score_clamp_max: FAIDataProviderFloatValue,
    pub scoring_factor: FAIDataProviderFloatValue,
    pub reference_value: FAIDataProviderFloatValue,
    pub b_define_reference_value: bool,
    pub flags_600: u8,
}
pub struct UEnvQueryTypes {}
pub struct UEQSRenderingComponent {}
pub struct AEQSTestingPawn {
    pub query_template: UPtr<UEnvQuery>,
    pub query_params: TArray<FEnvNamedValue>,
    pub query_config: TArray<FAIDynamicParam>,
    pub time_limit_per_step: f32,
    pub step_to_debug_draw: i32,
    pub highlight_mode: EEnvQueryHightlightMode,
    pub flags_2172: u8,
    pub querying_mode: EEnvQueryRunMode,
    pub nav_agent_properties: FNavAgentProperties,
    pub ed_render_comp: UPtr<UEQSRenderingComponent>,
    pub step_results: TArray<FEnvQueryInstance>,
}
pub struct UEnvQueryGenerator_ActorsOfClass {
    pub searched_actor_class: TSubclassOf<AActor>,
    pub generate_only_actors_in_radius: FAIDataProviderBoolValue,
    pub search_radius: FAIDataProviderFloatValue,
    pub search_center: TSubclassOf<UEnvQueryContext>,
}
pub struct UEnvQueryGenerator_BlueprintBase {
    pub generators_action_description: FText,
    pub context: TSubclassOf<UEnvQueryContext>,
    pub generated_item_type: TSubclassOf<UEnvQueryItemType>,
}
pub struct UEnvQueryGenerator_Composite {
    pub generators: TArray<UPtr<UEnvQueryGenerator>>,
    pub flags_104: u8,
    pub forced_item_type: TSubclassOf<UEnvQueryItemType>,
}
pub struct UEnvQueryGenerator_ProjectedPoints {
    pub projection_data: FEnvTraceData,
    pub nav_data_override_context: TSubclassOf<UEnvQueryContext>,
}
pub struct UEnvQueryGenerator_Cone {
    pub aligned_points_distance: FAIDataProviderFloatValue,
    pub cone_degrees: FAIDataProviderFloatValue,
    pub angle_step: FAIDataProviderFloatValue,
    pub range: FAIDataProviderFloatValue,
    pub center_actor: TSubclassOf<UEnvQueryContext>,
    pub flags_424: u8,
}
pub struct UEnvQueryGenerator_CurrentLocation {
    pub query_context: TSubclassOf<UEnvQueryContext>,
}
pub struct UEnvQueryGenerator_Donut {
    pub inner_radius: FAIDataProviderFloatValue,
    pub outer_radius: FAIDataProviderFloatValue,
    pub number_of_rings: FAIDataProviderIntValue,
    pub points_per_ring: FAIDataProviderIntValue,
    pub arc_direction: FEnvDirection,
    pub arc_angle: FAIDataProviderFloatValue,
    pub b_use_spiral_pattern: bool,
    pub center: TSubclassOf<UEnvQueryContext>,
    pub flags_528: u8,
}
pub struct UEnvQueryGenerator_OnCircle {
    pub circle_radius: FAIDataProviderFloatValue,
    pub point_on_circle_spacing_method: EPointOnCircleSpacingMethod,
    pub space_between: FAIDataProviderFloatValue,
    pub number_of_points: FAIDataProviderIntValue,
    pub arc_direction: FEnvDirection,
    pub arc_direction_offset_degrees: FAIDataProviderFloatValue,
    pub arc_angle: FAIDataProviderFloatValue,
    pub circle_center: TSubclassOf<UEnvQueryContext>,
    pub b_ignore_any_context_actors_when_generating_circle: bool,
    pub circle_center_z_offset: FAIDataProviderFloatValue,
    pub trace_data: FEnvTraceData,
    pub flags_664: u8,
}
pub struct UEnvQueryGenerator_SimpleGrid {
    pub grid_size: FAIDataProviderFloatValue,
    pub space_between: FAIDataProviderFloatValue,
    pub generate_around: TSubclassOf<UEnvQueryContext>,
}
pub struct UEnvQueryGenerator_PathingGrid {
    pub path_to_item: FAIDataProviderBoolValue,
    pub navigation_filter: TSubclassOf<UNavigationQueryFilter>,
    pub scan_range_multiplier: FAIDataProviderFloatValue,
}
pub struct UEnvQueryGenerator_PerceivedActors {
    pub allowed_actor_class: TSubclassOf<AActor>,
    pub search_radius: FAIDataProviderFloatValue,
    pub listener_context: TSubclassOf<UEnvQueryContext>,
    pub sense_to_use: TSubclassOf<UAISense>,
    pub b_include_known_actors: bool,
}
pub struct UEnvQueryItemType {}
pub struct UEnvQueryItemType_VectorBase {}
pub struct UEnvQueryItemType_ActorBase {}
pub struct UEnvQueryItemType_Actor {}
pub struct UEnvQueryItemType_Direction {}
pub struct UEnvQueryItemType_Point {}
pub struct UEnvQueryTest_Distance {
    pub test_mode: EEnvTestDistance,
    pub distance_to: TSubclassOf<UEnvQueryContext>,
}
pub struct UEnvQueryTest_Dot {
    pub line_a: FEnvDirection,
    pub line_b: FEnvDirection,
    pub test_mode: EEnvTestDot,
    pub b_absolute_value: bool,
}
pub struct UEnvQueryTest_GameplayTags {
    pub tag_query_to_match: FGameplayTagQuery,
    pub b_search_components_for_tags: bool,
    pub b_reject_incompatible_items: bool,
    pub b_updated_to_use_query: bool,
    pub tags_to_match: EGameplayContainerMatchType,
    pub gameplay_tags: FGameplayTagContainer,
}
pub struct UEnvQueryTest_Overlap {
    pub overlap_data: FEnvOverlapData,
}
pub struct UEnvQueryTest_Pathfinding {
    pub test_mode: EEnvTestPathfinding,
    pub context: TSubclassOf<UEnvQueryContext>,
    pub path_from_context: FAIDataProviderBoolValue,
    pub skip_unreachable: FAIDataProviderBoolValue,
    pub filter_class: TSubclassOf<UNavigationQueryFilter>,
}
pub struct UEnvQueryTest_PathfindingBatch {
    pub scan_range_multiplier: FAIDataProviderFloatValue,
}
pub struct UEnvQueryTest_Project {
    pub projection_data: FEnvTraceData,
}
pub struct UEnvQueryTest_Random {}
pub struct UEnvQueryTest_Trace {
    pub trace_data: FEnvTraceData,
    pub trace_from_context: FAIDataProviderBoolValue,
    pub item_height_offset: FAIDataProviderFloatValue,
    pub context_height_offset: FAIDataProviderFloatValue,
    pub context: TSubclassOf<UEnvQueryContext>,
}
pub struct UEnvQueryTest_Volume {
    pub volume_context: TSubclassOf<UEnvQueryContext>,
    pub volume_class: TSubclassOf<AVolume>,
    pub flags_624: u8,
}
pub struct AGridPathAIController {}
pub struct UAIHotSpotManager {}
pub struct UPathFollowingComponent {
    pub movement_comp: UPtr<UNavMovementComponent>,
    pub my_nav_data: UPtr<ANavigationData>,
}
pub struct UCrowdFollowingComponent {
    pub crowd_agent_move_direction: FVector,
    pub avoidance_group_deprecated: FNavAvoidanceMask,
    pub groups_to_avoid_deprecated: FNavAvoidanceMask,
    pub groups_to_ignore_deprecated: FNavAvoidanceMask,
}
pub struct UCrowdManager {
    pub my_nav_data: UPtr<ANavigationData>,
    pub avoidance_config: TArray<FCrowdAvoidanceConfig>,
    pub sampling_patterns: TArray<FCrowdAvoidanceSamplingPattern>,
    pub max_agents: i32,
    pub max_agent_radius: f32,
    pub max_avoided_agents: i32,
    pub max_avoided_walls: i32,
    pub navmesh_check_interval: f32,
    pub path_optimization_interval: f32,
    pub separation_dir_clamp: f32,
    pub path_offset_radius_multiplier: f32,
    pub flags_120: u8,
}
pub struct UGeneratedNavLinksProxy {
    pub on_smart_link_reached: FGeneratedNavLinksProxy_OnSmartLinkReached,
}
pub struct UGridPathFollowingComponent {
    pub grid_manager: UPtr<UNavLocalGridManager>,
}
pub struct UNavFilter_AIControllerDefault {}
pub struct ANavLinkProxy {
    pub point_links: TArray<FNavigationLink>,
    pub segment_links: TArray<FNavigationSegmentLink>,
    pub smart_link_comp: UPtr<UNavLinkCustomComponent>,
    pub b_smart_link_is_relevant: bool,
    pub ed_render_comp: UPtr<UNavLinkRenderingComponent>,
    pub sprite_component: UPtr<UBillboardComponent>,
    pub on_smart_link_reached: FNavLinkProxy_OnSmartLinkReached,
}
pub struct UNavLocalGridManager {}
pub struct UPathFollowingManager {}
pub struct UAIPerceptionComponent {
    pub senses_config: TArray<UPtr<UAISenseConfig>>,
    pub dominant_sense: TSubclassOf<UAISense>,
    pub ai_owner: UPtr<AAIController>,
    pub on_perception_updated: FAIPerceptionComponent_OnPerceptionUpdated,
    pub on_target_perception_forgotten: FAIPerceptionComponent_OnTargetPerceptionForgotten,
    pub on_target_perception_updated: FAIPerceptionComponent_OnTargetPerceptionUpdated,
    pub on_target_perception_info_updated: FAIPerceptionComponent_OnTargetPerceptionInfoUpdated,
}
pub struct UAIPerceptionListenerInterface {}
pub struct IAIPerceptionListenerInterface {}
pub struct UAIPerceptionStimuliSourceComponent {
    pub flags_240: u8,
    pub register_as_source_for_senses: TArray<TSubclassOf<UAISense>>,
}
pub struct UAIPerceptionSystem {
    pub senses: TArray<UPtr<UAISense>>,
    pub perception_aging_rate: f32,
}
pub struct UAISense {
    pub notify_type: EAISenseNotifyType,
    pub flags_52: u8,
    pub perception_system_instance: UPtr<UAIPerceptionSystem>,
}
pub struct UAISenseConfig_Damage {
    pub implementation: TSubclassOf<UAISense_Damage>,
}
pub struct UAISense_Blueprint {
    pub listener_data_type: TSubclassOf<UUserDefinedStruct>,
    pub listener_container: TArray<UPtr<UAIPerceptionComponent>>,
    pub unprocessed_events: TArray<UPtr<UAISenseEvent>>,
}
pub struct UAISense_Damage {
    pub registered_events: TArray<FAIDamageEvent>,
}
pub struct UAISense_Hearing {
    pub noise_events: TArray<FAINoiseEvent>,
    pub speed_of_sound_sq: f32,
}
pub struct UAISense_Prediction {
    pub registered_events: TArray<FAIPredictionEvent>,
}
pub struct UAISense_Sight {
    pub max_traces_per_tick: i32,
    pub max_async_traces_per_tick: i32,
    pub min_queries_per_time_slice_check: i32,
    pub max_time_slice_per_tick: f64,
    pub high_importance_query_distance_threshold: f32,
    pub max_query_importance: f32,
    pub sight_limit_query_importance: f32,
    pub pending_queries_budget_reduction_ratio: f32,
    pub b_use_asynchronous_trace_for_default_sight_queries: bool,
}
pub struct UAISense_Team {
    pub registered_events: TArray<FAITeamStimulusEvent>,
}
pub struct UAISense_Touch {
    pub registered_events: TArray<FAITouchEvent>,
}
pub struct UAISightTargetInterface {}
pub struct IAISightTargetInterface {}
pub struct UPawnSensingComponent {
    pub hearing_threshold: f32,
    pub los_hearing_threshold: f32,
    pub sight_radius: f32,
    pub sensing_interval: f32,
    pub hearing_max_sound_age: f32,
    pub flags_260: u8,
    pub on_see_pawn: FPawnSensingComponent_OnSeePawn,
    pub on_hear_noise: FPawnSensingComponent_OnHearNoise,
    pub peripheral_vision_angle: f32,
    pub peripheral_vision_cosine: f32,
}
pub struct UAITask {
    pub owner_controller: UPtr<AAIController>,
}
pub struct UAITask_LockLogic {}
pub struct UAITask_MoveTo {
    pub on_request_failed: FAITask_MoveTo_OnRequestFailed,
    pub on_move_finished: FAITask_MoveTo_OnMoveFinished,
    pub move_request: FAIMoveRequest,
}
pub struct UAITask_RunEQS {}
pub struct UVisualLoggerExtension {}
