#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FActorPerceptionUpdateInfo {
    pub target_id: i32,
    pub target: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub stimulus: FAIStimulus,
}
impl FActorPerceptionUpdateInfo {}
#[repr(C, align(8))]
pub struct FAIStimulus {
    pub age: f32,
    pub expiration_age: f32,
    pub strength: f32,
    pub stimulus_location: crate::bindings::core_u_object::FVector,
    pub receiver_location: crate::bindings::core_u_object::FVector,
    pub tag: FName,
    #[doc(hidden)]
    __padding_92: [u8; 16],
    pub flags_92: u8,
    __padding_end: [u8; 3],
}
impl FAIStimulus {}
#[repr(C, align(4))]
pub struct FAIRequestID {
    __padding_end: [u8; 4],
}
impl FAIRequestID {}
#[repr(C, align(1))]
pub struct FGenericTeamId {
    pub team_id: u8,
}
impl FGenericTeamId {}
#[repr(C, align(8))]
pub struct FBlackboardKeySelector {
    pub allowed_types: TArray<UPtr<UBlackboardKeyType>>,
    pub selected_key_name: FName,
    pub selected_key_type: TSubclassOf<UBlackboardKeyType>,
    pub selected_key_id: i32,
    pub flags_44: u8,
    __padding_end: [u8; 3],
}
impl FBlackboardKeySelector {}
#[repr(C, align(8))]
pub struct FValueOrBlackboardKeyBase {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub key: FName,
    __padding_end: [u8; 4],
}
impl FValueOrBlackboardKeyBase {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Bool {
    __padding_end: [u8; 32],
}
impl FValueOrBBKey_Bool {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Class {
    __padding_end: [u8; 40],
}
impl FValueOrBBKey_Class {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Enum {
    __padding_end: [u8; 56],
}
impl FValueOrBBKey_Enum {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Float {
    __padding_end: [u8; 32],
}
impl FValueOrBBKey_Float {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Int32 {
    __padding_end: [u8; 32],
}
impl FValueOrBBKey_Int32 {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Name {
    __padding_end: [u8; 40],
}
impl FValueOrBBKey_Name {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_String {
    __padding_end: [u8; 40],
}
impl FValueOrBBKey_String {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Object {
    __padding_end: [u8; 40],
}
impl FValueOrBBKey_Object {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Rotator {
    __padding_end: [u8; 48],
}
impl FValueOrBBKey_Rotator {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Vector {
    __padding_end: [u8; 48],
}
impl FValueOrBBKey_Vector {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Struct {
    __padding_end: [u8; 48],
}
impl FValueOrBBKey_Struct {}
#[repr(C, align(8))]
pub struct FEnvQueryResult {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item_type: TSubclassOf<UEnvQueryItemType>,
    #[doc(hidden)]
    __padding_44: [u8; 20],
    pub option_index: i32,
    pub query_id: i32,
    __padding_end: [u8; 12],
}
impl FEnvQueryResult {}
#[repr(C, align(8))]
pub struct FEnvQueryInstance {
    __padding_end: [u8; 448],
}
impl FEnvQueryInstance {}
#[repr(C, align(4))]
pub struct FEnvNamedValue {
    pub param_name: FName,
    pub param_type: EAIParamType,
    pub value: f32,
}
impl FEnvNamedValue {}
#[repr(C, align(8))]
pub struct FAIDynamicParam {
    pub param_name: FName,
    pub param_type: EAIParamType,
    pub flags_13: u8,
    pub value: f32,
    pub bb_key: FBlackboardKeySelector,
}
impl FAIDynamicParam {}
#[repr(C, align(8))]
pub struct FActorPerceptionBlueprintInfo {
    pub target: UPtr<crate::bindings::engine::AActor>,
    pub last_sensed_stimuli: TArray<FAIStimulus>,
    pub flags_24: u8,
    __padding_end: [u8; 7],
}
impl FActorPerceptionBlueprintInfo {}
#[repr(C, align(4))]
pub struct FAISenseAffiliationFilter {
    pub flags_0: u8,
    __padding_end: [u8; 3],
}
impl FAISenseAffiliationFilter {}
#[repr(C, align(8))]
pub struct FAIDamageEvent {
    pub amount: f32,
    pub location: crate::bindings::core_u_object::FVector,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub damaged_actor: UPtr<crate::bindings::engine::AActor>,
    pub instigator: UPtr<crate::bindings::engine::AActor>,
    pub tag: FName,
    __padding_end: [u8; 4],
}
impl FAIDamageEvent {}
#[repr(C, align(8))]
pub struct FAINoiseEvent {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub noise_location: crate::bindings::core_u_object::FVector,
    pub loudness: f32,
    pub max_range: f32,
    pub instigator: UPtr<crate::bindings::engine::AActor>,
    pub tag: FName,
    __padding_end: [u8; 4],
}
impl FAINoiseEvent {}
#[repr(C, align(8))]
pub struct UAIAsyncTaskBlueprintProxy {
    __padding_end: [u8; 128],
}
impl UAIAsyncTaskBlueprintProxy {}
pub struct UAIResourceInterface {}
pub struct IAIResourceInterface {}
#[repr(C, align(8))]
pub struct UAISenseBlueprintListener {
    __padding_end: [u8; 368],
}
impl UAISenseBlueprintListener {}
#[repr(C, align(8))]
pub struct UAISenseConfig {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub debug_color: crate::bindings::core_u_object::FColor,
    pub max_age: f32,
    pub flags_56: u8,
    __padding_end: [u8; 23],
}
impl UAISenseConfig {}
#[repr(C, align(8))]
pub struct UAISenseConfig_Blueprint {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub implementation: TSubclassOf<UAISense_Blueprint>,
}
impl UAISenseConfig_Blueprint {}
#[repr(C, align(8))]
pub struct UAISenseConfig_Hearing {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub implementation: TSubclassOf<UAISense_Hearing>,
    pub hearing_range: f32,
    pub lo_s_hearing_range: f32,
    #[doc(hidden)]
    __padding_100: [u8; 4],
    pub detection_by_affiliation: FAISenseAffiliationFilter,
}
impl UAISenseConfig_Hearing {}
#[repr(C, align(8))]
pub struct UAISenseConfig_Prediction {
    __padding_end: [u8; 80],
}
impl UAISenseConfig_Prediction {}
#[repr(C, align(8))]
pub struct UAISenseConfig_Sight {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub implementation: TSubclassOf<UAISense_Sight>,
    pub sight_radius: f32,
    pub lose_sight_radius: f32,
    pub peripheral_vision_angle_degrees: f32,
    pub detection_by_affiliation: FAISenseAffiliationFilter,
    pub auto_success_range_from_last_seen_location: f32,
    pub point_of_view_backward_offset: f32,
    pub near_clipping_radius: f32,
    __padding_end: [u8; 4],
}
impl UAISenseConfig_Sight {}
#[repr(C, align(8))]
pub struct UAISenseConfig_Team {
    __padding_end: [u8; 80],
}
impl UAISenseConfig_Team {}
#[repr(C, align(8))]
pub struct UAISenseConfig_Touch {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub detection_by_affiliation: FAISenseAffiliationFilter,
    __padding_end: [u8; 4],
}
impl UAISenseConfig_Touch {}
#[repr(C, align(8))]
pub struct UAISenseEvent {
    __padding_end: [u8; 48],
}
impl UAISenseEvent {}
#[repr(C, align(8))]
pub struct UAISenseEvent_Damage {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub event: FAIDamageEvent,
}
impl UAISenseEvent_Damage {}
#[repr(C, align(8))]
pub struct UAISenseEvent_Hearing {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub event: FAINoiseEvent,
}
impl UAISenseEvent_Hearing {}
pub struct UCrowdAgentInterface {}
pub struct ICrowdAgentInterface {}
pub struct UEQSQueryResultSourceInterface {}
pub struct IEQSQueryResultSourceInterface {}
pub struct UGenericTeamAgentInterface {}
pub struct IGenericTeamAgentInterface {}
#[repr(C, align(8))]
pub struct AAIController {
    #[doc(hidden)]
    __padding_1376: [u8; 1376],
    pub flags_1376: u8,
    #[doc(hidden)]
    __padding_1392: [u8; 8],
    pub brain_component: UPtr<UBrainComponent>,
    #[doc(hidden)]
    __padding_1408: [u8; 8],
    pub blackboard: UPtr<UBlackboardComponent>,
    #[doc(hidden)]
    __padding_1424: [u8; 8],
    pub default_navigation_filter_class: TSubclassOf<
        crate::bindings::navigation_system::UNavigationQueryFilter,
    >,
    __padding_end: [u8; 32],
}
impl AAIController {}
#[repr(C, align(8))]
pub struct UAIResource_Movement {
    __padding_end: [u8; 64],
}
impl UAIResource_Movement {}
#[repr(C, align(8))]
pub struct UAIResource_Logic {
    __padding_end: [u8; 64],
}
impl UAIResource_Logic {}
#[repr(C, align(8))]
pub struct UAISubsystem {
    __padding_end: [u8; 64],
}
impl UAISubsystem {}
#[repr(C, align(8))]
pub struct UAISystem {
    __padding_end: [u8; 472],
}
impl UAISystem {}
#[repr(C, align(8))]
pub struct UBehaviorTree {
    __padding_end: [u8; 136],
}
impl UBehaviorTree {}
#[repr(C, align(8))]
pub struct UBrainComponent {
    __padding_end: [u8; 328],
}
impl UBrainComponent {}
#[repr(C, align(8))]
pub struct UBehaviorTreeComponent {
    #[doc(hidden)]
    __padding_848: [u8; 848],
    pub default_behavior_tree_asset: UPtr<UBehaviorTree>,
    __padding_end: [u8; 32],
}
impl UBehaviorTreeComponent {}
#[repr(C, align(8))]
pub struct UBehaviorTreeManager {
    __padding_end: [u8; 88],
}
impl UBehaviorTreeManager {}
#[repr(C, align(8))]
pub struct UBehaviorTreeTypes {
    __padding_end: [u8; 48],
}
impl UBehaviorTreeTypes {}
pub struct UBlackboardAssetProvider {}
pub struct IBlackboardAssetProvider {}
#[repr(C, align(8))]
pub struct UBlackboardComponent {
    __padding_end: [u8; 504],
}
impl UBlackboardComponent {}
#[repr(C, align(8))]
pub struct UBlackboardData {
    __padding_end: [u8; 104],
}
impl UBlackboardData {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType {
    __padding_end: [u8; 56],
}
impl UBlackboardKeyType {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Bool {
    __padding_end: [u8; 64],
}
impl UBlackboardKeyType_Bool {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Class {
    __padding_end: [u8; 72],
}
impl UBlackboardKeyType_Class {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Enum {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_Enum {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Float {
    __padding_end: [u8; 64],
}
impl UBlackboardKeyType_Float {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Int {
    __padding_end: [u8; 64],
}
impl UBlackboardKeyType_Int {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Name {
    __padding_end: [u8; 72],
}
impl UBlackboardKeyType_Name {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_NativeEnum {
    __padding_end: [u8; 80],
}
impl UBlackboardKeyType_NativeEnum {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Object {
    __padding_end: [u8; 72],
}
impl UBlackboardKeyType_Object {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Rotator {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_Rotator {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_String {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_String {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Struct {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_Struct {}
#[repr(C, align(8))]
pub struct UBlackboardKeyType_Vector {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_Vector {}
#[repr(C, align(8))]
pub struct UBTNode {
    __padding_end: [u8; 104],
}
impl UBTNode {}
#[repr(C, align(8))]
pub struct UBTAuxiliaryNode {
    __padding_end: [u8; 112],
}
impl UBTAuxiliaryNode {}
#[repr(C, align(8))]
pub struct UBTCompositeNode {
    __padding_end: [u8; 144],
}
impl UBTCompositeNode {}
#[repr(C, align(8))]
pub struct UBTDecorator {
    __padding_end: [u8; 120],
}
impl UBTDecorator {}
#[repr(C, align(8))]
pub struct UBTFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBTFunctionLibrary {}
#[repr(C, align(8))]
pub struct UBTService {
    __padding_end: [u8; 128],
}
impl UBTService {}
#[repr(C, align(8))]
pub struct UBTTaskNode {
    __padding_end: [u8; 128],
}
impl UBTTaskNode {}
#[repr(C, align(8))]
pub struct UBTComposite_Selector {
    __padding_end: [u8; 144],
}
impl UBTComposite_Selector {}
#[repr(C, align(8))]
pub struct UBTComposite_Sequence {
    __padding_end: [u8; 144],
}
impl UBTComposite_Sequence {}
#[repr(C, align(8))]
pub struct UBTComposite_SimpleParallel {
    __padding_end: [u8; 152],
}
impl UBTComposite_SimpleParallel {}
#[repr(C, align(8))]
pub struct UBTDecorator_BlackboardBase {
    __padding_end: [u8; 168],
}
impl UBTDecorator_BlackboardBase {}
#[repr(C, align(8))]
pub struct UBTDecorator_Blackboard {
    __padding_end: [u8; 216],
}
impl UBTDecorator_Blackboard {}
#[repr(C, align(8))]
pub struct UBTDecorator_BlueprintBase {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub custom_description: FString,
    __padding_end: [u8; 8],
}
impl UBTDecorator_BlueprintBase {}
#[repr(C, align(8))]
pub struct UBTDecorator_CheckGameplayTagsOnActor {
    __padding_end: [u8; 224],
}
impl UBTDecorator_CheckGameplayTagsOnActor {}
#[repr(C, align(8))]
pub struct UBTDecorator_CompareBBEntries {
    __padding_end: [u8; 224],
}
impl UBTDecorator_CompareBBEntries {}
#[repr(C, align(8))]
pub struct UBTDecorator_ConditionalLoop {
    __padding_end: [u8; 216],
}
impl UBTDecorator_ConditionalLoop {}
#[repr(C, align(8))]
pub struct UBTDecorator_ConeCheck {
    __padding_end: [u8; 296],
}
impl UBTDecorator_ConeCheck {}
#[repr(C, align(8))]
pub struct UBTDecorator_Cooldown {
    __padding_end: [u8; 152],
}
impl UBTDecorator_Cooldown {}
#[repr(C, align(8))]
pub struct UBTDecorator_DoesPathExist {
    __padding_end: [u8; 320],
}
impl UBTDecorator_DoesPathExist {}
#[repr(C, align(8))]
pub struct UBTDecorator_ForceSuccess {
    __padding_end: [u8; 120],
}
impl UBTDecorator_ForceSuccess {}
#[repr(C, align(8))]
pub struct UBTDecorator_IsAtLocation {
    __padding_end: [u8; 312],
}
impl UBTDecorator_IsAtLocation {}
#[repr(C, align(8))]
pub struct UBTDecorator_IsBBEntryOfClass {
    __padding_end: [u8; 208],
}
impl UBTDecorator_IsBBEntryOfClass {}
#[repr(C, align(8))]
pub struct UBTDecorator_KeepInCone {
    __padding_end: [u8; 256],
}
impl UBTDecorator_KeepInCone {}
#[repr(C, align(8))]
pub struct UBTDecorator_Loop {
    __padding_end: [u8; 192],
}
impl UBTDecorator_Loop {}
#[repr(C, align(8))]
pub struct UBTDecorator_LoopUntil {
    __padding_end: [u8; 176],
}
impl UBTDecorator_LoopUntil {}
#[repr(C, align(8))]
pub struct UBTDecorator_ReachedMoveGoal {
    __padding_end: [u8; 120],
}
impl UBTDecorator_ReachedMoveGoal {}
#[repr(C, align(8))]
pub struct UBTDecorator_SetTagCooldown {
    __padding_end: [u8; 200],
}
impl UBTDecorator_SetTagCooldown {}
#[repr(C, align(8))]
pub struct UBTDecorator_TagCooldown {
    __padding_end: [u8; 232],
}
impl UBTDecorator_TagCooldown {}
#[repr(C, align(8))]
pub struct UBTDecorator_TimeLimit {
    __padding_end: [u8; 152],
}
impl UBTDecorator_TimeLimit {}
#[repr(C, align(8))]
pub struct UBTService_BlackboardBase {
    __padding_end: [u8; 176],
}
impl UBTService_BlackboardBase {}
#[repr(C, align(8))]
pub struct UBTService_BlueprintBase {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub custom_description: FString,
    __padding_end: [u8; 8],
}
impl UBTService_BlueprintBase {}
#[repr(C, align(8))]
pub struct UBTService_DefaultFocus {
    __padding_end: [u8; 184],
}
impl UBTService_DefaultFocus {}
#[repr(C, align(8))]
pub struct UBTService_RunEQS {
    __padding_end: [u8; 288],
}
impl UBTService_RunEQS {}
#[repr(C, align(8))]
pub struct UBTTask_BlackboardBase {
    __padding_end: [u8; 176],
}
impl UBTTask_BlackboardBase {}
#[repr(C, align(8))]
pub struct UBTTask_BlueprintBase {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub custom_description: FString,
    __padding_end: [u8; 8],
}
impl UBTTask_BlueprintBase {}
#[repr(C, align(8))]
pub struct UBTTask_FinishWithResult {
    __padding_end: [u8; 184],
}
impl UBTTask_FinishWithResult {}
#[repr(C, align(8))]
pub struct UBTTask_GameplayTaskBase {
    __padding_end: [u8; 160],
}
impl UBTTask_GameplayTaskBase {}
#[repr(C, align(8))]
pub struct UBTTask_MakeNoise {
    __padding_end: [u8; 160],
}
impl UBTTask_MakeNoise {}
#[repr(C, align(8))]
pub struct UBTTask_MoveTo {
    __padding_end: [u8; 544],
}
impl UBTTask_MoveTo {}
#[repr(C, align(8))]
pub struct UBTTask_MoveDirectlyToward {
    __padding_end: [u8; 544],
}
impl UBTTask_MoveDirectlyToward {}
#[repr(C, align(8))]
pub struct UBTTask_PlayAnimation {
    __padding_end: [u8; 280],
}
impl UBTTask_PlayAnimation {}
#[repr(C, align(8))]
pub struct UBTTask_PlaySound {
    __padding_end: [u8; 168],
}
impl UBTTask_PlaySound {}
#[repr(C, align(8))]
pub struct UBTTask_RotateToFaceBBEntry {
    __padding_end: [u8; 208],
}
impl UBTTask_RotateToFaceBBEntry {}
#[repr(C, align(8))]
pub struct UBTTask_RunBehavior {
    __padding_end: [u8; 136],
}
impl UBTTask_RunBehavior {}
#[repr(C, align(8))]
pub struct UBTTask_RunBehaviorDynamic {
    __padding_end: [u8; 160],
}
impl UBTTask_RunBehaviorDynamic {}
#[repr(C, align(8))]
pub struct UBTTask_RunEQSQuery {
    __padding_end: [u8; 296],
}
impl UBTTask_RunEQSQuery {}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueBool {
    __padding_end: [u8; 208],
}
impl UBTTask_SetKeyValueBool {}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueClass {
    __padding_end: [u8; 224],
}
impl UBTTask_SetKeyValueClass {}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueEnum {
    __padding_end: [u8; 240],
}
impl UBTTask_SetKeyValueEnum {}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueInt32 {
    __padding_end: [u8; 208],
}
impl UBTTask_SetKeyValueInt32 {}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueFloat {
    __padding_end: [u8; 208],
}
impl UBTTask_SetKeyValueFloat {}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueName {
    __padding_end: [u8; 216],
}
impl UBTTask_SetKeyValueName {}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueString {
    __padding_end: [u8; 216],
}
impl UBTTask_SetKeyValueString {}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueObject {
    __padding_end: [u8; 224],
}
impl UBTTask_SetKeyValueObject {}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueRotator {
    __padding_end: [u8; 224],
}
impl UBTTask_SetKeyValueRotator {}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueStruct {
    __padding_end: [u8; 232],
}
impl UBTTask_SetKeyValueStruct {}
#[repr(C, align(8))]
pub struct UBTTask_SetKeyValueVector {
    __padding_end: [u8; 224],
}
impl UBTTask_SetKeyValueVector {}
#[repr(C, align(8))]
pub struct UBTTask_SetTagCooldown {
    __padding_end: [u8; 208],
}
impl UBTTask_SetTagCooldown {}
#[repr(C, align(8))]
pub struct UBTTask_Wait {
    __padding_end: [u8; 192],
}
impl UBTTask_Wait {}
#[repr(C, align(8))]
pub struct UBTTask_WaitBlackboardTime {
    __padding_end: [u8; 240],
}
impl UBTTask_WaitBlackboardTime {}
#[repr(C, align(8))]
pub struct UValueOrBBKeyBlueprintUtility {
    __padding_end: [u8; 48],
}
impl UValueOrBBKeyBlueprintUtility {}
#[repr(C, align(8))]
pub struct UAIBlueprintHelperLibrary {
    __padding_end: [u8; 48],
}
impl UAIBlueprintHelperLibrary {}
#[repr(C, align(8))]
pub struct UAIDataProvider {
    __padding_end: [u8; 48],
}
impl UAIDataProvider {}
#[repr(C, align(8))]
pub struct UAIDataProvider_QueryParams {
    __padding_end: [u8; 72],
}
impl UAIDataProvider_QueryParams {}
#[repr(C, align(8))]
pub struct UAIDataProvider_Random {
    __padding_end: [u8; 88],
}
impl UAIDataProvider_Random {}
#[repr(C, align(8))]
pub struct ADetourCrowdAIController {
    __padding_end: [u8; 1464],
}
impl ADetourCrowdAIController {}
#[repr(C, align(8))]
pub struct UEnvQueryContext {
    __padding_end: [u8; 48],
}
impl UEnvQueryContext {}
#[repr(C, align(8))]
pub struct UEnvQueryContext_BlueprintBase {
    __padding_end: [u8; 56],
}
impl UEnvQueryContext_BlueprintBase {}
#[repr(C, align(8))]
pub struct UEnvQueryContext_Item {
    __padding_end: [u8; 48],
}
impl UEnvQueryContext_Item {}
#[repr(C, align(8))]
pub struct UEnvQueryContext_NavigationData {
    __padding_end: [u8; 112],
}
impl UEnvQueryContext_NavigationData {}
#[repr(C, align(8))]
pub struct UEnvQueryContext_Querier {
    __padding_end: [u8; 48],
}
impl UEnvQueryContext_Querier {}
#[repr(C, align(8))]
pub struct UEnvQuery {
    __padding_end: [u8; 96],
}
impl UEnvQuery {}
#[repr(C, align(8))]
pub struct UEnvQueryDebugHelpers {
    __padding_end: [u8; 48],
}
impl UEnvQueryDebugHelpers {}
#[repr(C, align(8))]
pub struct UEnvQueryNode {
    __padding_end: [u8; 56],
}
impl UEnvQueryNode {}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator {
    __padding_end: [u8; 88],
}
impl UEnvQueryGenerator {}
#[repr(C, align(8))]
pub struct UEnvQueryInstanceBlueprintWrapper {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub query_id: i32,
    #[doc(hidden)]
    __padding_96: [u8; 32],
    pub item_type: TSubclassOf<UEnvQueryItemType>,
    pub option_index: i32,
    __padding_end: [u8; 36],
}
impl UEnvQueryInstanceBlueprintWrapper {}
#[repr(C, align(8))]
pub struct UEnvQueryManager {
    __padding_end: [u8; 528],
}
impl UEnvQueryManager {}
#[repr(C, align(8))]
pub struct UEnvQueryOption {
    __padding_end: [u8; 72],
}
impl UEnvQueryOption {}
#[repr(C, align(8))]
pub struct UEnvQueryTest {
    __padding_end: [u8; 608],
}
impl UEnvQueryTest {}
#[repr(C, align(8))]
pub struct UEnvQueryTypes {
    __padding_end: [u8; 48],
}
impl UEnvQueryTypes {}
#[repr(C, align(16))]
pub struct UEQSRenderingComponent {
    __padding_end: [u8; 1792],
}
impl UEQSRenderingComponent {}
#[repr(C, align(16))]
pub struct AEQSTestingPawn {
    __padding_end: [u8; 2288],
}
impl AEQSTestingPawn {}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_ActorsOfClass {
    __padding_end: [u8; 232],
}
impl UEnvQueryGenerator_ActorsOfClass {}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_BlueprintBase {
    __padding_end: [u8; 136],
}
impl UEnvQueryGenerator_BlueprintBase {}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_Composite {
    __padding_end: [u8; 120],
}
impl UEnvQueryGenerator_Composite {}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_ProjectedPoints {
    __padding_end: [u8; 160],
}
impl UEnvQueryGenerator_ProjectedPoints {}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_Cone {
    __padding_end: [u8; 432],
}
impl UEnvQueryGenerator_Cone {}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_CurrentLocation {
    __padding_end: [u8; 168],
}
impl UEnvQueryGenerator_CurrentLocation {}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_Donut {
    __padding_end: [u8; 536],
}
impl UEnvQueryGenerator_Donut {}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_OnCircle {
    __padding_end: [u8; 672],
}
impl UEnvQueryGenerator_OnCircle {}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_SimpleGrid {
    __padding_end: [u8; 296],
}
impl UEnvQueryGenerator_SimpleGrid {}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_PathingGrid {
    __padding_end: [u8; 432],
}
impl UEnvQueryGenerator_PathingGrid {}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_PerceivedActors {
    __padding_end: [u8; 184],
}
impl UEnvQueryGenerator_PerceivedActors {}
#[repr(C, align(8))]
pub struct UEnvQueryItemType {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType {}
#[repr(C, align(8))]
pub struct UEnvQueryItemType_VectorBase {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_VectorBase {}
#[repr(C, align(8))]
pub struct UEnvQueryItemType_ActorBase {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_ActorBase {}
#[repr(C, align(8))]
pub struct UEnvQueryItemType_Actor {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_Actor {}
#[repr(C, align(8))]
pub struct UEnvQueryItemType_Direction {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_Direction {}
#[repr(C, align(8))]
pub struct UEnvQueryItemType_Point {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_Point {}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Distance {
    __padding_end: [u8; 624],
}
impl UEnvQueryTest_Distance {}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Dot {
    __padding_end: [u8; 680],
}
impl UEnvQueryTest_Dot {}
#[repr(C, align(8))]
pub struct UEnvQueryTest_GameplayTags {
    __padding_end: [u8; 720],
}
impl UEnvQueryTest_GameplayTags {}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Overlap {
    __padding_end: [u8; 656],
}
impl UEnvQueryTest_Overlap {}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Pathfinding {
    __padding_end: [u8; 760],
}
impl UEnvQueryTest_Pathfinding {}
#[repr(C, align(8))]
pub struct UEnvQueryTest_PathfindingBatch {
    __padding_end: [u8; 824],
}
impl UEnvQueryTest_PathfindingBatch {}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Project {
    __padding_end: [u8; 672],
}
impl UEnvQueryTest_Project {}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Random {
    __padding_end: [u8; 608],
}
impl UEnvQueryTest_Random {}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Trace {
    __padding_end: [u8; 872],
}
impl UEnvQueryTest_Trace {}
#[repr(C, align(8))]
pub struct UEnvQueryTest_Volume {
    __padding_end: [u8; 632],
}
impl UEnvQueryTest_Volume {}
#[repr(C, align(8))]
pub struct AGridPathAIController {
    __padding_end: [u8; 1464],
}
impl AGridPathAIController {}
#[repr(C, align(8))]
pub struct UAIHotSpotManager {
    __padding_end: [u8; 48],
}
impl UAIHotSpotManager {}
#[repr(C, align(8))]
pub struct UPathFollowingComponent {
    __padding_end: [u8; 888],
}
impl UPathFollowingComponent {}
#[repr(C, align(8))]
pub struct UCrowdFollowingComponent {
    __padding_end: [u8; 984],
}
impl UCrowdFollowingComponent {}
#[repr(C, align(8))]
pub struct UCrowdManager {
    __padding_end: [u8; 256],
}
impl UCrowdManager {}
#[repr(C, align(8))]
pub struct UGeneratedNavLinksProxy {
    __padding_end: [u8; 96],
}
impl UGeneratedNavLinksProxy {}
#[repr(C, align(8))]
pub struct UGridPathFollowingComponent {
    __padding_end: [u8; 936],
}
impl UGridPathFollowingComponent {}
#[repr(C, align(8))]
pub struct UNavFilter_AIControllerDefault {
    __padding_end: [u8; 80],
}
impl UNavFilter_AIControllerDefault {}
#[repr(C, align(8))]
pub struct ANavLinkProxy {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub point_links: TArray<crate::bindings::engine::FNavigationLink>,
    __padding_end: [u8; 88],
}
impl ANavLinkProxy {}
#[repr(C, align(8))]
pub struct UNavLocalGridManager {
    __padding_end: [u8; 96],
}
impl UNavLocalGridManager {}
#[repr(C, align(8))]
pub struct UPathFollowingManager {
    __padding_end: [u8; 48],
}
impl UPathFollowingManager {}
#[repr(C, align(8))]
pub struct UAIPerceptionComponent {
    __padding_end: [u8; 512],
}
impl UAIPerceptionComponent {}
pub struct UAIPerceptionListenerInterface {}
pub struct IAIPerceptionListenerInterface {}
#[repr(C, align(8))]
pub struct UAIPerceptionStimuliSourceComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub flags_240: u8,
    pub register_as_source_for_senses: TArray<TSubclassOf<UAISense>>,
}
impl UAIPerceptionStimuliSourceComponent {}
#[repr(C, align(8))]
pub struct UAIPerceptionSystem {
    __padding_end: [u8; 336],
}
impl UAIPerceptionSystem {}
#[repr(C, align(8))]
pub struct UAISense {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub notify_type: EAISenseNotifyType,
    #[doc(hidden)]
    __padding_52: [u8; 3],
    pub flags_52: u8,
    __padding_end: [u8; 107],
}
impl UAISense {}
#[repr(C, align(8))]
pub struct UAISenseConfig_Damage {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub implementation: TSubclassOf<UAISense_Damage>,
}
impl UAISenseConfig_Damage {}
#[repr(C, align(8))]
pub struct UAISense_Blueprint {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub listener_data_type: TSubclassOf<
        crate::bindings::core_u_object::UUserDefinedStruct,
    >,
    pub listener_container: TArray<UPtr<UAIPerceptionComponent>>,
    __padding_end: [u8; 16],
}
impl UAISense_Blueprint {}
#[repr(C, align(8))]
pub struct UAISense_Damage {
    __padding_end: [u8; 176],
}
impl UAISense_Damage {}
#[repr(C, align(8))]
pub struct UAISense_Hearing {
    __padding_end: [u8; 264],
}
impl UAISense_Hearing {}
#[repr(C, align(8))]
pub struct UAISense_Prediction {
    __padding_end: [u8; 176],
}
impl UAISense_Prediction {}
#[repr(C, align(8))]
pub struct UAISense_Sight {
    __padding_end: [u8; 488],
}
impl UAISense_Sight {}
#[repr(C, align(8))]
pub struct UAISense_Team {
    __padding_end: [u8; 176],
}
impl UAISense_Team {}
#[repr(C, align(8))]
pub struct UAISense_Touch {
    __padding_end: [u8; 256],
}
impl UAISense_Touch {}
pub struct UAISightTargetInterface {}
pub struct IAISightTargetInterface {}
#[repr(C, align(8))]
pub struct UPawnSensingComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub hearing_threshold: f32,
    pub los_hearing_threshold: f32,
    pub sight_radius: f32,
    pub sensing_interval: f32,
    pub hearing_max_sound_age: f32,
    pub flags_260: u8,
    #[doc(hidden)]
    __padding_320: [u8; 56],
    pub peripheral_vision_angle: f32,
    __padding_end: [u8; 4],
}
impl UPawnSensingComponent {}
#[repr(C, align(8))]
pub struct UAITask {
    #[doc(hidden)]
    __padding_128: [u8; 128],
    pub owner_controller: UPtr<AAIController>,
}
impl UAITask {}
#[repr(C, align(8))]
pub struct UAITask_LockLogic {
    __padding_end: [u8; 136],
}
impl UAITask_LockLogic {}
#[repr(C, align(8))]
pub struct UAITask_MoveTo {
    __padding_end: [u8; 360],
}
impl UAITask_MoveTo {}
#[repr(C, align(8))]
pub struct UAITask_RunEQS {
    __padding_end: [u8; 280],
}
impl UAITask_RunEQS {}
#[repr(C, align(8))]
pub struct UVisualLoggerExtension {
    __padding_end: [u8; 48],
}
impl UVisualLoggerExtension {}
#[repr(C, align(8))]
pub struct FAIAsyncTaskBlueprintProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIAsyncTaskBlueprintProxy_OnFail {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIController_ReceiveMoveCompleted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEnvQueryInstanceBlueprintWrapper_OnQueryFinishedEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGeneratedNavLinksProxy_OnSmartLinkReached {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FNavLinkProxy_OnSmartLinkReached {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIPerceptionComponent_OnPerceptionUpdated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIPerceptionComponent_OnTargetPerceptionForgotten {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIPerceptionComponent_OnTargetPerceptionUpdated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIPerceptionComponent_OnTargetPerceptionInfoUpdated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPawnSensingComponent_OnSeePawn {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPawnSensingComponent_OnHearNoise {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAITask_MoveTo_OnRequestFailed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAITask_MoveTo_OnMoveFinished {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EBTDecoratorLogic(pub u8);
impl EBTDecoratorLogic {
    pub const INVALID: EBTDecoratorLogic = EBTDecoratorLogic(0);
    pub const TEST: EBTDecoratorLogic = EBTDecoratorLogic(1);
    pub const AND: EBTDecoratorLogic = EBTDecoratorLogic(2);
    pub const OR: EBTDecoratorLogic = EBTDecoratorLogic(3);
    pub const NOT: EBTDecoratorLogic = EBTDecoratorLogic(4);
}
#[repr(transparent)]
pub struct EAIParamType(pub u8);
impl EAIParamType {
    pub const FLOAT: EAIParamType = EAIParamType(0);
    pub const INT: EAIParamType = EAIParamType(1);
    pub const BOOL: EAIParamType = EAIParamType(2);
    pub const MAX: EAIParamType = EAIParamType(3);
}
#[repr(transparent)]
pub struct EEnvDirection(pub u8);
impl EEnvDirection {
    pub const TWO_POINTS: EEnvDirection = EEnvDirection(0);
    pub const ROTATION: EEnvDirection = EEnvDirection(1);
}
#[repr(transparent)]
pub struct EEnvTraceShape(pub u8);
impl EEnvTraceShape {
    pub const LINE: EEnvTraceShape = EEnvTraceShape(0);
    pub const BOX: EEnvTraceShape = EEnvTraceShape(1);
    pub const SPHERE: EEnvTraceShape = EEnvTraceShape(2);
    pub const CAPSULE: EEnvTraceShape = EEnvTraceShape(3);
}
#[repr(transparent)]
pub struct EEnvQueryTrace(pub u8);
impl EEnvQueryTrace {
    pub const NONE: EEnvQueryTrace = EEnvQueryTrace(0);
    pub const NAVIGATION: EEnvQueryTrace = EEnvQueryTrace(1);
    pub const GEOMETRY_BY_CHANNEL: EEnvQueryTrace = EEnvQueryTrace(2);
    pub const GEOMETRY_BY_PROFILE: EEnvQueryTrace = EEnvQueryTrace(3);
    pub const NAVIGATION_OVER_LEDGES: EEnvQueryTrace = EEnvQueryTrace(4);
}
#[repr(transparent)]
pub struct EEnvOverlapShape(pub u8);
impl EEnvOverlapShape {
    pub const BOX: EEnvOverlapShape = EEnvOverlapShape(0);
    pub const SPHERE: EEnvOverlapShape = EEnvOverlapShape(1);
    pub const CAPSULE: EEnvOverlapShape = EEnvOverlapShape(2);
}
#[repr(transparent)]
pub struct EEnvQueryRunMode(pub u8);
impl EEnvQueryRunMode {
    pub const SINGLE_RESULT: EEnvQueryRunMode = EEnvQueryRunMode(0);
    pub const RANDOM_BEST5_PCT: EEnvQueryRunMode = EEnvQueryRunMode(1);
    pub const RANDOM_BEST25_PCT: EEnvQueryRunMode = EEnvQueryRunMode(2);
    pub const ALL_MATCHING: EEnvQueryRunMode = EEnvQueryRunMode(3);
}
#[repr(transparent)]
pub struct EGenericAICheck(pub u8);
impl EGenericAICheck {
    pub const LESS: EGenericAICheck = EGenericAICheck(0);
    pub const LESS_OR_EQUAL: EGenericAICheck = EGenericAICheck(1);
    pub const EQUAL: EGenericAICheck = EGenericAICheck(2);
    pub const NOT_EQUAL: EGenericAICheck = EGenericAICheck(3);
    pub const GREATER_OR_EQUAL: EGenericAICheck = EGenericAICheck(4);
    pub const GREATER: EGenericAICheck = EGenericAICheck(5);
    pub const IS_TRUE: EGenericAICheck = EGenericAICheck(6);
    pub const MAX: EGenericAICheck = EGenericAICheck(7);
}
#[repr(transparent)]
pub struct EPathFollowingResult(pub u8);
impl EPathFollowingResult {
    pub const SUCCESS: EPathFollowingResult = EPathFollowingResult(0);
    pub const BLOCKED: EPathFollowingResult = EPathFollowingResult(1);
    pub const OFF_PATH: EPathFollowingResult = EPathFollowingResult(2);
    pub const ABORTED: EPathFollowingResult = EPathFollowingResult(3);
    pub const SKIPPED_DEPRECATED: EPathFollowingResult = EPathFollowingResult(4);
    pub const INVALID: EPathFollowingResult = EPathFollowingResult(5);
}
#[repr(transparent)]
pub struct EPathFollowingStatus(pub u8);
impl EPathFollowingStatus {
    pub const IDLE: EPathFollowingStatus = EPathFollowingStatus(0);
    pub const WAITING: EPathFollowingStatus = EPathFollowingStatus(1);
    pub const PAUSED: EPathFollowingStatus = EPathFollowingStatus(2);
    pub const MOVING: EPathFollowingStatus = EPathFollowingStatus(3);
}
#[repr(transparent)]
pub struct EPathFollowingRequestResult(pub u8);
impl EPathFollowingRequestResult {
    pub const FAILED: EPathFollowingRequestResult = EPathFollowingRequestResult(0);
    pub const ALREADY_AT_GOAL: EPathFollowingRequestResult = EPathFollowingRequestResult(
        1,
    );
    pub const REQUEST_SUCCESSFUL: EPathFollowingRequestResult = EPathFollowingRequestResult(
        2,
    );
}
#[repr(transparent)]
pub struct EBTNodeResult(pub u8);
impl EBTNodeResult {
    pub const SUCCEEDED: EBTNodeResult = EBTNodeResult(0);
    pub const FAILED: EBTNodeResult = EBTNodeResult(1);
    pub const ABORTED: EBTNodeResult = EBTNodeResult(2);
    pub const IN_PROGRESS: EBTNodeResult = EBTNodeResult(3);
}
#[repr(transparent)]
pub struct EPathFollowingAction(pub u8);
impl EPathFollowingAction {
    pub const ERROR: EPathFollowingAction = EPathFollowingAction(0);
    pub const NO_MOVE: EPathFollowingAction = EPathFollowingAction(1);
    pub const DIRECT_MOVE: EPathFollowingAction = EPathFollowingAction(2);
    pub const PARTIAL_PATH: EPathFollowingAction = EPathFollowingAction(3);
    pub const PATH_TO_GOAL: EPathFollowingAction = EPathFollowingAction(4);
}
#[repr(transparent)]
pub struct EAIOptionFlag(pub u8);
impl EAIOptionFlag {
    pub const DEFAULT: EAIOptionFlag = EAIOptionFlag(0);
    pub const ENABLE: EAIOptionFlag = EAIOptionFlag(1);
    pub const DISABLE: EAIOptionFlag = EAIOptionFlag(2);
    pub const MAX: EAIOptionFlag = EAIOptionFlag(3);
}
#[repr(transparent)]
pub struct EBTFlowAbortMode(pub u8);
impl EBTFlowAbortMode {
    pub const NONE: EBTFlowAbortMode = EBTFlowAbortMode(0);
    pub const LOWER_PRIORITY: EBTFlowAbortMode = EBTFlowAbortMode(1);
    pub const SELF: EBTFlowAbortMode = EBTFlowAbortMode(2);
    pub const BOTH: EBTFlowAbortMode = EBTFlowAbortMode(3);
}
#[repr(transparent)]
pub struct EBTParallelMode(pub u8);
impl EBTParallelMode {
    pub const ABORT_BACKGROUND: EBTParallelMode = EBTParallelMode(0);
    pub const WAIT_FOR_BACKGROUND: EBTParallelMode = EBTParallelMode(1);
}
#[repr(transparent)]
pub struct EBTBlackboardRestart(pub u8);
impl EBTBlackboardRestart {
    pub const VALUE_CHANGE: EBTBlackboardRestart = EBTBlackboardRestart(0);
    pub const RESULT_CHANGE: EBTBlackboardRestart = EBTBlackboardRestart(1);
}
#[repr(transparent)]
pub struct EBasicKeyOperation(pub u8);
impl EBasicKeyOperation {
    pub const SET: EBasicKeyOperation = EBasicKeyOperation(0);
    pub const NOT_SET: EBasicKeyOperation = EBasicKeyOperation(1);
}
#[repr(transparent)]
pub struct EArithmeticKeyOperation(pub u8);
impl EArithmeticKeyOperation {
    pub const EQUAL: EArithmeticKeyOperation = EArithmeticKeyOperation(0);
    pub const NOT_EQUAL: EArithmeticKeyOperation = EArithmeticKeyOperation(1);
    pub const LESS: EArithmeticKeyOperation = EArithmeticKeyOperation(2);
    pub const LESS_OR_EQUAL: EArithmeticKeyOperation = EArithmeticKeyOperation(3);
    pub const GREATER: EArithmeticKeyOperation = EArithmeticKeyOperation(4);
    pub const GREATER_OR_EQUAL: EArithmeticKeyOperation = EArithmeticKeyOperation(5);
}
#[repr(transparent)]
pub struct ETextKeyOperation(pub u8);
impl ETextKeyOperation {
    pub const EQUAL: ETextKeyOperation = ETextKeyOperation(0);
    pub const NOT_EQUAL: ETextKeyOperation = ETextKeyOperation(1);
    pub const CONTAIN: ETextKeyOperation = ETextKeyOperation(2);
    pub const NOT_CONTAIN: ETextKeyOperation = ETextKeyOperation(3);
}
#[repr(transparent)]
pub struct EBlackBoardEntryComparison(pub u8);
impl EBlackBoardEntryComparison {
    pub const EQUAL: EBlackBoardEntryComparison = EBlackBoardEntryComparison(0);
    pub const NOT_EQUAL: EBlackBoardEntryComparison = EBlackBoardEntryComparison(1);
}
#[repr(transparent)]
pub struct FAIDistanceType(pub u8);
impl FAIDistanceType {
    pub const DISTANCE3_D: FAIDistanceType = FAIDistanceType(0);
    pub const DISTANCE2_D: FAIDistanceType = FAIDistanceType(1);
    pub const DISTANCE_Z: FAIDistanceType = FAIDistanceType(2);
    pub const MAX: FAIDistanceType = FAIDistanceType(3);
}
#[repr(transparent)]
pub struct EEnvQueryResultNormalizationOption(pub u8);
impl EEnvQueryResultNormalizationOption {
    pub const DEFAULT: EEnvQueryResultNormalizationOption = EEnvQueryResultNormalizationOption(
        0,
    );
    pub const NORMALIZED: EEnvQueryResultNormalizationOption = EEnvQueryResultNormalizationOption(
        1,
    );
    pub const UNALTERED: EEnvQueryResultNormalizationOption = EEnvQueryResultNormalizationOption(
        2,
    );
}
#[repr(transparent)]
pub struct EEnvQueryStatus(pub u8);
impl EEnvQueryStatus {
    pub const PROCESSING: EEnvQueryStatus = EEnvQueryStatus(0);
    pub const SUCCESS: EEnvQueryStatus = EEnvQueryStatus(1);
    pub const FAILED: EEnvQueryStatus = EEnvQueryStatus(2);
    pub const ABORTED: EEnvQueryStatus = EEnvQueryStatus(3);
    pub const OWNER_LOST: EEnvQueryStatus = EEnvQueryStatus(4);
    pub const MISSING_PARAM: EEnvQueryStatus = EEnvQueryStatus(5);
}
#[repr(transparent)]
pub struct EEnvTestPurpose(pub u8);
impl EEnvTestPurpose {
    pub const FILTER: EEnvTestPurpose = EEnvTestPurpose(0);
    pub const SCORE: EEnvTestPurpose = EEnvTestPurpose(1);
    pub const FILTER_AND_SCORE: EEnvTestPurpose = EEnvTestPurpose(2);
}
#[repr(transparent)]
pub struct EEnvTestFilterOperator(pub u8);
impl EEnvTestFilterOperator {
    pub const ALL_PASS: EEnvTestFilterOperator = EEnvTestFilterOperator(0);
    pub const ANY_PASS: EEnvTestFilterOperator = EEnvTestFilterOperator(1);
}
#[repr(transparent)]
pub struct EEnvTestScoreOperator(pub u8);
impl EEnvTestScoreOperator {
    pub const AVERAGE_SCORE: EEnvTestScoreOperator = EEnvTestScoreOperator(0);
    pub const MIN_SCORE: EEnvTestScoreOperator = EEnvTestScoreOperator(1);
    pub const MAX_SCORE: EEnvTestScoreOperator = EEnvTestScoreOperator(2);
    pub const MULTIPLY: EEnvTestScoreOperator = EEnvTestScoreOperator(3);
}
#[repr(transparent)]
pub struct EEnvTestFilterType(pub u8);
impl EEnvTestFilterType {
    pub const MINIMUM: EEnvTestFilterType = EEnvTestFilterType(0);
    pub const MAXIMUM: EEnvTestFilterType = EEnvTestFilterType(1);
    pub const RANGE: EEnvTestFilterType = EEnvTestFilterType(2);
    pub const MATCH: EEnvTestFilterType = EEnvTestFilterType(3);
}
#[repr(transparent)]
pub struct EEnvTestScoreEquation(pub u8);
impl EEnvTestScoreEquation {
    pub const LINEAR: EEnvTestScoreEquation = EEnvTestScoreEquation(0);
    pub const SQUARE: EEnvTestScoreEquation = EEnvTestScoreEquation(1);
    pub const INVERSE_LINEAR: EEnvTestScoreEquation = EEnvTestScoreEquation(2);
    pub const SQUARE_ROOT: EEnvTestScoreEquation = EEnvTestScoreEquation(3);
    pub const CONSTANT: EEnvTestScoreEquation = EEnvTestScoreEquation(4);
}
#[repr(transparent)]
pub struct EEnvQueryTestClamping(pub u8);
impl EEnvQueryTestClamping {
    pub const NONE: EEnvQueryTestClamping = EEnvQueryTestClamping(0);
    pub const SPECIFIED_VALUE: EEnvQueryTestClamping = EEnvQueryTestClamping(1);
    pub const FILTER_THRESHOLD: EEnvQueryTestClamping = EEnvQueryTestClamping(2);
}
#[repr(transparent)]
pub struct EEQSNormalizationType(pub u8);
impl EEQSNormalizationType {
    pub const ABSOLUTE: EEQSNormalizationType = EEQSNormalizationType(0);
    pub const RELATIVE_TO_SCORES: EEQSNormalizationType = EEQSNormalizationType(1);
}
#[repr(transparent)]
pub struct EEnvQueryHightlightMode(pub u8);
impl EEnvQueryHightlightMode {
    pub const ALL: EEnvQueryHightlightMode = EEnvQueryHightlightMode(0);
    pub const BEST5_PCT: EEnvQueryHightlightMode = EEnvQueryHightlightMode(1);
    pub const BEST25_PCT: EEnvQueryHightlightMode = EEnvQueryHightlightMode(2);
}
#[repr(transparent)]
pub struct EPointOnCircleSpacingMethod(pub u8);
impl EPointOnCircleSpacingMethod {
    pub const BY_SPACE_BETWEEN: EPointOnCircleSpacingMethod = EPointOnCircleSpacingMethod(
        0,
    );
    pub const BY_NUMBER_OF_POINTS: EPointOnCircleSpacingMethod = EPointOnCircleSpacingMethod(
        1,
    );
}
#[repr(transparent)]
pub struct EEnvTestDistance(pub u8);
impl EEnvTestDistance {
    pub const DISTANCE3_D: EEnvTestDistance = EEnvTestDistance(0);
    pub const DISTANCE2_D: EEnvTestDistance = EEnvTestDistance(1);
    pub const DISTANCE_Z: EEnvTestDistance = EEnvTestDistance(2);
    pub const DISTANCE_ABSOLUTE_Z: EEnvTestDistance = EEnvTestDistance(3);
}
#[repr(transparent)]
pub struct EEnvTestDot(pub u8);
impl EEnvTestDot {
    pub const DOT3_D: EEnvTestDot = EEnvTestDot(0);
    pub const DOT2_D: EEnvTestDot = EEnvTestDot(1);
}
#[repr(transparent)]
pub struct EEnvTestPathfinding(pub u8);
impl EEnvTestPathfinding {
    pub const PATH_EXIST: EEnvTestPathfinding = EEnvTestPathfinding(0);
    pub const PATH_COST: EEnvTestPathfinding = EEnvTestPathfinding(1);
    pub const PATH_LENGTH: EEnvTestPathfinding = EEnvTestPathfinding(2);
}
#[repr(transparent)]
pub struct EAISenseNotifyType(pub u8);
impl EAISenseNotifyType {
    pub const ON_EVERY_PERCEPTION: EAISenseNotifyType = EAISenseNotifyType(0);
    pub const ON_PERCEPTION_CHANGE: EAISenseNotifyType = EAISenseNotifyType(1);
}
