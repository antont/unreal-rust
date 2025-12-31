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
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub current: crate::bindings::core_u_object::FVector,
    pub initial: crate::bindings::core_u_object::FVector,
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
    pub shape_color: crate::bindings::core_u_object::FLinearColor,
    pub b_is_transient_control: bool,
    pub control_enum: UPtr<crate::bindings::core_u_object::UEnum>,
    pub customization: FRigControlElementCustomization,
    pub driven_controls: TArray<FRigElementKey>,
    pub b_group_with_parent_control: bool,
    pub b_restrict_space_switching: bool,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
    pub preferred_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub b_use_preferred_rotation_order: bool,
    pub b_animatable_deprecated: bool,
    pub b_shape_enabled_deprecated: bool,
    pub shape_transform: crate::bindings::core_u_object::FTransform,
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
    pub storage_deprecated: crate::bindings::core_u_object::FTransform,
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
    pub icon: crate::bindings::core_u_object::FSoftObjectPath,
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
    pub bone_filter: crate::bindings::engine::FInputBlendPose,
    pub weight: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub operand: crate::bindings::movie_scene::FMovieSceneEvaluationOperand,
}
#[repr(C, align(8))]
pub struct FRigUnit {}
#[repr(C, align(8))]
pub struct FRigUnit_DebugBase {
    pub debug_draw_settings: crate::bindings::rig_vm::FRigVMDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FRigUnitMutable {
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_DebugBaseMutable {
    pub debug_draw_settings: crate::bindings::rig_vm::FRigVMDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FRigUnit_HighlevelBase {}
#[repr(C, align(8))]
pub struct FRigUnit_HighlevelBaseMutable {}
#[repr(C, align(8))]
pub struct FStructReference {}
#[repr(C, align(8))]
pub struct FAnimNode_ControlRigBase {
    pub source: crate::bindings::engine::FPoseLink,
    pub b_reset_input_pose_to_initial: bool,
    pub b_transfer_input_pose: bool,
    pub b_transfer_input_curves: bool,
    pub b_transfer_pose_in_global_space: bool,
    pub input_bones_to_transfer: TArray<crate::bindings::engine::FBoneReference>,
    pub output_bones_to_transfer: TArray<crate::bindings::engine::FBoneReference>,
    pub asset_user_data: TArray<UPtr<crate::bindings::engine::UAssetUserData>>,
    pub node_mapping_container: TWeakObjectPtr<
        crate::bindings::engine::UNodeMappingContainer,
    >,
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
    pub control_rig_per_class: TMap<
        TSubclassOf<crate::bindings::core_u_object::UObject>,
        UPtr<UControlRig>,
    >,
    pub alpha: f32,
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub flags_797: u8,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
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
    pub component_reference: crate::bindings::engine::FSoftComponentReference,
    pub transform_index: i32,
    pub transform_name: FName,
    pub element_type: ERigElementType,
    pub element_name: FName,
    pub direction: EControlRigComponentMapDirection,
    pub offset: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub space: EControlRigComponentSpace,
    pub scene_component: UPtr<crate::bindings::engine::USceneComponent>,
    pub element_index: i32,
    pub sub_index: i32,
}
#[repr(C, align(8))]
pub struct FControlRigComponentMappedComponent {
    pub component: UPtr<crate::bindings::engine::USceneComponent>,
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
    pub static_mesh: TSoftObjectPtr<crate::bindings::engine::UStaticMesh>,
    pub transform: crate::bindings::core_u_object::FTransform,
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
    pub global_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub preferred_euler_angle: crate::bindings::core_u_object::FVector,
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
    pub data_a: crate::bindings::core_u_object::FVector,
    pub data_b: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FCRSimPointContainer {
    pub points: TArray<crate::bindings::rig_vm::FRigVMSimPoint>,
    pub springs: TArray<FCRSimLinearSpring>,
    pub forces: TArray<FCRSimPointForce>,
    pub collision_volumes: TArray<FCRSimSoftCollision>,
    pub constraints: TArray<FCRSimPointConstraint>,
    pub previous_step: TArray<crate::bindings::rig_vm::FRigVMSimPoint>,
}
#[repr(C, align(16))]
pub struct FCRSimSoftCollision {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub shape_type: ECRSimSoftCollisionType,
    pub minimum_distance: f32,
    pub maximum_distance: f32,
    pub falloff_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub coefficient: f32,
    pub b_inverted: bool,
}
#[repr(C, align(8))]
pub struct FCRSimPointForce {
    pub force_type: ECRSimPointForceType,
    pub vector: crate::bindings::core_u_object::FVector,
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
    pub variable_bindings: TMap<FName, crate::bindings::rig_vm::FRigVMExternalVariable>,
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
    pub class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
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
    pub controller: UPtr<crate::bindings::core_u_object::UObject>,
    pub previous_module_paths: TMap<FRigHierarchyModulePath, FName>,
}
#[repr(C, align(8))]
pub struct FRigHierarchyModulePath {
    pub module_path: FString,
}
#[repr(C, align(8))]
pub struct FModularRigModuleSettingsForClipboard {
    pub module_class: crate::bindings::core_u_object::FSoftObjectPath,
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
    pub relative_parent: crate::bindings::core_u_object::FTransform,
    pub constraint_offset: crate::bindings::animation_core::FConstraintOffset,
    pub linked_node: FName,
    pub constraints: TArray<crate::bindings::animation_core::FTransformConstraint>,
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
    pub initial_transform: crate::bindings::core_u_object::FTransform,
    pub global_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
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
    pub offset_transform: crate::bindings::core_u_object::FTransform,
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
    pub gizmo_transform: crate::bindings::core_u_object::FTransform,
    pub gizmo_color: crate::bindings::core_u_object::FLinearColor,
    pub dependents: TArray<i32>,
    pub b_is_transient_control: bool,
    pub control_enum: UPtr<crate::bindings::core_u_object::UEnum>,
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
    pub old_transform: crate::bindings::core_u_object::FTransform,
    pub new_transform: crate::bindings::core_u_object::FTransform,
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
    pub initial_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
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
    pub initial_local_transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
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
    pub poses: TArray<crate::bindings::core_u_object::FTransform>,
    pub dirty_states: TArray<bool>,
}
#[repr(C, align(8))]
pub struct FRigHierarchyCopyPasteContent {
    pub elements: TArray<FRigHierarchyCopyPasteContentPerElement>,
    pub types: TArray<ERigElementType>,
    pub contents: TArray<FString>,
    pub local_transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub global_transforms: TArray<crate::bindings::core_u_object::FTransform>,
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
    pub value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigVectorArrayMetadata {
    pub value: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FRigRotatorMetadata {
    pub value: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(8))]
pub struct FRigRotatorArrayMetadata {
    pub value: TArray<crate::bindings::core_u_object::FRotator>,
}
#[repr(C, align(16))]
pub struct FRigQuatMetadata {
    pub value: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(8))]
pub struct FRigQuatArrayMetadata {
    pub value: TArray<crate::bindings::core_u_object::FQuat>,
}
#[repr(C, align(16))]
pub struct FRigTransformMetadata {
    pub value: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRigTransformArrayMetadata {
    pub value: TArray<crate::bindings::core_u_object::FTransform>,
}
#[repr(C, align(8))]
pub struct FRigLinearColorMetadata {
    pub value: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigLinearColorArrayMetadata {
    pub value: TArray<crate::bindings::core_u_object::FLinearColor>,
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
    pub path: crate::bindings::core_u_object::FSoftObjectPath,
    pub settings: FRigModuleSettings,
}
#[repr(C, align(8))]
pub struct FModuleReferenceData {
    pub module_path: FString,
    pub referenced_module: crate::bindings::core_u_object::FSoftClassPath,
}
#[repr(C, align(8))]
pub struct FRigPhysicsSimulationBase {}
#[repr(C, align(4))]
pub struct FRigPhysicsSolverID {
    pub guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(4))]
pub struct FRigPhysicsSolverDescription {
    pub id: FRigPhysicsSolverID,
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FAnimNode_ControlRigInputPose {
    pub input_pose: crate::bindings::engine::FPoseLink,
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
    pub binding_ids: TArray<crate::bindings::core_u_object::FGuid>,
    pub references: TArray<FControlRigSequenceObjectReferences>,
}
#[repr(C, align(8))]
pub struct FEnumParameterNameAndCurve {
    pub parameter_curve: crate::bindings::movie_scene::FMovieSceneByteChannel,
}
#[repr(C, align(8))]
pub struct FIntegerParameterNameAndCurve {
    pub parameter_curve: crate::bindings::movie_scene::FMovieSceneIntegerChannel,
}
#[repr(C, align(8))]
pub struct FSpaceControlNameAndChannel {
    pub control_name: FName,
    pub space_curve: FMovieSceneControlRigSpaceChannel,
}
#[repr(C, align(8))]
pub struct FMovieSceneControlRigSpaceChannel {
    pub key_times: TArray<crate::bindings::core_u_object::FFrameNumber>,
    pub key_values: TArray<FMovieSceneControlRigSpaceBaseKey>,
    pub key_handles: crate::bindings::movie_scene::FMovieSceneKeyHandleMap,
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
    pub constraints: TArray<crate::bindings::constraints::FConstraintAndActiveChannel>,
}
#[repr(C, align(1))]
pub struct FControlRotationOrder {
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
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
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub parent_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub global_transform: crate::bindings::core_u_object::FTransform,
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
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTraceWorld {
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub channel: crate::bindings::engine::ECollisionChannel,
    pub radius: f32,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    pub work_data: FRigUnit_SphereTrace_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTraceByTraceChannel {
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub trace_channel: crate::bindings::engine::ETraceTypeQuery,
    pub radius: f32,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    pub work_data: FRigUnit_SphereTrace_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_LineTraceByTraceChannel {
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub trace_channel: crate::bindings::engine::ETraceTypeQuery,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    pub work_data: FRigUnit_SphereTrace_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTraceByObjectTypes {
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub object_types: TArray<crate::bindings::engine::EObjectTypeQuery>,
    pub radius: f32,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    pub work_data: FRigUnit_SphereTrace_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_LineTraceByObjectTypes {
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub object_types: TArray<crate::bindings::engine::EObjectTypeQuery>,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    pub work_data: FRigUnit_SphereTrace_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_Control {
    pub transform: crate::bindings::animation_core::FEulerTransform,
    pub base: crate::bindings::core_u_object::FTransform,
    pub init_transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
    pub filter: crate::bindings::animation_core::FTransformFilter,
    pub b_is_initialized: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_Control_StaticMesh {
    pub mesh_transform: crate::bindings::core_u_object::FTransform,
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
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub minimum_u: f32,
    pub maximum_u: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub detail: i32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugBezierItemSpace {
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub minimum_u: f32,
    pub maximum_u: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub detail: i32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugHierarchy {
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
    pub items: TArray<FRigElementKey>,
    pub scale: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugPose {
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
    pub pose: FRigPose,
    pub items: TArray<FRigElementKey>,
    pub scale: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLine {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLineItemSpace {
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLineStrip {
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLineStripItemSpace {
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugRectangle {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugRectangleItemSpace {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugArc {
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
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugArcItemSpace {
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
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransform {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformMutable {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformMutableItemSpace {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FRigUnit_DebugTransformArrayMutable_WorkData {
    pub draw_transforms: TArray<crate::bindings::core_u_object::FTransform>,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformArrayMutable {
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    pub work_data: FRigUnit_DebugTransformArrayMutable_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformArrayMutableItemSpace {
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub parent_indices: TArray<i32>,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
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
    pub value: crate::bindings::core_u_object::FVector,
    pub b_enabled: bool,
    pub mode: crate::bindings::rig_vm::ERigUnitVisualDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
#[repr(C, align(8))]
pub struct FRigUnit_VisualDebugVectorItemSpace {
    pub value: crate::bindings::core_u_object::FVector,
    pub b_enabled: bool,
    pub mode: crate::bindings::rig_vm::ERigUnitVisualDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugQuat {
    pub value: crate::bindings::core_u_object::FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugQuatItemSpace {
    pub value: crate::bindings::core_u_object::FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugTransform {
    pub value: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugTransformItemSpace {
    pub value: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertTransform {
    pub input: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::animation_core::FEulerTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertEulerTransform {
    pub input: crate::bindings::animation_core::FEulerTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertRotation {
    pub input: crate::bindings::core_u_object::FRotator,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertVectorRotation {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertQuaternion {
    pub input: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(8))]
pub struct FRigUnit_ConvertVectorToRotation {
    pub input: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertVectorToQuaternion {
    pub input: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(8))]
pub struct FRigUnit_ConvertRotationToVector {
    pub input: crate::bindings::core_u_object::FRotator,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertQuaternionToVector {
    pub input: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigUnit_ToSwingAndTwist {
    pub input: crate::bindings::core_u_object::FQuat,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub swing: crate::bindings::core_u_object::FQuat,
    pub twist: crate::bindings::core_u_object::FQuat,
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
    pub argument0: crate::bindings::core_u_object::FQuat,
    pub argument1: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigUnit_MultiplyQuaternion {}
#[repr(C, align(16))]
pub struct FRigUnit_UnaryQuaternionOp {
    pub argument: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigUnit_InverseQuaterion {}
#[repr(C, align(16))]
pub struct FRigUnit_QuaternionToAxisAndAngle {
    pub argument: crate::bindings::core_u_object::FQuat,
    pub axis: crate::bindings::core_u_object::FVector,
    pub angle: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_QuaternionFromAxisAndAngle {
    pub axis: crate::bindings::core_u_object::FVector,
    pub angle: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigUnit_QuaternionToAngle {
    pub axis: crate::bindings::core_u_object::FVector,
    pub argument: crate::bindings::core_u_object::FQuat,
    pub angle: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_BinaryTransformOp {
    pub argument0: crate::bindings::core_u_object::FTransform,
    pub argument1: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_MultiplyTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetRelativeTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_BinaryVectorOp {
    pub argument0: crate::bindings::core_u_object::FVector,
    pub argument1: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
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
    pub argument0: crate::bindings::core_u_object::FVector,
    pub argument1: crate::bindings::core_u_object::FVector,
    pub result: f32,
}
#[repr(C, align(16))]
pub struct FAimTarget {
    pub weight: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub align_vector: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_AimConstraint_WorkData {
    pub constraint_data: TArray<crate::bindings::animation_core::FConstraintData>,
}
#[repr(C, align(8))]
pub struct FRigUnit_AimConstraint {
    pub joint: FName,
    pub aim_mode: EAimMode,
    pub up_mode: EAimMode,
    pub aim_vector: crate::bindings::core_u_object::FVector,
    pub up_vector: crate::bindings::core_u_object::FVector,
    pub aim_targets: TArray<FAimTarget>,
    pub up_targets: TArray<FAimTarget>,
    pub work_data: FRigUnit_AimConstraint_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_ApplyFK {
    pub joint: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub filter: crate::bindings::animation_core::FTransformFilter,
    pub apply_transform_mode: EApplyTransformMode,
    pub apply_transform_space: ETransformSpaceMode,
    pub base_transform: crate::bindings::core_u_object::FTransform,
    pub base_joint: FName,
}
#[repr(C, align(16))]
pub struct FBlendTarget {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_BlendTransform {
    pub source: crate::bindings::core_u_object::FTransform,
    pub targets: TArray<FBlendTarget>,
    pub result: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetJointTransform {
    pub joint: FName,
    pub ty: ETransformGetterType,
    pub transform_space: ETransformSpaceMode,
    pub base_transform: crate::bindings::core_u_object::FTransform,
    pub base_joint: FName,
    pub output: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKFK {
    pub start_joint: FName,
    pub end_joint: FName,
    pub pole_target: crate::bindings::core_u_object::FVector,
    pub spin: f32,
    pub end_effector: crate::bindings::core_u_object::FTransform,
    pub ik_blend: f32,
    pub start_joint_fk_transform: crate::bindings::core_u_object::FTransform,
    pub mid_joint_fk_transform: crate::bindings::core_u_object::FTransform,
    pub end_joint_fk_transform: crate::bindings::core_u_object::FTransform,
    pub previous_fkik_blend: f32,
    pub start_joint_ik_transform: crate::bindings::core_u_object::FTransform,
    pub mid_joint_ik_transform: crate::bindings::core_u_object::FTransform,
    pub end_joint_ik_transform: crate::bindings::core_u_object::FTransform,
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
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_DrawContainerSetColor {
    pub instruction_name: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FRigUnit_DrawContainerSetThickness {
    pub instruction_name: FName,
    pub thickness: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_DrawContainerSetTransform {
    pub instruction_name: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_BeginExecution {
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_PreBeginExecution {
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_PostBeginExecution {
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
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
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddNull {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
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
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub transform: crate::bindings::core_u_object::FTransform,
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
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub offset_space: crate::bindings::rig_vm::ERigVMTransformSpace,
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
    pub control_enum: UPtr<crate::bindings::core_u_object::UEnum>,
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
    pub min_value: crate::bindings::core_u_object::FVector2D,
    pub max_value: crate::bindings::core_u_object::FVector2D,
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
    pub initial_value: crate::bindings::core_u_object::FVector2D,
    pub settings: FRigUnit_HierarchyAddControlVector2D_Settings,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlVector_LimitSettings {
    pub limit_x: FRigControlLimitEnabled,
    pub limit_y: FRigControlLimitEnabled,
    pub limit_z: FRigControlLimitEnabled,
    pub min_value: crate::bindings::core_u_object::FVector,
    pub max_value: crate::bindings::core_u_object::FVector,
    pub b_draw_limits: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector_Settings {
    pub initial_space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_is_position: bool,
    pub limits: FRigUnit_HierarchyAddControlVector_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector {
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub settings: FRigUnit_HierarchyAddControlVector_Settings,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlRotator_LimitSettings {
    pub limit_pitch: FRigControlLimitEnabled,
    pub limit_yaw: FRigControlLimitEnabled,
    pub limit_roll: FRigControlLimitEnabled,
    pub min_value: crate::bindings::core_u_object::FRotator,
    pub max_value: crate::bindings::core_u_object::FRotator,
    pub b_draw_limits: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlRotator_Settings {
    pub initial_space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub limits: FRigUnit_HierarchyAddControlRotator_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
    pub b_use_preferred_rotation_order: bool,
    pub preferred_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlRotator {
    pub initial_value: crate::bindings::core_u_object::FRotator,
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
    pub min_value: crate::bindings::animation_core::FEulerTransform,
    pub max_value: crate::bindings::animation_core::FEulerTransform,
    pub b_draw_limits: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlTransform_Settings {
    pub initial_space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_use_preferred_rotation_order: bool,
    pub preferred_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub limits: FRigUnit_HierarchyAddControlTransform_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlTransform {
    pub initial_value: crate::bindings::core_u_object::FTransform,
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
    pub control_enum: UPtr<crate::bindings::core_u_object::UEnum>,
}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannel2DLimitSettings {
    pub x: FRigControlLimitEnabled,
    pub y: FRigControlLimitEnabled,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelVector2D {
    pub initial_value: crate::bindings::core_u_object::FVector2D,
    pub minimum_value: crate::bindings::core_u_object::FVector2D,
    pub maximum_value: crate::bindings::core_u_object::FVector2D,
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
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub minimum_value: crate::bindings::core_u_object::FVector,
    pub maximum_value: crate::bindings::core_u_object::FVector,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelVectorLimitSettings,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelScaleVector {
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub minimum_value: crate::bindings::core_u_object::FVector,
    pub maximum_value: crate::bindings::core_u_object::FVector,
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
    pub initial_value: crate::bindings::core_u_object::FRotator,
    pub minimum_value: crate::bindings::core_u_object::FRotator,
    pub maximum_value: crate::bindings::core_u_object::FRotator,
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
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub color: crate::bindings::core_u_object::FLinearColor,
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
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub items_to_set: FRigElementKeyCollection,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchySetPoseItemArray {
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
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
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub items_to_compare: FRigElementKeyCollection,
    pub poses_are_equal: bool,
    pub items_with_delta: FRigElementKeyCollection,
}
#[repr(C, align(16))]
pub struct FRigUnit_PoseGetTransform {
    pub pose: FRigPose,
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub valid: bool,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub curve_value: f32,
    pub cached_pose_element_index: i32,
    pub cached_pose_hash: i32,
}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetTransformArray {
    pub pose: FRigPose,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub valid: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
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
    pub global_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub curve_value: f32,
    pub index: i32,
    pub count: i32,
    pub ratio: f32,
    pub completed: FControlRigExecuteContext,
}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyCreatePoseItemArray_Entry {
    pub item: FRigElementKey,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub global_transform: crate::bindings::core_u_object::FTransform,
    pub use_euler_angles: bool,
    pub euler_angles: crate::bindings::core_u_object::FVector,
    pub curve_value: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyCreatePoseItemArray {
    pub entries: TArray<FRigUnit_HierarchyCreatePoseItemArray_Entry>,
    pub pose: FRigPose,
}
#[repr(C, align(8))]
pub struct FRigUnit_InteractionExecution {
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_InverseExecution {
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
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
    pub transform: crate::bindings::core_u_object::FTransform,
    pub solver: FRigPhysicsSolverID,
}
#[repr(C, align(8))]
pub struct FRigUnit_PrepareForExecution {
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
#[repr(C, align(8))]
pub struct FRigUnit_PostPrepareForExecution {
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
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
    pub transform: crate::bindings::core_u_object::FTransform,
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
    pub value: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetVectorAnimationChannel {
    pub value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetRotatorAnimationChannel {
    pub value: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetTransformAnimationChannel {
    pub value: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetAnimationChannelBase {
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
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
    pub value: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetVectorAnimationChannel {
    pub value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetRotatorAnimationChannel {
    pub value: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetTransformAnimationChannel {
    pub value: crate::bindings::core_u_object::FTransform,
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
    pub value: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetVectorAnimationChannelFromItem {
    pub value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetRotatorAnimationChannelFromItem {
    pub value: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetTransformAnimationChannelFromItem {
    pub value: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetAnimationChannelBaseFromItem {
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
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
    pub value: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetVectorAnimationChannelFromItem {
    pub value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetRotatorAnimationChannelFromItem {
    pub value: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetTransformAnimationChannelFromItem {
    pub value: crate::bindings::core_u_object::FTransform,
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
    pub point: crate::bindings::core_u_object::FVector,
    pub item: FRigElementKey,
    pub cached_items: TArray<FCachedRigElement>,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetBoneTransform {
    pub bone: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub cached_bone: FCachedRigElement,
    pub b_first_update: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetControlInitialTransform {
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetControlOffset {
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub offset_transform: crate::bindings::core_u_object::FTransform,
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
    pub vector: crate::bindings::core_u_object::FVector2D,
    pub minimum: crate::bindings::core_u_object::FVector2D,
    pub maximum: crate::bindings::core_u_object::FVector2D,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlVector {
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub vector: crate::bindings::core_u_object::FVector,
    pub minimum: crate::bindings::core_u_object::FVector,
    pub maximum: crate::bindings::core_u_object::FVector,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlRotator {
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub minimum: crate::bindings::core_u_object::FRotator,
    pub maximum: crate::bindings::core_u_object::FRotator,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetControlTransform {
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub minimum: crate::bindings::core_u_object::FTransform,
    pub maximum: crate::bindings::core_u_object::FTransform,
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
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub cached_bone: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetRelativeBoneTransform {
    pub bone: FName,
    pub space: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub cached_bone: FCachedRigElement,
    pub cached_space: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetRelativeTransformForItem {
    pub child: FRigElementKey,
    pub b_child_initial: bool,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub relative_transform: crate::bindings::core_u_object::FTransform,
    pub cached_child: FCachedRigElement,
    pub cached_parent: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetSpaceTransform {
    pub space: FName,
    pub space_type: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub cached_space_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetTransform {
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetTransformArray {
    pub items: FRigElementKeyCollection,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub cached_index: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetTransformItemArray {
    pub items: TArray<FRigElementKey>,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
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
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_ParentSwitchConstraint {
    pub subject: FRigElementKey,
    pub parent_index: i32,
    pub parents: FRigElementKeyCollection,
    pub initial_global_transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub switched: bool,
    pub cached_subject: FCachedRigElement,
    pub cached_parent: FCachedRigElement,
    pub relative_offset: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_ParentSwitchConstraintArray {
    pub subject: FRigElementKey,
    pub parent_index: i32,
    pub parents: TArray<FRigElementKey>,
    pub initial_global_transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub switched: bool,
    pub cached_subject: FCachedRigElement,
    pub cached_parent: FCachedRigElement,
    pub relative_offset: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_ProjectTransformToNewParent {
    pub child: FRigElementKey,
    pub b_child_initial: bool,
    pub old_parent: FRigElementKey,
    pub b_old_parent_initial: bool,
    pub new_parent: FRigElementKey,
    pub b_new_parent_initial: bool,
    pub transform: crate::bindings::core_u_object::FTransform,
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
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_propagate_to_children: bool,
    pub cached_bone: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetBoneRotation {
    pub bone: FName,
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_bone: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetBoneTransform {
    pub bone: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_bone: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetBoneTranslation {
    pub bone: FName,
    pub translation: crate::bindings::core_u_object::FVector,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_bone: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlColor {
    pub control: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlColor {
    pub control: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
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
    pub offset: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlTranslationOffset {
    pub control: FName,
    pub offset: crate::bindings::core_u_object::FVector,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetControlRotationOffset {
    pub control: FName,
    pub offset: crate::bindings::core_u_object::FQuat,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlScaleOffset {
    pub control: FName,
    pub scale: crate::bindings::core_u_object::FVector,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_GetShapeTransform {
    pub control: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetShapeTransform {
    pub control: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
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
    pub vector: crate::bindings::core_u_object::FVector2D,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlVector2D_Entry {
    pub control: FName,
    pub vector: crate::bindings::core_u_object::FVector2D,
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
    pub vector: crate::bindings::core_u_object::FVector,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlRotator {
    pub control: FName,
    pub weight: f32,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub cached_control_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlRotator_Entry {
    pub control: FName,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
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
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
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
    pub transform: crate::bindings::core_u_object::FTransform,
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
    pub value: crate::bindings::core_u_object::FTransform,
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
    pub value: crate::bindings::core_u_object::FVector,
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
    pub value: crate::bindings::core_u_object::FQuat,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_child: FCachedRigElement,
    pub cached_parent: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetSpaceInitialTransform {
    pub space_name: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub cached_space_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetSpaceTransform {
    pub space: FName,
    pub weight: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space_type: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub cached_space_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetTransform {
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub value: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetTranslation {
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub value: crate::bindings::core_u_object::FVector,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_SetRotation {
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub value: crate::bindings::core_u_object::FQuat,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetScale {
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub scale: crate::bindings::core_u_object::FVector,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetTransformArray {
    pub items: FRigElementKeyCollection,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub cached_index: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_SetTransformItemArray {
    pub items: TArray<FRigElementKey>,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
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
    pub value: crate::bindings::core_u_object::FTransform,
    pub world: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_ToRigSpace_Transform {
    pub value: crate::bindings::core_u_object::FTransform,
    pub global: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_ToWorldSpace_Location {
    pub value: crate::bindings::core_u_object::FVector,
    pub world: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_ToRigSpace_Location {
    pub value: crate::bindings::core_u_object::FVector,
    pub global: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FRigUnit_ToWorldSpace_Rotation {
    pub value: crate::bindings::core_u_object::FQuat,
    pub world: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(16))]
pub struct FRigUnit_ToRigSpace_Rotation {
    pub value: crate::bindings::core_u_object::FQuat,
    pub global: crate::bindings::core_u_object::FQuat,
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
    pub wave_time: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRigUnit_BoneHarmonics {
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
    pub work_data: FRigUnit_BoneHarmonics_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_ItemHarmonics {
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
    pub work_data: FRigUnit_BoneHarmonics_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_ChainHarmonics_Reach {
    pub b_enabled: bool,
    pub reach_target: crate::bindings::core_u_object::FVector,
    pub reach_axis: crate::bindings::core_u_object::FVector,
    pub reach_minimum: f32,
    pub reach_maximum: f32,
    pub reach_ease: crate::bindings::rig_vm::ERigVMAnimEasingType,
}
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
}
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
#[repr(C, align(8))]
pub struct FRigUnit_ChainHarmonics_WorkData {
    pub time: crate::bindings::core_u_object::FVector,
    pub items: TArray<FCachedRigElement>,
    pub ratio: TArray<f32>,
    pub local_tip: TArray<crate::bindings::core_u_object::FVector>,
    pub pendulum_tip: TArray<crate::bindings::core_u_object::FVector>,
    pub pendulum_position: TArray<crate::bindings::core_u_object::FVector>,
    pub pendulum_velocity: TArray<crate::bindings::core_u_object::FVector>,
    pub hierarchy_line: TArray<crate::bindings::core_u_object::FVector>,
    pub velocity_lines: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(16))]
pub struct FRigUnit_ChainHarmonics {
    pub chain_root: FName,
    pub speed: crate::bindings::core_u_object::FVector,
    pub reach: FRigUnit_ChainHarmonics_Reach,
    pub wave: FRigUnit_ChainHarmonics_Wave,
    pub wave_curve: crate::bindings::engine::FRuntimeFloatCurve,
    pub pendulum: FRigUnit_ChainHarmonics_Pendulum,
    pub b_draw_debug: bool,
    pub draw_world_offset: crate::bindings::core_u_object::FTransform,
    pub work_data: FRigUnit_ChainHarmonics_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_ChainHarmonicsPerItem {
    pub chain_root: FRigElementKey,
    pub speed: crate::bindings::core_u_object::FVector,
    pub reach: FRigUnit_ChainHarmonics_Reach,
    pub wave: FRigUnit_ChainHarmonics_Wave,
    pub wave_curve: crate::bindings::engine::FRuntimeFloatCurve,
    pub pendulum: FRigUnit_ChainHarmonics_Pendulum,
    pub b_draw_debug: bool,
    pub draw_world_offset: crate::bindings::core_u_object::FTransform,
    pub work_data: FRigUnit_ChainHarmonics_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_AimBone_Target {
    pub weight: f32,
    pub axis: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub kind: EControlRigVectorKind,
    pub space: FName,
}
#[repr(C, align(8))]
pub struct FRigUnit_AimItem_Target {
    pub weight: f32,
    pub axis: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub kind: EControlRigVectorKind,
    pub space: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_AimBone_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_AimBoneMath {
    pub input_transform: crate::bindings::core_u_object::FTransform,
    pub primary: FRigUnit_AimItem_Target,
    pub secondary: FRigUnit_AimItem_Target,
    pub weight: f32,
    pub result: crate::bindings::core_u_object::FTransform,
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
    pub target: crate::bindings::core_u_object::FVector,
    pub kind: EControlRigVectorKind,
    pub space: FRigElementKey,
}
#[repr(C, align(16))]
pub struct FRigUnit_AimConstraint_AdvancedSettings {
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    pub rotation_order_for_filter: crate::bindings::animation_core::EEulerRotationOrder,
}
#[repr(C, align(16))]
pub struct FRigUnit_AimConstraintLocalSpaceOffset {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub aim_axis: crate::bindings::core_u_object::FVector,
    pub up_axis: crate::bindings::core_u_object::FVector,
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
    pub chain: TArray<crate::bindings::animation_core::FCCDIKChainLink>,
    pub cached_items: TArray<FCachedRigElement>,
    pub rotation_limit_index: TArray<i32>,
    pub rotation_limits_per_item: TArray<f32>,
    pub cached_effector: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_CCDIK {
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
    pub work_data: FRigUnit_CCDIK_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_CCDIKPerItem {
    pub items: FRigElementKeyCollection,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
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
    pub effector_transform: crate::bindings::core_u_object::FTransform,
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
    pub interpolated_transform: crate::bindings::core_u_object::FTransform,
    pub chain_length: f32,
    pub param_length: f32,
    pub chain_stretch_factor: f32,
    pub segment_info: FRigUnit_ChainInfo_SegmentInfo,
    pub cached_elements: TArray<FCachedRigElement>,
}
#[repr(C, align(16))]
pub struct FRigUnit_DistributeRotation_Rotation {
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub ratio: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotation_WorkData {
    pub cached_items: TArray<FCachedRigElement>,
    pub item_rotation_a: TArray<i32>,
    pub item_rotation_b: TArray<i32>,
    pub item_rotation_t: TArray<f32>,
    pub item_local_transforms: TArray<crate::bindings::core_u_object::FTransform>,
}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotation {
    pub start_bone: FName,
    pub end_bone: FName,
    pub rotations: TArray<FRigUnit_DistributeRotation_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_DistributeRotation_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotationForCollection {
    pub items: FRigElementKeyCollection,
    pub rotations: TArray<FRigUnit_DistributeRotation_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub work_data: FRigUnit_DistributeRotation_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotationForItemArray {
    pub items: TArray<FRigElementKey>,
    pub rotations: TArray<FRigUnit_DistributeRotation_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub work_data: FRigUnit_DistributeRotation_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_FABRIK_WorkData {
    pub chain: TArray<crate::bindings::animation_core::FFABRIKChainLink>,
    pub cached_items: TArray<FCachedRigElement>,
    pub cached_effector: FCachedRigElement,
}
#[repr(C, align(16))]
pub struct FRigUnit_FABRIK {
    pub start_bone: FName,
    pub effector_bone: FName,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
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
    pub effector_transform: crate::bindings::core_u_object::FTransform,
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
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    pub work_data: FRigUnit_FABRIK_WorkData,
    pub b_set_effector_transform: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurve_Rotation {
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub ratio: f32,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurve_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub curve_color: crate::bindings::core_u_object::FLinearColor,
    pub segments_color: crate::bindings::core_u_object::FLinearColor,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_FitChainToCurve_WorkData {
    pub chain_length: f32,
    pub item_positions: TArray<crate::bindings::core_u_object::FVector>,
    pub item_segments: TArray<f32>,
    pub curve_positions: TArray<crate::bindings::core_u_object::FVector>,
    pub curve_segments: TArray<f32>,
    pub cached_items: TArray<FCachedRigElement>,
    pub item_rotation_a: TArray<i32>,
    pub item_rotation_b: TArray<i32>,
    pub item_rotation_t: TArray<f32>,
    pub item_local_transforms: TArray<crate::bindings::core_u_object::FTransform>,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurve {
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
    pub work_data: FRigUnit_FitChainToCurve_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurvePerItem {
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
    pub work_data: FRigUnit_FitChainToCurve_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurveItemArray {
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
    pub work_data: FRigUnit_FitChainToCurve_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_ModifyBoneTransforms_PerBone {
    pub bone: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
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
    pub transform: crate::bindings::core_u_object::FTransform,
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
    pub location: crate::bindings::core_u_object::FVector,
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
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub blended_transforms: TArray<crate::bindings::core_u_object::FTransform>,
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
    pub inner_region: FSphericalRegion,
    pub outer_region: FSphericalRegion,
    pub driver_normal: crate::bindings::core_u_object::FVector,
    pub driver2_d: crate::bindings::core_u_object::FVector,
    pub driver_cache: FCachedRigElement,
    pub optional_parent_cache: FCachedRigElement,
    pub local_driver_transform_init: crate::bindings::core_u_object::FTransform,
    pub cached_rotation_offset: crate::bindings::core_u_object::FVector,
    pub b_cached_init_transforms: bool,
}
#[repr(C, align(16))]
pub struct FRigUnit_SpringIK_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FRigUnit_SpringIK_WorkData {
    pub cached_bones: TArray<FCachedRigElement>,
    pub cached_pole_vector: FCachedRigElement,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
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
    pub work_data: FRigUnit_SpringIK_WorkData,
}
#[repr(C, align(16))]
pub struct FConstraintTarget {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FTransformFilter,
}
#[repr(C, align(8))]
pub struct FRigUnit_TransformConstraint_WorkData {
    pub constraint_data: TArray<crate::bindings::animation_core::FConstraintData>,
    pub constraint_data_to_targets: TMap<i32, i32>,
}
#[repr(C, align(16))]
pub struct FRigUnit_TransformConstraint {
    pub bone: FName,
    pub base_transform_space: ETransformSpaceMode,
    pub base_transform: crate::bindings::core_u_object::FTransform,
    pub base_bone: FName,
    pub targets: TArray<FConstraintTarget>,
    pub b_use_initial_transforms: bool,
    pub work_data: FRigUnit_TransformConstraint_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_TransformConstraintPerItem {
    pub item: FRigElementKey,
    pub base_transform_space: ETransformSpaceMode,
    pub base_transform: crate::bindings::core_u_object::FTransform,
    pub base_item: FRigElementKey,
    pub targets: TArray<FConstraintTarget>,
    pub b_use_initial_transforms: bool,
    pub work_data: FRigUnit_TransformConstraint_WorkData,
}
#[repr(C, align(1))]
pub struct FRigUnit_ParentConstraint_AdvancedSettings {
    pub interpolation_type: EConstraintInterpType,
    pub rotation_order_for_filter: crate::bindings::animation_core::EEulerRotationOrder,
}
#[repr(C, align(8))]
pub struct FRigUnit_ParentConstraint {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FTransformFilter,
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
    pub input: crate::bindings::core_u_object::FTransform,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_ParentConstraintMath_AdvancedSettings,
    pub output: crate::bindings::core_u_object::FTransform,
    pub parent_caches: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_PositionConstraint {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_PositionConstraintLocalSpaceOffset {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
    pub child_cache: FCachedRigElement,
    pub parent_caches: TArray<FCachedRigElement>,
}
#[repr(C, align(1))]
pub struct FRigUnit_RotationConstraint_AdvancedSettings {
    pub interpolation_type: EConstraintInterpType,
    pub rotation_order_for_filter: crate::bindings::animation_core::EEulerRotationOrder,
}
#[repr(C, align(8))]
pub struct FRigUnit_RotationConstraint {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_RotationConstraint_AdvancedSettings,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_RotationConstraintLocalSpaceOffset {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
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
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FRigUnit_ScaleConstraintLocalSpaceOffset {
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
    pub child_cache: FCachedRigElement,
    pub parent_caches: TArray<FCachedRigElement>,
}
#[repr(C, align(8))]
pub struct FRigUnit_TwistBones_WorkData {
    pub cached_items: TArray<FCachedRigElement>,
    pub item_ratios: TArray<f32>,
    pub item_transforms: TArray<crate::bindings::core_u_object::FTransform>,
}
#[repr(C, align(8))]
pub struct FRigUnit_TwistBones {
    pub start_bone: FName,
    pub end_bone: FName,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub pole_axis: crate::bindings::core_u_object::FVector,
    pub twist_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_TwistBones_WorkData,
}
#[repr(C, align(8))]
pub struct FRigUnit_TwistBonesPerItem {
    pub items: FRigElementKeyCollection,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub pole_axis: crate::bindings::core_u_object::FVector,
    pub twist_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub work_data: FRigUnit_TwistBones_WorkData,
}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimple_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimple {
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
    pub cached_item_a_index: FCachedRigElement,
    pub cached_item_b_index: FCachedRigElement,
    pub cached_effector_item_index: FCachedRigElement,
    pub cached_pole_vector_space_index: FCachedRigElement,
}
#[repr(C, align(8))]
pub struct FRigUnit_TwoBoneIKSimpleVectors {
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
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimpleTransforms {
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
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub world_offset: crate::bindings::core_u_object::FTransform,
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
    pub spring_state: crate::bindings::engine::FFloatSpringState,
}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterpVector {
    pub current: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub stiffness: f32,
    pub critical_damping: f32,
    pub mass: f32,
    pub result: crate::bindings::core_u_object::FVector,
    pub spring_state: crate::bindings::engine::FVectorSpringState,
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
    pub spring_state: crate::bindings::engine::FFloatSpringState,
}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterpVectorV2 {
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
    pub simulated_result: crate::bindings::core_u_object::FVector,
    pub spring_state: crate::bindings::engine::FVectorSpringState,
}
#[repr(C, align(16))]
pub struct FRigUnit_SpringInterpQuaternionV2 {
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
    pub simulated_result: crate::bindings::core_u_object::FQuat,
    pub spring_state: crate::bindings::engine::FQuaternionSpringState,
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
    pub input_properties_deprecated: TMap<
        FName,
        crate::bindings::property_path::FCachedPropertyPath,
    >,
    pub output_properties_deprecated: TMap<
        FName,
        crate::bindings::property_path::FCachedPropertyPath,
    >,
    pub rig_vm_extended_execute_context: crate::bindings::rig_vm::FRigVMExtendedExecuteContext,
    pub data_source_registry: UPtr<
        crate::bindings::animation_core::UAnimationDataSourceRegistry,
    >,
    pub influences: FRigInfluenceMapPerEvent,
    pub external_variable_data_asset_links: TMap<
        FName,
        UPtr<crate::bindings::rig_vm::UDataAssetLink>,
    >,
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
    pub preview_skeletal_mesh: crate::bindings::core_u_object::FSoftObjectPath,
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
    pub actor_to_track: UPtr<crate::bindings::engine::AActor>,
    pub control_rig_class: TSubclassOf<UControlRig>,
    pub b_refresh_on_tick: bool,
    pub b_is_selectable: bool,
    pub material_override: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub color_parameter: FString,
    pub b_cast_shadows: bool,
    pub actor_root_component: UPtr<crate::bindings::engine::USceneComponent>,
    pub control_rig: TSoftObjectPtr<UControlRig>,
    pub control_names: TArray<FName>,
    pub shape_transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub components: TArray<UPtr<crate::bindings::engine::UStaticMeshComponent>>,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInstanceDynamic>>,
    pub color_parameter_name: FName,
}
pub struct AControlRigShapeActor {
    pub actor_root_component: UPtr<crate::bindings::engine::USceneComponent>,
    pub static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
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
    pub default_material: TSoftObjectPtr<crate::bindings::engine::UMaterial>,
    pub x_ray_material: TSoftObjectPtr<crate::bindings::engine::UMaterial>,
    pub material_color_parameter: FName,
    pub shapes: TArray<FControlRigShapeDefinition>,
}
pub struct UControlRigOverrideAsset {
    pub overrides: FControlRigOverrideContainer,
}
pub struct UControlRigReplay {
    pub description: FText,
    pub control_rig_object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub preview_skeletal_mesh_object_path: crate::bindings::core_u_object::FSoftObjectPath,
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
    pub transform_mask: crate::bindings::movie_scene_tracks::FMovieSceneTransformMask,
    pub weight: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub control_channel_map: TMap<FName, FChannelMapInfo>,
    pub enum_parameter_names_and_curves: TArray<FEnumParameterNameAndCurve>,
    pub integer_parameter_names_and_curves: TArray<FIntegerParameterNameAndCurve>,
    pub space_channels: TArray<FSpaceControlNameAndChannel>,
    pub constraints_channels: TArray<
        crate::bindings::constraints::FConstraintAndActiveChannel,
    >,
    pub override_assets: TArray<TSoftObjectPtr<UControlRigOverrideAsset>>,
}
pub struct UMovieSceneControlRigParameterTrack {
    pub section_to_key_per_control: TMap<
        FName,
        TWeakObjectPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    >,
    pub control_rig: UPtr<UControlRig>,
    pub section_to_key: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    pub sections: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>,
    pub track_name: FName,
    pub controls_rotation_order: TMap<FName, FControlRotationOrder>,
    pub priority_order: i32,
    pub control_rig_settings_overrides: crate::bindings::core_u_object::FInstancedPropertyBag,
    pub game_world_control_rigs: TMap<
        TWeakObjectPtr<crate::bindings::engine::UWorld>,
        UPtr<UControlRig>,
    >,
}
pub struct UMovieSceneControlRigParameterEvaluatorSystem {
    pub double_blender_system: UPtr<
        crate::bindings::movie_scene_tracks::UMovieScenePiecewiseDoubleBlenderSystem,
    >,
}
pub struct UControlRigSettings {
    pub default_shape_library: TSoftObjectPtr<UControlRigShapeLibrary>,
    pub default_root_module: crate::bindings::core_u_object::FSoftObjectPath,
}
pub struct UControlRigEditorSettings {
    pub b_reset_controls_on_compile: bool,
    pub b_reset_controls_on_pin_value_interaction: bool,
    pub b_reset_pose_when_toggling_event_queue: bool,
    pub b_enable_undo_for_pose_interaction: bool,
    pub b_reset_control_transforms_on_compile: bool,
    pub rig_unit_pin_expansion: TMap<FString, FControlRigSettingsPerPinBool>,
    pub b_enable_flashlight_in_dependency_viewer: bool,
    pub construction_event_border_color: crate::bindings::core_u_object::FLinearColor,
    pub backwards_solve_border_color: crate::bindings::core_u_object::FLinearColor,
    pub backwards_and_forwards_border_color: crate::bindings::core_u_object::FLinearColor,
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
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub axis_to_flip: crate::bindings::core_u_object::EAxis,
    pub mirror_match_tolerance: f64,
}
pub struct UControlRigPoseProjectSettings {
    pub root_save_dirs: TArray<crate::bindings::core_u_object::FDirectoryPath>,
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
pub struct FControlRig_OnControlSelected_BP;
pub struct FControlRigComponent_OnPreInitializeDelegate;
pub struct FControlRigComponent_OnPostInitializeDelegate;
pub struct FControlRigComponent_OnPreConstructionDelegate;
pub struct FControlRigComponent_OnPostConstructionDelegate;
pub struct FControlRigComponent_OnPreForwardsSolveDelegate;
pub struct FControlRigComponent_OnPostForwardsSolveDelegate;
pub struct FRigHierarchy_ModifiedEventDynamic;
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigControlAnimationType(pub u8);
impl ERigControlAnimationType {
    pub const ANIMATION_CONTROL: ERigControlAnimationType = ERigControlAnimationType(0);
    pub const ANIMATION_CHANNEL: ERigControlAnimationType = ERigControlAnimationType(1);
    pub const PROXY_CONTROL: ERigControlAnimationType = ERigControlAnimationType(2);
    pub const VISUAL_CUE: ERigControlAnimationType = ERigControlAnimationType(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigControlAxis(pub u8);
impl ERigControlAxis {
    pub const X: ERigControlAxis = ERigControlAxis(0);
    pub const Y: ERigControlAxis = ERigControlAxis(1);
    pub const Z: ERigControlAxis = ERigControlAxis(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigControlVisibility(pub u8);
impl ERigControlVisibility {
    pub const USER_DEFINED: ERigControlVisibility = ERigControlVisibility(0);
    pub const BASED_ON_SELECTION: ERigControlVisibility = ERigControlVisibility(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConnectorType(pub u8);
impl EConnectorType {
    pub const PRIMARY: EConnectorType = EConnectorType(0);
    pub const SECONDARY: EConnectorType = EConnectorType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EElementNameDisplayMode(pub u8);
impl EElementNameDisplayMode {
    pub const ASSET_DEFAULT: EElementNameDisplayMode = EElementNameDisplayMode(0);
    pub const AUTO: EElementNameDisplayMode = EElementNameDisplayMode(1);
    pub const FORCE_SHORT: EElementNameDisplayMode = EElementNameDisplayMode(2);
    pub const FORCE_LONG: EElementNameDisplayMode = EElementNameDisplayMode(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECRSimConstraintType(pub u8);
impl ECRSimConstraintType {
    pub const DISTANCE: ECRSimConstraintType = ECRSimConstraintType(0);
    pub const DISTANCE_FROM_A: ECRSimConstraintType = ECRSimConstraintType(1);
    pub const DISTANCE_FROM_B: ECRSimConstraintType = ECRSimConstraintType(2);
    pub const PLANE: ECRSimConstraintType = ECRSimConstraintType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECRSimPointForceType(pub u8);
impl ECRSimPointForceType {
    pub const DIRECTION: ECRSimPointForceType = ECRSimPointForceType(0);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECRSimSoftCollisionType(pub u8);
impl ECRSimSoftCollisionType {
    pub const PLANE: ECRSimSoftCollisionType = ECRSimSoftCollisionType(0);
    pub const SPHERE: ECRSimSoftCollisionType = ECRSimSoftCollisionType(1);
    pub const CONE: ECRSimSoftCollisionType = ECRSimSoftCollisionType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigBoneType(pub u8);
impl ERigBoneType {
    pub const IMPORTED: ERigBoneType = ERigBoneType(0);
    pub const USER: ERigBoneType = ERigBoneType(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigTransformType(pub u8);
impl ERigTransformType {
    pub const INITIAL_LOCAL: ERigTransformType = ERigTransformType(0);
    pub const CURRENT_LOCAL: ERigTransformType = ERigTransformType(1);
    pub const INITIAL_GLOBAL: ERigTransformType = ERigTransformType(2);
    pub const CURRENT_GLOBAL: ERigTransformType = ERigTransformType(3);
    pub const NUM_TRANSFORM_TYPES: ERigTransformType = ERigTransformType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigSpaceType(pub u8);
impl ERigSpaceType {
    pub const GLOBAL: ERigSpaceType = ERigSpaceType(0);
    pub const BONE: ERigSpaceType = ERigSpaceType(1);
    pub const CONTROL: ERigSpaceType = ERigSpaceType(2);
    pub const SPACE: ERigSpaceType = ERigSpaceType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigElementResolveState(pub u8);
impl ERigElementResolveState {
    pub const UNKNOWN: ERigElementResolveState = ERigElementResolveState(0);
    pub const INVALID_TARGET: ERigElementResolveState = ERigElementResolveState(1);
    pub const POSSIBLE_TARGET: ERigElementResolveState = ERigElementResolveState(2);
    pub const DEFAULT_TARGET: ERigElementResolveState = ERigElementResolveState(3);
    pub const MAX: ERigElementResolveState = ERigElementResolveState(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EModularRigResolveState(pub u8);
impl EModularRigResolveState {
    pub const SUCCESS: EModularRigResolveState = EModularRigResolveState(0);
    pub const ERROR: EModularRigResolveState = EModularRigResolveState(1);
    pub const MAX: EModularRigResolveState = EModularRigResolveState(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMovieSceneControlRigSpaceType(pub u8);
impl EMovieSceneControlRigSpaceType {
    pub const PARENT: EMovieSceneControlRigSpaceType = EMovieSceneControlRigSpaceType(0);
    pub const WORLD: EMovieSceneControlRigSpaceType = EMovieSceneControlRigSpaceType(1);
    pub const CONTROL_RIG: EMovieSceneControlRigSpaceType = EMovieSceneControlRigSpaceType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAimMode(pub u8);
impl EAimMode {
    pub const AIM_AT_TARGET: EAimMode = EAimMode(0);
    pub const ORIENT_TO_TARGET: EAimMode = EAimMode(1);
    pub const MAX: EAimMode = EAimMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EApplyTransformMode(pub u8);
impl EApplyTransformMode {
    pub const OVERRIDE: EApplyTransformMode = EApplyTransformMode(0);
    pub const ADDITIVE: EApplyTransformMode = EApplyTransformMode(1);
    pub const MAX: EApplyTransformMode = EApplyTransformMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETransformSpaceMode(pub u8);
impl ETransformSpaceMode {
    pub const LOCAL_SPACE: ETransformSpaceMode = ETransformSpaceMode(0);
    pub const GLOBAL_SPACE: ETransformSpaceMode = ETransformSpaceMode(1);
    pub const BASE_SPACE: ETransformSpaceMode = ETransformSpaceMode(2);
    pub const BASE_JOINT: ETransformSpaceMode = ETransformSpaceMode(3);
    pub const MAX: ETransformSpaceMode = ETransformSpaceMode(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETransformGetterType(pub u8);
impl ETransformGetterType {
    pub const INITIAL: ETransformGetterType = ETransformGetterType(0);
    pub const CURRENT: ETransformGetterType = ETransformGetterType(1);
    pub const MAX: ETransformGetterType = ETransformGetterType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigSwitchParentMode(pub u8);
impl ERigSwitchParentMode {
    pub const WORLD: ERigSwitchParentMode = ERigSwitchParentMode(0);
    pub const DEFAULT_PARENT: ERigSwitchParentMode = ERigSwitchParentMode(1);
    pub const PARENT_ITEM: ERigSwitchParentMode = ERigSwitchParentMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigMetaDataNameSpace(pub u8);
impl ERigMetaDataNameSpace {
    pub const NONE: ERigMetaDataNameSpace = ERigMetaDataNameSpace(0);
    pub const SELF: ERigMetaDataNameSpace = ERigMetaDataNameSpace(1);
    pub const PARENT: ERigMetaDataNameSpace = ERigMetaDataNameSpace(2);
    pub const ROOT: ERigMetaDataNameSpace = ERigMetaDataNameSpace(3);
    pub const LAST: ERigMetaDataNameSpace = ERigMetaDataNameSpace(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigEvent(pub u8);
impl ERigEvent {
    pub const NONE: ERigEvent = ERigEvent(0);
    pub const REQUEST_AUTO_KEY: ERigEvent = ERigEvent(1);
    pub const OPEN_UNDO_BRACKET: ERigEvent = ERigEvent(2);
    pub const CLOSE_UNDO_BRACKET: ERigEvent = ERigEvent(3);
    pub const MAX: ERigEvent = ERigEvent(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EControlRigVectorKind(pub u8);
impl EControlRigVectorKind {
    pub const DIRECTION: EControlRigVectorKind = EControlRigVectorKind(0);
    pub const LOCATION: EControlRigVectorKind = EControlRigVectorKind(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EControlRigCurveAlignment(pub u8);
impl EControlRigCurveAlignment {
    pub const FRONT: EControlRigCurveAlignment = EControlRigCurveAlignment(0);
    pub const STRETCHED: EControlRigCurveAlignment = EControlRigCurveAlignment(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EControlRigModifyBoneMode(pub u8);
impl EControlRigModifyBoneMode {
    pub const OVERRIDE_LOCAL: EControlRigModifyBoneMode = EControlRigModifyBoneMode(0);
    pub const OVERRIDE_GLOBAL: EControlRigModifyBoneMode = EControlRigModifyBoneMode(1);
    pub const ADDITIVE_LOCAL: EControlRigModifyBoneMode = EControlRigModifyBoneMode(2);
    pub const ADDITIVE_GLOBAL: EControlRigModifyBoneMode = EControlRigModifyBoneMode(3);
    pub const MAX: EControlRigModifyBoneMode = EControlRigModifyBoneMode(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConstraintInterpType(pub u8);
impl EConstraintInterpType {
    pub const AVERAGE: EConstraintInterpType = EConstraintInterpType(0);
    pub const SHORTEST: EConstraintInterpType = EConstraintInterpType(1);
    pub const MAX: EConstraintInterpType = EConstraintInterpType(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigControlValueType(pub u8);
impl ERigControlValueType {
    pub const INITIAL: ERigControlValueType = ERigControlValueType(0);
    pub const CURRENT: ERigControlValueType = ERigControlValueType(1);
    pub const MINIMUM: ERigControlValueType = ERigControlValueType(2);
    pub const MAXIMUM: ERigControlValueType = ERigControlValueType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EControlRigFKRigExecuteMode(pub u8);
impl EControlRigFKRigExecuteMode {
    pub const REPLACE: EControlRigFKRigExecuteMode = EControlRigFKRigExecuteMode(0);
    pub const ADDITIVE: EControlRigFKRigExecuteMode = EControlRigFKRigExecuteMode(1);
    pub const DIRECT: EControlRigFKRigExecuteMode = EControlRigFKRigExecuteMode(2);
    pub const MAX: EControlRigFKRigExecuteMode = EControlRigFKRigExecuteMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigExecutionType(pub u8);
impl ERigExecutionType {
    pub const RUNTIME: ERigExecutionType = ERigExecutionType(0);
    pub const EDITING: ERigExecutionType = ERigExecutionType(1);
    pub const MAX: ERigExecutionType = ERigExecutionType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EControlRigType(pub u8);
impl EControlRigType {
    pub const INDEPENDENT_RIG: EControlRigType = EControlRigType(0);
    pub const RIG_MODULE: EControlRigType = EControlRigType(1);
    pub const MODULAR_RIG: EControlRigType = EControlRigType(2);
    pub const MAX: EControlRigType = EControlRigType(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMultiRigTreeDisplayMode(pub i32);
impl EMultiRigTreeDisplayMode {
    pub const ALL: EMultiRigTreeDisplayMode = EMultiRigTreeDisplayMode(0);
    pub const SELECTED_RIGS: EMultiRigTreeDisplayMode = EMultiRigTreeDisplayMode(1);
    pub const SELECTED_MODULES: EMultiRigTreeDisplayMode = EMultiRigTreeDisplayMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EControlRigInteractionType(pub u8);
impl EControlRigInteractionType {
    pub const NONE: EControlRigInteractionType = EControlRigInteractionType(0);
    pub const TRANSLATE: EControlRigInteractionType = EControlRigInteractionType(1);
    pub const ROTATE: EControlRigInteractionType = EControlRigInteractionType(2);
    pub const SCALE: EControlRigInteractionType = EControlRigInteractionType(4);
    pub const ALL: EControlRigInteractionType = EControlRigInteractionType(7);
}
