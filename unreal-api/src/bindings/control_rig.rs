#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FRigElementKey {
    pub ty: ERigElementType,
    pub name: FName,
}
impl FRigElementKey {}
#[repr(C, align(8))]
pub struct FRigBaseElement {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub key: FRigElementKey,
    pub index: i32,
    pub sub_index: i32,
    pub created_at_instruction_index: i32,
    pub spawn_index: i32,
    pub b_selected: bool,
    __padding_end: [u8; 47],
}
impl FRigBaseElement {}
#[repr(C, align(8))]
pub struct FRigTransformElement {
    __padding_end: [u8; 288],
}
impl FRigTransformElement {}
#[repr(C, align(16))]
pub struct FRigMultiParentElement {
    __padding_end: [u8; 560],
}
impl FRigMultiParentElement {}
#[repr(C, align(16))]
pub struct FRigControlElement {
    #[doc(hidden)]
    __padding_560: [u8; 560],
    pub settings: FRigControlSettings,
    pub preferred_euler_angles: FRigPreferredEulerAngles,
    __padding_end: [u8; 264],
}
impl FRigControlElement {}
#[repr(C, align(8))]
pub struct FRigPreferredEulerAngles {
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub current: crate::bindings::core_u_object::FVector,
    pub initial: crate::bindings::core_u_object::FVector,
}
impl FRigPreferredEulerAngles {}
#[repr(C, align(16))]
pub struct FRigControlSettings {
    pub animation_type: ERigControlAnimationType,
    pub control_type: ERigControlType,
    pub display_name: FName,
    pub primary_axis: ERigControlAxis,
    pub limit_enabled: TArray<FRigControlLimitEnabled>,
    pub b_draw_limits: bool,
    pub minimum_value: FRigControlValue,
    pub maximum_value: FRigControlValue,
    pub b_shape_visible: bool,
    pub shape_visibility: ERigControlVisibility,
    pub shape_name: FName,
    pub shape_color: crate::bindings::core_u_object::FLinearColor,
    pub b_is_transient_control: bool,
    pub control_enum: UPtr<crate::bindings::core_u_object::UEnum>,
    pub customization: FRigControlElementCustomization,
    pub driven_controls: TArray<FRigElementKey>,
    #[doc(hidden)]
    __padding_640: [u8; 16],
    pub b_group_with_parent_control: bool,
    pub b_restrict_space_switching: bool,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
    pub preferred_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub b_use_preferred_rotation_order: bool,
    pub shape_transform: crate::bindings::core_u_object::FTransform,
}
impl FRigControlSettings {}
#[repr(C, align(8))]
pub struct FRigControlElementCustomization {
    pub available_spaces: TArray<FRigElementKeyWithLabel>,
    pub removed_spaces: TArray<FRigElementKey>,
}
impl FRigControlElementCustomization {}
#[repr(C, align(4))]
pub struct FRigElementKeyWithLabel {
    pub key: FRigElementKey,
    pub label: FName,
}
impl FRigElementKeyWithLabel {}
#[repr(C, align(16))]
pub struct FRigControlValue {
    __padding_end: [u8; 240],
}
impl FRigControlValue {}
#[repr(C, align(4))]
pub struct FRigControlValueStorage {
    __padding_end: [u8; 132],
}
impl FRigControlValueStorage {}
#[repr(C, align(1))]
pub struct FRigControlLimitEnabled {
    pub b_minimum: bool,
    pub b_maximum: bool,
}
impl FRigControlLimitEnabled {}
#[repr(C, align(8))]
pub struct FRigModuleSettings {
    pub identifier: FRigModuleIdentifier,
    pub icon: crate::bindings::core_u_object::FSoftObjectPath,
    pub category: FString,
    pub keywords: FString,
    pub description: FString,
    pub exposed_connectors: TArray<FRigModuleConnector>,
}
impl FRigModuleSettings {}
#[repr(C, align(8))]
pub struct FRigModuleConnector {
    pub name: FString,
    pub settings: FRigConnectorSettings,
}
impl FRigModuleConnector {}
#[repr(C, align(8))]
pub struct FRigConnectorSettings {
    pub description: FString,
    pub ty: EConnectorType,
    pub b_optional: bool,
    pub b_is_array: bool,
    pub b_post_construction: bool,
    pub rules: TArray<FRigConnectionRuleStash>,
}
impl FRigConnectorSettings {}
#[repr(C, align(8))]
pub struct FRigConnectionRuleStash {
    pub script_struct_path: FString,
    pub exported_text: FString,
}
impl FRigConnectionRuleStash {}
#[repr(C, align(8))]
pub struct FRigModuleIdentifier {
    pub name: FString,
    pub ty: FString,
}
impl FRigModuleIdentifier {}
#[repr(C, align(8))]
pub struct FRigInfluenceMapPerEvent {
    __padding_end: [u8; 96],
}
impl FRigInfluenceMapPerEvent {}
#[repr(C, align(8))]
pub struct FRigInfluenceMap {
    __padding_end: [u8; 112],
}
impl FRigInfluenceMap {}
#[repr(C, align(8))]
pub struct FRigInfluenceEntry {
    __padding_end: [u8; 32],
}
impl FRigInfluenceEntry {}
#[repr(C, align(8))]
pub struct FRigUnit {
    __padding_end: [u8; 8],
}
impl FRigUnit {}
#[repr(C, align(8))]
pub struct FRigUnit_DebugBase {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub debug_draw_settings: crate::bindings::rig_vm::FRigVMDebugDrawSettings,
}
impl FRigUnit_DebugBase {}
#[repr(C, align(8))]
pub struct FRigUnitMutable {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnitMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_DebugBaseMutable {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub debug_draw_settings: crate::bindings::rig_vm::FRigVMDebugDrawSettings,
}
impl FRigUnit_DebugBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_HighlevelBase {
    __padding_end: [u8; 8],
}
impl FRigUnit_HighlevelBase {}
#[repr(C, align(8))]
pub struct FRigUnit_HighlevelBaseMutable {
    __padding_end: [u8; 16],
}
impl FRigUnit_HighlevelBaseMutable {}
#[repr(C, align(8))]
pub struct FControlRigReference {
    __padding_end: [u8; 16],
}
impl FControlRigReference {}
#[repr(C, align(16))]
pub struct FControlRigComponentMappedElement {
    pub component_reference: crate::bindings::engine::FSoftComponentReference,
    pub transform_index: i32,
    pub transform_name: FName,
    pub element_type: ERigElementType,
    pub element_name: FName,
    pub direction: EControlRigComponentMapDirection,
    pub offset: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub space: EControlRigComponentSpace,
    __padding_end: [u8; 27],
}
impl FControlRigComponentMappedElement {}
#[repr(C, align(8))]
pub struct FControlRigComponentMappedComponent {
    pub component: UPtr<crate::bindings::engine::USceneComponent>,
    pub element_name: FName,
    pub element_type: ERigElementType,
    pub direction: EControlRigComponentMapDirection,
    __padding_end: [u8; 2],
}
impl FControlRigComponentMappedComponent {}
#[repr(C, align(4))]
pub struct FControlRigComponentMappedBone {
    pub source: FName,
    pub target: FName,
}
impl FControlRigComponentMappedBone {}
#[repr(C, align(4))]
pub struct FControlRigComponentMappedCurve {
    pub source: FName,
    pub target: FName,
}
impl FControlRigComponentMappedCurve {}
#[repr(C, align(16))]
pub struct FControlRigShapeDefinition {
    pub shape_name: FName,
    pub static_mesh: TSoftObjectPtr<crate::bindings::engine::UStaticMesh>,
    pub transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 16],
}
impl FControlRigShapeDefinition {}
#[repr(C, align(8))]
pub struct FControlRigOverrideValue {
    __padding_end: [u8; 88],
}
impl FControlRigOverrideValue {}
#[repr(C, align(8))]
pub struct FControlRigOverrideContainer {
    __padding_end: [u8; 344],
}
impl FControlRigOverrideContainer {}
#[repr(C, align(8))]
pub struct FControlRigReplayVariable {
    pub name: FName,
    pub cpp_type: FName,
    pub value: FString,
}
impl FControlRigReplayVariable {}
#[repr(C, align(8))]
pub struct FControlRigTestDataFrame {
    pub absolute_time: f64,
    pub delta_time: f64,
    pub variables: TArray<FControlRigReplayVariable>,
    pub pose: FRigPose,
    pub metadata: TArray<u8>,
    __padding_end: [u8; 104],
}
impl FControlRigTestDataFrame {}
#[repr(C, align(8))]
pub struct FRigPose {
    __padding_end: [u8; 112],
}
impl FRigPose {}
#[repr(C, align(16))]
pub struct FRigPoseElement {
    __padding_end: [u8; 272],
}
impl FRigPoseElement {}
#[repr(C, align(8))]
pub struct FCachedRigElement {
    __padding_end: [u8; 32],
}
impl FCachedRigElement {}
#[repr(C, align(4))]
pub struct FCRSimLinearSpring {
    __padding_end: [u8; 16],
}
impl FCRSimLinearSpring {}
#[repr(C, align(16))]
pub struct FCRSimSoftCollision {
    __padding_end: [u8; 128],
}
impl FCRSimSoftCollision {}
#[repr(C, align(8))]
pub struct FCRSimPointForce {
    __padding_end: [u8; 40],
}
impl FCRSimPointForce {}
#[repr(C, align(8))]
pub struct FRigModuleInstance {
    __padding_end: [u8; 208],
}
impl FRigModuleInstance {}
#[repr(C, align(8))]
pub struct FRigModuleExecutionElement {
    __padding_end: [u8; 48],
}
impl FRigModuleExecutionElement {}
#[repr(C, align(8))]
pub struct FRigModuleExecutionQueue {
    __padding_end: [u8; 16],
}
impl FRigModuleExecutionQueue {}
#[repr(C, align(8))]
pub struct FRigModuleReference {
    __padding_end: [u8; 752],
}
impl FRigModuleReference {}
#[repr(C, align(8))]
pub struct FModularRigSingleConnection {
    __padding_end: [u8; 48],
}
impl FModularRigSingleConnection {}
#[repr(C, align(8))]
pub struct FModularRigConnections {
    __padding_end: [u8; 96],
}
impl FModularRigConnections {}
#[repr(C, align(8))]
pub struct FModularRigModel {
    __padding_end: [u8; 272],
}
impl FModularRigModel {}
#[repr(C, align(8))]
pub struct FRigHierarchyModulePath {
    __padding_end: [u8; 72],
}
impl FRigHierarchyModulePath {}
#[repr(C, align(8))]
pub struct FModularRigModuleSettingsForClipboard {
    pub module_class: crate::bindings::core_u_object::FSoftObjectPath,
    pub defaults: TMap<FString, FString>,
    pub overrides: TMap<FString, FString>,
    pub bindings: TMap<FName, FString>,
}
impl FModularRigModuleSettingsForClipboard {}
#[repr(C, align(8))]
pub struct FModularRigModuleSettingsSetForClipboard {
    pub settings: TMap<FName, FModularRigModuleSettingsForClipboard>,
}
impl FModularRigModuleSettingsSetForClipboard {}
#[repr(C, align(8))]
pub struct FRigElement {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub name: FName,
    pub index: i32,
}
impl FRigElement {}
#[repr(C, align(16))]
pub struct FRigBone {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub parent_name: FName,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
    pub global_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    #[doc(hidden)]
    __padding_352: [u8; 16],
    pub ty: ERigBoneType,
    __padding_end: [u8; 15],
}
impl FRigBone {}
#[repr(C, align(8))]
pub struct FRigAndConnectionRule {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub child_rules: TArray<FRigConnectionRuleStash>,
}
impl FRigAndConnectionRule {}
#[repr(C, align(8))]
pub struct FRigOrConnectionRule {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub child_rules: TArray<FRigConnectionRuleStash>,
}
impl FRigOrConnectionRule {}
#[repr(C, align(8))]
pub struct FRigTypeConnectionRule {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub element_type: ERigElementType,
    __padding_end: [u8; 7],
}
impl FRigTypeConnectionRule {}
#[repr(C, align(8))]
pub struct FRigTagConnectionRule {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub tag: FName,
    __padding_end: [u8; 4],
}
impl FRigTagConnectionRule {}
#[repr(C, align(8))]
pub struct FRigChildOfPrimaryConnectionRule {
    __padding_end: [u8; 8],
}
impl FRigChildOfPrimaryConnectionRule {}
#[repr(C, align(16))]
pub struct FRigControl {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub control_type: ERigControlType,
    pub display_name: FName,
    pub parent_name: FName,
    pub parent_index: i32,
    pub space_name: FName,
    pub space_index: i32,
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub initial_value: FRigControlValue,
    pub value: FRigControlValue,
    pub primary_axis: ERigControlAxis,
    #[doc(hidden)]
    __padding_658: [u8; 1],
    pub b_animatable: bool,
    pub b_limit_translation: bool,
    pub b_limit_rotation: bool,
    pub b_limit_scale: bool,
    pub b_draw_limits: bool,
    pub minimum_value: FRigControlValue,
    pub maximum_value: FRigControlValue,
    pub b_gizmo_enabled: bool,
    pub b_gizmo_visible: bool,
    pub gizmo_name: FName,
    pub gizmo_transform: crate::bindings::core_u_object::FTransform,
    pub gizmo_color: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_1296: [u8; 16],
    pub b_is_transient_control: bool,
    pub control_enum: UPtr<crate::bindings::core_u_object::UEnum>,
}
impl FRigControl {}
#[repr(C, align(8))]
pub struct FRigCurve {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub value: f32,
    __padding_end: [u8; 4],
}
impl FRigCurve {}
#[repr(C, align(8))]
pub struct FCachedRigComponent {
    __padding_end: [u8; 56],
}
impl FCachedRigComponent {}
#[repr(C, align(8))]
pub struct FRigBaseComponent {
    __padding_end: [u8; 88],
}
impl FRigBaseComponent {}
#[repr(C, align(16))]
pub struct FRigSpace {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub space_type: ERigSpaceType,
    pub parent_name: FName,
    pub parent_index: i32,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
}
impl FRigSpace {}
#[repr(C, align(4))]
pub struct FRigControlModifiedContext {
    __padding_end: [u8; 28],
}
impl FRigControlModifiedContext {}
#[repr(C, align(4))]
pub struct FRigComponentKey {
    pub element_key: FRigElementKey,
    pub name: FName,
}
impl FRigComponentKey {}
#[repr(C, align(4))]
pub struct FRigHierarchyKey {
    __padding_end: [u8; 52],
}
impl FRigHierarchyKey {}
#[repr(C, align(8))]
pub struct FRigElementKeyCollection {
    pub keys: TArray<FRigElementKey>,
}
impl FRigElementKeyCollection {}
#[repr(C, align(8))]
pub struct FRigEventContext {
    __padding_end: [u8; 48],
}
impl FRigEventContext {}
#[repr(C, align(8))]
pub struct FRigElementResolveResult {
    __padding_end: [u8; 40],
}
impl FRigElementResolveResult {}
#[repr(C, align(8))]
pub struct FModularRigResolveResult {
    __padding_end: [u8; 72],
}
impl FModularRigResolveResult {}
#[repr(C, align(8))]
pub struct FRigTransformDirtyState {
    __padding_end: [u8; 16],
}
impl FRigTransformDirtyState {}
#[repr(C, align(8))]
pub struct FRigLocalAndGlobalDirtyState {
    pub global: FRigTransformDirtyState,
    pub local: FRigTransformDirtyState,
}
impl FRigLocalAndGlobalDirtyState {}
#[repr(C, align(8))]
pub struct FRigCurrentAndInitialDirtyState {
    pub current: FRigLocalAndGlobalDirtyState,
    pub initial: FRigLocalAndGlobalDirtyState,
}
impl FRigCurrentAndInitialDirtyState {}
#[repr(C, align(8))]
pub struct FRigComputedTransform {
    __padding_end: [u8; 16],
}
impl FRigComputedTransform {}
#[repr(C, align(8))]
pub struct FRigLocalAndGlobalTransform {
    pub local: FRigComputedTransform,
    pub global: FRigComputedTransform,
}
impl FRigLocalAndGlobalTransform {}
#[repr(C, align(8))]
pub struct FRigCurrentAndInitialTransform {
    pub current: FRigLocalAndGlobalTransform,
    pub initial: FRigLocalAndGlobalTransform,
}
impl FRigCurrentAndInitialTransform {}
#[repr(C, align(8))]
pub struct FRigSingleParentElement {
    __padding_end: [u8; 296],
}
impl FRigSingleParentElement {}
#[repr(C, align(4))]
pub struct FRigElementWeight {
    pub location: f32,
    pub rotation: f32,
    pub scale: f32,
}
impl FRigElementWeight {}
#[repr(C, align(8))]
pub struct FRigBoneElement {
    #[doc(hidden)]
    __padding_296: [u8; 296],
    pub bone_type: ERigBoneType,
    __padding_end: [u8; 7],
}
impl FRigBoneElement {}
#[repr(C, align(16))]
pub struct FRigNullElement {
    __padding_end: [u8; 560],
}
impl FRigNullElement {}
#[repr(C, align(8))]
pub struct FRigCurveElement {
    __padding_end: [u8; 112],
}
impl FRigCurveElement {}
#[repr(C, align(8))]
pub struct FRigReferenceElement {
    __padding_end: [u8; 320],
}
impl FRigReferenceElement {}
#[repr(C, align(8))]
pub struct FRigConnectorState {
    pub name: FName,
    pub resolved_target: FRigElementKey,
    pub settings: FRigConnectorSettings,
}
impl FRigConnectorState {}
#[repr(C, align(8))]
pub struct FRigConnectorElement {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub settings: FRigConnectorSettings,
}
impl FRigConnectorElement {}
#[repr(C, align(16))]
pub struct FRigSocketState {
    pub name: FName,
    pub parent: FRigElementKey,
    pub initial_local_transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub description: FString,
}
impl FRigSocketState {}
#[repr(C, align(8))]
pub struct FRigSocketElement {
    __padding_end: [u8; 296],
}
impl FRigSocketElement {}
#[repr(C, align(1))]
pub struct FModularRigSettings {
    pub b_auto_resolve: bool,
}
impl FModularRigSettings {}
#[repr(C, align(8))]
pub struct FRigModuleDescription {
    pub path: crate::bindings::core_u_object::FSoftObjectPath,
    pub settings: FRigModuleSettings,
}
impl FRigModuleDescription {}
#[repr(C, align(8))]
pub struct FModuleReferenceData {
    __padding_end: [u8; 56],
}
impl FModuleReferenceData {}
#[repr(C, align(4))]
pub struct FRigPhysicsSolverID {
    pub guid: crate::bindings::core_u_object::FGuid,
}
impl FRigPhysicsSolverID {}
#[repr(C, align(16))]
pub struct FRigControlCopy {
    pub name: FName,
    pub control_type: ERigControlType,
    pub parent_key: FRigElementKey,
    #[doc(hidden)]
    __padding_272: [u8; 240],
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub parent_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub global_transform: crate::bindings::core_u_object::FTransform,
}
impl FRigControlCopy {}
#[repr(C, align(8))]
pub struct FControlRigControlPose {
    pub copy_of_controls: TArray<FRigControlCopy>,
    __padding_end: [u8; 80],
}
impl FControlRigControlPose {}
#[repr(C, align(8))]
pub struct FRigDispatchFactory {
    __padding_end: [u8; 168],
}
impl FRigDispatchFactory {}
#[repr(C, align(8))]
pub struct FRigDispatch_AnimAttributeBase {
    __padding_end: [u8; 216],
}
impl FRigDispatch_AnimAttributeBase {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetAnimAttribute {
    __padding_end: [u8; 216],
}
impl FRigDispatch_GetAnimAttribute {}
#[repr(C, align(8))]
pub struct FRigDispatch_SetAnimAttribute {
    __padding_end: [u8; 216],
}
impl FRigDispatch_SetAnimAttribute {}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTraceWorld {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub channel: crate::bindings::engine::ECollisionChannel,
    pub radius: f32,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 56],
}
impl FRigUnit_SphereTraceWorld {}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTraceByTraceChannel {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub trace_channel: crate::bindings::engine::ETraceTypeQuery,
    pub radius: f32,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 56],
}
impl FRigUnit_SphereTraceByTraceChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_LineTraceByTraceChannel {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub trace_channel: crate::bindings::engine::ETraceTypeQuery,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 56],
}
impl FRigUnit_LineTraceByTraceChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTraceByObjectTypes {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub object_types: TArray<crate::bindings::engine::EObjectTypeQuery>,
    pub radius: f32,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 56],
}
impl FRigUnit_SphereTraceByObjectTypes {}
#[repr(C, align(8))]
pub struct FRigUnit_LineTraceByObjectTypes {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub object_types: TArray<crate::bindings::engine::EObjectTypeQuery>,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 56],
}
impl FRigUnit_LineTraceByObjectTypes {}
#[repr(C, align(16))]
pub struct FRigUnit_Control {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub transform: crate::bindings::animation_core::FEulerTransform,
    pub base: crate::bindings::core_u_object::FTransform,
    pub init_transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
    pub filter: crate::bindings::animation_core::FTransformFilter,
    __padding_end: [u8; 7],
}
impl FRigUnit_Control {}
#[repr(C, align(16))]
pub struct FRigUnit_Control_StaticMesh {
    #[doc(hidden)]
    __padding_384: [u8; 384],
    pub mesh_transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_Control_StaticMesh {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetUserData {
    __padding_end: [u8; 168],
}
impl FRigDispatch_GetUserData {}
#[repr(C, align(8))]
pub struct FRigUnit_SetupShapeLibraryFromUserData {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub name_space: FString,
    pub path: FString,
    pub library_name: FString,
    pub log_shape_libraries: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_SetupShapeLibraryFromUserData {}
#[repr(C, align(8))]
pub struct FRigUnit_ShapeExists {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub shape_name: FName,
    pub result: bool,
    __padding_end: [u8; 3],
}
impl FRigUnit_ShapeExists {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugBezier {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub minimum_u: f32,
    pub maximum_u: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub detail: i32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugBezier {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugBezierItemSpace {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub minimum_u: f32,
    pub maximum_u: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub detail: i32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugBezierItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugHierarchy {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
    pub items: TArray<FRigElementKey>,
    pub scale: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugHierarchy {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugPose {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
    pub pose: FRigPose,
    pub items: TArray<FRigElementKey>,
    pub scale: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugPose {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLine {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugLine {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLineItemSpace {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugLineItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLineStrip {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugLineStrip {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLineStripItemSpace {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugLineStripItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugRectangle {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugRectangle {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugRectangleItemSpace {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugRectangleItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugArc {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub radius: f32,
    pub minimum_degrees: f32,
    pub maximum_degrees: f32,
    pub thickness: f32,
    pub detail: i32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugArc {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugArcItemSpace {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub radius: f32,
    pub minimum_degrees: f32,
    pub maximum_degrees: f32,
    pub thickness: f32,
    pub detail: i32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugArcItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformMutable {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugTransformMutable {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformMutableItemSpace {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugTransformMutableItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformArrayMutable {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 31],
}
impl FRigUnit_DebugTransformArrayMutable {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformArrayMutableItemSpace {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub parent_indices: TArray<i32>,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_DebugTransformArrayMutableItemSpace {}
#[repr(C, align(8))]
pub struct FRigUnit_StartProfilingTimer {
    __padding_end: [u8; 24],
}
impl FRigUnit_StartProfilingTimer {}
#[repr(C, align(8))]
pub struct FRigUnit_EndProfilingTimer {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub number_of_measurements: i32,
    pub prefix: FString,
    __padding_end: [u8; 16],
}
impl FRigUnit_EndProfilingTimer {}
#[repr(C, align(8))]
pub struct FRigUnit_VisualDebugVector {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FVector,
    pub b_enabled: bool,
    pub mode: crate::bindings::rig_vm::ERigUnitVisualDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
impl FRigUnit_VisualDebugVector {}
#[repr(C, align(8))]
pub struct FRigUnit_VisualDebugVectorItemSpace {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FVector,
    pub b_enabled: bool,
    pub mode: crate::bindings::rig_vm::ERigUnitVisualDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
    __padding_end: [u8; 4],
}
impl FRigUnit_VisualDebugVectorItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugQuat {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
    __padding_end: [u8; 8],
}
impl FRigUnit_VisualDebugQuat {}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugQuatItemSpace {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
    __padding_end: [u8; 4],
}
impl FRigUnit_VisualDebugQuatItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
    __padding_end: [u8; 8],
}
impl FRigUnit_VisualDebugTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugTransformItemSpace {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
    __padding_end: [u8; 4],
}
impl FRigUnit_VisualDebugTransformItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub input: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::animation_core::FEulerTransform,
    __padding_end: [u8; 8],
}
impl FRigUnit_ConvertTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertEulerTransform {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub input: crate::bindings::animation_core::FEulerTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_ConvertEulerTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertRotation {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub input: crate::bindings::core_u_object::FRotator,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_ConvertRotation {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertVectorRotation {
    __padding_end: [u8; 64],
}
impl FRigUnit_ConvertVectorRotation {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertQuaternion {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub input: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FRotator,
    __padding_end: [u8; 8],
}
impl FRigUnit_ConvertQuaternion {}
#[repr(C, align(8))]
pub struct FRigUnit_ConvertVectorToRotation {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub input: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FRotator,
}
impl FRigUnit_ConvertVectorToRotation {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertVectorToQuaternion {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub input: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_ConvertVectorToQuaternion {}
#[repr(C, align(8))]
pub struct FRigUnit_ConvertRotationToVector {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub input: crate::bindings::core_u_object::FRotator,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_ConvertRotationToVector {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertQuaternionToVector {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub input: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 8],
}
impl FRigUnit_ConvertQuaternionToVector {}
#[repr(C, align(16))]
pub struct FRigUnit_ToSwingAndTwist {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub input: crate::bindings::core_u_object::FQuat,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub swing: crate::bindings::core_u_object::FQuat,
    pub twist: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_ToSwingAndTwist {}
#[repr(C, align(8))]
pub struct FRigUnit_BinaryFloatOp {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub argument0: f32,
    pub argument1: f32,
    pub result: f32,
    __padding_end: [u8; 4],
}
impl FRigUnit_BinaryFloatOp {}
#[repr(C, align(8))]
pub struct FRigUnit_Multiply_FloatFloat {
    __padding_end: [u8; 24],
}
impl FRigUnit_Multiply_FloatFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_Add_FloatFloat {
    __padding_end: [u8; 24],
}
impl FRigUnit_Add_FloatFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_Subtract_FloatFloat {
    __padding_end: [u8; 24],
}
impl FRigUnit_Subtract_FloatFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_Divide_FloatFloat {
    __padding_end: [u8; 24],
}
impl FRigUnit_Divide_FloatFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_Clamp_Float {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub result: f32,
}
impl FRigUnit_Clamp_Float {}
#[repr(C, align(8))]
pub struct FRigUnit_MapRange_Float {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub value: f32,
    pub min_in: f32,
    pub max_in: f32,
    pub min_out: f32,
    pub max_out: f32,
    pub result: f32,
}
impl FRigUnit_MapRange_Float {}
#[repr(C, align(16))]
pub struct FRigUnit_BinaryQuaternionOp {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub argument0: crate::bindings::core_u_object::FQuat,
    pub argument1: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_BinaryQuaternionOp {}
#[repr(C, align(16))]
pub struct FRigUnit_MultiplyQuaternion {
    __padding_end: [u8; 112],
}
impl FRigUnit_MultiplyQuaternion {}
#[repr(C, align(16))]
pub struct FRigUnit_UnaryQuaternionOp {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub argument: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_UnaryQuaternionOp {}
#[repr(C, align(16))]
pub struct FRigUnit_InverseQuaterion {
    __padding_end: [u8; 80],
}
impl FRigUnit_InverseQuaterion {}
#[repr(C, align(16))]
pub struct FRigUnit_QuaternionToAxisAndAngle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub argument: crate::bindings::core_u_object::FQuat,
    pub axis: crate::bindings::core_u_object::FVector,
    pub angle: f32,
    __padding_end: [u8; 4],
}
impl FRigUnit_QuaternionToAxisAndAngle {}
#[repr(C, align(16))]
pub struct FRigUnit_QuaternionFromAxisAndAngle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub axis: crate::bindings::core_u_object::FVector,
    pub angle: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_QuaternionFromAxisAndAngle {}
#[repr(C, align(16))]
pub struct FRigUnit_QuaternionToAngle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub axis: crate::bindings::core_u_object::FVector,
    pub argument: crate::bindings::core_u_object::FQuat,
    pub angle: f32,
    __padding_end: [u8; 12],
}
impl FRigUnit_QuaternionToAngle {}
#[repr(C, align(16))]
pub struct FRigUnit_BinaryTransformOp {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub argument0: crate::bindings::core_u_object::FTransform,
    pub argument1: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_BinaryTransformOp {}
#[repr(C, align(16))]
pub struct FRigUnit_MultiplyTransform {
    __padding_end: [u8; 304],
}
impl FRigUnit_MultiplyTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetRelativeTransform {
    __padding_end: [u8; 304],
}
impl FRigUnit_GetRelativeTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_BinaryVectorOp {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub argument0: crate::bindings::core_u_object::FVector,
    pub argument1: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_BinaryVectorOp {}
#[repr(C, align(8))]
pub struct FRigUnit_Multiply_VectorVector {
    __padding_end: [u8; 80],
}
impl FRigUnit_Multiply_VectorVector {}
#[repr(C, align(8))]
pub struct FRigUnit_Add_VectorVector {
    __padding_end: [u8; 80],
}
impl FRigUnit_Add_VectorVector {}
#[repr(C, align(8))]
pub struct FRigUnit_Subtract_VectorVector {
    __padding_end: [u8; 80],
}
impl FRigUnit_Subtract_VectorVector {}
#[repr(C, align(8))]
pub struct FRigUnit_Divide_VectorVector {
    __padding_end: [u8; 80],
}
impl FRigUnit_Divide_VectorVector {}
#[repr(C, align(8))]
pub struct FRigUnit_Distance_VectorVector {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub argument0: crate::bindings::core_u_object::FVector,
    pub argument1: crate::bindings::core_u_object::FVector,
    pub result: f32,
    __padding_end: [u8; 4],
}
impl FRigUnit_Distance_VectorVector {}
#[repr(C, align(16))]
pub struct FAimTarget {
    __padding_end: [u8; 144],
}
impl FAimTarget {}
#[repr(C, align(8))]
pub struct FRigUnit_AimConstraint {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub joint: FName,
    pub aim_mode: EAimMode,
    pub up_mode: EAimMode,
    pub aim_vector: crate::bindings::core_u_object::FVector,
    pub up_vector: crate::bindings::core_u_object::FVector,
    pub aim_targets: TArray<FAimTarget>,
    pub up_targets: TArray<FAimTarget>,
    __padding_end: [u8; 16],
}
impl FRigUnit_AimConstraint {}
#[repr(C, align(16))]
pub struct FRigUnit_ApplyFK {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub joint: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub filter: crate::bindings::animation_core::FTransformFilter,
    pub apply_transform_mode: EApplyTransformMode,
    pub apply_transform_space: ETransformSpaceMode,
    pub base_transform: crate::bindings::core_u_object::FTransform,
    pub base_joint: FName,
    __padding_end: [u8; 4],
}
impl FRigUnit_ApplyFK {}
#[repr(C, align(16))]
pub struct FBlendTarget {
    __padding_end: [u8; 112],
}
impl FBlendTarget {}
#[repr(C, align(16))]
pub struct FRigUnit_BlendTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub source: crate::bindings::core_u_object::FTransform,
    pub targets: TArray<FBlendTarget>,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_BlendTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetJointTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub joint: FName,
    pub ty: ETransformGetterType,
    pub transform_space: ETransformSpaceMode,
    pub base_transform: crate::bindings::core_u_object::FTransform,
    pub base_joint: FName,
    pub output: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_GetJointTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKFK {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub start_joint: FName,
    pub end_joint: FName,
    pub pole_target: crate::bindings::core_u_object::FVector,
    pub spin: f32,
    pub end_effector: crate::bindings::core_u_object::FTransform,
    pub ik_blend: f32,
    __padding_end: [u8; 636],
}
impl FRigUnit_TwoBoneIKFK {}
#[repr(C, align(16))]
pub struct FRigUnit_DrawContainerGetInstruction {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub instruction_name: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_DrawContainerGetInstruction {}
#[repr(C, align(8))]
pub struct FRigUnit_DrawContainerSetColor {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub instruction_name: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 4],
}
impl FRigUnit_DrawContainerSetColor {}
#[repr(C, align(8))]
pub struct FRigUnit_DrawContainerSetThickness {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub instruction_name: FName,
    pub thickness: f32,
}
impl FRigUnit_DrawContainerSetThickness {}
#[repr(C, align(16))]
pub struct FRigUnit_DrawContainerSetTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub instruction_name: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_DrawContainerSetTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_BeginExecution {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_BeginExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_PreBeginExecution {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_PreBeginExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_PostBeginExecution {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_PostBeginExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionBase {
    __padding_end: [u8; 8],
}
impl FRigUnit_CollectionBase {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionBaseMutable {
    __padding_end: [u8; 16],
}
impl FRigUnit_CollectionBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionChain {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub first_item: FRigElementKey,
    pub last_item: FRigElementKey,
    pub reverse: bool,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionChain {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionChainArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub first_item: FRigElementKey,
    pub last_item: FRigElementKey,
    pub reverse: bool,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_CollectionChainArray {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionNameSearch {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub partial_name: FName,
    pub type_to_search: ERigElementType,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionNameSearch {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionNameSearchArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub partial_name: FName,
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_CollectionNameSearchArray {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionChildren {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub parent: FRigElementKey,
    pub b_include_parent: bool,
    pub b_recursive: bool,
    pub type_to_search: ERigElementType,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionChildren {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionChildrenArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub parent: FRigElementKey,
    pub b_include_parent: bool,
    pub b_recursive: bool,
    pub b_default_children: bool,
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_CollectionChildrenArray {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionGetAll {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_CollectionGetAll {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionReplaceItems {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub items: FRigElementKeyCollection,
    pub old: FName,
    pub new: FName,
    pub remove_invalid_items: bool,
    pub b_allow_duplicates: bool,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionReplaceItems {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionReplaceItemsArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
    pub old: FName,
    pub new: FName,
    pub remove_invalid_items: bool,
    pub b_allow_duplicates: bool,
    pub result: TArray<FRigElementKey>,
}
impl FRigUnit_CollectionReplaceItemsArray {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionItems {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
    pub b_allow_duplicates: bool,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionItems {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionGetItems {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub collection: FRigElementKeyCollection,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_CollectionGetItems {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionGetParentIndices {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub collection: FRigElementKeyCollection,
    pub parent_indices: TArray<i32>,
}
impl FRigUnit_CollectionGetParentIndices {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionGetParentIndicesItemArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
    pub parent_indices: TArray<i32>,
}
impl FRigUnit_CollectionGetParentIndicesItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionUnion {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub a: FRigElementKeyCollection,
    pub b: FRigElementKeyCollection,
    pub b_allow_duplicates: bool,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionUnion {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionIntersection {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub a: FRigElementKeyCollection,
    pub b: FRigElementKeyCollection,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionIntersection {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionDifference {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub a: FRigElementKeyCollection,
    pub b: FRigElementKeyCollection,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionDifference {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionReverse {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub collection: FRigElementKeyCollection,
    pub reversed: FRigElementKeyCollection,
}
impl FRigUnit_CollectionReverse {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionCount {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub collection: FRigElementKeyCollection,
    pub count: i32,
    __padding_end: [u8; 4],
}
impl FRigUnit_CollectionCount {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionItemAtIndex {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub collection: FRigElementKeyCollection,
    pub index: i32,
    pub item: FRigElementKey,
    __padding_end: [u8; 4],
}
impl FRigUnit_CollectionItemAtIndex {}
#[repr(C, align(16))]
pub struct FRigUnit_CollectionLoop {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub collection: FRigElementKeyCollection,
    pub item: FRigElementKey,
    pub index: i32,
    pub count: i32,
    pub ratio: f32,
    pub completed: FControlRigExecuteContext,
}
impl FRigUnit_CollectionLoop {}
#[repr(C, align(16))]
pub struct FControlRigExecuteContext {
    __padding_end: [u8; 608],
}
impl FControlRigExecuteContext {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionAddItem {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub collection: FRigElementKeyCollection,
    pub item: FRigElementKey,
    pub result: FRigElementKeyCollection,
}
impl FRigUnit_CollectionAddItem {}
#[repr(C, align(8))]
pub struct FRigUnit_DynamicHierarchyBase {
    __padding_end: [u8; 8],
}
impl FRigUnit_DynamicHierarchyBase {}
#[repr(C, align(8))]
pub struct FRigUnit_DynamicHierarchyBaseMutable {
    __padding_end: [u8; 16],
}
impl FRigUnit_DynamicHierarchyBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_AddParent {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub display_label: FName,
    __padding_end: [u8; 4],
}
impl FRigUnit_AddParent {}
#[repr(C, align(8))]
pub struct FRigUnit_SetDefaultParent {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
}
impl FRigUnit_SetDefaultParent {}
#[repr(C, align(8))]
pub struct FRigUnit_AddAvailableSpaces {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FRigElementKey,
    pub spaces: TArray<FRigElementKeyWithLabel>,
}
impl FRigUnit_AddAvailableSpaces {}
#[repr(C, align(8))]
pub struct FRigUnit_SetChannelHosts {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub channel: FRigElementKey,
    pub hosts: TArray<FRigElementKey>,
}
impl FRigUnit_SetChannelHosts {}
#[repr(C, align(8))]
pub struct FRigUnit_SwitchParent {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub mode: ERigSwitchParentMode,
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub b_maintain_global: bool,
    __padding_end: [u8; 3],
}
impl FRigUnit_SwitchParent {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParentWeights {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub weights: TArray<FRigElementWeight>,
    pub parents: FRigElementKeyCollection,
}
impl FRigUnit_HierarchyGetParentWeights {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParentWeightsArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub weights: TArray<FRigElementWeight>,
    pub parents: TArray<FRigElementKey>,
}
impl FRigUnit_HierarchyGetParentWeightsArray {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchySetParentWeights {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub weights: TArray<FRigElementWeight>,
}
impl FRigUnit_HierarchySetParentWeights {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyReset {
    __padding_end: [u8; 16],
}
impl FRigUnit_HierarchyReset {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyImportFromSkeleton {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub name_space: FName,
    pub b_include_curves: bool,
    pub b_include_mesh_sockets: bool,
    pub b_include_virtual_bones: bool,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_HierarchyImportFromSkeleton {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyRemoveElement {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub b_success: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_HierarchyRemoveElement {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddElement {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub parent: FRigElementKey,
    pub name: FName,
    pub item: FRigElementKey,
    __padding_end: [u8; 4],
}
impl FRigUnit_HierarchyAddElement {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddBone {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 15],
}
impl FRigUnit_HierarchyAddBone {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddNull {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 15],
}
impl FRigUnit_HierarchyAddNull {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControl_Settings {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub display_name: FName,
    pub b_selectable: bool,
    __padding_end: [u8; 3],
}
impl FRigUnit_HierarchyAddControl_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControl_ShapeSettings {
    pub b_visible: bool,
    pub name: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_HierarchyAddControl_ShapeSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControl_ProxySettings {
    pub b_is_proxy: bool,
    pub driven_controls: TArray<FRigElementKey>,
    pub shape_visibility: ERigControlVisibility,
    __padding_end: [u8; 7],
}
impl FRigUnit_HierarchyAddControl_ProxySettings {}
#[repr(C, align(4))]
pub struct FRigUnit_HierarchyAddControlFloat_LimitSettings {
    pub limit: FRigControlLimitEnabled,
    pub min_value: f32,
    pub max_value: f32,
    pub b_draw_limits: bool,
    __padding_end: [u8; 3],
}
impl FRigUnit_HierarchyAddControlFloat_LimitSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlFloat_Settings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub primary_axis: ERigControlAxis,
    pub b_is_scale: bool,
    pub limits: FRigUnit_HierarchyAddControlFloat_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
}
impl FRigUnit_HierarchyAddControlFloat_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlElement {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub offset_space: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 15],
}
impl FRigUnit_HierarchyAddControlElement {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlFloat {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub initial_value: f32,
    pub settings: FRigUnit_HierarchyAddControlFloat_Settings,
}
impl FRigUnit_HierarchyAddControlFloat {}
#[repr(C, align(4))]
pub struct FRigUnit_HierarchyAddControlInteger_LimitSettings {
    pub limit: FRigControlLimitEnabled,
    pub min_value: i32,
    pub max_value: i32,
    pub b_draw_limits: bool,
    __padding_end: [u8; 3],
}
impl FRigUnit_HierarchyAddControlInteger_LimitSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlInteger_Settings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub primary_axis: ERigControlAxis,
    pub control_enum: UPtr<crate::bindings::core_u_object::UEnum>,
    pub limits: FRigUnit_HierarchyAddControlInteger_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
}
impl FRigUnit_HierarchyAddControlInteger_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlInteger {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub initial_value: i32,
    pub settings: FRigUnit_HierarchyAddControlInteger_Settings,
}
impl FRigUnit_HierarchyAddControlInteger {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlVector2D_LimitSettings {
    pub limit_x: FRigControlLimitEnabled,
    pub limit_y: FRigControlLimitEnabled,
    pub min_value: crate::bindings::core_u_object::FVector2D,
    pub max_value: crate::bindings::core_u_object::FVector2D,
    pub b_draw_limits: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_HierarchyAddControlVector2D_LimitSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector2D_Settings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub primary_axis: ERigControlAxis,
    pub limits: FRigUnit_HierarchyAddControlVector2D_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
}
impl FRigUnit_HierarchyAddControlVector2D_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector2D {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub initial_value: crate::bindings::core_u_object::FVector2D,
    pub settings: FRigUnit_HierarchyAddControlVector2D_Settings,
}
impl FRigUnit_HierarchyAddControlVector2D {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlVector_LimitSettings {
    pub limit_x: FRigControlLimitEnabled,
    pub limit_y: FRigControlLimitEnabled,
    pub limit_z: FRigControlLimitEnabled,
    pub min_value: crate::bindings::core_u_object::FVector,
    pub max_value: crate::bindings::core_u_object::FVector,
    pub b_draw_limits: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_HierarchyAddControlVector_LimitSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector_Settings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub initial_space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_is_position: bool,
    pub limits: FRigUnit_HierarchyAddControlVector_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
}
impl FRigUnit_HierarchyAddControlVector_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub settings: FRigUnit_HierarchyAddControlVector_Settings,
}
impl FRigUnit_HierarchyAddControlVector {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlRotator_LimitSettings {
    pub limit_pitch: FRigControlLimitEnabled,
    pub limit_yaw: FRigControlLimitEnabled,
    pub limit_roll: FRigControlLimitEnabled,
    pub min_value: crate::bindings::core_u_object::FRotator,
    pub max_value: crate::bindings::core_u_object::FRotator,
    pub b_draw_limits: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_HierarchyAddControlRotator_LimitSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlRotator_Settings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub initial_space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub limits: FRigUnit_HierarchyAddControlRotator_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
    pub b_use_preferred_rotation_order: bool,
    pub preferred_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    __padding_end: [u8; 14],
}
impl FRigUnit_HierarchyAddControlRotator_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlRotator {
    #[doc(hidden)]
    __padding_168: [u8; 168],
    pub initial_value: crate::bindings::core_u_object::FRotator,
    pub settings: FRigUnit_HierarchyAddControlRotator_Settings,
}
impl FRigUnit_HierarchyAddControlRotator {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlTransform_LimitSettings {
    pub limit_translation_x: FRigControlLimitEnabled,
    pub limit_translation_y: FRigControlLimitEnabled,
    pub limit_translation_z: FRigControlLimitEnabled,
    pub limit_pitch: FRigControlLimitEnabled,
    pub limit_yaw: FRigControlLimitEnabled,
    pub limit_roll: FRigControlLimitEnabled,
    pub limit_scale_x: FRigControlLimitEnabled,
    pub limit_scale_y: FRigControlLimitEnabled,
    pub limit_scale_z: FRigControlLimitEnabled,
    pub min_value: crate::bindings::animation_core::FEulerTransform,
    pub max_value: crate::bindings::animation_core::FEulerTransform,
    pub b_draw_limits: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_HierarchyAddControlTransform_LimitSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlTransform_Settings {
    #[doc(hidden)]
    __padding_24: [u8; 24],
    pub initial_space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_use_preferred_rotation_order: bool,
    pub preferred_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub limits: FRigUnit_HierarchyAddControlTransform_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
}
impl FRigUnit_HierarchyAddControlTransform_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlTransform {
    #[doc(hidden)]
    __padding_176: [u8; 176],
    pub initial_value: crate::bindings::core_u_object::FTransform,
    pub settings: FRigUnit_HierarchyAddControlTransform_Settings,
}
impl FRigUnit_HierarchyAddControlTransform {}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannelEmptyLimitSettings {
    __padding_end: [u8; 1],
}
impl FRigUnit_HierarchyAddAnimationChannelEmptyLimitSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelBool {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub initial_value: bool,
    pub minimum_value: bool,
    pub maximum_value: bool,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelEmptyLimitSettings,
    __padding_end: [u8; 4],
}
impl FRigUnit_HierarchyAddAnimationChannelBool {}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings {
    pub enabled: FRigControlLimitEnabled,
}
impl FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelFloat {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub initial_value: f32,
    pub minimum_value: f32,
    pub maximum_value: f32,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings,
    __padding_end: [u8; 2],
}
impl FRigUnit_HierarchyAddAnimationChannelFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelScaleFloat {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub initial_value: f32,
    pub minimum_value: f32,
    pub maximum_value: f32,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings,
    __padding_end: [u8; 2],
}
impl FRigUnit_HierarchyAddAnimationChannelScaleFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelInteger {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub initial_value: i32,
    pub minimum_value: i32,
    pub maximum_value: i32,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings,
    pub control_enum: UPtr<crate::bindings::core_u_object::UEnum>,
}
impl FRigUnit_HierarchyAddAnimationChannelInteger {}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannel2DLimitSettings {
    pub x: FRigControlLimitEnabled,
    pub y: FRigControlLimitEnabled,
}
impl FRigUnit_HierarchyAddAnimationChannel2DLimitSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelVector2D {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub initial_value: crate::bindings::core_u_object::FVector2D,
    pub minimum_value: crate::bindings::core_u_object::FVector2D,
    pub maximum_value: crate::bindings::core_u_object::FVector2D,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannel2DLimitSettings,
    __padding_end: [u8; 4],
}
impl FRigUnit_HierarchyAddAnimationChannelVector2D {}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannelVectorLimitSettings {
    pub x: FRigControlLimitEnabled,
    pub y: FRigControlLimitEnabled,
    pub z: FRigControlLimitEnabled,
}
impl FRigUnit_HierarchyAddAnimationChannelVectorLimitSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelVector {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub minimum_value: crate::bindings::core_u_object::FVector,
    pub maximum_value: crate::bindings::core_u_object::FVector,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelVectorLimitSettings,
    __padding_end: [u8; 2],
}
impl FRigUnit_HierarchyAddAnimationChannelVector {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelScaleVector {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub minimum_value: crate::bindings::core_u_object::FVector,
    pub maximum_value: crate::bindings::core_u_object::FVector,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelVectorLimitSettings,
    __padding_end: [u8; 2],
}
impl FRigUnit_HierarchyAddAnimationChannelScaleVector {}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannelRotatorLimitSettings {
    pub pitch: FRigControlLimitEnabled,
    pub yaw: FRigControlLimitEnabled,
    pub roll: FRigControlLimitEnabled,
}
impl FRigUnit_HierarchyAddAnimationChannelRotatorLimitSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelRotator {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub initial_value: crate::bindings::core_u_object::FRotator,
    pub minimum_value: crate::bindings::core_u_object::FRotator,
    pub maximum_value: crate::bindings::core_u_object::FRotator,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelRotatorLimitSettings,
    __padding_end: [u8; 2],
}
impl FRigUnit_HierarchyAddAnimationChannelRotator {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyGetShapeSettings {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub settings: FRigUnit_HierarchyAddControl_ShapeSettings,
}
impl FRigUnit_HierarchyGetShapeSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchySetShapeSettings {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub settings: FRigUnit_HierarchyAddControl_ShapeSettings,
}
impl FRigUnit_HierarchySetShapeSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddSocket {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub description: FString,
    __padding_end: [u8; 8],
}
impl FRigUnit_HierarchyAddSocket {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyBase {
    __padding_end: [u8; 8],
}
impl FRigUnit_HierarchyBase {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyBaseMutable {
    __padding_end: [u8; 16],
}
impl FRigUnit_HierarchyBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParent {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub b_default_parent: bool,
    pub parent: FRigElementKey,
    __padding_end: [u8; 68],
}
impl FRigUnit_HierarchyGetParent {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParents {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub b_include_child: bool,
    pub b_reverse: bool,
    pub parents: FRigElementKeyCollection,
    __padding_end: [u8; 48],
}
impl FRigUnit_HierarchyGetParents {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParentsItemArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub b_include_child: bool,
    pub b_reverse: bool,
    pub b_default_parent: bool,
    pub parents: TArray<FRigElementKey>,
    __padding_end: [u8; 48],
}
impl FRigUnit_HierarchyGetParentsItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetChildren {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub parent: FRigElementKey,
    pub b_include_parent: bool,
    pub b_recursive: bool,
    pub children: FRigElementKeyCollection,
    __padding_end: [u8; 48],
}
impl FRigUnit_HierarchyGetChildren {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetSiblings {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub b_include_item: bool,
    pub siblings: FRigElementKeyCollection,
    __padding_end: [u8; 48],
}
impl FRigUnit_HierarchyGetSiblings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetSiblingsItemArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub b_include_item: bool,
    pub b_default_siblings: bool,
    pub siblings: TArray<FRigElementKey>,
    __padding_end: [u8; 48],
}
impl FRigUnit_HierarchyGetSiblingsItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetChainItemArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub start: FRigElementKey,
    pub end: FRigElementKey,
    pub b_include_start: bool,
    pub b_include_end: bool,
    pub b_reverse: bool,
    pub chain: TArray<FRigElementKey>,
    __padding_end: [u8; 80],
}
impl FRigUnit_HierarchyGetChainItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetPose {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub initial: bool,
    pub element_type: ERigElementType,
    pub items_to_get: FRigElementKeyCollection,
    pub pose: FRigPose,
}
impl FRigUnit_HierarchyGetPose {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetPoseItemArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub initial: bool,
    pub element_type: ERigElementType,
    pub items_to_get: TArray<FRigElementKey>,
    pub pose: FRigPose,
}
impl FRigUnit_HierarchyGetPoseItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchySetPose {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub items_to_set: FRigElementKeyCollection,
    pub weight: f32,
    __padding_end: [u8; 4],
}
impl FRigUnit_HierarchySetPose {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchySetPoseItemArray {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub items_to_set: TArray<FRigElementKey>,
    pub weight: f32,
    __padding_end: [u8; 4],
}
impl FRigUnit_HierarchySetPoseItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_PoseIsEmpty {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub pose: FRigPose,
    pub is_empty: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_PoseIsEmpty {}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetItems {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub items: FRigElementKeyCollection,
}
impl FRigUnit_PoseGetItems {}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetItemsItemArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_PoseGetItemsItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetDelta {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub pose_a: FRigPose,
    pub pose_b: FRigPose,
    pub position_threshold: f32,
    pub rotation_threshold: f32,
    pub scale_threshold: f32,
    pub curve_threshold: f32,
    pub element_type: ERigElementType,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub items_to_compare: FRigElementKeyCollection,
    pub poses_are_equal: bool,
    pub items_with_delta: FRigElementKeyCollection,
}
impl FRigUnit_PoseGetDelta {}
#[repr(C, align(16))]
pub struct FRigUnit_PoseGetTransform {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub pose: FRigPose,
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub valid: bool,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub curve_value: f32,
    __padding_end: [u8; 12],
}
impl FRigUnit_PoseGetTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetTransformArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub pose: FRigPose,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub valid: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
}
impl FRigUnit_PoseGetTransformArray {}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetCurve {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub pose: FRigPose,
    pub curve: FName,
    pub valid: bool,
    pub curve_value: f32,
    __padding_end: [u8; 12],
}
impl FRigUnit_PoseGetCurve {}
#[repr(C, align(16))]
pub struct FRigUnit_PoseLoop {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub pose: FRigPose,
    pub item: FRigElementKey,
    pub global_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub curve_value: f32,
    pub index: i32,
    pub count: i32,
    pub ratio: f32,
    pub completed: FControlRigExecuteContext,
}
impl FRigUnit_PoseLoop {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyCreatePoseItemArray_Entry {
    pub item: FRigElementKey,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub global_transform: crate::bindings::core_u_object::FTransform,
    pub use_euler_angles: bool,
    pub euler_angles: crate::bindings::core_u_object::FVector,
    pub curve_value: f32,
    __padding_end: [u8; 12],
}
impl FRigUnit_HierarchyCreatePoseItemArray_Entry {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyCreatePoseItemArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub entries: TArray<FRigUnit_HierarchyCreatePoseItemArray_Entry>,
    pub pose: FRigPose,
}
impl FRigUnit_HierarchyCreatePoseItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_InteractionExecution {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_InteractionExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_InverseExecution {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_InverseExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_IsInteracting {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub b_is_interacting: bool,
    pub b_is_translating: bool,
    pub b_is_rotating: bool,
    pub b_is_scaling: bool,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_IsInteracting {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemBase {
    __padding_end: [u8; 8],
}
impl FRigUnit_ItemBase {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemBaseMutable {
    __padding_end: [u8; 16],
}
impl FRigUnit_ItemBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemExists {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub exists: bool,
    __padding_end: [u8; 39],
}
impl FRigUnit_ItemExists {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemReplace {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub old: FName,
    pub new: FName,
    pub result: FRigElementKey,
}
impl FRigUnit_ItemReplace {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemEquals {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub a: FRigElementKey,
    pub b: FRigElementKey,
    pub result: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_ItemEquals {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemNotEquals {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub a: FRigElementKey,
    pub b: FRigElementKey,
    pub result: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_ItemNotEquals {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemTypeEquals {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub a: FRigElementKey,
    pub b: FRigElementKey,
    pub result: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_ItemTypeEquals {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemTypeNotEquals {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub a: FRigElementKey,
    pub b: FRigElementKey,
    pub result: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_ItemTypeNotEquals {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemToName {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub value: FRigElementKey,
    pub result: FName,
    __padding_end: [u8; 4],
}
impl FRigUnit_ItemToName {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddPhysicsSolver {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub name: FName,
    pub solver: FRigPhysicsSolverID,
    __padding_end: [u8; 4],
}
impl FRigUnit_HierarchyAddPhysicsSolver {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddPhysicsJoint {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub solver: FRigPhysicsSolverID,
}
impl FRigUnit_HierarchyAddPhysicsJoint {}
#[repr(C, align(8))]
pub struct FRigUnit_PrepareForExecution {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_PrepareForExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_PostPrepareForExecution {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_PostPrepareForExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_RigModulesBase {
    __padding_end: [u8; 8],
}
impl FRigUnit_RigModulesBase {}
#[repr(C, align(8))]
pub struct FRigUnit_RigModulesBaseMutable {
    __padding_end: [u8; 16],
}
impl FRigUnit_RigModulesBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_ResolveConnector {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub connector: FRigElementKey,
    pub skip_socket: bool,
    pub result: FRigElementKey,
    pub b_is_connected: bool,
    __padding_end: [u8; 3],
}
impl FRigUnit_ResolveConnector {}
#[repr(C, align(8))]
pub struct FRigUnit_ResolveArrayConnector {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub connector: FRigElementKey,
    pub skip_socket: bool,
    pub result: TArray<FRigElementKey>,
    pub b_is_connected: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_ResolveArrayConnector {}
#[repr(C, align(8))]
pub struct FRigUnit_GetCurrentNameSpace {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub name_space: FString,
}
impl FRigUnit_GetCurrentNameSpace {}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemShortName {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub short_name: FName,
    __padding_end: [u8; 4],
}
impl FRigUnit_GetItemShortName {}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemNameSpace {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub has_name_space: bool,
    pub name_space: FString,
}
impl FRigUnit_GetItemNameSpace {}
#[repr(C, align(8))]
pub struct FRigUnit_IsItemInCurrentNameSpace {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub result: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_IsItemInCurrentNameSpace {}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemsInNameSpace {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_GetItemsInNameSpace {}
#[repr(C, align(8))]
pub struct FRigUnit_GetModuleName {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub module: FString,
}
impl FRigUnit_GetModuleName {}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemModuleName {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub is_part_of_module: bool,
    pub module: FString,
}
impl FRigUnit_GetItemModuleName {}
#[repr(C, align(8))]
pub struct FRigUnit_IsItemInCurrentModule {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub result: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_IsItemInCurrentModule {}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemsInModule {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_GetItemsInModule {}
#[repr(C, align(16))]
pub struct FRigUnit_SequenceExecution {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub execute_context: FControlRigExecuteContext,
    pub a: FControlRigExecuteContext,
    pub b: FControlRigExecuteContext,
    pub c: FControlRigExecuteContext,
    pub d: FControlRigExecuteContext,
}
impl FRigUnit_SequenceExecution {}
#[repr(C, align(16))]
pub struct FRigUnit_AddBoneTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub bone: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_post_multiply: bool,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 42],
}
impl FRigUnit_AddBoneTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_Item {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
}
impl FRigUnit_Item {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_ItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_BoneName {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub bone: FName,
    __padding_end: [u8; 4],
}
impl FRigUnit_BoneName {}
#[repr(C, align(8))]
pub struct FRigUnit_SpaceName {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub space: FName,
    __padding_end: [u8; 4],
}
impl FRigUnit_SpaceName {}
#[repr(C, align(8))]
pub struct FRigUnit_ControlName {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    __padding_end: [u8; 4],
}
impl FRigUnit_ControlName {}
#[repr(C, align(8))]
pub struct FRigDispatch_ComponentBase {
    __padding_end: [u8; 208],
}
impl FRigDispatch_ComponentBase {}
#[repr(C, align(8))]
pub struct FRigDispatch_SpawnComponent {
    __padding_end: [u8; 208],
}
impl FRigDispatch_SpawnComponent {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetComponentContent {
    __padding_end: [u8; 208],
}
impl FRigDispatch_GetComponentContent {}
#[repr(C, align(8))]
pub struct FRigDispatch_SetComponentContent {
    __padding_end: [u8; 208],
}
impl FRigDispatch_SetComponentContent {}
#[repr(C, align(8))]
pub struct FRigUnit_GetAnimationChannelBase {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub channel: FName,
    pub b_initial: bool,
    __padding_end: [u8; 23],
}
impl FRigUnit_GetAnimationChannelBase {}
#[repr(C, align(8))]
pub struct FRigUnit_GetBoolAnimationChannel {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub value: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_GetBoolAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_GetFloatAnimationChannel {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub value: f32,
    __padding_end: [u8; 4],
}
impl FRigUnit_GetFloatAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_GetIntAnimationChannel {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub value: i32,
    __padding_end: [u8; 4],
}
impl FRigUnit_GetIntAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_GetVector2DAnimationChannel {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub value: crate::bindings::core_u_object::FVector2D,
}
impl FRigUnit_GetVector2DAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_GetVectorAnimationChannel {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub value: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_GetVectorAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_GetRotatorAnimationChannel {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub value: crate::bindings::core_u_object::FRotator,
}
impl FRigUnit_GetRotatorAnimationChannel {}
#[repr(C, align(16))]
pub struct FRigUnit_GetTransformAnimationChannel {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub value: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_GetTransformAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SetAnimationChannelBase {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_SetAnimationChannelBase {}
#[repr(C, align(8))]
pub struct FRigUnit_SetBoolAnimationChannel {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub value: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_SetBoolAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SetFloatAnimationChannel {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub value: f32,
    __padding_end: [u8; 4],
}
impl FRigUnit_SetFloatAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SetIntAnimationChannel {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub value: i32,
    __padding_end: [u8; 4],
}
impl FRigUnit_SetIntAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SetVector2DAnimationChannel {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub value: crate::bindings::core_u_object::FVector2D,
}
impl FRigUnit_SetVector2DAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SetVectorAnimationChannel {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub value: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_SetVectorAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SetRotatorAnimationChannel {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub value: crate::bindings::core_u_object::FRotator,
}
impl FRigUnit_SetRotatorAnimationChannel {}
#[repr(C, align(16))]
pub struct FRigUnit_SetTransformAnimationChannel {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub value: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_SetTransformAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_GetAnimationChannelFromItemBase {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub b_initial: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_GetAnimationChannelFromItemBase {}
#[repr(C, align(8))]
pub struct FRigUnit_GetBoolAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub value: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_GetBoolAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_GetFloatAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub value: f32,
    __padding_end: [u8; 4],
}
impl FRigUnit_GetFloatAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_GetIntAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub value: i32,
    __padding_end: [u8; 4],
}
impl FRigUnit_GetIntAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_GetVector2DAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub value: crate::bindings::core_u_object::FVector2D,
}
impl FRigUnit_GetVector2DAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_GetVectorAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub value: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_GetVectorAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_GetRotatorAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub value: crate::bindings::core_u_object::FRotator,
}
impl FRigUnit_GetRotatorAnimationChannelFromItem {}
#[repr(C, align(16))]
pub struct FRigUnit_GetTransformAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub value: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_GetTransformAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetAnimationChannelBaseFromItem {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_SetAnimationChannelBaseFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetBoolAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub value: bool,
    __padding_end: [u8; 7],
}
impl FRigUnit_SetBoolAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetFloatAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub value: f32,
    __padding_end: [u8; 4],
}
impl FRigUnit_SetFloatAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetIntAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub value: i32,
    __padding_end: [u8; 4],
}
impl FRigUnit_SetIntAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetVector2DAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub value: crate::bindings::core_u_object::FVector2D,
}
impl FRigUnit_SetVector2DAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetVectorAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub value: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_SetVectorAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetRotatorAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_40: [u8; 40],
    pub value: crate::bindings::core_u_object::FRotator,
}
impl FRigUnit_SetRotatorAnimationChannelFromItem {}
#[repr(C, align(16))]
pub struct FRigUnit_SetTransformAnimationChannelFromItem {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub value: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_SetTransformAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_CurveExists {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub curve: FName,
    pub exists: bool,
    __padding_end: [u8; 35],
}
impl FRigUnit_CurveExists {}
#[repr(C, align(8))]
pub struct FRigUnit_FindClosestItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub point: crate::bindings::core_u_object::FVector,
    pub item: FRigElementKey,
    __padding_end: [u8; 16],
}
impl FRigUnit_FindClosestItem {}
#[repr(C, align(16))]
pub struct FRigUnit_GetBoneTransform {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub bone: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 48],
}
impl FRigUnit_GetBoneTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetControlInitialTransform {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetControlInitialTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetControlOffset {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetControlOffset {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlBool {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub bool_value: bool,
    __padding_end: [u8; 35],
}
impl FRigUnit_GetControlBool {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlFloat {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub float_value: f32,
    pub minimum: f32,
    pub maximum: f32,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetControlFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlInteger {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub integer_value: i32,
    pub minimum: i32,
    pub maximum: i32,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetControlInteger {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlVector2D {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub vector: crate::bindings::core_u_object::FVector2D,
    pub minimum: crate::bindings::core_u_object::FVector2D,
    pub maximum: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetControlVector2D {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlVector {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub vector: crate::bindings::core_u_object::FVector,
    pub minimum: crate::bindings::core_u_object::FVector,
    pub maximum: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetControlVector {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlRotator {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub minimum: crate::bindings::core_u_object::FRotator,
    pub maximum: crate::bindings::core_u_object::FRotator,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetControlRotator {}
#[repr(C, align(16))]
pub struct FRigUnit_GetControlTransform {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub minimum: crate::bindings::core_u_object::FTransform,
    pub maximum: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetControlTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_GetCurveValue {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub curve: FName,
    pub valid: bool,
    pub value: f32,
    __padding_end: [u8; 36],
}
impl FRigUnit_GetCurveValue {}
#[repr(C, align(16))]
pub struct FRigUnit_GetInitialBoneTransform {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub bone: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetInitialBoneTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetRelativeBoneTransform {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub bone: FName,
    pub space: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 64],
}
impl FRigUnit_GetRelativeBoneTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetRelativeTransformForItem {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub b_child_initial: bool,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub relative_transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 64],
}
impl FRigUnit_GetRelativeTransformForItem {}
#[repr(C, align(16))]
pub struct FRigUnit_GetSpaceTransform {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub space: FName,
    pub space_type: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetSpaceTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetTransform {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_GetTransformArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub items: FRigElementKeyCollection,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    __padding_end: [u8; 16],
}
impl FRigUnit_GetTransformArray {}
#[repr(C, align(8))]
pub struct FRigUnit_GetTransformItemArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    __padding_end: [u8; 16],
}
impl FRigUnit_GetTransformItemArray {}
#[repr(C, align(8))]
pub struct FRigDispatch_MetadataBase {
    __padding_end: [u8; 216],
}
impl FRigDispatch_MetadataBase {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetMetadata {
    __padding_end: [u8; 216],
}
impl FRigDispatch_GetMetadata {}
#[repr(C, align(8))]
pub struct FRigDispatch_SetMetadata {
    __padding_end: [u8; 216],
}
impl FRigDispatch_SetMetadata {}
#[repr(C, align(8))]
pub struct FRigUnit_RemoveMetadata {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub name: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub removed: bool,
    __padding_end: [u8; 34],
}
impl FRigUnit_RemoveMetadata {}
#[repr(C, align(8))]
pub struct FRigUnit_RemoveAllMetadata {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub name_space: ERigMetaDataNameSpace,
    pub removed: bool,
    __padding_end: [u8; 38],
}
impl FRigUnit_RemoveAllMetadata {}
#[repr(C, align(8))]
pub struct FRigUnit_HasMetadata {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub name: FName,
    pub ty: ERigMetadataType,
    pub name_space: ERigMetaDataNameSpace,
    pub found: bool,
    __padding_end: [u8; 33],
}
impl FRigUnit_HasMetadata {}
#[repr(C, align(8))]
pub struct FRigUnit_FindItemsWithMetadata {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub name: FName,
    pub ty: ERigMetadataType,
    pub name_space: ERigMetaDataNameSpace,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_FindItemsWithMetadata {}
#[repr(C, align(8))]
pub struct FRigUnit_GetMetadataTags {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub tags: TArray<FName>,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetMetadataTags {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMetadataTag {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub tag: FName,
    pub name_space: ERigMetaDataNameSpace,
    __padding_end: [u8; 35],
}
impl FRigUnit_SetMetadataTag {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMetadataTagArray {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub tags: TArray<FName>,
    pub name_space: ERigMetaDataNameSpace,
    __padding_end: [u8; 39],
}
impl FRigUnit_SetMetadataTagArray {}
#[repr(C, align(8))]
pub struct FRigUnit_RemoveMetadataTag {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub tag: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub removed: bool,
    __padding_end: [u8; 34],
}
impl FRigUnit_RemoveMetadataTag {}
#[repr(C, align(8))]
pub struct FRigUnit_HasMetadataTag {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub tag: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub found: bool,
    __padding_end: [u8; 34],
}
impl FRigUnit_HasMetadataTag {}
#[repr(C, align(8))]
pub struct FRigUnit_HasMetadataTagArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub tags: TArray<FName>,
    pub name_space: ERigMetaDataNameSpace,
    pub found: bool,
    __padding_end: [u8; 38],
}
impl FRigUnit_HasMetadataTagArray {}
#[repr(C, align(8))]
pub struct FRigUnit_FindItemsWithMetadataTag {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub tag: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_FindItemsWithMetadataTag {}
#[repr(C, align(8))]
pub struct FRigUnit_FindItemsWithMetadataTagArray {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub tags: TArray<FName>,
    pub name_space: ERigMetaDataNameSpace,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_FindItemsWithMetadataTagArray {}
#[repr(C, align(8))]
pub struct FRigUnit_FilterItemsByMetadataTags {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
    pub tags: TArray<FName>,
    pub name_space: ERigMetaDataNameSpace,
    pub inclusive: bool,
    pub result: TArray<FRigElementKey>,
    __padding_end: [u8; 16],
}
impl FRigUnit_FilterItemsByMetadataTags {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetModuleMetadata {
    __padding_end: [u8; 216],
}
impl FRigDispatch_GetModuleMetadata {}
#[repr(C, align(8))]
pub struct FRigDispatch_SetModuleMetadata {
    __padding_end: [u8; 216],
}
impl FRigDispatch_SetModuleMetadata {}
#[repr(C, align(16))]
pub struct FRigUnit_OffsetTransformForItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 43],
}
impl FRigUnit_OffsetTransformForItem {}
#[repr(C, align(16))]
pub struct FRigUnit_ParentSwitchConstraint {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub subject: FRigElementKey,
    pub parent_index: i32,
    pub parents: FRigElementKeyCollection,
    pub initial_global_transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub switched: bool,
    __padding_end: [u8; 175],
}
impl FRigUnit_ParentSwitchConstraint {}
#[repr(C, align(16))]
pub struct FRigUnit_ParentSwitchConstraintArray {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub subject: FRigElementKey,
    pub parent_index: i32,
    pub parents: TArray<FRigElementKey>,
    pub initial_global_transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub switched: bool,
    __padding_end: [u8; 175],
}
impl FRigUnit_ParentSwitchConstraintArray {}
#[repr(C, align(16))]
pub struct FRigUnit_ProjectTransformToNewParent {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub b_child_initial: bool,
    pub old_parent: FRigElementKey,
    pub b_old_parent_initial: bool,
    pub new_parent: FRigElementKey,
    pub b_new_parent_initial: bool,
    pub transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 96],
}
impl FRigUnit_ProjectTransformToNewParent {}
#[repr(C, align(8))]
pub struct FRigUnit_PropagateTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub b_recompute_global: bool,
    pub b_apply_to_children: bool,
    pub b_recursive: bool,
    __padding_end: [u8; 37],
}
impl FRigUnit_PropagateTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_SendEvent {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub event: ERigEvent,
    pub item: FRigElementKey,
    pub offset_in_seconds: f32,
    pub b_enable: bool,
    pub b_only_during_interaction: bool,
    __padding_end: [u8; 6],
}
impl FRigUnit_SendEvent {}
#[repr(C, align(16))]
pub struct FRigUnit_SetBoneInitialTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub bone: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 46],
}
impl FRigUnit_SetBoneInitialTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_SetBoneRotation {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub bone: FName,
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 39],
}
impl FRigUnit_SetBoneRotation {}
#[repr(C, align(16))]
pub struct FRigUnit_SetBoneTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub bone: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 39],
}
impl FRigUnit_SetBoneTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_SetBoneTranslation {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub bone: FName,
    pub translation: crate::bindings::core_u_object::FVector,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 39],
}
impl FRigUnit_SetBoneTranslation {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlColor {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 36],
}
impl FRigUnit_GetControlColor {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlColor {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 36],
}
impl FRigUnit_SetControlColor {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlDrivenList {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub driven: TArray<FRigElementKey>,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetControlDrivenList {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlDrivenList {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub driven: TArray<FRigElementKey>,
    __padding_end: [u8; 32],
}
impl FRigUnit_SetControlDrivenList {}
#[repr(C, align(16))]
pub struct FRigUnit_SetControlOffset {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub offset: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 47],
}
impl FRigUnit_SetControlOffset {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlTranslationOffset {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub offset: crate::bindings::core_u_object::FVector,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 39],
}
impl FRigUnit_SetControlTranslationOffset {}
#[repr(C, align(16))]
pub struct FRigUnit_SetControlRotationOffset {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub offset: crate::bindings::core_u_object::FQuat,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 47],
}
impl FRigUnit_SetControlRotationOffset {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlScaleOffset {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub scale: crate::bindings::core_u_object::FVector,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 39],
}
impl FRigUnit_SetControlScaleOffset {}
#[repr(C, align(16))]
pub struct FRigUnit_GetShapeTransform {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub control: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 32],
}
impl FRigUnit_GetShapeTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_SetShapeTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 32],
}
impl FRigUnit_SetShapeTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlBool {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub bool_value: bool,
    __padding_end: [u8; 35],
}
impl FRigUnit_SetControlBool {}
#[repr(C, align(4))]
pub struct FRigUnit_SetMultiControlBool_Entry {
    pub control: FName,
    pub bool_value: bool,
    __padding_end: [u8; 3],
}
impl FRigUnit_SetMultiControlBool_Entry {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlBool {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub entries: TArray<FRigUnit_SetMultiControlBool_Entry>,
    __padding_end: [u8; 16],
}
impl FRigUnit_SetMultiControlBool {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlFloat {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub weight: f32,
    pub float_value: f32,
    __padding_end: [u8; 36],
}
impl FRigUnit_SetControlFloat {}
#[repr(C, align(4))]
pub struct FRigUnit_SetMultiControlFloat_Entry {
    pub control: FName,
    pub float_value: f32,
}
impl FRigUnit_SetMultiControlFloat_Entry {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlFloat {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub entries: TArray<FRigUnit_SetMultiControlFloat_Entry>,
    pub weight: f32,
    __padding_end: [u8; 20],
}
impl FRigUnit_SetMultiControlFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlInteger {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub weight: i32,
    pub integer_value: i32,
    __padding_end: [u8; 36],
}
impl FRigUnit_SetControlInteger {}
#[repr(C, align(4))]
pub struct FRigUnit_SetMultiControlInteger_Entry {
    pub control: FName,
    pub integer_value: i32,
}
impl FRigUnit_SetMultiControlInteger_Entry {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlInteger {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub entries: TArray<FRigUnit_SetMultiControlInteger_Entry>,
    pub weight: f32,
    __padding_end: [u8; 20],
}
impl FRigUnit_SetMultiControlInteger {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlVector2D {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub weight: f32,
    pub vector: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 32],
}
impl FRigUnit_SetControlVector2D {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlVector2D_Entry {
    pub control: FName,
    pub vector: crate::bindings::core_u_object::FVector2D,
}
impl FRigUnit_SetMultiControlVector2D_Entry {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlVector2D {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub entries: TArray<FRigUnit_SetMultiControlVector2D_Entry>,
    pub weight: f32,
    __padding_end: [u8; 20],
}
impl FRigUnit_SetMultiControlVector2D {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlVector {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub weight: f32,
    pub vector: crate::bindings::core_u_object::FVector,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 39],
}
impl FRigUnit_SetControlVector {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlRotator {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub weight: f32,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 39],
}
impl FRigUnit_SetControlRotator {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlRotator_Entry {
    pub control: FName,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 7],
}
impl FRigUnit_SetMultiControlRotator_Entry {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlRotator {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub entries: TArray<FRigUnit_SetMultiControlRotator_Entry>,
    pub weight: f32,
    __padding_end: [u8; 20],
}
impl FRigUnit_SetMultiControlRotator {}
#[repr(C, align(16))]
pub struct FRigUnit_SetControlTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control: FName,
    pub weight: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 47],
}
impl FRigUnit_SetControlTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlVisibility {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub b_visible: bool,
    __padding_end: [u8; 39],
}
impl FRigUnit_GetControlVisibility {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlVisibility {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub pattern: FString,
    pub b_visible: bool,
    __padding_end: [u8; 23],
}
impl FRigUnit_SetControlVisibility {}
#[repr(C, align(8))]
pub struct FRigUnit_SetCurveValue {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub curve: FName,
    pub value: f32,
    __padding_end: [u8; 32],
}
impl FRigUnit_SetCurveValue {}
#[repr(C, align(16))]
pub struct FRigUnit_SetRelativeBoneTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub bone: FName,
    pub space: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 75],
}
impl FRigUnit_SetRelativeBoneTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_SetRelativeTransformForItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub value: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 75],
}
impl FRigUnit_SetRelativeTransformForItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetRelativeTranslationForItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub value: crate::bindings::core_u_object::FVector,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 67],
}
impl FRigUnit_SetRelativeTranslationForItem {}
#[repr(C, align(16))]
pub struct FRigUnit_SetRelativeRotationForItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub value: crate::bindings::core_u_object::FQuat,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 75],
}
impl FRigUnit_SetRelativeRotationForItem {}
#[repr(C, align(16))]
pub struct FRigUnit_SetSpaceInitialTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub space_name: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 47],
}
impl FRigUnit_SetSpaceInitialTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_SetSpaceTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub space: FName,
    pub weight: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space_type: crate::bindings::rig_vm::ERigVMTransformSpace,
    __padding_end: [u8; 47],
}
impl FRigUnit_SetSpaceTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_SetTransform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub value: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 43],
}
impl FRigUnit_SetTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_SetTranslation {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub value: crate::bindings::core_u_object::FVector,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 35],
}
impl FRigUnit_SetTranslation {}
#[repr(C, align(16))]
pub struct FRigUnit_SetRotation {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub value: crate::bindings::core_u_object::FQuat,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 43],
}
impl FRigUnit_SetRotation {}
#[repr(C, align(8))]
pub struct FRigUnit_SetScale {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub scale: crate::bindings::core_u_object::FVector,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 35],
}
impl FRigUnit_SetScale {}
#[repr(C, align(8))]
pub struct FRigUnit_SetTransformArray {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 19],
}
impl FRigUnit_SetTransformArray {}
#[repr(C, align(8))]
pub struct FRigUnit_SetTransformItemArray {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 19],
}
impl FRigUnit_SetTransformItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_UnsetCurveValue {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub curve: FName,
    __padding_end: [u8; 36],
}
impl FRigUnit_UnsetCurveValue {}
#[repr(C, align(16))]
pub struct FRigUnit_ToWorldSpace_Transform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub world: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_ToWorldSpace_Transform {}
#[repr(C, align(16))]
pub struct FRigUnit_ToRigSpace_Transform {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub global: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_ToRigSpace_Transform {}
#[repr(C, align(8))]
pub struct FRigUnit_ToWorldSpace_Location {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub world: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_ToWorldSpace_Location {}
#[repr(C, align(8))]
pub struct FRigUnit_ToRigSpace_Location {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub global: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_ToRigSpace_Location {}
#[repr(C, align(16))]
pub struct FRigUnit_ToWorldSpace_Rotation {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub world: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_ToWorldSpace_Rotation {}
#[repr(C, align(16))]
pub struct FRigUnit_ToRigSpace_Rotation {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub global: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_ToRigSpace_Rotation {}
#[repr(C, align(4))]
pub struct FRigUnit_BoneHarmonics_BoneTarget {
    pub bone: FName,
    pub ratio: f32,
}
impl FRigUnit_BoneHarmonics_BoneTarget {}
#[repr(C, align(4))]
pub struct FRigUnit_Harmonics_TargetItem {
    pub item: FRigElementKey,
    pub ratio: f32,
}
impl FRigUnit_Harmonics_TargetItem {}
#[repr(C, align(8))]
pub struct FRigUnit_BoneHarmonics {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub bones: TArray<FRigUnit_BoneHarmonics_BoneTarget>,
    pub wave_speed: crate::bindings::core_u_object::FVector,
    pub wave_frequency: crate::bindings::core_u_object::FVector,
    pub wave_amplitude: crate::bindings::core_u_object::FVector,
    pub wave_offset: crate::bindings::core_u_object::FVector,
    pub wave_noise: crate::bindings::core_u_object::FVector,
    pub wave_ease: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub wave_minimum: f32,
    pub wave_maximum: f32,
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 42],
}
impl FRigUnit_BoneHarmonics {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemHarmonics {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub targets: TArray<FRigUnit_Harmonics_TargetItem>,
    pub wave_speed: crate::bindings::core_u_object::FVector,
    pub wave_frequency: crate::bindings::core_u_object::FVector,
    pub wave_amplitude: crate::bindings::core_u_object::FVector,
    pub wave_offset: crate::bindings::core_u_object::FVector,
    pub wave_noise: crate::bindings::core_u_object::FVector,
    pub wave_ease: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub wave_minimum: f32,
    pub wave_maximum: f32,
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    __padding_end: [u8; 43],
}
impl FRigUnit_ItemHarmonics {}
#[repr(C, align(8))]
pub struct FRigUnit_ChainHarmonics_Reach {
    pub b_enabled: bool,
    pub reach_target: crate::bindings::core_u_object::FVector,
    pub reach_axis: crate::bindings::core_u_object::FVector,
    pub reach_minimum: f32,
    pub reach_maximum: f32,
    pub reach_ease: crate::bindings::rig_vm::ERigVMAnimEasingType,
    __padding_end: [u8; 7],
}
impl FRigUnit_ChainHarmonics_Reach {}
#[repr(C, align(8))]
pub struct FRigUnit_ChainHarmonics_Wave {
    pub b_enabled: bool,
    pub wave_frequency: crate::bindings::core_u_object::FVector,
    pub wave_amplitude: crate::bindings::core_u_object::FVector,
    pub wave_offset: crate::bindings::core_u_object::FVector,
    pub wave_noise: crate::bindings::core_u_object::FVector,
    pub wave_minimum: f32,
    pub wave_maximum: f32,
    pub wave_ease: crate::bindings::rig_vm::ERigVMAnimEasingType,
    __padding_end: [u8; 7],
}
impl FRigUnit_ChainHarmonics_Wave {}
#[repr(C, align(8))]
pub struct FRigUnit_ChainHarmonics_Pendulum {
    pub b_enabled: bool,
    pub pendulum_stiffness: f32,
    pub pendulum_gravity: crate::bindings::core_u_object::FVector,
    pub pendulum_blend: f32,
    pub pendulum_drag: f32,
    pub pendulum_minimum: f32,
    pub pendulum_maximum: f32,
    pub pendulum_ease: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub unwind_axis: crate::bindings::core_u_object::FVector,
    pub unwind_minimum: f32,
    pub unwind_maximum: f32,
}
impl FRigUnit_ChainHarmonics_Pendulum {}
#[repr(C, align(16))]
pub struct FRigUnit_ChainHarmonics {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub chain_root: FName,
    pub speed: crate::bindings::core_u_object::FVector,
    pub reach: FRigUnit_ChainHarmonics_Reach,
    pub wave: FRigUnit_ChainHarmonics_Wave,
    pub wave_curve: crate::bindings::engine::FRuntimeFloatCurve,
    pub pendulum: FRigUnit_ChainHarmonics_Pendulum,
    pub b_draw_debug: bool,
    pub draw_world_offset: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 160],
}
impl FRigUnit_ChainHarmonics {}
#[repr(C, align(16))]
pub struct FRigUnit_ChainHarmonicsPerItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub chain_root: FRigElementKey,
    pub speed: crate::bindings::core_u_object::FVector,
    pub reach: FRigUnit_ChainHarmonics_Reach,
    pub wave: FRigUnit_ChainHarmonics_Wave,
    pub wave_curve: crate::bindings::engine::FRuntimeFloatCurve,
    pub pendulum: FRigUnit_ChainHarmonics_Pendulum,
    pub b_draw_debug: bool,
    pub draw_world_offset: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 160],
}
impl FRigUnit_ChainHarmonicsPerItem {}
#[repr(C, align(8))]
pub struct FRigUnit_AimBone_Target {
    pub weight: f32,
    pub axis: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub kind: EControlRigVectorKind,
    pub space: FName,
}
impl FRigUnit_AimBone_Target {}
#[repr(C, align(8))]
pub struct FRigUnit_AimItem_Target {
    pub weight: f32,
    pub axis: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub kind: EControlRigVectorKind,
    pub space: FRigElementKey,
    __padding_end: [u8; 4],
}
impl FRigUnit_AimItem_Target {}
#[repr(C, align(16))]
pub struct FRigUnit_AimBone_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_AimBone_DebugSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_AimBoneMath {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub input_transform: crate::bindings::core_u_object::FTransform,
    pub primary: FRigUnit_AimItem_Target,
    pub secondary: FRigUnit_AimItem_Target,
    pub weight: f32,
    pub result: crate::bindings::core_u_object::FTransform,
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    __padding_end: [u8; 80],
}
impl FRigUnit_AimBoneMath {}
#[repr(C, align(16))]
pub struct FRigUnit_AimBone {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub bone: FName,
    pub primary: FRigUnit_AimBone_Target,
    pub secondary: FRigUnit_AimBone_Target,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    __padding_end: [u8; 112],
}
impl FRigUnit_AimBone {}
#[repr(C, align(16))]
pub struct FRigUnit_AimItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub primary: FRigUnit_AimItem_Target,
    pub secondary: FRigUnit_AimItem_Target,
    pub weight: f32,
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    __padding_end: [u8; 112],
}
impl FRigUnit_AimItem {}
#[repr(C, align(8))]
pub struct FRigUnit_AimConstraint_WorldUp {
    pub target: crate::bindings::core_u_object::FVector,
    pub kind: EControlRigVectorKind,
    pub space: FRigElementKey,
    __padding_end: [u8; 4],
}
impl FRigUnit_AimConstraint_WorldUp {}
#[repr(C, align(16))]
pub struct FRigUnit_AimConstraint_AdvancedSettings {
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    pub rotation_order_for_filter: crate::bindings::animation_core::EEulerRotationOrder,
    __padding_end: [u8; 15],
}
impl FRigUnit_AimConstraint_AdvancedSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_AimConstraintLocalSpaceOffset {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub aim_axis: crate::bindings::core_u_object::FVector,
    pub up_axis: crate::bindings::core_u_object::FVector,
    pub world_up: FRigUnit_AimConstraint_WorldUp,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_AimConstraint_AdvancedSettings,
    pub weight: f32,
    __padding_end: [u8; 92],
}
impl FRigUnit_AimConstraintLocalSpaceOffset {}
#[repr(C, align(4))]
pub struct FConstraintParent {
    pub item: FRigElementKey,
    pub weight: f32,
}
impl FConstraintParent {}
#[repr(C, align(4))]
pub struct FRigUnit_CCDIK_RotationLimit {
    pub bone: FName,
    pub limit: f32,
}
impl FRigUnit_CCDIK_RotationLimit {}
#[repr(C, align(4))]
pub struct FRigUnit_CCDIK_RotationLimitPerItem {
    pub item: FRigElementKey,
    pub limit: f32,
}
impl FRigUnit_CCDIK_RotationLimitPerItem {}
#[repr(C, align(16))]
pub struct FRigUnit_CCDIK {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub start_bone: FName,
    pub effector_bone: FName,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub max_iterations: i32,
    pub b_start_from_tail: bool,
    pub base_rotation_limit: f32,
    pub rotation_limits: TArray<FRigUnit_CCDIK_RotationLimit>,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 103],
}
impl FRigUnit_CCDIK {}
#[repr(C, align(16))]
pub struct FRigUnit_CCDIKPerItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub max_iterations: i32,
    pub b_start_from_tail: bool,
    pub base_rotation_limit: f32,
    pub rotation_limits: TArray<FRigUnit_CCDIK_RotationLimitPerItem>,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 103],
}
impl FRigUnit_CCDIKPerItem {}
#[repr(C, align(16))]
pub struct FRigUnit_CCDIKItemArray {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub max_iterations: i32,
    pub b_start_from_tail: bool,
    pub base_rotation_limit: f32,
    pub rotation_limits: TArray<FRigUnit_CCDIK_RotationLimitPerItem>,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 103],
}
impl FRigUnit_CCDIKItemArray {}
#[repr(C, align(4))]
pub struct FRigUnit_ChainInfo_SegmentInfo {
    __padding_end: [u8; 60],
}
impl FRigUnit_ChainInfo_SegmentInfo {}
#[repr(C, align(16))]
pub struct FRigUnit_ChainInfo {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
    pub param: f32,
    pub b_calculate_stretch: bool,
    pub b_initial: bool,
    pub b_debug: bool,
    pub debug_scale: f32,
    pub interpolated_transform: crate::bindings::core_u_object::FTransform,
    pub chain_length: f32,
    pub param_length: f32,
    pub chain_stretch_factor: f32,
    pub segment_info: FRigUnit_ChainInfo_SegmentInfo,
    __padding_end: [u8; 24],
}
impl FRigUnit_ChainInfo {}
#[repr(C, align(16))]
pub struct FRigUnit_DistributeRotation_Rotation {
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub ratio: f32,
    __padding_end: [u8; 12],
}
impl FRigUnit_DistributeRotation_Rotation {}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotation {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub start_bone: FName,
    pub end_bone: FName,
    pub rotations: TArray<FRigUnit_DistributeRotation_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 87],
}
impl FRigUnit_DistributeRotation {}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotationForCollection {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub rotations: TArray<FRigUnit_DistributeRotation_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    __padding_end: [u8; 80],
}
impl FRigUnit_DistributeRotationForCollection {}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotationForItemArray {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub rotations: TArray<FRigUnit_DistributeRotation_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    __padding_end: [u8; 80],
}
impl FRigUnit_DistributeRotationForItemArray {}
#[repr(C, align(16))]
pub struct FRigUnit_FABRIK {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub start_bone: FName,
    pub effector_bone: FName,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    #[doc(hidden)]
    __padding_224: [u8; 64],
    pub b_set_effector_transform: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_FABRIK {}
#[repr(C, align(16))]
pub struct FRigUnit_FABRIKPerItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    #[doc(hidden)]
    __padding_208: [u8; 64],
    pub b_set_effector_transform: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_FABRIKPerItem {}
#[repr(C, align(16))]
pub struct FRigUnit_FABRIKItemArray {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    #[doc(hidden)]
    __padding_208: [u8; 64],
    pub b_set_effector_transform: bool,
    __padding_end: [u8; 15],
}
impl FRigUnit_FABRIKItemArray {}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurve_Rotation {
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub ratio: f32,
    __padding_end: [u8; 12],
}
impl FRigUnit_FitChainToCurve_Rotation {}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurve_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub curve_color: crate::bindings::core_u_object::FLinearColor,
    pub segments_color: crate::bindings::core_u_object::FLinearColor,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_FitChainToCurve_DebugSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurve {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub start_bone: FName,
    pub end_bone: FName,
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub alignment: EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub pole_vector_position: crate::bindings::core_u_object::FVector,
    pub rotations: TArray<FRigUnit_FitChainToCurve_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_FitChainToCurve_DebugSettings,
    __padding_end: [u8; 160],
}
impl FRigUnit_FitChainToCurve {}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurvePerItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub alignment: EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub pole_vector_position: crate::bindings::core_u_object::FVector,
    pub rotations: TArray<FRigUnit_FitChainToCurve_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_FitChainToCurve_DebugSettings,
    __padding_end: [u8; 160],
}
impl FRigUnit_FitChainToCurvePerItem {}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurveItemArray {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub alignment: EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub pole_vector_position: crate::bindings::core_u_object::FVector,
    pub rotations: TArray<FRigUnit_FitChainToCurve_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_FitChainToCurve_DebugSettings,
    __padding_end: [u8; 160],
}
impl FRigUnit_FitChainToCurveItemArray {}
#[repr(C, align(16))]
pub struct FRigUnit_ModifyBoneTransforms_PerBone {
    pub bone: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_ModifyBoneTransforms_PerBone {}
#[repr(C, align(8))]
pub struct FRigUnit_ModifyBoneTransforms {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub bone_to_modify: TArray<FRigUnit_ModifyBoneTransforms_PerBone>,
    pub weight: f32,
    pub weight_minimum: f32,
    pub weight_maximum: f32,
    pub mode: EControlRigModifyBoneMode,
    __padding_end: [u8; 19],
}
impl FRigUnit_ModifyBoneTransforms {}
#[repr(C, align(16))]
pub struct FRigUnit_ModifyTransforms_PerItem {
    pub item: FRigElementKey,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_ModifyTransforms_PerItem {}
#[repr(C, align(8))]
pub struct FRigUnit_ModifyTransforms {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item_to_modify: TArray<FRigUnit_ModifyTransforms_PerItem>,
    pub weight: f32,
    pub weight_minimum: f32,
    pub weight_maximum: f32,
    pub mode: EControlRigModifyBoneMode,
    __padding_end: [u8; 19],
}
impl FRigUnit_ModifyTransforms {}
#[repr(C, align(8))]
pub struct FRigUnit_MultiFABRIK_EndEffector {
    pub bone: FName,
    pub location: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_MultiFABRIK_EndEffector {}
#[repr(C, align(8))]
pub struct FRigUnit_MultiFABRIK {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub root_bone: FName,
    pub effectors: TArray<FRigUnit_MultiFABRIK_EndEffector>,
    pub precision: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    __padding_end: [u8; 116],
}
impl FRigUnit_MultiFABRIK {}
#[repr(C, align(8))]
pub struct FRigUnit_SlideChain {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub start_bone: FName,
    pub end_bone: FName,
    pub slide_amount: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 75],
}
impl FRigUnit_SlideChain {}
#[repr(C, align(8))]
pub struct FRigUnit_SlideChainPerItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub slide_amount: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 75],
}
impl FRigUnit_SlideChainPerItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SlideChainItemArray {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub slide_amount: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 75],
}
impl FRigUnit_SlideChainItemArray {}
#[repr(C, align(4))]
pub struct FRegionScaleFactors {
    pub positive_width: f32,
    pub negative_width: f32,
    pub positive_height: f32,
    pub negative_height: f32,
}
impl FRegionScaleFactors {}
#[repr(C, align(4))]
pub struct FSphericalPoseReaderDebugSettings {
    pub b_draw_debug: bool,
    pub b_draw2_d: bool,
    pub b_draw_local_axes: bool,
    pub debug_scale: f32,
    pub debug_segments: i32,
    pub debug_thickness: f32,
}
impl FSphericalPoseReaderDebugSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_SphericalPoseReader {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub output_param: f32,
    pub driver_item: FRigElementKey,
    pub driver_axis: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FVector,
    pub active_region_size: f32,
    pub active_region_scale_factors: FRegionScaleFactors,
    pub falloff_size: f32,
    pub falloff_region_scale_factors: FRegionScaleFactors,
    pub flip_width_scaling: bool,
    pub flip_height_scaling: bool,
    pub optional_parent_item: FRigElementKey,
    pub debug: FSphericalPoseReaderDebugSettings,
    __padding_end: [u8; 284],
}
impl FRigUnit_SphericalPoseReader {}
#[repr(C, align(16))]
pub struct FRigUnit_SpringIK_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_SpringIK_DebugSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_SpringIK {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub start_bone: FName,
    pub end_bone: FName,
    pub hierarchy_strength: f32,
    pub effector_strength: f32,
    pub effector_ratio: f32,
    pub root_strength: f32,
    pub root_ratio: f32,
    pub damping: f32,
    pub pole_vector: crate::bindings::core_u_object::FVector,
    pub b_flip_pole_plane: bool,
    pub pole_vector_kind: EControlRigVectorKind,
    pub pole_vector_space: FName,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub b_live_simulation: bool,
    pub iterations: i32,
    pub b_limit_local_position: bool,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_SpringIK_DebugSettings,
    __padding_end: [u8; 192],
}
impl FRigUnit_SpringIK {}
#[repr(C, align(16))]
pub struct FConstraintTarget {
    #[doc(hidden)]
    __padding_101: [u8; 101],
    pub filter: crate::bindings::animation_core::FTransformFilter,
    __padding_end: [u8; 2],
}
impl FConstraintTarget {}
#[repr(C, align(16))]
pub struct FRigUnit_TransformConstraint {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub bone: FName,
    pub base_transform_space: ETransformSpaceMode,
    pub base_transform: crate::bindings::core_u_object::FTransform,
    pub base_bone: FName,
    pub targets: TArray<FConstraintTarget>,
    pub b_use_initial_transforms: bool,
    __padding_end: [u8; 111],
}
impl FRigUnit_TransformConstraint {}
#[repr(C, align(16))]
pub struct FRigUnit_TransformConstraintPerItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub base_transform_space: ETransformSpaceMode,
    pub base_transform: crate::bindings::core_u_object::FTransform,
    pub base_item: FRigElementKey,
    pub targets: TArray<FConstraintTarget>,
    pub b_use_initial_transforms: bool,
    __padding_end: [u8; 111],
}
impl FRigUnit_TransformConstraintPerItem {}
#[repr(C, align(1))]
pub struct FRigUnit_ParentConstraint_AdvancedSettings {
    pub interpolation_type: EConstraintInterpType,
    pub rotation_order_for_filter: crate::bindings::animation_core::EEulerRotationOrder,
}
impl FRigUnit_ParentConstraint_AdvancedSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_ParentConstraint {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FTransformFilter,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_ParentConstraint_AdvancedSettings,
    pub weight: f32,
    __padding_end: [u8; 48],
}
impl FRigUnit_ParentConstraint {}
#[repr(C, align(1))]
pub struct FRigUnit_ParentConstraintMath_AdvancedSettings {
    pub interpolation_type: EConstraintInterpType,
}
impl FRigUnit_ParentConstraintMath_AdvancedSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_ParentConstraintMath {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub input: crate::bindings::core_u_object::FTransform,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_ParentConstraintMath_AdvancedSettings,
    pub output: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 16],
}
impl FRigUnit_ParentConstraintMath {}
#[repr(C, align(8))]
pub struct FRigUnit_PositionConstraint {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
    __padding_end: [u8; 4],
}
impl FRigUnit_PositionConstraint {}
#[repr(C, align(8))]
pub struct FRigUnit_PositionConstraintLocalSpaceOffset {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
    __padding_end: [u8; 52],
}
impl FRigUnit_PositionConstraintLocalSpaceOffset {}
#[repr(C, align(1))]
pub struct FRigUnit_RotationConstraint_AdvancedSettings {
    pub interpolation_type: EConstraintInterpType,
    pub rotation_order_for_filter: crate::bindings::animation_core::EEulerRotationOrder,
}
impl FRigUnit_RotationConstraint_AdvancedSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_RotationConstraint {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_RotationConstraint_AdvancedSettings,
    pub weight: f32,
}
impl FRigUnit_RotationConstraint {}
#[repr(C, align(8))]
pub struct FRigUnit_RotationConstraintLocalSpaceOffset {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_RotationConstraint_AdvancedSettings,
    pub weight: f32,
    __padding_end: [u8; 48],
}
impl FRigUnit_RotationConstraintLocalSpaceOffset {}
#[repr(C, align(8))]
pub struct FRigUnit_ScaleConstraint {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
    __padding_end: [u8; 4],
}
impl FRigUnit_ScaleConstraint {}
#[repr(C, align(8))]
pub struct FRigUnit_ScaleConstraintLocalSpaceOffset {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
    __padding_end: [u8; 52],
}
impl FRigUnit_ScaleConstraintLocalSpaceOffset {}
#[repr(C, align(8))]
pub struct FRigUnit_TwistBones {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub start_bone: FName,
    pub end_bone: FName,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub pole_axis: crate::bindings::core_u_object::FVector,
    pub twist_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 55],
}
impl FRigUnit_TwistBones {}
#[repr(C, align(8))]
pub struct FRigUnit_TwistBonesPerItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub pole_axis: crate::bindings::core_u_object::FVector,
    pub twist_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    __padding_end: [u8; 55],
}
impl FRigUnit_TwistBonesPerItem {}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimple_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_TwoBoneIKSimple_DebugSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimple {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub bone_a: FName,
    pub bone_b: FName,
    pub effector_bone: FName,
    pub effector: crate::bindings::core_u_object::FTransform,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis_weight: f32,
    pub pole_vector: crate::bindings::core_u_object::FVector,
    pub pole_vector_kind: EControlRigVectorKind,
    pub pole_vector_space: FName,
    pub b_enable_stretch: bool,
    pub stretch_start_ratio: f32,
    pub stretch_maximum_ratio: f32,
    pub weight: f32,
    pub bone_a_length: f32,
    pub bone_b_length: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_TwoBoneIKSimple_DebugSettings,
    __padding_end: [u8; 128],
}
impl FRigUnit_TwoBoneIKSimple {}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimplePerItem {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub item_a: FRigElementKey,
    pub item_b: FRigElementKey,
    pub effector_item: FRigElementKey,
    pub effector: crate::bindings::core_u_object::FTransform,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis_weight: f32,
    pub pole_vector: crate::bindings::core_u_object::FVector,
    pub pole_vector_kind: EControlRigVectorKind,
    pub pole_vector_space: FRigElementKey,
    pub b_enable_stretch: bool,
    pub stretch_start_ratio: f32,
    pub stretch_maximum_ratio: f32,
    pub weight: f32,
    pub item_a_length: f32,
    pub item_b_length: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_TwoBoneIKSimple_DebugSettings,
    __padding_end: [u8; 128],
}
impl FRigUnit_TwoBoneIKSimplePerItem {}
#[repr(C, align(8))]
pub struct FRigUnit_TwoBoneIKSimpleVectors {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub root: crate::bindings::core_u_object::FVector,
    pub pole_vector: crate::bindings::core_u_object::FVector,
    pub effector: crate::bindings::core_u_object::FVector,
    pub b_enable_stretch: bool,
    pub stretch_start_ratio: f32,
    pub stretch_maximum_ratio: f32,
    pub bone_a_length: f32,
    pub bone_b_length: f32,
    pub elbow: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_TwoBoneIKSimpleVectors {}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimpleTransforms {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub root: crate::bindings::core_u_object::FTransform,
    pub pole_vector: crate::bindings::core_u_object::FVector,
    pub effector: crate::bindings::core_u_object::FTransform,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis_weight: f32,
    pub b_enable_stretch: bool,
    pub stretch_start_ratio: f32,
    pub stretch_maximum_ratio: f32,
    pub bone_a_length: f32,
    pub bone_b_length: f32,
    pub elbow: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_TwoBoneIKSimpleTransforms {}
#[repr(C, align(8))]
pub struct FRigUnit_GetCandidates {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub connector: FRigElementKey,
    pub candidates: TArray<FRigElementKey>,
}
impl FRigUnit_GetCandidates {}
#[repr(C, align(8))]
pub struct FRigUnit_DiscardMatches {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub excluded: TArray<FRigElementKey>,
    pub message: FString,
}
impl FRigUnit_DiscardMatches {}
#[repr(C, align(8))]
pub struct FRigUnit_SetDefaultMatch {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub default: FRigElementKey,
}
impl FRigUnit_SetDefaultMatch {}
#[repr(C, align(16))]
pub struct FRigUnit_ConnectorExecution {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub execute_context: FControlRigExecuteContext,
}
impl FRigUnit_ConnectorExecution {}
#[repr(C, align(16))]
pub struct FRigUnit_PointSimulation_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub collision_scale: f32,
    pub b_draw_points_as_spheres: bool,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_PointSimulation_DebugSettings {}
#[repr(C, align(4))]
pub struct FRigUnit_PointSimulation_BoneTarget {
    pub bone: FName,
    pub translation_point: i32,
    pub primary_aim_point: i32,
    pub secondary_aim_point: i32,
}
impl FRigUnit_PointSimulation_BoneTarget {}
#[repr(C, align(16))]
pub struct FRigUnit_PointSimulation {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub points: TArray<crate::bindings::rig_vm::FRigVMSimPoint>,
    pub links: TArray<FCRSimLinearSpring>,
    pub forces: TArray<FCRSimPointForce>,
    pub collision_volumes: TArray<FCRSimSoftCollision>,
    pub simulated_steps_per_second: f32,
    pub integrator_type: crate::bindings::rig_vm::ERigVMSimPointIntegrateType,
    pub verlet_blend: f32,
    pub bone_targets: TArray<FRigUnit_PointSimulation_BoneTarget>,
    pub b_limit_local_position: bool,
    pub b_propagate_to_children: bool,
    pub primary_aim_axis: crate::bindings::core_u_object::FVector,
    pub secondary_aim_axis: crate::bindings::core_u_object::FVector,
    pub debug_settings: FRigUnit_PointSimulation_DebugSettings,
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    __padding_end: [u8; 144],
}
impl FRigUnit_PointSimulation {}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterp {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub current: f32,
    pub target: f32,
    pub stiffness: f32,
    pub critical_damping: f32,
    pub mass: f32,
    pub result: f32,
    __padding_end: [u8; 16],
}
impl FRigUnit_SpringInterp {}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterpVector {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub current: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub stiffness: f32,
    pub critical_damping: f32,
    pub mass: f32,
    pub result: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 56],
}
impl FRigUnit_SpringInterpVector {}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterpV2 {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub target: f32,
    pub strength: f32,
    pub critical_damping: f32,
    pub force: f32,
    pub b_use_current_input: bool,
    pub current: f32,
    pub target_velocity_amount: f32,
    pub b_initialize_from_target: bool,
    pub result: f32,
    pub velocity: f32,
    __padding_end: [u8; 16],
}
impl FRigUnit_SpringInterpV2 {}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterpVectorV2 {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub target: crate::bindings::core_u_object::FVector,
    pub strength: f32,
    pub critical_damping: f32,
    pub force: crate::bindings::core_u_object::FVector,
    pub b_use_current_input: bool,
    pub current: crate::bindings::core_u_object::FVector,
    pub target_velocity_amount: f32,
    pub b_initialize_from_target: bool,
    pub result: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 80],
}
impl FRigUnit_SpringInterpVectorV2 {}
#[repr(C, align(16))]
pub struct FRigUnit_SpringInterpQuaternionV2 {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub target: crate::bindings::core_u_object::FQuat,
    pub strength: f32,
    pub critical_damping: f32,
    pub torque: crate::bindings::core_u_object::FVector,
    pub b_use_current_input: bool,
    pub current: crate::bindings::core_u_object::FQuat,
    pub target_velocity_amount: f32,
    pub b_initialize_from_target: bool,
    pub result: crate::bindings::core_u_object::FQuat,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 104],
}
impl FRigUnit_SpringInterpQuaternionV2 {}
#[repr(C, align(8))]
pub struct UAnimNodeControlRigLibrary {
    __padding_end: [u8; 48],
}
impl UAnimNodeControlRigLibrary {}
#[repr(C, align(16))]
pub struct UTransformableControlHandle {
    #[doc(hidden)]
    __padding_112: [u8; 112],
    pub control_rig: TSoftObjectPtr<UControlRig>,
    pub control_name: FName,
    __padding_end: [u8; 180],
}
impl UTransformableControlHandle {}
#[repr(C, align(16))]
pub struct UControlRig {
    __padding_end: [u8; 5040],
}
impl UControlRig {}
#[repr(C, align(16))]
pub struct UControlRigAnimInstance {
    __padding_end: [u8; 1136],
}
impl UControlRigAnimInstance {}
#[repr(C, align(8))]
pub struct UControlRigShapeLibraryLink {
    #[doc(hidden)]
    __padding_264: [u8; 264],
    pub shape_library: TSoftObjectPtr<UControlRigShapeLibrary>,
    __padding_end: [u8; 24],
}
impl UControlRigShapeLibraryLink {}
#[repr(C, align(8))]
pub struct UControlRigBlueprintGeneratedClass {
    __padding_end: [u8; 1960],
}
impl UControlRigBlueprintGeneratedClass {}
#[repr(C, align(16))]
pub struct UControlRigComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub control_rig_class: TSubclassOf<UControlRig>,
    __padding_end: [u8; 552],
}
impl UControlRigComponent {}
#[repr(C, align(8))]
pub struct AControlRigControlActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub actor_to_track: UPtr<crate::bindings::engine::AActor>,
    pub control_rig_class: TSubclassOf<UControlRig>,
    pub b_refresh_on_tick: bool,
    pub b_is_selectable: bool,
    pub material_override: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub color_parameter: FString,
    pub b_cast_shadows: bool,
    __padding_end: [u8; 143],
}
impl AControlRigControlActor {}
#[repr(C, align(16))]
pub struct AControlRigShapeActor {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    #[doc(hidden)]
    __padding_1360: [u8; 200],
    pub flags_1360: u8,
    __padding_end: [u8; 31],
}
impl AControlRigShapeActor {}
#[repr(C, align(16))]
pub struct UControlRigShapeLibrary {
    __padding_end: [u8; 368],
}
impl UControlRigShapeLibrary {}
#[repr(C, align(8))]
pub struct UControlRigOverrideAsset {
    __padding_end: [u8; 424],
}
impl UControlRigOverrideAsset {}
#[repr(C, align(8))]
pub struct UControlRigReplay {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub description: FText,
    pub control_rig_object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub preview_skeletal_mesh_object_path: crate::bindings::core_u_object::FSoftObjectPath,
    #[doc(hidden)]
    __padding_896: [u8; 752],
    pub tolerance: f64,
    pub b_validate_hierarchy_topology: bool,
    pub b_validate_pose: bool,
    pub b_validate_metadata: bool,
    pub b_validate_variables: bool,
    pub frames_to_skip: TArray<i32>,
    pub enable_test: bool,
    __padding_end: [u8; 95],
}
impl UControlRigReplay {}
#[repr(C, align(8))]
pub struct UControlRigTestData {
    #[doc(hidden)]
    __padding_1024: [u8; 1024],
    pub initial: FControlRigTestDataFrame,
    pub input_frames: TArray<FControlRigTestDataFrame>,
    pub output_frames: TArray<FControlRigTestDataFrame>,
    pub event_queue: TArray<FName>,
    __padding_end: [u8; 8],
}
impl UControlRigTestData {}
#[repr(C, align(8))]
pub struct UControlRigValidator {
    __padding_end: [u8; 128],
}
impl UControlRigValidator {}
#[repr(C, align(8))]
pub struct UControlRigValidationPass {
    __padding_end: [u8; 48],
}
impl UControlRigValidationPass {}
#[repr(C, align(16))]
pub struct UModularRig {
    __padding_end: [u8; 5504],
}
impl UModularRig {}
#[repr(C, align(8))]
pub struct UModularRigController {
    __padding_end: [u8; 96],
}
impl UModularRigController {}
#[repr(C, align(8))]
pub struct UModularRigRuleManager {
    __padding_end: [u8; 56],
}
impl UModularRigRuleManager {}
#[repr(C, align(16))]
pub struct UAdditiveControlRig {
    __padding_end: [u8; 5056],
}
impl UAdditiveControlRig {}
#[repr(C, align(16))]
pub struct UFKControlRig {
    __padding_end: [u8; 5104],
}
impl UFKControlRig {}
#[repr(C, align(16))]
pub struct URigHierarchy {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub modified_event_dynamic: FRigHierarchy_ModifiedEventDynamic,
    __padding_end: [u8; 1656],
}
impl URigHierarchy {}
pub struct URigHierarchyProvider {}
pub struct IRigHierarchyProvider {}
#[repr(C, align(16))]
pub struct URigHierarchyController {
    __padding_end: [u8; 144],
}
impl URigHierarchyController {}
#[repr(C, align(16))]
pub struct UControlRigLayerInstance {
    __padding_end: [u8; 1136],
}
impl UControlRigLayerInstance {}
#[repr(C, align(8))]
pub struct UMovieSceneControlRigParameterSection {
    __padding_end: [u8; 1256],
}
impl UMovieSceneControlRigParameterSection {}
#[repr(C, align(8))]
pub struct UMovieSceneControlRigParameterTrack {
    __padding_end: [u8; 808],
}
impl UMovieSceneControlRigParameterTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneControlRigParameterEvaluatorSystem {
    __padding_end: [u8; 616],
}
impl UMovieSceneControlRigParameterEvaluatorSystem {}
#[repr(C, align(8))]
pub struct UControlRigSettings {
    __padding_end: [u8; 192],
}
impl UControlRigSettings {}
#[repr(C, align(8))]
pub struct UControlRigEditorSettings {
    __padding_end: [u8; 272],
}
impl UControlRigEditorSettings {}
#[repr(C, align(8))]
pub struct UControlRigPoseAsset {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub pose: FControlRigControlPose,
    __padding_end: [u8; 8],
}
impl UControlRigPoseAsset {}
#[repr(C, align(8))]
pub struct UControlRigPoseMirrorSettings {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub axis_to_flip: crate::bindings::core_u_object::EAxis,
    pub mirror_match_tolerance: f64,
}
impl UControlRigPoseMirrorSettings {}
#[repr(C, align(8))]
pub struct UControlRigPoseProjectSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub root_save_dirs: TArray<crate::bindings::core_u_object::FDirectoryPath>,
}
impl UControlRigPoseProjectSettings {}
#[repr(C, align(8))]
pub struct UControlRigWorkflowOptions {
    #[doc(hidden)]
    __padding_192: [u8; 192],
    pub hierarchy: UPtr<URigHierarchy>,
    pub selection: TArray<FRigElementKey>,
}
impl UControlRigWorkflowOptions {}
#[repr(C, align(8))]
pub struct UControlRigTransformWorkflowOptions {
    #[doc(hidden)]
    __padding_216: [u8; 216],
    pub transform_type: ERigTransformType,
    __padding_end: [u8; 7],
}
impl UControlRigTransformWorkflowOptions {}
#[repr(C, align(8))]
pub struct UControlRigNumericalValidationPass {
    __padding_end: [u8; 208],
}
impl UControlRigNumericalValidationPass {}
#[repr(C, align(1))]
pub struct FControlRig_OnControlSelected_BP {
    _opague: [u8; 1],
}
#[repr(C, align(8))]
pub struct FControlRigComponent_OnPreInitializeDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FControlRigComponent_OnPostInitializeDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FControlRigComponent_OnPreConstructionDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FControlRigComponent_OnPostConstructionDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FControlRigComponent_OnPreForwardsSolveDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FControlRigComponent_OnPostForwardsSolveDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FRigHierarchy_ModifiedEventDynamic {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ERigElementType(pub u8);
impl ERigElementType {
    pub const NONE: ERigElementType = ERigElementType(0);
    pub const BONE: ERigElementType = ERigElementType(1);
    pub const NULL: ERigElementType = ERigElementType(2);
    pub const SPACE: ERigElementType = ERigElementType(2);
    pub const CONTROL: ERigElementType = ERigElementType(4);
    pub const CURVE: ERigElementType = ERigElementType(8);
    pub const PHYSICS: ERigElementType = ERigElementType(16);
    pub const REFERENCE: ERigElementType = ERigElementType(32);
    pub const CONNECTOR: ERigElementType = ERigElementType(64);
    pub const SOCKET: ERigElementType = ERigElementType(128);
    pub const ALL: ERigElementType = ERigElementType(239);
    pub const FIRST: ERigElementType = ERigElementType(1);
    pub const LAST: ERigElementType = ERigElementType(128);
    pub const TO_RESET_AFTER_CONSTRUCTION_EVENT: ERigElementType = ERigElementType(141);
    pub const TRANSFORM_TYPES: ERigElementType = ERigElementType(167);
}
#[repr(transparent)]
pub struct ERigControlAnimationType(pub u8);
impl ERigControlAnimationType {
    pub const ANIMATION_CONTROL: ERigControlAnimationType = ERigControlAnimationType(0);
    pub const ANIMATION_CHANNEL: ERigControlAnimationType = ERigControlAnimationType(1);
    pub const PROXY_CONTROL: ERigControlAnimationType = ERigControlAnimationType(2);
    pub const VISUAL_CUE: ERigControlAnimationType = ERigControlAnimationType(3);
}
#[repr(transparent)]
pub struct ERigControlType(pub u8);
impl ERigControlType {
    pub const BOOL: ERigControlType = ERigControlType(0);
    pub const FLOAT: ERigControlType = ERigControlType(1);
    pub const INTEGER: ERigControlType = ERigControlType(2);
    pub const VECTOR2_D: ERigControlType = ERigControlType(3);
    pub const POSITION: ERigControlType = ERigControlType(4);
    pub const SCALE: ERigControlType = ERigControlType(5);
    pub const ROTATOR: ERigControlType = ERigControlType(6);
    pub const TRANSFORM: ERigControlType = ERigControlType(7);
    pub const TRANSFORM_NO_SCALE: ERigControlType = ERigControlType(8);
    pub const EULER_TRANSFORM: ERigControlType = ERigControlType(9);
    pub const SCALE_FLOAT: ERigControlType = ERigControlType(10);
}
#[repr(transparent)]
pub struct ERigControlAxis(pub u8);
impl ERigControlAxis {
    pub const X: ERigControlAxis = ERigControlAxis(0);
    pub const Y: ERigControlAxis = ERigControlAxis(1);
    pub const Z: ERigControlAxis = ERigControlAxis(2);
}
#[repr(transparent)]
pub struct ERigControlVisibility(pub u8);
impl ERigControlVisibility {
    pub const USER_DEFINED: ERigControlVisibility = ERigControlVisibility(0);
    pub const BASED_ON_SELECTION: ERigControlVisibility = ERigControlVisibility(1);
}
#[repr(transparent)]
pub struct ERigControlTransformChannel(pub u8);
impl ERigControlTransformChannel {
    pub const TRANSLATION_X: ERigControlTransformChannel = ERigControlTransformChannel(
        0,
    );
    pub const TRANSLATION_Y: ERigControlTransformChannel = ERigControlTransformChannel(
        1,
    );
    pub const TRANSLATION_Z: ERigControlTransformChannel = ERigControlTransformChannel(
        2,
    );
    pub const PITCH: ERigControlTransformChannel = ERigControlTransformChannel(3);
    pub const YAW: ERigControlTransformChannel = ERigControlTransformChannel(4);
    pub const ROLL: ERigControlTransformChannel = ERigControlTransformChannel(5);
    pub const SCALE_X: ERigControlTransformChannel = ERigControlTransformChannel(6);
    pub const SCALE_Y: ERigControlTransformChannel = ERigControlTransformChannel(7);
    pub const SCALE_Z: ERigControlTransformChannel = ERigControlTransformChannel(8);
}
#[repr(transparent)]
pub struct EConnectorType(pub u8);
impl EConnectorType {
    pub const PRIMARY: EConnectorType = EConnectorType(0);
    pub const SECONDARY: EConnectorType = EConnectorType(1);
}
#[repr(transparent)]
pub struct EElementNameDisplayMode(pub u8);
impl EElementNameDisplayMode {
    pub const ASSET_DEFAULT: EElementNameDisplayMode = EElementNameDisplayMode(0);
    pub const AUTO: EElementNameDisplayMode = EElementNameDisplayMode(1);
    pub const FORCE_SHORT: EElementNameDisplayMode = EElementNameDisplayMode(2);
    pub const FORCE_LONG: EElementNameDisplayMode = EElementNameDisplayMode(3);
}
#[repr(transparent)]
pub struct EControlRigComponentMapDirection(pub u8);
impl EControlRigComponentMapDirection {
    pub const INPUT: EControlRigComponentMapDirection = EControlRigComponentMapDirection(
        0,
    );
    pub const OUTPUT: EControlRigComponentMapDirection = EControlRigComponentMapDirection(
        1,
    );
}
#[repr(transparent)]
pub struct EControlRigComponentSpace(pub u8);
impl EControlRigComponentSpace {
    pub const WORLD_SPACE: EControlRigComponentSpace = EControlRigComponentSpace(0);
    pub const ACTOR_SPACE: EControlRigComponentSpace = EControlRigComponentSpace(1);
    pub const COMPONENT_SPACE: EControlRigComponentSpace = EControlRigComponentSpace(2);
    pub const RIG_SPACE: EControlRigComponentSpace = EControlRigComponentSpace(3);
    pub const LOCAL_SPACE: EControlRigComponentSpace = EControlRigComponentSpace(4);
    pub const MAX: EControlRigComponentSpace = EControlRigComponentSpace(5);
}
#[repr(transparent)]
pub struct ECRSimConstraintType(pub u8);
impl ECRSimConstraintType {
    pub const DISTANCE: ECRSimConstraintType = ECRSimConstraintType(0);
    pub const DISTANCE_FROM_A: ECRSimConstraintType = ECRSimConstraintType(1);
    pub const DISTANCE_FROM_B: ECRSimConstraintType = ECRSimConstraintType(2);
    pub const PLANE: ECRSimConstraintType = ECRSimConstraintType(3);
}
#[repr(transparent)]
pub struct ECRSimPointForceType(pub u8);
impl ECRSimPointForceType {
    pub const DIRECTION: ECRSimPointForceType = ECRSimPointForceType(0);
}
#[repr(transparent)]
pub struct ECRSimSoftCollisionType(pub u8);
impl ECRSimSoftCollisionType {
    pub const PLANE: ECRSimSoftCollisionType = ECRSimSoftCollisionType(0);
    pub const SPHERE: ECRSimSoftCollisionType = ECRSimSoftCollisionType(1);
    pub const CONE: ECRSimSoftCollisionType = ECRSimSoftCollisionType(2);
}
#[repr(transparent)]
pub struct ERigBoneType(pub u8);
impl ERigBoneType {
    pub const IMPORTED: ERigBoneType = ERigBoneType(0);
    pub const USER: ERigBoneType = ERigBoneType(1);
}
#[repr(transparent)]
pub struct ERigTransformStackEntryType(pub u8);
impl ERigTransformStackEntryType {
    pub const TRANSFORM_POSE: ERigTransformStackEntryType = ERigTransformStackEntryType(
        0,
    );
    pub const CONTROL_OFFSET: ERigTransformStackEntryType = ERigTransformStackEntryType(
        1,
    );
    pub const CONTROL_SHAPE: ERigTransformStackEntryType = ERigTransformStackEntryType(
        2,
    );
    pub const CURVE_VALUE: ERigTransformStackEntryType = ERigTransformStackEntryType(3);
}
#[repr(transparent)]
pub struct ERigTransformType(pub u8);
impl ERigTransformType {
    pub const INITIAL_LOCAL: ERigTransformType = ERigTransformType(0);
    pub const CURRENT_LOCAL: ERigTransformType = ERigTransformType(1);
    pub const INITIAL_GLOBAL: ERigTransformType = ERigTransformType(2);
    pub const CURRENT_GLOBAL: ERigTransformType = ERigTransformType(3);
    pub const NUM_TRANSFORM_TYPES: ERigTransformType = ERigTransformType(4);
}
#[repr(transparent)]
pub struct ERigSpaceType(pub u8);
impl ERigSpaceType {
    pub const GLOBAL: ERigSpaceType = ERigSpaceType(0);
    pub const BONE: ERigSpaceType = ERigSpaceType(1);
    pub const CONTROL: ERigSpaceType = ERigSpaceType(2);
    pub const SPACE: ERigSpaceType = ERigSpaceType(3);
}
#[repr(transparent)]
pub struct ERigElementResolveState(pub u8);
impl ERigElementResolveState {
    pub const UNKNOWN: ERigElementResolveState = ERigElementResolveState(0);
    pub const INVALID_TARGET: ERigElementResolveState = ERigElementResolveState(1);
    pub const POSSIBLE_TARGET: ERigElementResolveState = ERigElementResolveState(2);
    pub const DEFAULT_TARGET: ERigElementResolveState = ERigElementResolveState(3);
    pub const MAX: ERigElementResolveState = ERigElementResolveState(4);
}
#[repr(transparent)]
pub struct EModularRigResolveState(pub u8);
impl EModularRigResolveState {
    pub const SUCCESS: EModularRigResolveState = EModularRigResolveState(0);
    pub const ERROR: EModularRigResolveState = EModularRigResolveState(1);
    pub const MAX: EModularRigResolveState = EModularRigResolveState(2);
}
#[repr(transparent)]
pub struct ERigMetadataType(pub u8);
impl ERigMetadataType {
    pub const BOOL: ERigMetadataType = ERigMetadataType(0);
    pub const BOOL_ARRAY: ERigMetadataType = ERigMetadataType(1);
    pub const FLOAT: ERigMetadataType = ERigMetadataType(2);
    pub const FLOAT_ARRAY: ERigMetadataType = ERigMetadataType(3);
    pub const INT32: ERigMetadataType = ERigMetadataType(4);
    pub const INT32_ARRAY: ERigMetadataType = ERigMetadataType(5);
    pub const NAME: ERigMetadataType = ERigMetadataType(6);
    pub const NAME_ARRAY: ERigMetadataType = ERigMetadataType(7);
    pub const VECTOR: ERigMetadataType = ERigMetadataType(8);
    pub const VECTOR_ARRAY: ERigMetadataType = ERigMetadataType(9);
    pub const ROTATOR: ERigMetadataType = ERigMetadataType(10);
    pub const ROTATOR_ARRAY: ERigMetadataType = ERigMetadataType(11);
    pub const QUAT: ERigMetadataType = ERigMetadataType(12);
    pub const QUAT_ARRAY: ERigMetadataType = ERigMetadataType(13);
    pub const TRANSFORM: ERigMetadataType = ERigMetadataType(14);
    pub const TRANSFORM_ARRAY: ERigMetadataType = ERigMetadataType(15);
    pub const LINEAR_COLOR: ERigMetadataType = ERigMetadataType(16);
    pub const LINEAR_COLOR_ARRAY: ERigMetadataType = ERigMetadataType(17);
    pub const RIG_ELEMENT_KEY: ERigMetadataType = ERigMetadataType(18);
    pub const RIG_ELEMENT_KEY_ARRAY: ERigMetadataType = ERigMetadataType(19);
    pub const INVALID: ERigMetadataType = ERigMetadataType(20);
}
#[repr(transparent)]
pub struct EMovieSceneControlRigSpaceType(pub u8);
impl EMovieSceneControlRigSpaceType {
    pub const PARENT: EMovieSceneControlRigSpaceType = EMovieSceneControlRigSpaceType(0);
    pub const WORLD: EMovieSceneControlRigSpaceType = EMovieSceneControlRigSpaceType(1);
    pub const CONTROL_RIG: EMovieSceneControlRigSpaceType = EMovieSceneControlRigSpaceType(
        2,
    );
}
#[repr(transparent)]
pub struct EAimMode(pub u8);
impl EAimMode {
    pub const AIM_AT_TARGET: EAimMode = EAimMode(0);
    pub const ORIENT_TO_TARGET: EAimMode = EAimMode(1);
    pub const MAX: EAimMode = EAimMode(2);
}
#[repr(transparent)]
pub struct EApplyTransformMode(pub u8);
impl EApplyTransformMode {
    pub const OVERRIDE: EApplyTransformMode = EApplyTransformMode(0);
    pub const ADDITIVE: EApplyTransformMode = EApplyTransformMode(1);
    pub const MAX: EApplyTransformMode = EApplyTransformMode(2);
}
#[repr(transparent)]
pub struct ETransformSpaceMode(pub u8);
impl ETransformSpaceMode {
    pub const LOCAL_SPACE: ETransformSpaceMode = ETransformSpaceMode(0);
    pub const GLOBAL_SPACE: ETransformSpaceMode = ETransformSpaceMode(1);
    pub const BASE_SPACE: ETransformSpaceMode = ETransformSpaceMode(2);
    pub const BASE_JOINT: ETransformSpaceMode = ETransformSpaceMode(3);
    pub const MAX: ETransformSpaceMode = ETransformSpaceMode(4);
}
#[repr(transparent)]
pub struct ETransformGetterType(pub u8);
impl ETransformGetterType {
    pub const INITIAL: ETransformGetterType = ETransformGetterType(0);
    pub const CURRENT: ETransformGetterType = ETransformGetterType(1);
    pub const MAX: ETransformGetterType = ETransformGetterType(2);
}
#[repr(transparent)]
pub struct ERigSwitchParentMode(pub u8);
impl ERigSwitchParentMode {
    pub const WORLD: ERigSwitchParentMode = ERigSwitchParentMode(0);
    pub const DEFAULT_PARENT: ERigSwitchParentMode = ERigSwitchParentMode(1);
    pub const PARENT_ITEM: ERigSwitchParentMode = ERigSwitchParentMode(2);
}
#[repr(transparent)]
pub struct ERigMetaDataNameSpace(pub u8);
impl ERigMetaDataNameSpace {
    pub const NONE: ERigMetaDataNameSpace = ERigMetaDataNameSpace(0);
    pub const SELF: ERigMetaDataNameSpace = ERigMetaDataNameSpace(1);
    pub const PARENT: ERigMetaDataNameSpace = ERigMetaDataNameSpace(2);
    pub const ROOT: ERigMetaDataNameSpace = ERigMetaDataNameSpace(3);
    pub const LAST: ERigMetaDataNameSpace = ERigMetaDataNameSpace(4);
}
#[repr(transparent)]
pub struct ERigEvent(pub u8);
impl ERigEvent {
    pub const NONE: ERigEvent = ERigEvent(0);
    pub const REQUEST_AUTO_KEY: ERigEvent = ERigEvent(1);
    pub const OPEN_UNDO_BRACKET: ERigEvent = ERigEvent(2);
    pub const CLOSE_UNDO_BRACKET: ERigEvent = ERigEvent(3);
    pub const MAX: ERigEvent = ERigEvent(4);
}
#[repr(transparent)]
pub struct EControlRigVectorKind(pub u8);
impl EControlRigVectorKind {
    pub const DIRECTION: EControlRigVectorKind = EControlRigVectorKind(0);
    pub const LOCATION: EControlRigVectorKind = EControlRigVectorKind(1);
}
#[repr(transparent)]
pub struct EControlRigCurveAlignment(pub u8);
impl EControlRigCurveAlignment {
    pub const FRONT: EControlRigCurveAlignment = EControlRigCurveAlignment(0);
    pub const STRETCHED: EControlRigCurveAlignment = EControlRigCurveAlignment(1);
}
#[repr(transparent)]
pub struct EControlRigModifyBoneMode(pub u8);
impl EControlRigModifyBoneMode {
    pub const OVERRIDE_LOCAL: EControlRigModifyBoneMode = EControlRigModifyBoneMode(0);
    pub const OVERRIDE_GLOBAL: EControlRigModifyBoneMode = EControlRigModifyBoneMode(1);
    pub const ADDITIVE_LOCAL: EControlRigModifyBoneMode = EControlRigModifyBoneMode(2);
    pub const ADDITIVE_GLOBAL: EControlRigModifyBoneMode = EControlRigModifyBoneMode(3);
    pub const MAX: EControlRigModifyBoneMode = EControlRigModifyBoneMode(4);
}
#[repr(transparent)]
pub struct EConstraintInterpType(pub u8);
impl EConstraintInterpType {
    pub const AVERAGE: EConstraintInterpType = EConstraintInterpType(0);
    pub const SHORTEST: EConstraintInterpType = EConstraintInterpType(1);
    pub const MAX: EConstraintInterpType = EConstraintInterpType(2);
}
#[repr(transparent)]
pub struct EControlRigReplayPlaybackMode(pub u8);
impl EControlRigReplayPlaybackMode {
    pub const LIVE: EControlRigReplayPlaybackMode = EControlRigReplayPlaybackMode(0);
    pub const REPLAY_INPUTS: EControlRigReplayPlaybackMode = EControlRigReplayPlaybackMode(
        1,
    );
    pub const GROUND_TRUTH: EControlRigReplayPlaybackMode = EControlRigReplayPlaybackMode(
        2,
    );
    pub const MAX: EControlRigReplayPlaybackMode = EControlRigReplayPlaybackMode(3);
}
#[repr(transparent)]
pub struct ERigControlValueType(pub u8);
impl ERigControlValueType {
    pub const INITIAL: ERigControlValueType = ERigControlValueType(0);
    pub const CURRENT: ERigControlValueType = ERigControlValueType(1);
    pub const MINIMUM: ERigControlValueType = ERigControlValueType(2);
    pub const MAXIMUM: ERigControlValueType = ERigControlValueType(3);
}
#[repr(transparent)]
pub struct EControlRigFKRigExecuteMode(pub u8);
impl EControlRigFKRigExecuteMode {
    pub const REPLACE: EControlRigFKRigExecuteMode = EControlRigFKRigExecuteMode(0);
    pub const ADDITIVE: EControlRigFKRigExecuteMode = EControlRigFKRigExecuteMode(1);
    pub const DIRECT: EControlRigFKRigExecuteMode = EControlRigFKRigExecuteMode(2);
    pub const MAX: EControlRigFKRigExecuteMode = EControlRigFKRigExecuteMode(3);
}
#[repr(transparent)]
pub struct ERigExecutionType(pub u8);
impl ERigExecutionType {
    pub const RUNTIME: ERigExecutionType = ERigExecutionType(0);
    pub const EDITING: ERigExecutionType = ERigExecutionType(1);
    pub const MAX: ERigExecutionType = ERigExecutionType(2);
}
#[repr(transparent)]
pub struct EControlRigType(pub u8);
impl EControlRigType {
    pub const INDEPENDENT_RIG: EControlRigType = EControlRigType(0);
    pub const RIG_MODULE: EControlRigType = EControlRigType(1);
    pub const MODULAR_RIG: EControlRigType = EControlRigType(2);
    pub const MAX: EControlRigType = EControlRigType(3);
}
#[repr(transparent)]
pub struct ERigHierarchyNotification(pub u8);
impl ERigHierarchyNotification {
    pub const ELEMENT_ADDED: ERigHierarchyNotification = ERigHierarchyNotification(0);
    pub const ELEMENT_REMOVED: ERigHierarchyNotification = ERigHierarchyNotification(1);
    pub const ELEMENT_RENAMED: ERigHierarchyNotification = ERigHierarchyNotification(2);
    pub const ELEMENT_SELECTED: ERigHierarchyNotification = ERigHierarchyNotification(3);
    pub const ELEMENT_DESELECTED: ERigHierarchyNotification = ERigHierarchyNotification(
        4,
    );
    pub const PARENT_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(5);
    pub const HIERARCHY_RESET: ERigHierarchyNotification = ERigHierarchyNotification(6);
    pub const CONTROL_SETTING_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        7,
    );
    pub const CONTROL_VISIBILITY_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        8,
    );
    pub const CONTROL_DRIVEN_LIST_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        9,
    );
    pub const CONTROL_SHAPE_TRANSFORM_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        10,
    );
    pub const PARENT_WEIGHTS_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        11,
    );
    pub const INTERACTION_BRACKET_OPENED: ERigHierarchyNotification = ERigHierarchyNotification(
        12,
    );
    pub const INTERACTION_BRACKET_CLOSED: ERigHierarchyNotification = ERigHierarchyNotification(
        13,
    );
    pub const ELEMENT_REORDERED: ERigHierarchyNotification = ERigHierarchyNotification(
        14,
    );
    pub const CONNECTOR_SETTING_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        15,
    );
    pub const SOCKET_COLOR_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        16,
    );
    pub const SOCKET_DESCRIPTION_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        17,
    );
    pub const SOCKET_DESIRED_PARENT_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        18,
    );
    pub const HIERARCHY_COPIED: ERigHierarchyNotification = ERigHierarchyNotification(
        19,
    );
    pub const COMPONENT_ADDED: ERigHierarchyNotification = ERigHierarchyNotification(20);
    pub const COMPONENT_REMOVED: ERigHierarchyNotification = ERigHierarchyNotification(
        21,
    );
    pub const COMPONENT_CONTENT_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        22,
    );
    pub const COMPONENT_SELECTED: ERigHierarchyNotification = ERigHierarchyNotification(
        23,
    );
    pub const COMPONENT_DESELECTED: ERigHierarchyNotification = ERigHierarchyNotification(
        24,
    );
    pub const COMPONENT_RENAMED: ERigHierarchyNotification = ERigHierarchyNotification(
        25,
    );
    pub const COMPONENT_REPARENTED: ERigHierarchyNotification = ERigHierarchyNotification(
        26,
    );
    pub const SHORT_NAME_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        27,
    );
    pub const MAX: ERigHierarchyNotification = ERigHierarchyNotification(28);
}
#[repr(transparent)]
pub struct EMultiRigTreeDisplayMode(pub i32);
impl EMultiRigTreeDisplayMode {
    pub const ALL: EMultiRigTreeDisplayMode = EMultiRigTreeDisplayMode(0);
    pub const SELECTED_RIGS: EMultiRigTreeDisplayMode = EMultiRigTreeDisplayMode(1);
    pub const SELECTED_MODULES: EMultiRigTreeDisplayMode = EMultiRigTreeDisplayMode(2);
}
#[repr(transparent)]
pub struct EControlRigInteractionType(pub u8);
impl EControlRigInteractionType {
    pub const NONE: EControlRigInteractionType = EControlRigInteractionType(0);
    pub const TRANSLATE: EControlRigInteractionType = EControlRigInteractionType(1);
    pub const ROTATE: EControlRigInteractionType = EControlRigInteractionType(2);
    pub const SCALE: EControlRigInteractionType = EControlRigInteractionType(4);
    pub const ALL: EControlRigInteractionType = EControlRigInteractionType(7);
}
