#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FRigElementKey {
    pub ty: ERigElementType,
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FRigBaseElement {
    pub key: FRigElementKey,
    pub index: i32,
    pub sub_index: i32,
    pub created_at_instruction_index: i32,
    pub spawn_index: i32,
    pub b_selected: bool,
}
#[repr(C, align(8))]
pub struct FRigTransformElement {}
#[repr(C, align(16))]
pub struct FRigMultiParentElement {}
#[repr(C, align(16))]
pub struct FRigControlElement {
    pub settings: FRigControlSettings,
    pub preferred_euler_angles: FRigPreferredEulerAngles,
}
#[repr(C, align(8))]
pub struct FRigPreferredEulerAngles {
    pub rotation_order: EEulerRotationOrder,
    pub current: FVector,
    pub initial: FVector,
}
#[repr(C, align(16))]
pub struct FRigControlSettings {
    pub animation_type: ERigControlAnimationType,
    pub control_type: ERigControlType,
    pub display_name: FName,
    pub primary_axis: ERigControlAxis,
    pub b_is_curve: bool,
    pub limit_enabled: TArray<FRigControlLimitEnabled>,
    pub b_draw_limits: bool,
    pub minimum_value: FRigControlValue,
    pub maximum_value: FRigControlValue,
    pub b_shape_visible: bool,
    pub shape_visibility: ERigControlVisibility,
    pub shape_name: FName,
    pub shape_color: FLinearColor,
    pub b_is_transient_control: bool,
    pub control_enum: UPtr<UEnum>,
    pub customization: FRigControlElementCustomization,
    pub driven_controls: TArray<FRigElementKey>,
    pub b_group_with_parent_control: bool,
    pub b_restrict_space_switching: bool,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
    pub preferred_rotation_order: EEulerRotationOrder,
    pub b_use_preferred_rotation_order: bool,
    pub b_animatable_deprecated: bool,
    pub b_shape_enabled_deprecated: bool,
    pub shape_transform: FTransform,
}
#[repr(C, align(8))]
pub struct FRigControlElementCustomization {
    pub available_spaces: TArray<FRigElementKeyWithLabel>,
    pub removed_spaces: TArray<FRigElementKey>,
}
#[repr(C, align(4))]
pub struct FRigElementKeyWithLabel {
    pub key: FRigElementKey,
    pub label: FName,
}
#[repr(C, align(16))]
pub struct FRigControlValue {
    pub float_storage: FRigControlValueStorage,
    pub storage_deprecated: FTransform,
}
#[repr(C, align(4))]
pub struct FRigControlValueStorage {
    pub float00: f32,
    pub float01: f32,
    pub float02: f32,
    pub float03: f32,
    pub float10: f32,
    pub float11: f32,
    pub float12: f32,
    pub float13: f32,
    pub float20: f32,
    pub float21: f32,
    pub float22: f32,
    pub float23: f32,
    pub float30: f32,
    pub float31: f32,
    pub float32: f32,
    pub float33: f32,
    pub float00_2: f32,
    pub float01_2: f32,
    pub float02_2: f32,
    pub float03_2: f32,
    pub float10_2: f32,
    pub float11_2: f32,
    pub float12_2: f32,
    pub float13_2: f32,
    pub float20_2: f32,
    pub float21_2: f32,
    pub float22_2: f32,
    pub float23_2: f32,
    pub float30_2: f32,
    pub float31_2: f32,
    pub float32_2: f32,
    pub float33_2: f32,
    pub b_valid: bool,
}
#[repr(C, align(1))]
pub struct FRigControlLimitEnabled {
    pub b_minimum: bool,
    pub b_maximum: bool,
}
#[repr(C, align(8))]
pub struct FRigModuleSettings {
    pub identifier: FRigModuleIdentifier,
    pub icon: FSoftObjectPath,
    pub category: FString,
    pub keywords: FString,
    pub description: FString,
    pub exposed_connectors: TArray<FRigModuleConnector>,
}
#[repr(C, align(8))]
pub struct FRigModuleConnector {
    pub name: FString,
    pub settings: FRigConnectorSettings,
}
#[repr(C, align(8))]
pub struct FRigConnectorSettings {
    pub description: FString,
    pub ty: EConnectorType,
    pub b_optional: bool,
    pub b_is_array: bool,
    pub b_post_construction: bool,
    pub rules: TArray<FRigConnectionRuleStash>,
}
#[repr(C, align(8))]
pub struct FRigConnectionRuleStash {
    pub script_struct_path: FString,
    pub exported_text: FString,
}
#[repr(C, align(8))]
pub struct FRigModuleIdentifier {
    pub name: FString,
    pub ty: FString,
}
#[repr(C, align(8))]
pub struct FRigInfluenceMapPerEvent {
    pub maps: TArray<FRigInfluenceMap>,
    pub event_to_index: TMap<FName, i32>,
}
#[repr(C, align(8))]
pub struct FRigInfluenceMap {
    pub event_name: FName,
    pub entries: TArray<FRigInfluenceEntry>,
    pub key_to_index: TMap<FRigElementKey, i32>,
}
#[repr(C, align(8))]
pub struct FRigInfluenceEntry {
    pub source: FRigElementKey,
    pub affected_list: TArray<FRigElementKey>,
}
#[repr(C, align(4))]
pub struct FRigHierarchySettings {
    pub element_name_display_mode: EElementNameDisplayMode,
    pub procedural_element_limit: i32,
}
#[repr(C, align(1))]
pub struct FControlRigIOSettings {
    pub b_update_pose: bool,
    pub b_update_curves: bool,
}
#[repr(C, align(8))]
pub struct FMovieSceneControlRigInstanceData {
    pub b_additive: bool,
    pub b_apply_bone_filter: bool,
    pub bone_filter: FInputBlendPose,
    pub weight: FMovieSceneFloatChannel,
    pub operand: FMovieSceneEvaluationOperand,
}
#[repr(C, align(8))]
pub struct FRigUnit {}
#[repr(C, align(8))]
pub struct FRigUnit_DebugBase {
    pub debug_draw_settings: FRigVMDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FRigUnitMutable {
    pub execute_pin: FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_DebugBaseMutable {
    pub debug_draw_settings: FRigVMDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FRigUnit_HighlevelBase {}
#[repr(C, align(8))]
pub struct FRigUnit_HighlevelBaseMutable {}
#[repr(C, align(8))]
pub struct FStructReference {}
#[repr(C, align(8))]
pub struct FAnimNode_ControlRigBase {
    pub source: FPoseLink,
    pub b_reset_input_pose_to_initial: bool,
    pub b_transfer_input_pose: bool,
    pub b_transfer_input_curves: bool,
    pub b_transfer_pose_in_global_space: bool,
    pub input_bones_to_transfer: TArray<FBoneReference>,
    pub output_bones_to_transfer: TArray<FBoneReference>,
    pub asset_user_data: TArray<UPtr<UAssetUserData>>,
    pub node_mapping_container: TWeakObjectPtr<UNodeMappingContainer>,
    pub input_settings: FControlRigIOSettings,
    pub output_settings: FControlRigIOSettings,
    pub b_execute: bool,
    pub event_queue: TArray<FControlRigAnimNodeEventName>,
}
#[repr(C, align(4))]
pub struct FControlRigAnimNodeEventName {
    pub event_name: FName,
}
#[repr(C, align(8))]
pub struct FAnimNode_ControlRig {
    pub control_rig_class: TSubclassOf<UControlRig>,
    pub default_control_rig_class: TSubclassOf<UControlRig>,
    pub control_rig: UPtr<UControlRig>,
    pub control_rig_per_class: TMap<TSubclassOf<UObject>, UPtr<UControlRig>>,
    pub alpha: f32,
    pub alpha_input_type: EAnimAlphaInputType,
    pub flags_797: u8,
    pub alpha_scale_bias: FInputScaleBias,
    pub alpha_bool_blend: FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: FInputScaleBiasClamp,
    pub input_mapping: TMap<FName, FName>,
    pub output_mapping: TMap<FName, FName>,
    pub lod_threshold: i32,
}
#[repr(C, align(8))]
pub struct FAnimNode_ControlRig_ExternalSource {
    pub control_rig: TWeakObjectPtr<UControlRig>,
}
#[repr(C, align(8))]
pub struct FControlRigReference {}
#[repr(C, align(16))]
pub struct FControlRigAnimInstanceProxy {}
#[repr(C, align(16))]
pub struct FControlRigComponentMappedElement {
    pub component_reference: FSoftComponentReference,
    pub transform_index: i32,
    pub transform_name: FName,
    pub element_type: ERigElementType,
    pub element_name: FName,
    pub direction: EControlRigComponentMapDirection,
    pub offset: FTransform,
    pub weight: f32,
    pub space: EControlRigComponentSpace,
    pub scene_component: UPtr<USceneComponent>,
    pub element_index: i32,
    pub sub_index: i32,
}
#[repr(C, align(8))]
pub struct FControlRigComponentMappedComponent {
    pub component: UPtr<USceneComponent>,
    pub element_name: FName,
    pub element_type: ERigElementType,
    pub direction: EControlRigComponentMapDirection,
}
#[repr(C, align(4))]
pub struct FControlRigComponentMappedBone {
    pub source: FName,
    pub target: FName,
}
#[repr(C, align(4))]
pub struct FControlRigComponentMappedCurve {
    pub source: FName,
    pub target: FName,
}
#[repr(C, align(16))]
pub struct FControlShapeActorCreationParam {}
#[repr(C, align(16))]
pub struct FControlRigShapeDefinition {
    pub shape_name: FName,
    pub static_mesh: TSoftObjectPtr<UStaticMesh>,
    pub transform: FTransform,
}
#[repr(C, align(8))]
pub struct FControlRigOverrideValue {}
#[repr(C, align(8))]
pub struct FControlRigOverrideContainer {}
#[repr(C, align(8))]
pub struct FControlRigReplayVariable {
    pub name: FName,
    pub cpp_type: FName,
    pub value: FString,
}
#[repr(C, align(8))]
pub struct FSampleTrackHost {}
#[repr(C, align(8))]
pub struct FControlRigReplayTracks {}
#[repr(C, align(8))]
pub struct FControlRigTestDataFrame {
    pub absolute_time: f64,
    pub delta_time: f64,
    pub variables: TArray<FControlRigReplayVariable>,
    pub pose: FRigPose,
    pub metadata: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FRigPose {
    pub elements: TArray<FRigPoseElement>,
    pub hierarchy_topology_version: i32,
    pub pose_hash: i32,
}
#[repr(C, align(16))]
pub struct FRigPoseElement {
    pub index: FCachedRigElement,
    pub global_transform: FTransform,
    pub local_transform: FTransform,
    pub preferred_euler_angle: FVector,
    pub active_parent: FRigElementKey,
    pub curve_value: f32,
}
#[repr(C, align(8))]
pub struct FCachedRigElement {
    pub key: FRigElementKey,
    pub index: u16,
    pub container_version: i32,
}
#[repr(C, align(8))]
pub struct FControlRigValidationContext {}
#[repr(C, align(8))]
pub struct FCRSimContainer {
    pub time_step: f32,
    pub accumulated_time: f32,
    pub time_left_for_step: f32,
}
#[repr(C, align(4))]
pub struct FCRSimLinearSpring {
    pub subject_a: i32,
    pub subject_b: i32,
    pub coefficient: f32,
    pub equilibrium: f32,
}
#[repr(C, align(8))]
pub struct FCRSimPointConstraint {
    pub ty: ECRSimConstraintType,
    pub subject_a: i32,
    pub subject_b: i32,
    pub data_a: FVector,
    pub data_b: FVector,
}
#[repr(C, align(8))]
pub struct FCRSimPointContainer {
    pub points: TArray<FRigVMSimPoint>,
    pub springs: TArray<FCRSimLinearSpring>,
    pub forces: TArray<FCRSimPointForce>,
    pub collision_volumes: TArray<FCRSimSoftCollision>,
    pub constraints: TArray<FCRSimPointConstraint>,
    pub previous_step: TArray<FRigVMSimPoint>,
}
#[repr(C, align(16))]
pub struct FCRSimSoftCollision {
    pub transform: FTransform,
    pub shape_type: ECRSimSoftCollisionType,
    pub minimum_distance: f32,
    pub maximum_distance: f32,
    pub falloff_type: ERigVMAnimEasingType,
    pub coefficient: f32,
    pub b_inverted: bool,
}
#[repr(C, align(8))]
pub struct FCRSimPointForce {
    pub force_type: ECRSimPointForceType,
    pub vector: FVector,
    pub coefficient: f32,
    pub b_normalize: bool,
}
#[repr(C, align(8))]
pub struct FRigModuleInstance {
    pub name: FName,
    pub rig_class: TSubclassOf<UControlRig>,
    pub rig_ptr: UPtr<UControlRig>,
    pub parent_path_deprecated: FString,
    pub parent_module_name: FName,
    pub variable_bindings: TMap<FName, FRigVMExternalVariable>,
}
#[repr(C, align(8))]
pub struct FRigModuleExecutionElement {
    pub module_name: FName,
    pub event_name: FName,
    pub b_executed: bool,
    pub duration_in_micro_seconds: f64,
}
#[repr(C, align(8))]
pub struct FRigModuleExecutionQueue {
    pub elements: TArray<FRigModuleExecutionElement>,
}
#[repr(C, align(8))]
pub struct FRigModuleReference {
    pub name: FName,
    pub short_name_deprecated: FString,
    pub b_short_name_based_on_path_deprecated: bool,
    pub parent_path_deprecated: FString,
    pub parent_module_name: FName,
    pub class: TSoftObjectPtr<UClass>,
    pub connections_deprecated: TMap<FRigElementKey, FRigElementKey>,
    pub config_values_deprecated: TMap<FName, FString>,
    pub config_overrides: FControlRigOverrideContainer,
    pub bindings: TMap<FName, FString>,
    pub previous_name: FName,
    pub previous_parent_name: FName,
}
#[repr(C, align(8))]
pub struct FModularRigSingleConnection {
    pub connector: FRigElementKey,
    pub target_deprecated: FRigElementKey,
    pub targets: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FModularRigConnections {
    pub connection_list: TArray<FModularRigSingleConnection>,
}
#[repr(C, align(8))]
pub struct FModularRigModel {
    pub modules: TArray<FRigModuleReference>,
    pub connections: FModularRigConnections,
    pub controller: UPtr<UObject>,
    pub previous_module_paths: TMap<FRigHierarchyModulePath, FName>,
}
#[repr(C, align(8))]
pub struct FRigHierarchyModulePath {
    pub module_path: FString,
}
#[repr(C, align(8))]
pub struct FModularRigModuleSettingsForClipboard {
    pub module_class: FSoftObjectPath,
    pub defaults: TMap<FString, FString>,
    pub overrides: TMap<FString, FString>,
    pub bindings: TMap<FName, FString>,
}
#[repr(C, align(8))]
pub struct FModularRigModuleSettingsSetForClipboard {
    pub settings: TMap<FName, FModularRigModuleSettingsForClipboard>,
}
#[repr(C, align(16))]
pub struct FConstraintNodeData {
    pub relative_parent: FTransform,
    pub constraint_offset: FConstraintOffset,
    pub linked_node: FName,
    pub constraints: TArray<FTransformConstraint>,
}
#[repr(C, align(8))]
pub struct FAnimationHierarchy {
    pub user_data: TArray<FConstraintNodeData>,
}
#[repr(C, align(8))]
pub struct FRigElement {
    pub name: FName,
    pub index: i32,
}
#[repr(C, align(16))]
pub struct FRigBone {
    pub parent_name: FName,
    pub parent_index: i32,
    pub initial_transform: FTransform,
    pub global_transform: FTransform,
    pub local_transform: FTransform,
    pub dependents: TArray<i32>,
    pub ty: ERigBoneType,
}
#[repr(C, align(8))]
pub struct FRigBoneHierarchy {
    pub bones: TArray<FRigBone>,
}
#[repr(C, align(8))]
pub struct FRigConnectionRule {}
#[repr(C, align(8))]
pub struct FRigAndConnectionRule {
    pub child_rules: TArray<FRigConnectionRuleStash>,
}
#[repr(C, align(8))]
pub struct FRigOrConnectionRule {
    pub child_rules: TArray<FRigConnectionRuleStash>,
}
#[repr(C, align(8))]
pub struct FRigTypeConnectionRule {
    pub element_type: ERigElementType,
}
#[repr(C, align(8))]
pub struct FRigTagConnectionRule {
    pub tag: FName,
}
#[repr(C, align(8))]
pub struct FRigChildOfPrimaryConnectionRule {}
#[repr(C, align(16))]
pub struct FRigControl {
    pub control_type: ERigControlType,
    pub display_name: FName,
    pub parent_name: FName,
    pub parent_index: i32,
    pub space_name: FName,
    pub space_index: i32,
    pub offset_transform: FTransform,
    pub initial_value: FRigControlValue,
    pub value: FRigControlValue,
    pub primary_axis: ERigControlAxis,
    pub b_is_curve: bool,
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
    pub gizmo_transform: FTransform,
    pub gizmo_color: FLinearColor,
    pub dependents: TArray<i32>,
    pub b_is_transient_control: bool,
    pub control_enum: UPtr<UEnum>,
}
#[repr(C, align(8))]
pub struct FRigControlHierarchy {
    pub controls: TArray<FRigControl>,
}
#[repr(C, align(8))]
pub struct FRigCurve {
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FRigCurveContainer {
    pub curves: TArray<FRigCurve>,
}
#[repr(C, align(16))]
pub struct FRigTransformStackEntry {
    pub key: FRigElementKey,
    pub entry_type: ERigTransformStackEntryType,
    pub transform_type: ERigTransformType,
    pub old_transform: FTransform,
    pub new_transform: FTransform,
    pub b_affect_children: bool,
    pub callstack: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FCachedRigComponent {
    pub cached_element: FCachedRigElement,
    pub name: FName,
    pub index: u16,
}
#[repr(C, align(8))]
pub struct FRigBaseComponent {}
#[repr(C, align(8))]
pub struct FRigHierarchyContainer {
    pub bone_hierarchy: FRigBoneHierarchy,
    pub space_hierarchy: FRigSpaceHierarchy,
    pub control_hierarchy: FRigControlHierarchy,
    pub curve_container: FRigCurveContainer,
}
#[repr(C, align(8))]
pub struct FRigSpaceHierarchy {
    pub spaces: TArray<FRigSpace>,
}
#[repr(C, align(16))]
pub struct FRigSpace {
    pub space_type: ERigSpaceType,
    pub parent_name: FName,
    pub parent_index: i32,
    pub initial_transform: FTransform,
    pub local_transform: FTransform,
}
#[repr(C, align(1))]
pub struct FRigHierarchyRef {}
#[repr(C, align(4))]
pub struct FRigControlModifiedContext {}
#[repr(C, align(4))]
pub struct FRigComponentKey {
    pub element_key: FRigElementKey,
    pub name: FName,
}
#[repr(C, align(4))]
pub struct FRigHierarchyKey {}
#[repr(C, align(8))]
pub struct FRigElementKeyCollection {
    pub keys: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigEventContext {}
#[repr(C, align(8))]
pub struct FRigElementResolveResult {
    pub key: FRigElementKey,
    pub state: ERigElementResolveState,
    pub message: FText,
}
#[repr(C, align(8))]
pub struct FModularRigResolveResult {
    pub connector: FRigElementKey,
    pub matches: TArray<FRigElementResolveResult>,
    pub excluded: TArray<FRigElementResolveResult>,
    pub state: EModularRigResolveState,
    pub message: FText,
}
#[repr(C, align(8))]
pub struct FRigTransformDirtyState {}
#[repr(C, align(8))]
pub struct FRigLocalAndGlobalDirtyState {
    pub global: FRigTransformDirtyState,
    pub local: FRigTransformDirtyState,
}
#[repr(C, align(8))]
pub struct FRigCurrentAndInitialDirtyState {
    pub current: FRigLocalAndGlobalDirtyState,
    pub initial: FRigLocalAndGlobalDirtyState,
}
#[repr(C, align(8))]
pub struct FRigComputedTransform {}
#[repr(C, align(8))]
pub struct FRigLocalAndGlobalTransform {
    pub local: FRigComputedTransform,
    pub global: FRigComputedTransform,
}
#[repr(C, align(8))]
pub struct FRigCurrentAndInitialTransform {
    pub current: FRigLocalAndGlobalTransform,
    pub initial: FRigLocalAndGlobalTransform,
}
#[repr(C, align(8))]
pub struct FRigSingleParentElement {}
#[repr(C, align(4))]
pub struct FRigElementWeight {
    pub location: f32,
    pub rotation: f32,
    pub scale: f32,
}
#[repr(C, align(16))]
pub struct FRigElementParentConstraint {}
#[repr(C, align(8))]
pub struct FRigBoneElement {
    pub bone_type: ERigBoneType,
}
#[repr(C, align(16))]
pub struct FRigNullElement {}
#[repr(C, align(8))]
pub struct FRigCurveElement {}
#[repr(C, align(8))]
pub struct FRigReferenceElement {}
#[repr(C, align(8))]
pub struct FRigConnectorState {
    pub name: FName,
    pub resolved_target: FRigElementKey,
    pub settings: FRigConnectorSettings,
}
#[repr(C, align(8))]
pub struct FRigConnectorElement {
    pub settings: FRigConnectorSettings,
}
#[repr(C, align(16))]
pub struct FRigSocketState {
    pub name: FName,
    pub parent: FRigElementKey,
    pub initial_local_transform: FTransform,
    pub color: FLinearColor,
    pub description: FString,
}
#[repr(C, align(8))]
pub struct FRigSocketElement {}
#[repr(C, align(8))]
pub struct FRigHierarchyCopyPasteContentPerElement {
    pub key: FRigElementKey,
    pub content: FString,
    pub parents: TArray<FRigElementKeyWithLabel>,
    pub parent_weights: TArray<FRigElementWeight>,
    pub poses: TArray<FTransform>,
    pub dirty_states: TArray<bool>,
}
#[repr(C, align(8))]
pub struct FRigHierarchyCopyPasteContent {
    pub elements: TArray<FRigHierarchyCopyPasteContentPerElement>,
    pub types: TArray<ERigElementType>,
    pub contents: TArray<FString>,
    pub local_transforms: TArray<FTransform>,
    pub global_transforms: TArray<FTransform>,
}
#[repr(C, align(8))]
pub struct FRigBaseMetadata {
    pub name: FName,
    pub ty: ERigMetadataType,
}
#[repr(C, align(8))]
pub struct FRigBoolMetadata {
    pub value: bool,
}
#[repr(C, align(8))]
pub struct FRigBoolArrayMetadata {
    pub value: TArray<bool>,
}
#[repr(C, align(8))]
pub struct FRigFloatMetadata {
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FRigFloatArrayMetadata {
    pub value: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FRigInt32Metadata {
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FRigInt32ArrayMetadata {
    pub value: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FRigNameMetadata {
    pub value: FName,
}
#[repr(C, align(8))]
pub struct FRigNameArrayMetadata {
    pub value: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FRigVectorMetadata {
    pub value: FVector,
}
#[repr(C, align(8))]
pub struct FRigVectorArrayMetadata {
    pub value: TArray<FVector>,
}
#[repr(C, align(8))]
pub struct FRigRotatorMetadata {
    pub value: FRotator,
}
#[repr(C, align(8))]
pub struct FRigRotatorArrayMetadata {
    pub value: TArray<FRotator>,
}
#[repr(C, align(16))]
pub struct FRigQuatMetadata {
    pub value: FQuat,
}
#[repr(C, align(8))]
pub struct FRigQuatArrayMetadata {
    pub value: TArray<FQuat>,
}
#[repr(C, align(16))]
pub struct FRigTransformMetadata {
    pub value: FTransform,
}
#[repr(C, align(8))]
pub struct FRigTransformArrayMetadata {
    pub value: TArray<FTransform>,
}
#[repr(C, align(8))]
pub struct FRigLinearColorMetadata {
    pub value: FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigLinearColorArrayMetadata {
    pub value: TArray<FLinearColor>,
}
#[repr(C, align(8))]
pub struct FRigElementKeyMetadata {
    pub value: FRigElementKey,
}
#[repr(C, align(8))]
pub struct FRigElementKeyArrayMetadata {
    pub value: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigInfluenceEntryModifier {
    pub affected_list: TArray<FRigElementKey>,
}
#[repr(C, align(1))]
pub struct FModularRigSettings {
    pub b_auto_resolve: bool,
}
#[repr(C, align(8))]
pub struct FRigModuleDescription {
    pub path: FSoftObjectPath,
    pub settings: FRigModuleSettings,
}
#[repr(C, align(8))]
pub struct FModuleReferenceData {
    pub module_path: FString,
    pub referenced_module: FSoftClassPath,
}
#[repr(C, align(8))]
pub struct FRigPhysicsSimulationBase {}
#[repr(C, align(4))]
pub struct FRigPhysicsSolverID {
    pub guid: FGuid,
}
#[repr(C, align(4))]
pub struct FRigPhysicsSolverDescription {
    pub id: FRigPhysicsSolverID,
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FAnimNode_ControlRigInputPose {
    pub input_pose: FPoseLink,
}
#[repr(C, align(16))]
pub struct FControlRigLayerInstanceProxy {}
#[repr(C, align(8))]
pub struct FControlRigSequenceObjectReference {
    pub control_rig_class: TSubclassOf<UControlRig>,
}
#[repr(C, align(8))]
pub struct FControlRigSequenceObjectReferences {
    pub array: TArray<FControlRigSequenceObjectReference>,
}
#[repr(C, align(8))]
pub struct FControlRigSequenceObjectReferenceMap {
    pub binding_ids: TArray<FGuid>,
    pub references: TArray<FControlRigSequenceObjectReferences>,
}
#[repr(C, align(8))]
pub struct FEnumParameterNameAndCurve {
    pub parameter_curve: FMovieSceneByteChannel,
}
#[repr(C, align(8))]
pub struct FIntegerParameterNameAndCurve {
    pub parameter_curve: FMovieSceneIntegerChannel,
}
#[repr(C, align(8))]
pub struct FSpaceControlNameAndChannel {
    pub control_name: FName,
    pub space_curve: FMovieSceneControlRigSpaceChannel,
}
#[repr(C, align(8))]
pub struct FMovieSceneControlRigSpaceChannel {
    pub key_times: TArray<FFrameNumber>,
    pub key_values: TArray<FMovieSceneControlRigSpaceBaseKey>,
    pub key_handles: FMovieSceneKeyHandleMap,
    pub default_value: FMovieSceneControlRigSpaceBaseKey,
    pub b_has_default_value: bool,
}
#[repr(C, align(4))]
pub struct FMovieSceneControlRigSpaceBaseKey {
    pub space_type: EMovieSceneControlRigSpaceType,
    pub control_rig_element: FRigElementKey,
}
#[repr(C, align(8))]
pub struct FChannelMapInfo {
    pub control_index: i32,
    pub total_channel_index: i32,
    pub channel_index: i32,
    pub parent_control_index: i32,
    pub channel_type_name: FName,
    pub b_does_have_space: bool,
    pub space_channel_index: i32,
    pub mask_index: i32,
    pub category_index: i32,
    pub constraints_index: TArray<u32>,
}
#[repr(C, align(8))]
pub struct FMovieSceneControlRigParameterTemplate {
    pub enums: TArray<FEnumParameterNameAndCurve>,
    pub integers: TArray<FIntegerParameterNameAndCurve>,
    pub spaces: TArray<FSpaceControlNameAndChannel>,
    pub constraints: TArray<FConstraintAndActiveChannel>,
}
#[repr(C, align(1))]
pub struct FControlRotationOrder {
    pub rotation_order: EEulerRotationOrder,
    pub b_override_setting: bool,
}
#[repr(C, align(8))]
pub struct FControlRigSettingsPerPinBool {
    pub values: TMap<FString, bool>,
}
#[repr(C, align(16))]
pub struct FRigControlCopy {
    pub name: FName,
    pub control_type: ERigControlType,
    pub parent_key: FRigElementKey,
    pub value: FRigControlValue,
    pub offset_transform: FTransform,
    pub parent_transform: FTransform,
    pub local_transform: FTransform,
    pub global_transform: FTransform,
}
#[repr(C, align(8))]
pub struct FControlRigControlPose {
    pub copy_of_controls: TArray<FRigControlCopy>,
}
#[repr(C, align(8))]
pub struct FSampleTrackMemoryData {
    pub buffer: TArray<u8>,
    pub names: TArray<FName>,
    pub object_paths: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FRigDispatchFactory {}
#[repr(C, align(8))]
pub struct FRigDispatch_AnimAttributeBase {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetAnimAttribute {}
#[repr(C, align(8))]
pub struct FRigDispatch_SetAnimAttribute {}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTrace_WorkData {
    pub hash: u32,
    pub b_hit: bool,
    pub hit_location: FVector,
    pub hit_normal: FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTraceWorld {
    pub start: FVector,
    pub end: FVector,
    pub channel: ECollisionChannel,
    pub radius: f32,
    pub b_hit: bool,
    pub hit_location: FVector,
    pub hit_normal: FVector,
    pub work_data: FRigUnit_SphereTrace_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTraceByTraceChannel {
    pub start: FVector,
    pub end: FVector,
    pub trace_channel: ETraceTypeQuery,
    pub radius: f32,
    pub b_hit: bool,
    pub hit_location: FVector,
    pub hit_normal: FVector,
    pub work_data: FRigUnit_SphereTrace_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_LineTraceByTraceChannel {
    pub start: FVector,
    pub end: FVector,
    pub trace_channel: ETraceTypeQuery,
    pub b_hit: bool,
    pub hit_location: FVector,
    pub hit_normal: FVector,
    pub work_data: FRigUnit_SphereTrace_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTraceByObjectTypes {
    pub start: FVector,
    pub end: FVector,
    pub object_types: TArray<EObjectTypeQuery>,
    pub radius: f32,
    pub b_hit: bool,
    pub hit_location: FVector,
    pub hit_normal: FVector,
    pub work_data: FRigUnit_SphereTrace_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_LineTraceByObjectTypes {
    pub start: FVector,
    pub end: FVector,
    pub object_types: TArray<EObjectTypeQuery>,
    pub b_hit: bool,
    pub hit_location: FVector,
    pub hit_normal: FVector,
    pub work_data: FRigUnit_SphereTrace_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_Control {
    pub transform: FEulerTransform,
    pub base: FTransform,
    pub init_transform: FTransform,
    pub result: FTransform,
    pub filter: FTransformFilter,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_Control_StaticMesh {
    pub mesh_transform: FTransform,
}
#[repr(C, align(8))]
pub struct FRigDispatch_GetUserData {}
#[repr(C, align(8))]
pub struct FRigUnit_SetupShapeLibraryFromUserData {
    pub name_space: FString,
    pub path: FString,
    pub library_name: FString,
    pub log_shape_libraries: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_ShapeExists {
    pub shape_name: FName,
    pub result: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugBezier {
    pub bezier: FRigVMFourPointBezier,
    pub minimum_u: f32,
    pub maximum_u: f32,
    pub color: FLinearColor,
    pub thickness: f32,
    pub detail: i32,
    pub space: FName,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugBezierItemSpace {
    pub bezier: FRigVMFourPointBezier,
    pub minimum_u: f32,
    pub maximum_u: f32,
    pub color: FLinearColor,
    pub thickness: f32,
    pub detail: i32,
    pub space: FRigElementKey,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugHierarchy {
    pub execute_pin: FRigVMExecutePin,
    pub items: TArray<FRigElementKey>,
    pub scale: f32,
    pub color: FLinearColor,
    pub thickness: f32,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugPose {
    pub execute_pin: FRigVMExecutePin,
    pub pose: FRigPose,
    pub items: TArray<FRigElementKey>,
    pub scale: f32,
    pub color: FLinearColor,
    pub thickness: f32,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLine {
    pub a: FVector,
    pub b: FVector,
    pub color: FLinearColor,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLineItemSpace {
    pub a: FVector,
    pub b: FVector,
    pub color: FLinearColor,
    pub thickness: f32,
    pub space: FRigElementKey,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLineStrip {
    pub points: TArray<FVector>,
    pub color: FLinearColor,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLineStripItemSpace {
    pub points: TArray<FVector>,
    pub color: FLinearColor,
    pub thickness: f32,
    pub space: FRigElementKey,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugRectangle {
    pub transform: FTransform,
    pub color: FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugRectangleItemSpace {
    pub transform: FTransform,
    pub color: FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FRigElementKey,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugArc {
    pub transform: FTransform,
    pub color: FLinearColor,
    pub radius: f32,
    pub minimum_degrees: f32,
    pub maximum_degrees: f32,
    pub thickness: f32,
    pub detail: i32,
    pub space: FName,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugArcItemSpace {
    pub transform: FTransform,
    pub color: FLinearColor,
    pub radius: f32,
    pub minimum_degrees: f32,
    pub maximum_degrees: f32,
    pub thickness: f32,
    pub detail: i32,
    pub space: FRigElementKey,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransform {
    pub transform: FTransform,
    pub mode: ERigUnitDebugTransformMode,
    pub color: FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FName,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformMutable {
    pub transform: FTransform,
    pub mode: ERigUnitDebugTransformMode,
    pub color: FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FName,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformMutableItemSpace {
    pub transform: FTransform,
    pub mode: ERigUnitDebugTransformMode,
    pub color: FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_DebugTransformArrayMutable_WorkData {
    pub draw_transforms: TArray<FTransform>,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformArrayMutable {
    pub transforms: TArray<FTransform>,
    pub mode: ERigUnitDebugTransformMode,
    pub color: FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FName,
    pub world_offset: FTransform,
    pub b_enabled: bool,
    pub work_data: FRigUnit_DebugTransformArrayMutable_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformArrayMutableItemSpace {
    pub transforms: TArray<FTransform>,
    pub parent_indices: TArray<i32>,
    pub mode: ERigUnitDebugTransformMode,
    pub color: FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
    pub world_offset: FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_StartProfilingTimer {}
#[repr(C, align(8))]
pub struct FRigUnit_EndProfilingTimer {
    pub number_of_measurements: i32,
    pub prefix: FString,
    pub accumulated_time: f32,
    pub measurements_left: i32,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_VisualDebugVector {
    pub value: FVector,
    pub b_enabled: bool,
    pub mode: ERigUnitVisualDebugPointMode,
    pub color: FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
#[repr(C, align(8))]
pub struct FRigUnit_VisualDebugVectorItemSpace {
    pub value: FVector,
    pub b_enabled: bool,
    pub mode: ERigUnitVisualDebugPointMode,
    pub color: FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugQuat {
    pub value: FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugQuatItemSpace {
    pub value: FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugTransform {
    pub value: FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugTransformItemSpace {
    pub value: FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertTransform {
    pub input: FTransform,
    pub result: FEulerTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertEulerTransform {
    pub input: FEulerTransform,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertRotation {
    pub input: FRotator,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertVectorRotation {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertQuaternion {
    pub input: FQuat,
    pub result: FRotator,
}
#[repr(C, align(8))]
pub struct FRigUnit_ConvertVectorToRotation {
    pub input: FVector,
    pub result: FRotator,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertVectorToQuaternion {
    pub input: FVector,
    pub result: FQuat,
}
#[repr(C, align(8))]
pub struct FRigUnit_ConvertRotationToVector {
    pub input: FRotator,
    pub result: FVector,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertQuaternionToVector {
    pub input: FQuat,
    pub result: FVector,
}
#[repr(C, align(16))]
pub struct FRigUnit_ToSwingAndTwist {
    pub input: FQuat,
    pub twist_axis: FVector,
    pub swing: FQuat,
    pub twist: FQuat,
}
#[repr(C, align(8))]
pub struct FRigUnit_BinaryFloatOp {
    pub argument0: f32,
    pub argument1: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_Multiply_FloatFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_Add_FloatFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_Subtract_FloatFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_Divide_FloatFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_Clamp_Float {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub result: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_MapRange_Float {
    pub value: f32,
    pub min_in: f32,
    pub max_in: f32,
    pub min_out: f32,
    pub max_out: f32,
    pub result: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_BinaryQuaternionOp {
    pub argument0: FQuat,
    pub argument1: FQuat,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigUnit_MultiplyQuaternion {}
#[repr(C, align(16))]
pub struct FRigUnit_UnaryQuaternionOp {
    pub argument: FQuat,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigUnit_InverseQuaterion {}
#[repr(C, align(16))]
pub struct FRigUnit_QuaternionToAxisAndAngle {
    pub argument: FQuat,
    pub axis: FVector,
    pub angle: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_QuaternionFromAxisAndAngle {
    pub axis: FVector,
    pub angle: f32,
    pub result: FQuat,
}
#[repr(C, align(16))]
pub struct FRigUnit_QuaternionToAngle {
    pub axis: FVector,
    pub argument: FQuat,
    pub angle: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_BinaryTransformOp {
    pub argument0: FTransform,
    pub argument1: FTransform,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_MultiplyTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetRelativeTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_BinaryVectorOp {
    pub argument0: FVector,
    pub argument1: FVector,
    pub result: FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_Multiply_VectorVector {}
#[repr(C, align(8))]
pub struct FRigUnit_Add_VectorVector {}
#[repr(C, align(8))]
pub struct FRigUnit_Subtract_VectorVector {}
#[repr(C, align(8))]
pub struct FRigUnit_Divide_VectorVector {}
#[repr(C, align(8))]
pub struct FRigUnit_Distance_VectorVector {
    pub argument0: FVector,
    pub argument1: FVector,
    pub result: f32,
}
#[repr(C, align(16))]
pub struct FAimTarget {
    pub weight: f32,
    pub transform: FTransform,
    pub align_vector: FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_AimConstraint_WorkData {
    pub constraint_data: TArray<FConstraintData>,
}
#[repr(C, align(8))]
pub struct FRigUnit_AimConstraint {
    pub joint: FName,
    pub aim_mode: EAimMode,
    pub up_mode: EAimMode,
    pub aim_vector: FVector,
    pub up_vector: FVector,
    pub aim_targets: TArray<FAimTarget>,
    pub up_targets: TArray<FAimTarget>,
    pub work_data: FRigUnit_AimConstraint_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_ApplyFK {
    pub joint: FName,
    pub transform: FTransform,
    pub filter: FTransformFilter,
    pub apply_transform_mode: EApplyTransformMode,
    pub apply_transform_space: ETransformSpaceMode,
    pub base_transform: FTransform,
    pub base_joint: FName,
}
#[repr(C, align(16))]
pub struct FBlendTarget {
    pub transform: FTransform,
    pub weight: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_BlendTransform {
    pub source: FTransform,
    pub targets: TArray<FBlendTarget>,
    pub result: FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetJointTransform {
    pub joint: FName,
    pub ty: ETransformGetterType,
    pub transform_space: ETransformSpaceMode,
    pub base_transform: FTransform,
    pub base_joint: FName,
    pub output: FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKFK {
    pub start_joint: FName,
    pub end_joint: FName,
    pub pole_target: FVector,
    pub spin: f32,
    pub end_effector: FTransform,
    pub ik_blend: f32,
    pub start_joint_fk_transform: FTransform,
    pub mid_joint_fk_transform: FTransform,
    pub end_joint_fk_transform: FTransform,
    pub previous_fkik_blend: f32,
    pub start_joint_ik_transform: FTransform,
    pub mid_joint_ik_transform: FTransform,
    pub end_joint_ik_transform: FTransform,
    pub start_joint_index: i32,
    pub mid_joint_index: i32,
    pub end_joint_index: i32,
    pub upper_limb_length: f32,
    pub lower_limb_length: f32,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DrawContainerGetInstruction {
    pub instruction_name: FName,
    pub color: FLinearColor,
    pub transform: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_DrawContainerSetColor {
    pub instruction_name: FName,
    pub color: FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigUnit_DrawContainerSetThickness {
    pub instruction_name: FName,
    pub thickness: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_DrawContainerSetTransform {
    pub instruction_name: FName,
    pub transform: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_BeginExecution {
    pub execute_pin: FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_PreBeginExecution {
    pub execute_pin: FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_PostBeginExecution {
    pub execute_pin: FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionBase {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionChain {
    pub first_item: FRigElementKey,
    pub last_item: FRigElementKey,
    pub reverse: bool,
    pub collection: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionChainArray {
    pub first_item: FRigElementKey,
    pub last_item: FRigElementKey,
    pub reverse: bool,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionNameSearch {
    pub partial_name: FName,
    pub type_to_search: ERigElementType,
    pub collection: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionNameSearchArray {
    pub partial_name: FName,
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionChildren {
    pub parent: FRigElementKey,
    pub b_include_parent: bool,
    pub b_recursive: bool,
    pub type_to_search: ERigElementType,
    pub collection: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionChildrenArray {
    pub parent: FRigElementKey,
    pub b_include_parent: bool,
    pub b_recursive: bool,
    pub b_default_children: bool,
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionGetAll {
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionReplaceItems {
    pub items: FRigElementKeyCollection,
    pub old: FName,
    pub new: FName,
    pub remove_invalid_items: bool,
    pub b_allow_duplicates: bool,
    pub collection: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionReplaceItemsArray {
    pub items: TArray<FRigElementKey>,
    pub old: FName,
    pub new: FName,
    pub remove_invalid_items: bool,
    pub b_allow_duplicates: bool,
    pub result: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionItems {
    pub items: TArray<FRigElementKey>,
    pub b_allow_duplicates: bool,
    pub collection: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionGetItems {
    pub collection: FRigElementKeyCollection,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionGetParentIndices {
    pub collection: FRigElementKeyCollection,
    pub parent_indices: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionGetParentIndicesItemArray {
    pub items: TArray<FRigElementKey>,
    pub parent_indices: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionUnion {
    pub a: FRigElementKeyCollection,
    pub b: FRigElementKeyCollection,
    pub b_allow_duplicates: bool,
    pub collection: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionIntersection {
    pub a: FRigElementKeyCollection,
    pub b: FRigElementKeyCollection,
    pub collection: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionDifference {
    pub a: FRigElementKeyCollection,
    pub b: FRigElementKeyCollection,
    pub collection: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionReverse {
    pub collection: FRigElementKeyCollection,
    pub reversed: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionCount {
    pub collection: FRigElementKeyCollection,
    pub count: i32,
}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionItemAtIndex {
    pub collection: FRigElementKeyCollection,
    pub index: i32,
    pub item: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_CollectionLoop {
    pub block_to_run: FName,
    pub collection: FRigElementKeyCollection,
    pub item: FRigElementKey,
    pub index: i32,
    pub count: i32,
    pub ratio: f32,
    pub completed: FControlRigExecuteContext,
}
#[repr(C, align(16))]
pub struct FControlRigExecuteContext {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionAddItem {
    pub collection: FRigElementKeyCollection,
    pub item: FRigElementKey,
    pub result: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_DynamicHierarchyBase {}
#[repr(C, align(8))]
pub struct FRigUnit_DynamicHierarchyBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_AddParent {
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub display_label: FName,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetDefaultParent {
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
}
#[repr(C, align(8))]
pub struct FRigUnit_AddAvailableSpaces {
    pub control: FRigElementKey,
    pub spaces: TArray<FRigElementKeyWithLabel>,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetChannelHosts {
    pub channel: FRigElementKey,
    pub hosts: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_SwitchParent {
    pub mode: ERigSwitchParentMode,
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub b_maintain_global: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParentWeights {
    pub child: FRigElementKey,
    pub weights: TArray<FRigElementWeight>,
    pub parents: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParentWeightsArray {
    pub child: FRigElementKey,
    pub weights: TArray<FRigElementWeight>,
    pub parents: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchySetParentWeights {
    pub child: FRigElementKey,
    pub weights: TArray<FRigElementWeight>,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyReset {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyImportFromSkeleton {
    pub name_space: FName,
    pub b_include_curves: bool,
    pub b_include_mesh_sockets: bool,
    pub b_include_virtual_bones: bool,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyRemoveElement {
    pub item: FRigElementKey,
    pub b_success: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddElement {
    pub parent: FRigElementKey,
    pub name: FName,
    pub item: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddBone {
    pub transform: FTransform,
    pub space: ERigVMTransformSpace,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddNull {
    pub transform: FTransform,
    pub space: ERigVMTransformSpace,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControl_Settings {
    pub display_name: FName,
    pub b_selectable: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControl_ShapeSettings {
    pub b_visible: bool,
    pub name: FName,
    pub color: FLinearColor,
    pub transform: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControl_ProxySettings {
    pub b_is_proxy: bool,
    pub driven_controls: TArray<FRigElementKey>,
    pub shape_visibility: ERigControlVisibility,
}
#[repr(C, align(4))]
pub struct FRigUnit_HierarchyAddControlFloat_LimitSettings {
    pub limit: FRigControlLimitEnabled,
    pub min_value: f32,
    pub max_value: f32,
    pub b_draw_limits: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlFloat_Settings {
    pub primary_axis: ERigControlAxis,
    pub b_is_scale: bool,
    pub limits: FRigUnit_HierarchyAddControlFloat_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlElement {
    pub offset_transform: FTransform,
    pub offset_space: ERigVMTransformSpace,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlFloat {
    pub initial_value: f32,
    pub settings: FRigUnit_HierarchyAddControlFloat_Settings,
}
#[repr(C, align(4))]
pub struct FRigUnit_HierarchyAddControlInteger_LimitSettings {
    pub limit: FRigControlLimitEnabled,
    pub min_value: i32,
    pub max_value: i32,
    pub b_draw_limits: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlInteger_Settings {
    pub primary_axis: ERigControlAxis,
    pub control_enum: UPtr<UEnum>,
    pub limits: FRigUnit_HierarchyAddControlInteger_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlInteger {
    pub initial_value: i32,
    pub settings: FRigUnit_HierarchyAddControlInteger_Settings,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlVector2D_LimitSettings {
    pub limit_x: FRigControlLimitEnabled,
    pub limit_y: FRigControlLimitEnabled,
    pub min_value: FVector2D,
    pub max_value: FVector2D,
    pub b_draw_limits: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector2D_Settings {
    pub primary_axis: ERigControlAxis,
    pub limits: FRigUnit_HierarchyAddControlVector2D_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector2D {
    pub initial_value: FVector2D,
    pub settings: FRigUnit_HierarchyAddControlVector2D_Settings,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlVector_LimitSettings {
    pub limit_x: FRigControlLimitEnabled,
    pub limit_y: FRigControlLimitEnabled,
    pub limit_z: FRigControlLimitEnabled,
    pub min_value: FVector,
    pub max_value: FVector,
    pub b_draw_limits: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector_Settings {
    pub initial_space: ERigVMTransformSpace,
    pub b_is_position: bool,
    pub limits: FRigUnit_HierarchyAddControlVector_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector {
    pub initial_value: FVector,
    pub settings: FRigUnit_HierarchyAddControlVector_Settings,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlRotator_LimitSettings {
    pub limit_pitch: FRigControlLimitEnabled,
    pub limit_yaw: FRigControlLimitEnabled,
    pub limit_roll: FRigControlLimitEnabled,
    pub min_value: FRotator,
    pub max_value: FRotator,
    pub b_draw_limits: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlRotator_Settings {
    pub initial_space: ERigVMTransformSpace,
    pub limits: FRigUnit_HierarchyAddControlRotator_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
    pub b_use_preferred_rotation_order: bool,
    pub preferred_rotation_order: EEulerRotationOrder,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlRotator {
    pub initial_value: FRotator,
    pub settings: FRigUnit_HierarchyAddControlRotator_Settings,
}
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
    pub min_value: FEulerTransform,
    pub max_value: FEulerTransform,
    pub b_draw_limits: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlTransform_Settings {
    pub initial_space: ERigVMTransformSpace,
    pub b_use_preferred_rotation_order: bool,
    pub preferred_rotation_order: EEulerRotationOrder,
    pub limits: FRigUnit_HierarchyAddControlTransform_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlTransform {
    pub initial_value: FTransform,
    pub settings: FRigUnit_HierarchyAddControlTransform_Settings,
}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannelEmptyLimitSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelBool {
    pub initial_value: bool,
    pub minimum_value: bool,
    pub maximum_value: bool,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelEmptyLimitSettings,
}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings {
    pub enabled: FRigControlLimitEnabled,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelFloat {
    pub initial_value: f32,
    pub minimum_value: f32,
    pub maximum_value: f32,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelScaleFloat {
    pub initial_value: f32,
    pub minimum_value: f32,
    pub maximum_value: f32,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelInteger {
    pub initial_value: i32,
    pub minimum_value: i32,
    pub maximum_value: i32,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings,
    pub control_enum: UPtr<UEnum>,
}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannel2DLimitSettings {
    pub x: FRigControlLimitEnabled,
    pub y: FRigControlLimitEnabled,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelVector2D {
    pub initial_value: FVector2D,
    pub minimum_value: FVector2D,
    pub maximum_value: FVector2D,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannel2DLimitSettings,
}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannelVectorLimitSettings {
    pub x: FRigControlLimitEnabled,
    pub y: FRigControlLimitEnabled,
    pub z: FRigControlLimitEnabled,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelVector {
    pub initial_value: FVector,
    pub minimum_value: FVector,
    pub maximum_value: FVector,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelVectorLimitSettings,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelScaleVector {
    pub initial_value: FVector,
    pub minimum_value: FVector,
    pub maximum_value: FVector,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelVectorLimitSettings,
}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannelRotatorLimitSettings {
    pub pitch: FRigControlLimitEnabled,
    pub yaw: FRigControlLimitEnabled,
    pub roll: FRigControlLimitEnabled,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelRotator {
    pub initial_value: FRotator,
    pub minimum_value: FRotator,
    pub maximum_value: FRotator,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelRotatorLimitSettings,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyGetShapeSettings {
    pub item: FRigElementKey,
    pub settings: FRigUnit_HierarchyAddControl_ShapeSettings,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchySetShapeSettings {
    pub item: FRigElementKey,
    pub settings: FRigUnit_HierarchyAddControl_ShapeSettings,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddSocket {
    pub transform: FTransform,
    pub space: ERigVMTransformSpace,
    pub color: FLinearColor,
    pub description: FString,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyBase {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParent {
    pub child: FRigElementKey,
    pub b_default_parent: bool,
    pub parent: FRigElementKey,
    pub cached_child: FCachedRigElement,
    pub cached_parent: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParents {
    pub child: FRigElementKey,
    pub b_include_child: bool,
    pub b_reverse: bool,
    pub parents: FRigElementKeyCollection,
    pub cached_child: FCachedRigElement,
    pub cached_parents: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParentsItemArray {
    pub child: FRigElementKey,
    pub b_include_child: bool,
    pub b_reverse: bool,
    pub b_default_parent: bool,
    pub parents: TArray<FRigElementKey>,
    pub cached_child: FCachedRigElement,
    pub cached_parents: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetChildren {
    pub parent: FRigElementKey,
    pub b_include_parent: bool,
    pub b_recursive: bool,
    pub children: FRigElementKeyCollection,
    pub cached_parent: FCachedRigElement,
    pub cached_children: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetSiblings {
    pub item: FRigElementKey,
    pub b_include_item: bool,
    pub siblings: FRigElementKeyCollection,
    pub cached_item: FCachedRigElement,
    pub cached_siblings: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetSiblingsItemArray {
    pub item: FRigElementKey,
    pub b_include_item: bool,
    pub b_default_siblings: bool,
    pub siblings: TArray<FRigElementKey>,
    pub cached_item: FCachedRigElement,
    pub cached_siblings: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetChainItemArray {
    pub start: FRigElementKey,
    pub end: FRigElementKey,
    pub b_include_start: bool,
    pub b_include_end: bool,
    pub b_reverse: bool,
    pub chain: TArray<FRigElementKey>,
    pub cached_start: FCachedRigElement,
    pub cached_end: FCachedRigElement,
    pub cached_chain: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetPose {
    pub initial: bool,
    pub element_type: ERigElementType,
    pub items_to_get: FRigElementKeyCollection,
    pub pose: FRigPose,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetPoseItemArray {
    pub initial: bool,
    pub element_type: ERigElementType,
    pub items_to_get: TArray<FRigElementKey>,
    pub pose: FRigPose,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchySetPose {
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub space: ERigVMTransformSpace,
    pub items_to_set: FRigElementKeyCollection,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchySetPoseItemArray {
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub space: ERigVMTransformSpace,
    pub items_to_set: TArray<FRigElementKey>,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_PoseIsEmpty {
    pub pose: FRigPose,
    pub is_empty: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetItems {
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub items: FRigElementKeyCollection,
}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetItemsItemArray {
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetDelta {
    pub pose_a: FRigPose,
    pub pose_b: FRigPose,
    pub position_threshold: f32,
    pub rotation_threshold: f32,
    pub scale_threshold: f32,
    pub curve_threshold: f32,
    pub element_type: ERigElementType,
    pub space: ERigVMTransformSpace,
    pub items_to_compare: FRigElementKeyCollection,
    pub poses_are_equal: bool,
    pub items_with_delta: FRigElementKeyCollection,
}
#[repr(C, align(16))]
pub struct FRigUnit_PoseGetTransform {
    pub pose: FRigPose,
    pub item: FRigElementKey,
    pub space: ERigVMTransformSpace,
    pub valid: bool,
    pub transform: FTransform,
    pub curve_value: f32,
    pub cached_pose_element_index: i32,
    pub cached_pose_hash: i32,
}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetTransformArray {
    pub pose: FRigPose,
    pub space: ERigVMTransformSpace,
    pub valid: bool,
    pub transforms: TArray<FTransform>,
}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetCurve {
    pub pose: FRigPose,
    pub curve: FName,
    pub valid: bool,
    pub curve_value: f32,
    pub cached_pose_element_index: i32,
    pub cached_pose_hash: i32,
}
#[repr(C, align(16))]
pub struct FRigUnit_PoseLoop {
    pub block_to_run: FName,
    pub pose: FRigPose,
    pub item: FRigElementKey,
    pub global_transform: FTransform,
    pub local_transform: FTransform,
    pub curve_value: f32,
    pub index: i32,
    pub count: i32,
    pub ratio: f32,
    pub completed: FControlRigExecuteContext,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyCreatePoseItemArray_Entry {
    pub item: FRigElementKey,
    pub local_transform: FTransform,
    pub global_transform: FTransform,
    pub use_euler_angles: bool,
    pub euler_angles: FVector,
    pub curve_value: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyCreatePoseItemArray {
    pub entries: TArray<FRigUnit_HierarchyCreatePoseItemArray_Entry>,
    pub pose: FRigPose,
}
#[repr(C, align(8))]
pub struct FRigUnit_InteractionExecution {
    pub execute_pin: FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_InverseExecution {
    pub execute_pin: FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_IsInteracting {
    pub b_is_interacting: bool,
    pub b_is_translating: bool,
    pub b_is_rotating: bool,
    pub b_is_scaling: bool,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_ItemBase {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemExists {
    pub item: FRigElementKey,
    pub exists: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_ItemReplace {
    pub item: FRigElementKey,
    pub old: FName,
    pub new: FName,
    pub result: FRigElementKey,
}
#[repr(C, align(8))]
pub struct FRigUnit_ItemEquals {
    pub a: FRigElementKey,
    pub b: FRigElementKey,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_ItemNotEquals {
    pub a: FRigElementKey,
    pub b: FRigElementKey,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_ItemTypeEquals {
    pub a: FRigElementKey,
    pub b: FRigElementKey,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_ItemTypeNotEquals {
    pub a: FRigElementKey,
    pub b: FRigElementKey,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_ItemToName {
    pub value: FRigElementKey,
    pub result: FName,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddPhysicsSolver {
    pub name: FName,
    pub solver: FRigPhysicsSolverID,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddPhysicsJoint {
    pub transform: FTransform,
    pub solver: FRigPhysicsSolverID,
}
#[repr(C, align(8))]
pub struct FRigUnit_PrepareForExecution {
    pub execute_pin: FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_PostPrepareForExecution {
    pub execute_pin: FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_RigModulesBase {}
#[repr(C, align(8))]
pub struct FRigUnit_RigModulesBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_ResolveConnector {
    pub connector: FRigElementKey,
    pub skip_socket: bool,
    pub result: FRigElementKey,
    pub b_is_connected: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_ResolveArrayConnector {
    pub connector: FRigElementKey,
    pub skip_socket: bool,
    pub result: TArray<FRigElementKey>,
    pub b_is_connected: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetCurrentNameSpace {
    pub name_space: FString,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemShortName {
    pub item: FRigElementKey,
    pub short_name: FName,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemNameSpace {
    pub item: FRigElementKey,
    pub has_name_space: bool,
    pub name_space: FString,
}
#[repr(C, align(8))]
pub struct FRigUnit_IsItemInCurrentNameSpace {
    pub item: FRigElementKey,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemsInNameSpace {
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetModuleName {
    pub module: FString,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemModuleName {
    pub item: FRigElementKey,
    pub is_part_of_module: bool,
    pub module: FString,
}
#[repr(C, align(8))]
pub struct FRigUnit_IsItemInCurrentModule {
    pub item: FRigElementKey,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemsInModule {
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(16))]
pub struct FRigUnit_SequenceExecution {
    pub execute_context: FControlRigExecuteContext,
    pub a: FControlRigExecuteContext,
    pub b: FControlRigExecuteContext,
    pub c: FControlRigExecuteContext,
    pub d: FControlRigExecuteContext,
}
#[repr(C, align(16))]
pub struct FRigUnit_AddBoneTransform {
    pub bone: FName,
    pub transform: FTransform,
    pub weight: f32,
    pub b_post_multiply: bool,
    pub b_propagate_to_children: bool,
    pub cached_bone: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_Item {
    pub item: FRigElementKey,
}
#[repr(C, align(8))]
pub struct FRigUnit_ItemArray {
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_BoneName {
    pub bone: FName,
}
#[repr(C, align(8))]
pub struct FRigUnit_SpaceName {
    pub space: FName,
}
#[repr(C, align(8))]
pub struct FRigUnit_ControlName {
    pub control: FName,
}
#[repr(C, align(8))]
pub struct FRigDispatch_ComponentBase {}
#[repr(C, align(8))]
pub struct FRigDispatch_SpawnComponent {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetComponentContent {}
#[repr(C, align(8))]
pub struct FRigDispatch_SetComponentContent {}
#[repr(C, align(8))]
pub struct FRigUnit_GetAnimationChannelBase {
    pub control: FName,
    pub channel: FName,
    pub b_initial: bool,
    pub cached_channel_key: FRigElementKey,
    pub cached_channel_hash: i32,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetBoolAnimationChannel {
    pub value: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetFloatAnimationChannel {
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetIntAnimationChannel {
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetVector2DAnimationChannel {
    pub value: FVector2D,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetVectorAnimationChannel {
    pub value: FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetRotatorAnimationChannel {
    pub value: FRotator,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetTransformAnimationChannel {
    pub value: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetAnimationChannelBase {
    pub execute_pin: FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetBoolAnimationChannel {
    pub value: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetFloatAnimationChannel {
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetIntAnimationChannel {
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetVector2DAnimationChannel {
    pub value: FVector2D,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetVectorAnimationChannel {
    pub value: FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetRotatorAnimationChannel {
    pub value: FRotator,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetTransformAnimationChannel {
    pub value: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetAnimationChannelFromItemBase {
    pub item: FRigElementKey,
    pub b_initial: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetBoolAnimationChannelFromItem {
    pub value: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetFloatAnimationChannelFromItem {
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetIntAnimationChannelFromItem {
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetVector2DAnimationChannelFromItem {
    pub value: FVector2D,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetVectorAnimationChannelFromItem {
    pub value: FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetRotatorAnimationChannelFromItem {
    pub value: FRotator,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetTransformAnimationChannelFromItem {
    pub value: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetAnimationChannelBaseFromItem {
    pub execute_pin: FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetBoolAnimationChannelFromItem {
    pub value: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetFloatAnimationChannelFromItem {
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetIntAnimationChannelFromItem {
    pub value: i32,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetVector2DAnimationChannelFromItem {
    pub value: FVector2D,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetVectorAnimationChannelFromItem {
    pub value: FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetRotatorAnimationChannelFromItem {
    pub value: FRotator,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetTransformAnimationChannelFromItem {
    pub value: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_CurveExists {
    pub curve: FName,
    pub exists: bool,
    pub cached_curve_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_FindClosestItem {
    pub items: TArray<FRigElementKey>,
    pub point: FVector,
    pub item: FRigElementKey,
    pub cached_items: TArray<FCachedRigElement>,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetBoneTransform {
    pub bone: FName,
    pub space: ERigVMTransformSpace,
    pub transform: FTransform,
    pub cached_bone: FCachedRigElement,
    pub b_first_update: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetControlInitialTransform {
    pub control: FName,
    pub space: ERigVMTransformSpace,
    pub transform: FTransform,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetControlOffset {
    pub control: FName,
    pub space: ERigVMTransformSpace,
    pub offset_transform: FTransform,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlBool {
    pub control: FName,
    pub bool_value: bool,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlFloat {
    pub control: FName,
    pub float_value: f32,
    pub minimum: f32,
    pub maximum: f32,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlInteger {
    pub control: FName,
    pub integer_value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlVector2D {
    pub control: FName,
    pub vector: FVector2D,
    pub minimum: FVector2D,
    pub maximum: FVector2D,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlVector {
    pub control: FName,
    pub space: ERigVMTransformSpace,
    pub vector: FVector,
    pub minimum: FVector,
    pub maximum: FVector,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlRotator {
    pub control: FName,
    pub space: ERigVMTransformSpace,
    pub rotator: FRotator,
    pub minimum: FRotator,
    pub maximum: FRotator,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetControlTransform {
    pub control: FName,
    pub space: ERigVMTransformSpace,
    pub transform: FTransform,
    pub minimum: FTransform,
    pub maximum: FTransform,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetCurveValue {
    pub curve: FName,
    pub valid: bool,
    pub value: f32,
    pub cached_curve_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetInitialBoneTransform {
    pub bone: FName,
    pub space: ERigVMTransformSpace,
    pub transform: FTransform,
    pub cached_bone: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetRelativeBoneTransform {
    pub bone: FName,
    pub space: FName,
    pub transform: FTransform,
    pub cached_bone: FCachedRigElement,
    pub cached_space: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetRelativeTransformForItem {
    pub child: FRigElementKey,
    pub b_child_initial: bool,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub relative_transform: FTransform,
    pub cached_child: FCachedRigElement,
    pub cached_parent: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetSpaceTransform {
    pub space: FName,
    pub space_type: ERigVMTransformSpace,
    pub transform: FTransform,
    pub cached_space_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetTransform {
    pub item: FRigElementKey,
    pub space: ERigVMTransformSpace,
    pub b_initial: bool,
    pub transform: FTransform,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetTransformArray {
    pub items: FRigElementKeyCollection,
    pub space: ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<FTransform>,
    pub cached_index: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetTransformItemArray {
    pub items: TArray<FRigElementKey>,
    pub space: ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<FTransform>,
    pub cached_index: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigDispatch_MetadataBase {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetMetadata {}
#[repr(C, align(8))]
pub struct FRigDispatch_SetMetadata {}
#[repr(C, align(8))]
pub struct FRigUnit_RemoveMetadata {
    pub item: FRigElementKey,
    pub name: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub removed: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_RemoveAllMetadata {
    pub item: FRigElementKey,
    pub name_space: ERigMetaDataNameSpace,
    pub removed: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_HasMetadata {
    pub item: FRigElementKey,
    pub name: FName,
    pub ty: ERigMetadataType,
    pub name_space: ERigMetaDataNameSpace,
    pub found: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_FindItemsWithMetadata {
    pub name: FName,
    pub ty: ERigMetadataType,
    pub name_space: ERigMetaDataNameSpace,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetMetadataTags {
    pub item: FRigElementKey,
    pub tags: TArray<FName>,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetMetadataTag {
    pub item: FRigElementKey,
    pub tag: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetMetadataTagArray {
    pub item: FRigElementKey,
    pub tags: TArray<FName>,
    pub name_space: ERigMetaDataNameSpace,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_RemoveMetadataTag {
    pub item: FRigElementKey,
    pub tag: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub removed: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_HasMetadataTag {
    pub item: FRigElementKey,
    pub tag: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub found: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_HasMetadataTagArray {
    pub item: FRigElementKey,
    pub tags: TArray<FName>,
    pub name_space: ERigMetaDataNameSpace,
    pub found: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_FindItemsWithMetadataTag {
    pub tag: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_FindItemsWithMetadataTagArray {
    pub tags: TArray<FName>,
    pub name_space: ERigMetaDataNameSpace,
    pub items: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_FilterItemsByMetadataTags {
    pub items: TArray<FRigElementKey>,
    pub tags: TArray<FName>,
    pub name_space: ERigMetaDataNameSpace,
    pub inclusive: bool,
    pub result: TArray<FRigElementKey>,
    pub cached_indices: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigDispatch_GetModuleMetadata {}
#[repr(C, align(8))]
pub struct FRigDispatch_SetModuleMetadata {}
#[repr(C, align(16))]
pub struct FRigUnit_OffsetTransformForItem {
    pub item: FRigElementKey,
    pub offset_transform: FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_ParentSwitchConstraint {
    pub subject: FRigElementKey,
    pub parent_index: i32,
    pub parents: FRigElementKeyCollection,
    pub initial_global_transform: FTransform,
    pub weight: f32,
    pub transform: FTransform,
    pub switched: bool,
    pub cached_subject: FCachedRigElement,
    pub cached_parent: FCachedRigElement,
    pub relative_offset: FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_ParentSwitchConstraintArray {
    pub subject: FRigElementKey,
    pub parent_index: i32,
    pub parents: TArray<FRigElementKey>,
    pub initial_global_transform: FTransform,
    pub weight: f32,
    pub transform: FTransform,
    pub switched: bool,
    pub cached_subject: FCachedRigElement,
    pub cached_parent: FCachedRigElement,
    pub relative_offset: FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_ProjectTransformToNewParent {
    pub child: FRigElementKey,
    pub b_child_initial: bool,
    pub old_parent: FRigElementKey,
    pub b_old_parent_initial: bool,
    pub new_parent: FRigElementKey,
    pub b_new_parent_initial: bool,
    pub transform: FTransform,
    pub cached_child: FCachedRigElement,
    pub cached_old_parent: FCachedRigElement,
    pub cached_new_parent: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_PropagateTransform {
    pub item: FRigElementKey,
    pub b_recompute_global: bool,
    pub b_apply_to_children: bool,
    pub b_recursive: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SendEvent {
    pub event: ERigEvent,
    pub item: FRigElementKey,
    pub offset_in_seconds: f32,
    pub b_enable: bool,
    pub b_only_during_interaction: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetBoneInitialTransform {
    pub bone: FName,
    pub transform: FTransform,
    pub result: FTransform,
    pub space: ERigVMTransformSpace,
    pub b_propagate_to_children: bool,
    pub cached_bone: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetBoneRotation {
    pub bone: FName,
    pub rotation: FQuat,
    pub space: ERigVMTransformSpace,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_bone: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetBoneTransform {
    pub bone: FName,
    pub transform: FTransform,
    pub result: FTransform,
    pub space: ERigVMTransformSpace,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_bone: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetBoneTranslation {
    pub bone: FName,
    pub translation: FVector,
    pub space: ERigVMTransformSpace,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_bone: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlColor {
    pub control: FName,
    pub color: FLinearColor,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlColor {
    pub control: FName,
    pub color: FLinearColor,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlDrivenList {
    pub control: FName,
    pub driven: TArray<FRigElementKey>,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlDrivenList {
    pub control: FName,
    pub driven: TArray<FRigElementKey>,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetControlOffset {
    pub control: FName,
    pub offset: FTransform,
    pub space: ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlTranslationOffset {
    pub control: FName,
    pub offset: FVector,
    pub space: ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetControlRotationOffset {
    pub control: FName,
    pub offset: FQuat,
    pub space: ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlScaleOffset {
    pub control: FName,
    pub scale: FVector,
    pub space: ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetShapeTransform {
    pub control: FName,
    pub transform: FTransform,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetShapeTransform {
    pub control: FName,
    pub transform: FTransform,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlBool {
    pub control: FName,
    pub bool_value: bool,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(4))]
pub struct FRigUnit_SetMultiControlBool_Entry {
    pub control: FName,
    pub bool_value: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlBool {
    pub entries: TArray<FRigUnit_SetMultiControlBool_Entry>,
    pub cached_control_indices: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlFloat {
    pub control: FName,
    pub weight: f32,
    pub float_value: f32,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(4))]
pub struct FRigUnit_SetMultiControlFloat_Entry {
    pub control: FName,
    pub float_value: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlFloat {
    pub entries: TArray<FRigUnit_SetMultiControlFloat_Entry>,
    pub weight: f32,
    pub cached_control_indices: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlInteger {
    pub control: FName,
    pub weight: i32,
    pub integer_value: i32,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(4))]
pub struct FRigUnit_SetMultiControlInteger_Entry {
    pub control: FName,
    pub integer_value: i32,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlInteger {
    pub entries: TArray<FRigUnit_SetMultiControlInteger_Entry>,
    pub weight: f32,
    pub cached_control_indices: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlVector2D {
    pub control: FName,
    pub weight: f32,
    pub vector: FVector2D,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlVector2D_Entry {
    pub control: FName,
    pub vector: FVector2D,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlVector2D {
    pub entries: TArray<FRigUnit_SetMultiControlVector2D_Entry>,
    pub weight: f32,
    pub cached_control_indices: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlVector {
    pub control: FName,
    pub weight: f32,
    pub vector: FVector,
    pub space: ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlRotator {
    pub control: FName,
    pub weight: f32,
    pub rotator: FRotator,
    pub space: ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlRotator_Entry {
    pub control: FName,
    pub rotator: FRotator,
    pub space: ERigVMTransformSpace,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlRotator {
    pub entries: TArray<FRigUnit_SetMultiControlRotator_Entry>,
    pub weight: f32,
    pub cached_control_indices: TArray<FCachedRigElement>,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetControlTransform {
    pub control: FName,
    pub weight: f32,
    pub transform: FTransform,
    pub space: ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlVisibility {
    pub item: FRigElementKey,
    pub b_visible: bool,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlVisibility {
    pub item: FRigElementKey,
    pub pattern: FString,
    pub b_visible: bool,
    pub cached_control_indices: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetCurveValue {
    pub curve: FName,
    pub value: f32,
    pub cached_curve_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetRelativeBoneTransform {
    pub bone: FName,
    pub space: FName,
    pub transform: FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_bone: FCachedRigElement,
    pub cached_space_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetRelativeTransformForItem {
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub value: FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_child: FCachedRigElement,
    pub cached_parent: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetRelativeTranslationForItem {
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub value: FVector,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_child: FCachedRigElement,
    pub cached_parent: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetRelativeRotationForItem {
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub value: FQuat,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_child: FCachedRigElement,
    pub cached_parent: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetSpaceInitialTransform {
    pub space_name: FName,
    pub transform: FTransform,
    pub result: FTransform,
    pub space: ERigVMTransformSpace,
    pub cached_space_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetSpaceTransform {
    pub space: FName,
    pub weight: f32,
    pub transform: FTransform,
    pub space_type: ERigVMTransformSpace,
    pub cached_space_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetTransform {
    pub item: FRigElementKey,
    pub space: ERigVMTransformSpace,
    pub b_initial: bool,
    pub value: FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetTranslation {
    pub item: FRigElementKey,
    pub space: ERigVMTransformSpace,
    pub b_initial: bool,
    pub value: FVector,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetRotation {
    pub item: FRigElementKey,
    pub space: ERigVMTransformSpace,
    pub b_initial: bool,
    pub value: FQuat,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetScale {
    pub item: FRigElementKey,
    pub space: ERigVMTransformSpace,
    pub b_initial: bool,
    pub scale: FVector,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetTransformArray {
    pub items: FRigElementKeyCollection,
    pub space: ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<FTransform>,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetTransformItemArray {
    pub items: TArray<FRigElementKey>,
    pub space: ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<FTransform>,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_UnsetCurveValue {
    pub curve: FName,
    pub cached_curve_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_ToWorldSpace_Transform {
    pub value: FTransform,
    pub world: FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_ToRigSpace_Transform {
    pub value: FTransform,
    pub global: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_ToWorldSpace_Location {
    pub value: FVector,
    pub world: FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_ToRigSpace_Location {
    pub value: FVector,
    pub global: FVector,
}
#[repr(C, align(16))]
pub struct FRigUnit_ToWorldSpace_Rotation {
    pub value: FQuat,
    pub world: FQuat,
}
#[repr(C, align(16))]
pub struct FRigUnit_ToRigSpace_Rotation {
    pub value: FQuat,
    pub global: FQuat,
}
#[repr(C, align(4))]
pub struct FRigUnit_BoneHarmonics_BoneTarget {
    pub bone: FName,
    pub ratio: f32,
}
#[repr(C, align(4))]
pub struct FRigUnit_Harmonics_TargetItem {
    pub item: FRigElementKey,
    pub ratio: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_BoneHarmonics_WorkData {
    pub cached_items: TArray<FCachedRigElement>,
    pub wave_time: FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_BoneHarmonics {
    pub bones: TArray<FRigUnit_BoneHarmonics_BoneTarget>,
    pub wave_speed: FVector,
    pub wave_frequency: FVector,
    pub wave_amplitude: FVector,
    pub wave_offset: FVector,
    pub wave_noise: FVector,
    pub wave_ease: ERigVMAnimEasingType,
    pub wave_minimum: f32,
    pub wave_maximum: f32,
    pub rotation_order: EEulerRotationOrder,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_BoneHarmonics_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_ItemHarmonics {
    pub targets: TArray<FRigUnit_Harmonics_TargetItem>,
    pub wave_speed: FVector,
    pub wave_frequency: FVector,
    pub wave_amplitude: FVector,
    pub wave_offset: FVector,
    pub wave_noise: FVector,
    pub wave_ease: ERigVMAnimEasingType,
    pub wave_minimum: f32,
    pub wave_maximum: f32,
    pub rotation_order: EEulerRotationOrder,
    pub work_data: FRigUnit_BoneHarmonics_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_ChainHarmonics_Reach {
    pub b_enabled: bool,
    pub reach_target: FVector,
    pub reach_axis: FVector,
    pub reach_minimum: f32,
    pub reach_maximum: f32,
    pub reach_ease: ERigVMAnimEasingType,
}
#[repr(C, align(8))]
pub struct FRigUnit_ChainHarmonics_Wave {
    pub b_enabled: bool,
    pub wave_frequency: FVector,
    pub wave_amplitude: FVector,
    pub wave_offset: FVector,
    pub wave_noise: FVector,
    pub wave_minimum: f32,
    pub wave_maximum: f32,
    pub wave_ease: ERigVMAnimEasingType,
}
#[repr(C, align(8))]
pub struct FRigUnit_ChainHarmonics_Pendulum {
    pub b_enabled: bool,
    pub pendulum_stiffness: f32,
    pub pendulum_gravity: FVector,
    pub pendulum_blend: f32,
    pub pendulum_drag: f32,
    pub pendulum_minimum: f32,
    pub pendulum_maximum: f32,
    pub pendulum_ease: ERigVMAnimEasingType,
    pub unwind_axis: FVector,
    pub unwind_minimum: f32,
    pub unwind_maximum: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_ChainHarmonics_WorkData {
    pub time: FVector,
    pub items: TArray<FCachedRigElement>,
    pub ratio: TArray<f32>,
    pub local_tip: TArray<FVector>,
    pub pendulum_tip: TArray<FVector>,
    pub pendulum_position: TArray<FVector>,
    pub pendulum_velocity: TArray<FVector>,
    pub hierarchy_line: TArray<FVector>,
    pub velocity_lines: TArray<FVector>,
}
#[repr(C, align(16))]
pub struct FRigUnit_ChainHarmonics {
    pub chain_root: FName,
    pub speed: FVector,
    pub reach: FRigUnit_ChainHarmonics_Reach,
    pub wave: FRigUnit_ChainHarmonics_Wave,
    pub wave_curve: FRuntimeFloatCurve,
    pub pendulum: FRigUnit_ChainHarmonics_Pendulum,
    pub b_draw_debug: bool,
    pub draw_world_offset: FTransform,
    pub work_data: FRigUnit_ChainHarmonics_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_ChainHarmonicsPerItem {
    pub chain_root: FRigElementKey,
    pub speed: FVector,
    pub reach: FRigUnit_ChainHarmonics_Reach,
    pub wave: FRigUnit_ChainHarmonics_Wave,
    pub wave_curve: FRuntimeFloatCurve,
    pub pendulum: FRigUnit_ChainHarmonics_Pendulum,
    pub b_draw_debug: bool,
    pub draw_world_offset: FTransform,
    pub work_data: FRigUnit_ChainHarmonics_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_AimBone_Target {
    pub weight: f32,
    pub axis: FVector,
    pub target: FVector,
    pub kind: EControlRigVectorKind,
    pub space: FName,
}
#[repr(C, align(8))]
pub struct FRigUnit_AimItem_Target {
    pub weight: f32,
    pub axis: FVector,
    pub target: FVector,
    pub kind: EControlRigVectorKind,
    pub space: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_AimBone_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub world_offset: FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_AimBoneMath {
    pub input_transform: FTransform,
    pub primary: FRigUnit_AimItem_Target,
    pub secondary: FRigUnit_AimItem_Target,
    pub weight: f32,
    pub result: FTransform,
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    pub primary_cached_space: FCachedRigElement,
    pub secondary_cached_space: FCachedRigElement,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_AimBone {
    pub bone: FName,
    pub primary: FRigUnit_AimBone_Target,
    pub secondary: FRigUnit_AimBone_Target,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    pub cached_bone_index: FCachedRigElement,
    pub primary_cached_space: FCachedRigElement,
    pub secondary_cached_space: FCachedRigElement,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_AimItem {
    pub item: FRigElementKey,
    pub primary: FRigUnit_AimItem_Target,
    pub secondary: FRigUnit_AimItem_Target,
    pub weight: f32,
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    pub cached_item: FCachedRigElement,
    pub primary_cached_space: FCachedRigElement,
    pub secondary_cached_space: FCachedRigElement,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_AimConstraint_WorldUp {
    pub target: FVector,
    pub kind: EControlRigVectorKind,
    pub space: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_AimConstraint_AdvancedSettings {
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    pub rotation_order_for_filter: EEulerRotationOrder,
}
#[repr(C, align(16))]
pub struct FRigUnit_AimConstraintLocalSpaceOffset {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: FFilterOptionPerAxis,
    pub aim_axis: FVector,
    pub up_axis: FVector,
    pub world_up: FRigUnit_AimConstraint_WorldUp,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_AimConstraint_AdvancedSettings,
    pub weight: f32,
    pub world_up_space_cache: FCachedRigElement,
    pub child_cache: FCachedRigElement,
    pub parent_caches: TArray<FCachedRigElement>,
    pub b_is_initialized: bool,
}
#[repr(C, align(4))]
pub struct FConstraintParent {
    pub item: FRigElementKey,
    pub weight: f32,
}
#[repr(C, align(4))]
pub struct FRigUnit_CCDIK_RotationLimit {
    pub bone: FName,
    pub limit: f32,
}
#[repr(C, align(4))]
pub struct FRigUnit_CCDIK_RotationLimitPerItem {
    pub item: FRigElementKey,
    pub limit: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_CCDIK_WorkData {
    pub chain: TArray<FCCDIKChainLink>,
    pub cached_items: TArray<FCachedRigElement>,
    pub rotation_limit_index: TArray<i32>,
    pub rotation_limits_per_item: TArray<f32>,
    pub cached_effector: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_CCDIK {
    pub start_bone: FName,
    pub effector_bone: FName,
    pub effector_transform: FTransform,
    pub precision: f32,
    pub weight: f32,
    pub max_iterations: i32,
    pub b_start_from_tail: bool,
    pub base_rotation_limit: f32,
    pub rotation_limits: TArray<FRigUnit_CCDIK_RotationLimit>,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_CCDIK_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_CCDIKPerItem {
    pub items: FRigElementKeyCollection,
    pub effector_transform: FTransform,
    pub precision: f32,
    pub weight: f32,
    pub max_iterations: i32,
    pub b_start_from_tail: bool,
    pub base_rotation_limit: f32,
    pub rotation_limits: TArray<FRigUnit_CCDIK_RotationLimitPerItem>,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_CCDIK_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_CCDIKItemArray {
    pub items: TArray<FRigElementKey>,
    pub effector_transform: FTransform,
    pub precision: f32,
    pub weight: f32,
    pub max_iterations: i32,
    pub b_start_from_tail: bool,
    pub base_rotation_limit: f32,
    pub rotation_limits: TArray<FRigUnit_CCDIK_RotationLimitPerItem>,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_CCDIK_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_ChainInfo_Segment {
    pub start_item: FCachedRigElement,
    pub start_item_index: i32,
    pub end_item: FCachedRigElement,
    pub end_item_index: i32,
    pub initial_length: f32,
    pub initial_cum_length: f32,
    pub length: f32,
    pub cum_length: f32,
}
#[repr(C, align(4))]
pub struct FRigUnit_ChainInfo_SegmentInfo {
    pub segment_index: i32,
    pub segment_length: f32,
    pub segment_param: f32,
    pub segment_param_length: f32,
    pub segment_start_item: FRigElementKey,
    pub segment_start_item_index: i32,
    pub segment_end_item: FRigElementKey,
    pub segment_end_item_index: i32,
    pub segment_stretch_factor: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_ChainInfo {
    pub items: TArray<FRigElementKey>,
    pub param: f32,
    pub b_calculate_stretch: bool,
    pub b_initial: bool,
    pub b_debug: bool,
    pub debug_scale: f32,
    pub interpolated_transform: FTransform,
    pub chain_length: f32,
    pub param_length: f32,
    pub chain_stretch_factor: f32,
    pub segment_info: FRigUnit_ChainInfo_SegmentInfo,
    pub cached_elements: TArray<FCachedRigElement>,
}
#[repr(C, align(16))]
pub struct FRigUnit_DistributeRotation_Rotation {
    pub rotation: FQuat,
    pub ratio: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotation_WorkData {
    pub cached_items: TArray<FCachedRigElement>,
    pub item_rotation_a: TArray<i32>,
    pub item_rotation_b: TArray<i32>,
    pub item_rotation_t: TArray<f32>,
    pub item_local_transforms: TArray<FTransform>,
}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotation {
    pub start_bone: FName,
    pub end_bone: FName,
    pub rotations: TArray<FRigUnit_DistributeRotation_Rotation>,
    pub rotation_ease_type: ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_DistributeRotation_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotationForCollection {
    pub items: FRigElementKeyCollection,
    pub rotations: TArray<FRigUnit_DistributeRotation_Rotation>,
    pub rotation_ease_type: ERigVMAnimEasingType,
    pub weight: f32,
    pub work_data: FRigUnit_DistributeRotation_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotationForItemArray {
    pub items: TArray<FRigElementKey>,
    pub rotations: TArray<FRigUnit_DistributeRotation_Rotation>,
    pub rotation_ease_type: ERigVMAnimEasingType,
    pub weight: f32,
    pub work_data: FRigUnit_DistributeRotation_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_FABRIK_WorkData {
    pub chain: TArray<FFABRIKChainLink>,
    pub cached_items: TArray<FCachedRigElement>,
    pub cached_effector: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_FABRIK {
    pub start_bone: FName,
    pub effector_bone: FName,
    pub effector_transform: FTransform,
    pub precision: f32,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    pub work_data: FRigUnit_FABRIK_WorkData,
    pub b_set_effector_transform: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_FABRIKPerItem {
    pub items: FRigElementKeyCollection,
    pub effector_transform: FTransform,
    pub precision: f32,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    pub work_data: FRigUnit_FABRIK_WorkData,
    pub b_set_effector_transform: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_FABRIKItemArray {
    pub items: TArray<FRigElementKey>,
    pub effector_transform: FTransform,
    pub precision: f32,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    pub work_data: FRigUnit_FABRIK_WorkData,
    pub b_set_effector_transform: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurve_Rotation {
    pub rotation: FQuat,
    pub ratio: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurve_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub curve_color: FLinearColor,
    pub segments_color: FLinearColor,
    pub world_offset: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_FitChainToCurve_WorkData {
    pub chain_length: f32,
    pub item_positions: TArray<FVector>,
    pub item_segments: TArray<f32>,
    pub curve_positions: TArray<FVector>,
    pub curve_segments: TArray<f32>,
    pub cached_items: TArray<FCachedRigElement>,
    pub item_rotation_a: TArray<i32>,
    pub item_rotation_b: TArray<i32>,
    pub item_rotation_t: TArray<f32>,
    pub item_local_transforms: TArray<FTransform>,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurve {
    pub start_bone: FName,
    pub end_bone: FName,
    pub bezier: FRigVMFourPointBezier,
    pub alignment: EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: FVector,
    pub secondary_axis: FVector,
    pub pole_vector_position: FVector,
    pub rotations: TArray<FRigUnit_FitChainToCurve_Rotation>,
    pub rotation_ease_type: ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_FitChainToCurve_DebugSettings,
    pub work_data: FRigUnit_FitChainToCurve_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurvePerItem {
    pub items: FRigElementKeyCollection,
    pub bezier: FRigVMFourPointBezier,
    pub alignment: EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: FVector,
    pub secondary_axis: FVector,
    pub pole_vector_position: FVector,
    pub rotations: TArray<FRigUnit_FitChainToCurve_Rotation>,
    pub rotation_ease_type: ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_FitChainToCurve_DebugSettings,
    pub work_data: FRigUnit_FitChainToCurve_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurveItemArray {
    pub items: TArray<FRigElementKey>,
    pub bezier: FRigVMFourPointBezier,
    pub alignment: EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: FVector,
    pub secondary_axis: FVector,
    pub pole_vector_position: FVector,
    pub rotations: TArray<FRigUnit_FitChainToCurve_Rotation>,
    pub rotation_ease_type: ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_FitChainToCurve_DebugSettings,
    pub work_data: FRigUnit_FitChainToCurve_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_ModifyBoneTransforms_PerBone {
    pub bone: FName,
    pub transform: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_ModifyTransforms_WorkData {
    pub cached_items: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_ModifyBoneTransforms_WorkData {}
#[repr(C, align(8))]
pub struct FRigUnit_ModifyBoneTransforms {
    pub bone_to_modify: TArray<FRigUnit_ModifyBoneTransforms_PerBone>,
    pub weight: f32,
    pub weight_minimum: f32,
    pub weight_maximum: f32,
    pub mode: EControlRigModifyBoneMode,
    pub work_data: FRigUnit_ModifyBoneTransforms_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_ModifyTransforms_PerItem {
    pub item: FRigElementKey,
    pub transform: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_ModifyTransforms {
    pub item_to_modify: TArray<FRigUnit_ModifyTransforms_PerItem>,
    pub weight: f32,
    pub weight_minimum: f32,
    pub weight_maximum: f32,
    pub mode: EControlRigModifyBoneMode,
    pub work_data: FRigUnit_ModifyTransforms_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_MultiFABRIK_WorkData {}
#[repr(C, align(8))]
pub struct FRigUnit_MultiFABRIK_EndEffector {
    pub bone: FName,
    pub location: FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_MultiFABRIK {
    pub root_bone: FName,
    pub effectors: TArray<FRigUnit_MultiFABRIK_EndEffector>,
    pub precision: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    pub work_data: FRigUnit_MultiFABRIK_WorkData,
    pub b_is_initialized: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_SlideChain_WorkData {
    pub chain_length: f32,
    pub item_segments: TArray<f32>,
    pub cached_items: TArray<FCachedRigElement>,
    pub transforms: TArray<FTransform>,
    pub blended_transforms: TArray<FTransform>,
}
#[repr(C, align(8))]
pub struct FRigUnit_SlideChain {
    pub start_bone: FName,
    pub end_bone: FName,
    pub slide_amount: f32,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_SlideChain_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_SlideChainPerItem {
    pub items: FRigElementKeyCollection,
    pub slide_amount: f32,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_SlideChain_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_SlideChainItemArray {
    pub items: TArray<FRigElementKey>,
    pub slide_amount: f32,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_SlideChain_WorkData,
}
#[repr(C, align(4))]
pub struct FRegionScaleFactors {
    pub positive_width: f32,
    pub negative_width: f32,
    pub positive_height: f32,
    pub negative_height: f32,
}
#[repr(C, align(4))]
pub struct FSphericalRegion {}
#[repr(C, align(4))]
pub struct FSphericalPoseReaderDebugSettings {
    pub b_draw_debug: bool,
    pub b_draw2_d: bool,
    pub b_draw_local_axes: bool,
    pub debug_scale: f32,
    pub debug_segments: i32,
    pub debug_thickness: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_SphericalPoseReader {
    pub output_param: f32,
    pub driver_item: FRigElementKey,
    pub driver_axis: FVector,
    pub rotation_offset: FVector,
    pub active_region_size: f32,
    pub active_region_scale_factors: FRegionScaleFactors,
    pub falloff_size: f32,
    pub falloff_region_scale_factors: FRegionScaleFactors,
    pub flip_width_scaling: bool,
    pub flip_height_scaling: bool,
    pub optional_parent_item: FRigElementKey,
    pub debug: FSphericalPoseReaderDebugSettings,
    pub inner_region: FSphericalRegion,
    pub outer_region: FSphericalRegion,
    pub driver_normal: FVector,
    pub driver2_d: FVector,
    pub driver_cache: FCachedRigElement,
    pub optional_parent_cache: FCachedRigElement,
    pub local_driver_transform_init: FTransform,
    pub cached_rotation_offset: FVector,
    pub b_cached_init_transforms: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_SpringIK_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub color: FLinearColor,
    pub world_offset: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_SpringIK_WorkData {
    pub cached_bones: TArray<FCachedRigElement>,
    pub cached_pole_vector: FCachedRigElement,
    pub transforms: TArray<FTransform>,
    pub simulation: FCRSimPointContainer,
}
#[repr(C, align(16))]
pub struct FRigUnit_SpringIK {
    pub start_bone: FName,
    pub end_bone: FName,
    pub hierarchy_strength: f32,
    pub effector_strength: f32,
    pub effector_ratio: f32,
    pub root_strength: f32,
    pub root_ratio: f32,
    pub damping: f32,
    pub pole_vector: FVector,
    pub b_flip_pole_plane: bool,
    pub pole_vector_kind: EControlRigVectorKind,
    pub pole_vector_space: FName,
    pub primary_axis: FVector,
    pub secondary_axis: FVector,
    pub b_live_simulation: bool,
    pub iterations: i32,
    pub b_limit_local_position: bool,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_SpringIK_DebugSettings,
    pub work_data: FRigUnit_SpringIK_WorkData,
}
#[repr(C, align(16))]
pub struct FConstraintTarget {
    pub transform: FTransform,
    pub weight: f32,
    pub b_maintain_offset: bool,
    pub filter: FTransformFilter,
}
#[repr(C, align(8))]
pub struct FRigUnit_TransformConstraint_WorkData {
    pub constraint_data: TArray<FConstraintData>,
    pub constraint_data_to_targets: TMap<i32, i32>,
}
#[repr(C, align(16))]
pub struct FRigUnit_TransformConstraint {
    pub bone: FName,
    pub base_transform_space: ETransformSpaceMode,
    pub base_transform: FTransform,
    pub base_bone: FName,
    pub targets: TArray<FConstraintTarget>,
    pub b_use_initial_transforms: bool,
    pub work_data: FRigUnit_TransformConstraint_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_TransformConstraintPerItem {
    pub item: FRigElementKey,
    pub base_transform_space: ETransformSpaceMode,
    pub base_transform: FTransform,
    pub base_item: FRigElementKey,
    pub targets: TArray<FConstraintTarget>,
    pub b_use_initial_transforms: bool,
    pub work_data: FRigUnit_TransformConstraint_WorkData,
}
#[repr(C, align(1))]
pub struct FRigUnit_ParentConstraint_AdvancedSettings {
    pub interpolation_type: EConstraintInterpType,
    pub rotation_order_for_filter: EEulerRotationOrder,
}
#[repr(C, align(8))]
pub struct FRigUnit_ParentConstraint {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: FTransformFilter,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_ParentConstraint_AdvancedSettings,
    pub weight: f32,
    pub child_cache: FCachedRigElement,
    pub parent_caches: TArray<FCachedRigElement>,
}
#[repr(C, align(1))]
pub struct FRigUnit_ParentConstraintMath_AdvancedSettings {
    pub interpolation_type: EConstraintInterpType,
}
#[repr(C, align(16))]
pub struct FRigUnit_ParentConstraintMath {
    pub input: FTransform,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_ParentConstraintMath_AdvancedSettings,
    pub output: FTransform,
    pub parent_caches: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_PositionConstraint {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_PositionConstraintLocalSpaceOffset {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
    pub child_cache: FCachedRigElement,
    pub parent_caches: TArray<FCachedRigElement>,
}
#[repr(C, align(1))]
pub struct FRigUnit_RotationConstraint_AdvancedSettings {
    pub interpolation_type: EConstraintInterpType,
    pub rotation_order_for_filter: EEulerRotationOrder,
}
#[repr(C, align(8))]
pub struct FRigUnit_RotationConstraint {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_RotationConstraint_AdvancedSettings,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_RotationConstraintLocalSpaceOffset {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_RotationConstraint_AdvancedSettings,
    pub weight: f32,
    pub child_cache: FCachedRigElement,
    pub parent_caches: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_ScaleConstraint {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_ScaleConstraintLocalSpaceOffset {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
    pub child_cache: FCachedRigElement,
    pub parent_caches: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_TwistBones_WorkData {
    pub cached_items: TArray<FCachedRigElement>,
    pub item_ratios: TArray<f32>,
    pub item_transforms: TArray<FTransform>,
}
#[repr(C, align(8))]
pub struct FRigUnit_TwistBones {
    pub start_bone: FName,
    pub end_bone: FName,
    pub twist_axis: FVector,
    pub pole_axis: FVector,
    pub twist_ease_type: ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_TwistBones_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_TwistBonesPerItem {
    pub items: FRigElementKeyCollection,
    pub twist_axis: FVector,
    pub pole_axis: FVector,
    pub twist_ease_type: ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_TwistBones_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimple_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub world_offset: FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimple {
    pub bone_a: FName,
    pub bone_b: FName,
    pub effector_bone: FName,
    pub effector: FTransform,
    pub primary_axis: FVector,
    pub secondary_axis: FVector,
    pub secondary_axis_weight: f32,
    pub pole_vector: FVector,
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
    pub cached_bone_a_index: FCachedRigElement,
    pub cached_bone_b_index: FCachedRigElement,
    pub cached_effector_bone_index: FCachedRigElement,
    pub cached_pole_vector_space_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimplePerItem {
    pub item_a: FRigElementKey,
    pub item_b: FRigElementKey,
    pub effector_item: FRigElementKey,
    pub effector: FTransform,
    pub primary_axis: FVector,
    pub secondary_axis: FVector,
    pub secondary_axis_weight: f32,
    pub pole_vector: FVector,
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
    pub cached_item_a_index: FCachedRigElement,
    pub cached_item_b_index: FCachedRigElement,
    pub cached_effector_item_index: FCachedRigElement,
    pub cached_pole_vector_space_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_TwoBoneIKSimpleVectors {
    pub root: FVector,
    pub pole_vector: FVector,
    pub effector: FVector,
    pub b_enable_stretch: bool,
    pub stretch_start_ratio: f32,
    pub stretch_maximum_ratio: f32,
    pub bone_a_length: f32,
    pub bone_b_length: f32,
    pub elbow: FVector,
}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimpleTransforms {
    pub root: FTransform,
    pub pole_vector: FVector,
    pub effector: FTransform,
    pub primary_axis: FVector,
    pub secondary_axis: FVector,
    pub secondary_axis_weight: f32,
    pub b_enable_stretch: bool,
    pub stretch_start_ratio: f32,
    pub stretch_maximum_ratio: f32,
    pub bone_a_length: f32,
    pub bone_b_length: f32,
    pub elbow: FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetCandidates {
    pub connector: FRigElementKey,
    pub candidates: TArray<FRigElementKey>,
}
#[repr(C, align(8))]
pub struct FRigUnit_DiscardMatches {
    pub excluded: TArray<FRigElementKey>,
    pub message: FString,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetDefaultMatch {
    pub default: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConnectorExecution {
    pub execute_context: FControlRigExecuteContext,
}
#[repr(C, align(16))]
pub struct FRigUnit_PointSimulation_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub collision_scale: f32,
    pub b_draw_points_as_spheres: bool,
    pub color: FLinearColor,
    pub world_offset: FTransform,
}
#[repr(C, align(4))]
pub struct FRigUnit_PointSimulation_BoneTarget {
    pub bone: FName,
    pub translation_point: i32,
    pub primary_aim_point: i32,
    pub secondary_aim_point: i32,
}
#[repr(C, align(8))]
pub struct FRigUnit_PointSimulation_WorkData {
    pub simulation: FCRSimPointContainer,
    pub bone_indices: TArray<FCachedRigElement>,
}
#[repr(C, align(16))]
pub struct FRigUnit_PointSimulation {
    pub points: TArray<FRigVMSimPoint>,
    pub links: TArray<FCRSimLinearSpring>,
    pub forces: TArray<FCRSimPointForce>,
    pub collision_volumes: TArray<FCRSimSoftCollision>,
    pub simulated_steps_per_second: f32,
    pub integrator_type: ERigVMSimPointIntegrateType,
    pub verlet_blend: f32,
    pub bone_targets: TArray<FRigUnit_PointSimulation_BoneTarget>,
    pub b_limit_local_position: bool,
    pub b_propagate_to_children: bool,
    pub primary_aim_axis: FVector,
    pub secondary_aim_axis: FVector,
    pub debug_settings: FRigUnit_PointSimulation_DebugSettings,
    pub bezier: FRigVMFourPointBezier,
    pub work_data: FRigUnit_PointSimulation_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterp {
    pub current: f32,
    pub target: f32,
    pub stiffness: f32,
    pub critical_damping: f32,
    pub mass: f32,
    pub result: f32,
    pub spring_state: FFloatSpringState,
}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterpVector {
    pub current: FVector,
    pub target: FVector,
    pub stiffness: f32,
    pub critical_damping: f32,
    pub mass: f32,
    pub result: FVector,
    pub spring_state: FVectorSpringState,
}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterpV2 {
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
    pub simulated_result: f32,
    pub spring_state: FFloatSpringState,
}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterpVectorV2 {
    pub target: FVector,
    pub strength: f32,
    pub critical_damping: f32,
    pub force: FVector,
    pub b_use_current_input: bool,
    pub current: FVector,
    pub target_velocity_amount: f32,
    pub b_initialize_from_target: bool,
    pub result: FVector,
    pub velocity: FVector,
    pub simulated_result: FVector,
    pub spring_state: FVectorSpringState,
}
#[repr(C, align(16))]
pub struct FRigUnit_SpringInterpQuaternionV2 {
    pub target: FQuat,
    pub strength: f32,
    pub critical_damping: f32,
    pub torque: FVector,
    pub b_use_current_input: bool,
    pub current: FQuat,
    pub target_velocity_amount: f32,
    pub b_initialize_from_target: bool,
    pub result: FQuat,
    pub angular_velocity: FVector,
    pub simulated_result: FQuat,
    pub spring_state: FQuaternionSpringState,
}
pub struct UAnimNodeControlRigLibrary {}
pub struct UTransformableControlHandle {
    pub control_rig: TSoftObjectPtr<UControlRig>,
    pub control_name: FName,
}
pub struct UControlRig {
    pub override_assets: TArray<UPtr<UControlRigOverrideAsset>>,
    pub execution_type: ERigExecutionType,
    pub hierarchy_settings: FRigHierarchySettings,
    pub control_customizations: TMap<FRigElementKey, FRigControlElementCustomization>,
    pub dynamic_hierarchy: UPtr<URigHierarchy>,
    pub gizmo_library_deprecated: TSoftObjectPtr<UControlRigShapeLibrary>,
    pub shape_libraries: TArray<TSoftObjectPtr<UControlRigShapeLibrary>>,
    pub shape_library_name_map: TMap<FString, FString>,
    pub input_properties_deprecated: TMap<FName, FCachedPropertyPath>,
    pub output_properties_deprecated: TMap<FName, FCachedPropertyPath>,
    pub rig_vm_extended_execute_context: FRigVMExtendedExecuteContext,
    pub data_source_registry: UPtr<UAnimationDataSourceRegistry>,
    pub influences: FRigInfluenceMapPerEvent,
    pub external_variable_data_asset_links: TMap<FName, UPtr<UDataAssetLink>>,
    pub on_control_selected_bp: FControlRig_OnControlSelected_BP,
    pub b_controls_visible: bool,
    pub b_is_additive: bool,
    pub rig_module_settings: FRigModuleSettings,
    pub rig_module_prefix: FString,
    pub interaction_rig_deprecated: UPtr<UControlRig>,
    pub interaction_rig_class_deprecated: TSubclassOf<UControlRig>,
}
pub struct UControlRigAnimInstance {}
pub struct UControlRigShapeLibraryLink {
    pub shape_library: TSoftObjectPtr<UControlRigShapeLibrary>,
    pub shape_names: TArray<FName>,
    pub shape_library_cached: UPtr<UControlRigShapeLibrary>,
}
pub struct UControlRigBlueprintGeneratedClass {
    pub preview_skeletal_mesh: FSoftObjectPath,
    pub b_exposes_animatable_controls: bool,
    pub b_allow_multiple_instances: bool,
    pub control_rig_type: EControlRigType,
    pub item_type_display_name: FName,
    pub rig_module_settings: FRigModuleSettings,
    pub module_reference_data: TArray<FModuleReferenceData>,
    pub custom_thumbnail: FString,
    pub b_supports_inversion: bool,
    pub b_supports_controls: bool,
}
pub struct UControlRigComponent {
    pub control_rig_class: TSubclassOf<UControlRig>,
    pub on_pre_initialize_delegate: FControlRigComponent_OnPreInitializeDelegate,
    pub on_post_initialize_delegate: FControlRigComponent_OnPostInitializeDelegate,
    pub on_pre_construction_delegate: FControlRigComponent_OnPreConstructionDelegate,
    pub on_post_construction_delegate: FControlRigComponent_OnPostConstructionDelegate,
    pub on_pre_forwards_solve_delegate: FControlRigComponent_OnPreForwardsSolveDelegate,
    pub on_post_forwards_solve_delegate: FControlRigComponent_OnPostForwardsSolveDelegate,
    pub user_defined_elements: TArray<FControlRigComponentMappedElement>,
    pub mapped_elements: TArray<FControlRigComponentMappedElement>,
    pub b_enable_lazy_evaluation: bool,
    pub lazy_evaluation_position_threshold: f32,
    pub lazy_evaluation_rotation_threshold: f32,
    pub lazy_evaluation_scale_threshold: f32,
    pub b_reset_transform_before_tick: bool,
    pub b_reset_initials_before_construction: bool,
    pub b_update_rig_on_tick: bool,
    pub b_update_in_editor: bool,
    pub b_draw_bones: bool,
    pub b_show_debug_drawing: bool,
    pub control_rig: UPtr<UControlRig>,
}
pub struct AControlRigControlActor {
    pub actor_to_track: UPtr<AActor>,
    pub control_rig_class: TSubclassOf<UControlRig>,
    pub b_refresh_on_tick: bool,
    pub b_is_selectable: bool,
    pub material_override: UPtr<UMaterialInterface>,
    pub color_parameter: FString,
    pub b_cast_shadows: bool,
    pub actor_root_component: UPtr<USceneComponent>,
    pub control_rig: TSoftObjectPtr<UControlRig>,
    pub control_names: TArray<FName>,
    pub shape_transforms: TArray<FTransform>,
    pub components: TArray<UPtr<UStaticMeshComponent>>,
    pub materials: TArray<UPtr<UMaterialInstanceDynamic>>,
    pub color_parameter_name: FName,
}
pub struct AControlRigShapeActor {
    pub actor_root_component: UPtr<USceneComponent>,
    pub static_mesh_component: UPtr<UStaticMeshComponent>,
    pub control_rig_index: u32,
    pub control_rig: TWeakObjectPtr<UControlRig>,
    pub control_name: FName,
    pub shape_name: FName,
    pub color_parameter_name: FName,
    pub cached_index: FCachedRigElement,
    pub flags_1360: u8,
}
pub struct UControlRigShapeLibrary {
    pub default_shape: FControlRigShapeDefinition,
    pub default_material: TSoftObjectPtr<UMaterial>,
    pub x_ray_material: TSoftObjectPtr<UMaterial>,
    pub material_color_parameter: FName,
    pub shapes: TArray<FControlRigShapeDefinition>,
}
pub struct UControlRigOverrideAsset {
    pub overrides: FControlRigOverrideContainer,
}
pub struct UControlRigReplay {
    pub description: FText,
    pub control_rig_object_path: FSoftObjectPath,
    pub preview_skeletal_mesh_object_path: FSoftObjectPath,
    pub input_tracks: FControlRigReplayTracks,
    pub output_tracks: FControlRigReplayTracks,
    pub tolerance: f64,
    pub b_validate_hierarchy_topology: bool,
    pub b_validate_pose: bool,
    pub b_validate_metadata: bool,
    pub b_validate_variables: bool,
    pub frames_to_skip: TArray<i32>,
    pub enable_test: bool,
}
pub struct UControlRigTestData {
    pub initial: FControlRigTestDataFrame,
    pub input_frames: TArray<FControlRigTestDataFrame>,
    pub output_frames: TArray<FControlRigTestDataFrame>,
    pub event_queue: TArray<FName>,
}
pub struct UControlRigValidator {
    pub passes: TArray<UPtr<UControlRigValidationPass>>,
}
pub struct UControlRigValidationPass {}
pub struct UModularRig {
    pub modules: TArray<FRigModuleInstance>,
    pub modular_rig_settings: FModularRigSettings,
    pub modular_rig_model: FModularRigModel,
    pub execution_queue: TArray<FRigModuleExecutionElement>,
}
pub struct UModularRigController {}
pub struct UModularRigRuleManager {}
pub struct UAdditiveControlRig {}
pub struct UFKControlRig {
    pub is_control_active: TArray<bool>,
    pub apply_mode: EControlRigFKRigExecuteMode,
}
pub struct URigHierarchy {
    pub modified_event_dynamic: FRigHierarchy_ModifiedEventDynamic,
    pub topology_version: u32,
    pub parent_weight_version: u32,
    pub metadata_version: u32,
    pub metadata_tag_version: u16,
    pub b_enable_dirty_propagation: bool,
    pub transform_stack_index: i32,
    pub hierarchy_controller: UPtr<URigHierarchyController>,
    pub rule_manager: UPtr<UModularRigRuleManager>,
    pub hierarchy_for_cache_validation: UPtr<URigHierarchy>,
}
pub struct URigHierarchyProvider {}
pub struct IRigHierarchyProvider {}
pub struct URigHierarchyController {
    pub b_report_warnings_and_errors: bool,
}
pub struct UControlRigLayerInstance {}
pub struct UMovieSceneControlRigParameterSection {
    pub control_rig: UPtr<UControlRig>,
    pub control_rig_class: TSubclassOf<UControlRig>,
    pub controls_mask: TArray<bool>,
    pub control_name_mask: TSet<FName>,
    pub transform_mask: FMovieSceneTransformMask,
    pub weight: FMovieSceneFloatChannel,
    pub control_channel_map: TMap<FName, FChannelMapInfo>,
    pub enum_parameter_names_and_curves: TArray<FEnumParameterNameAndCurve>,
    pub integer_parameter_names_and_curves: TArray<FIntegerParameterNameAndCurve>,
    pub space_channels: TArray<FSpaceControlNameAndChannel>,
    pub constraints_channels: TArray<FConstraintAndActiveChannel>,
    pub override_assets: TArray<TSoftObjectPtr<UControlRigOverrideAsset>>,
}
pub struct UMovieSceneControlRigParameterTrack {
    pub section_to_key_per_control: TMap<FName, TWeakObjectPtr<UMovieSceneSection>>,
    pub control_rig: UPtr<UControlRig>,
    pub section_to_key: UPtr<UMovieSceneSection>,
    pub sections: TArray<UPtr<UMovieSceneSection>>,
    pub track_name: FName,
    pub controls_rotation_order: TMap<FName, FControlRotationOrder>,
    pub priority_order: i32,
    pub control_rig_settings_overrides: FInstancedPropertyBag,
    pub game_world_control_rigs: TMap<TWeakObjectPtr<UWorld>, UPtr<UControlRig>>,
}
pub struct UMovieSceneControlRigParameterEvaluatorSystem {
    pub double_blender_system: UPtr<UMovieScenePiecewiseDoubleBlenderSystem>,
}
pub struct UControlRigSettings {
    pub default_shape_library: TSoftObjectPtr<UControlRigShapeLibrary>,
    pub default_root_module: FSoftObjectPath,
}
pub struct UControlRigEditorSettings {
    pub b_reset_controls_on_compile: bool,
    pub b_reset_controls_on_pin_value_interaction: bool,
    pub b_reset_pose_when_toggling_event_queue: bool,
    pub b_enable_undo_for_pose_interaction: bool,
    pub b_reset_control_transforms_on_compile: bool,
    pub rig_unit_pin_expansion: TMap<FString, FControlRigSettingsPerPinBool>,
    pub b_enable_flashlight_in_dependency_viewer: bool,
    pub construction_event_border_color: FLinearColor,
    pub backwards_solve_border_color: FLinearColor,
    pub backwards_and_forwards_border_color: FLinearColor,
    pub b_show_schematic_view_in_modular_rig: bool,
    pub b_show_stacked_hierarchy: bool,
    pub max_stack_size: i32,
    pub b_left_mouse_drag_does_marquee: bool,
    pub b_arrange_by_modules: bool,
    pub b_flatten_modules: bool,
    pub outliner_multi_rig_display_mode: EMultiRigTreeDisplayMode,
    pub element_name_display_mode: EElementNameDisplayMode,
    pub b_focus_on_selection: bool,
}
pub struct UControlRigPoseAsset {
    pub pose: FControlRigControlPose,
}
pub struct UControlRigPoseMirrorSettings {
    pub right_side: FString,
    pub left_side: FString,
    pub mirror_axis: EAxis,
    pub axis_to_flip: EAxis,
    pub mirror_match_tolerance: f64,
}
pub struct UControlRigPoseProjectSettings {
    pub root_save_dirs: TArray<FDirectoryPath>,
}
pub struct UControlRigWorkflowOptions {
    pub hierarchy: UPtr<URigHierarchy>,
    pub selection: TArray<FRigElementKey>,
}
pub struct UControlRigTransformWorkflowOptions {
    pub transform_type: ERigTransformType,
}
pub struct UControlRigNumericalValidationPass {
    pub b_check_controls: bool,
    pub b_check_bones: bool,
    pub b_check_curves: bool,
    pub translation_precision: f32,
    pub rotation_precision: f32,
    pub scale_precision: f32,
    pub curve_precision: f32,
    pub event_name_a: FName,
    pub event_name_b: FName,
    pub pose: FRigPose,
}
