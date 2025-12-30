#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FCameraContextDataID {
    pub value: u32,
}
#[repr(C, align(2))]
pub struct FCameraNodeEvaluatorAllocationInfo {
    pub total_sizeof: i16,
    pub max_alignof: i16,
}
#[repr(C, align(8))]
pub struct FCameraObjectInterfaceParameterDefinition {
    pub parameter_name: FName,
    pub parameter_guid: FGuid,
    pub parameter_type: ECameraObjectInterfaceParameterType,
    pub variable_id: FCameraVariableID,
    pub variable_type: ECameraVariableType,
    pub blendable_struct_type: UPtr<UScriptStruct>,
    pub data_id: FCameraContextDataID,
    pub data_type: ECameraContextDataType,
    pub data_container_type: ECameraContextDataContainerType,
    pub data_type_object: UPtr<UObject>,
}
#[repr(C, align(4))]
pub struct FCameraVariableID {
    pub value: u32,
}
#[repr(C, align(4))]
pub struct FCameraRigInstanceID {
    pub value: u32,
    pub layer: ECameraRigLayer,
}
#[repr(C, align(4))]
pub struct FCameraShakeInstanceID {
    pub value: u32,
}
#[repr(C, align(8))]
pub struct FCameraObjectAllocationInfo {
    pub evaluator_info: FCameraNodeEvaluatorAllocationInfo,
    pub variable_table_info: FCameraVariableTableAllocationInfo,
    pub context_data_table_info: FCameraContextDataTableAllocationInfo,
}
#[repr(C, align(8))]
pub struct FCameraContextDataTableAllocationInfo {
    pub data_definitions: TArray<FCameraContextDataDefinition>,
}
#[repr(C, align(8))]
pub struct FCameraContextDataDefinition {
    pub data_id: FCameraContextDataID,
    pub data_type: ECameraContextDataType,
    pub data_container_type: ECameraContextDataContainerType,
    pub data_type_object: UPtr<UObject>,
    pub b_auto_reset: bool,
    pub data_name: FString,
}
#[repr(C, align(8))]
pub struct FCameraVariableTableAllocationInfo {
    pub variable_definitions: TArray<FCameraVariableDefinition>,
}
#[repr(C, align(8))]
pub struct FCameraVariableDefinition {
    pub variable_id: FCameraVariableID,
    pub variable_type: ECameraVariableType,
    pub blendable_struct_type: UPtr<UScriptStruct>,
    pub b_is_private: bool,
    pub b_is_input: bool,
    pub b_auto_reset: bool,
    pub variable_name: FString,
}
#[repr(C, align(4))]
pub struct FCameraObjectInterfaceParameterMetaData {
    pub parameter_guid: FGuid,
    pub override_variable_id: FCameraVariableID,
    pub override_data_id: FCameraContextDataID,
    pub b_is_overridden_deprecated: bool,
}
#[repr(C, align(8))]
pub struct FBaseCameraObjectReference {
    pub parameters: FInstancedOverridablePropertyBag,
    pub parameter_meta_data: TArray<FCameraObjectInterfaceParameterMetaData>,
}
#[repr(C, align(8))]
pub struct FInstancedOverridablePropertyBag {
    pub overriden_property_i_ds: TArray<FGuid>,
}
#[repr(C, align(8))]
pub struct FCameraAssetAllocationInfo {
    pub variable_table_info: FCameraVariableTableAllocationInfo,
    pub context_data_table_info: FCameraContextDataTableAllocationInfo,
}
#[repr(C, align(8))]
pub struct FCameraAssetReference {
    pub camera_asset: UPtr<UCameraAsset>,
    pub parameters: FInstancedOverridablePropertyBag,
    pub parameter_override_guids_deprecated: TArray<FGuid>,
}
#[repr(C, align(8))]
pub struct FCameraObjectInterface {
    pub blendable_parameters: TArray<UPtr<UCameraObjectInterfaceBlendableParameter>>,
    pub data_parameters: TArray<UPtr<UCameraObjectInterfaceDataParameter>>,
    pub display_name_deprecated: FString,
}
#[repr(C, align(8))]
pub struct FBooleanCameraParameter {
    pub value: bool,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UBooleanCameraVariable>,
}
#[repr(C, align(8))]
pub struct FInteger32CameraParameter {
    pub value: i32,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UInteger32CameraVariable>,
}
#[repr(C, align(8))]
pub struct FFloatCameraParameter {
    pub value: f32,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UFloatCameraVariable>,
}
#[repr(C, align(8))]
pub struct FDoubleCameraParameter {
    pub value: f64,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UDoubleCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector2fCameraParameter {
    pub value: FVector2f,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector2fCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector2dCameraParameter {
    pub value: FVector2D,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector2dCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector3fCameraParameter {
    pub value: FVector3f,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector3fCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector3dCameraParameter {
    pub value: FVector,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector3dCameraVariable>,
}
#[repr(C, align(16))]
pub struct FVector4fCameraParameter {
    pub value: FVector4f,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector4fCameraVariable>,
}
#[repr(C, align(16))]
pub struct FVector4dCameraParameter {
    pub value: FVector4,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector4dCameraVariable>,
}
#[repr(C, align(8))]
pub struct FRotator3fCameraParameter {
    pub value: FRotator3f,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<URotator3fCameraVariable>,
}
#[repr(C, align(8))]
pub struct FRotator3dCameraParameter {
    pub value: FRotator,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<URotator3dCameraVariable>,
}
#[repr(C, align(16))]
pub struct FTransform3fCameraParameter {
    pub value: FTransform3f,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UTransform3fCameraVariable>,
}
#[repr(C, align(16))]
pub struct FTransform3dCameraParameter {
    pub value: FTransform,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UTransform3dCameraVariable>,
}
#[repr(C, align(8))]
pub struct FCameraPose {
    pub location: FVector3d,
    pub rotation: FRotator3d,
    pub target_distance: f64,
    pub field_of_view: f32,
    pub focal_length: f32,
    pub orthographic_width: f32,
    pub aperture: f32,
    pub shutter_speed: f32,
    pub focus_distance: f32,
    pub sensor_width: f32,
    pub sensor_height: f32,
    pub sensor_horizontal_offset: f32,
    pub sensor_vertical_offset: f32,
    pub iso: f32,
    pub squeeze_factor: f32,
    pub overscan: f32,
    pub diaphragm_blade_count: i32,
    pub near_clipping_plane: f32,
    pub far_clipping_plane: f32,
    pub physical_camera_blend_weight: f32,
    pub enable_physical_camera: bool,
    pub constrain_aspect_ratio: bool,
    pub override_aspect_ratio_axis_constraint: bool,
    pub aspect_ratio_axis_constraint: EAspectRatioAxisConstraint,
    pub projection_mode: ECameraProjectionMode,
}
#[repr(C, align(8))]
pub struct FCameraRigParameterOverrideBase {
    pub interface_parameter_guid: FGuid,
    pub private_variable_guid: FGuid,
    pub interface_parameter_name: FString,
    pub b_invalid: bool,
}
#[repr(C, align(8))]
pub struct FBooleanCameraRigParameterOverride {
    pub value: FBooleanCameraParameter,
}
#[repr(C, align(8))]
pub struct FInteger32CameraRigParameterOverride {
    pub value: FInteger32CameraParameter,
}
#[repr(C, align(8))]
pub struct FFloatCameraRigParameterOverride {
    pub value: FFloatCameraParameter,
}
#[repr(C, align(8))]
pub struct FDoubleCameraRigParameterOverride {
    pub value: FDoubleCameraParameter,
}
#[repr(C, align(8))]
pub struct FVector2fCameraRigParameterOverride {
    pub value: FVector2fCameraParameter,
}
#[repr(C, align(8))]
pub struct FVector2dCameraRigParameterOverride {
    pub value: FVector2dCameraParameter,
}
#[repr(C, align(8))]
pub struct FVector3fCameraRigParameterOverride {
    pub value: FVector3fCameraParameter,
}
#[repr(C, align(8))]
pub struct FVector3dCameraRigParameterOverride {
    pub value: FVector3dCameraParameter,
}
#[repr(C, align(16))]
pub struct FVector4fCameraRigParameterOverride {
    pub value: FVector4fCameraParameter,
}
#[repr(C, align(16))]
pub struct FVector4dCameraRigParameterOverride {
    pub value: FVector4dCameraParameter,
}
#[repr(C, align(8))]
pub struct FRotator3fCameraRigParameterOverride {
    pub value: FRotator3fCameraParameter,
}
#[repr(C, align(8))]
pub struct FRotator3dCameraRigParameterOverride {
    pub value: FRotator3dCameraParameter,
}
#[repr(C, align(16))]
pub struct FTransform3fCameraRigParameterOverride {
    pub value: FTransform3fCameraParameter,
}
#[repr(C, align(16))]
pub struct FTransform3dCameraRigParameterOverride {
    pub value: FTransform3dCameraParameter,
}
#[repr(C, align(8))]
pub struct FCameraRigParameterOverrides {
    pub boolean_overrides: TArray<FBooleanCameraRigParameterOverride>,
    pub integer32_overrides: TArray<FInteger32CameraRigParameterOverride>,
    pub float_overrides: TArray<FFloatCameraRigParameterOverride>,
    pub double_overrides: TArray<FDoubleCameraRigParameterOverride>,
    pub vector2f_overrides: TArray<FVector2fCameraRigParameterOverride>,
    pub vector2d_overrides: TArray<FVector2dCameraRigParameterOverride>,
    pub vector3f_overrides: TArray<FVector3fCameraRigParameterOverride>,
    pub vector3d_overrides: TArray<FVector3dCameraRigParameterOverride>,
    pub vector4f_overrides: TArray<FVector4fCameraRigParameterOverride>,
    pub vector4d_overrides: TArray<FVector4dCameraRigParameterOverride>,
    pub rotator3f_overrides: TArray<FRotator3fCameraRigParameterOverride>,
    pub rotator3d_overrides: TArray<FRotator3dCameraRigParameterOverride>,
    pub transform3f_overrides: TArray<FTransform3fCameraRigParameterOverride>,
    pub transform3d_overrides: TArray<FTransform3dCameraRigParameterOverride>,
}
#[repr(C, align(8))]
pub struct FCameraRigAssetReference {
    pub camera_rig: UPtr<UCameraRigAsset>,
    pub parameter_override_guids_deprecated: TArray<FGuid>,
    pub parameter_overrides_deprecated: FCameraRigParameterOverrides,
}
#[repr(C, align(8))]
pub struct FCameraRigProxyRedirectTableEntry {
    pub camera_rig_proxy: UPtr<UCameraRigProxyAsset>,
    pub camera_rig: UPtr<UCameraRigAsset>,
}
#[repr(C, align(8))]
pub struct FCameraRigProxyRedirectTable {
    pub entries: TArray<FCameraRigProxyRedirectTableEntry>,
}
#[repr(C, align(8))]
pub struct FCameraShakeAssetReference {
    pub camera_shake: UPtr<UCameraShakeAsset>,
}
#[repr(C, align(8))]
pub struct FBooleanCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UBooleanCameraVariable>,
}
#[repr(C, align(8))]
pub struct FInteger32CameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UInteger32CameraVariable>,
}
#[repr(C, align(8))]
pub struct FFloatCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UFloatCameraVariable>,
}
#[repr(C, align(8))]
pub struct FDoubleCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UDoubleCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector2fCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector2fCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector2dCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector2dCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector3fCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector3fCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector3dCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector3dCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector4fCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector4fCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector4dCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector4dCameraVariable>,
}
#[repr(C, align(8))]
pub struct FRotator3fCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<URotator3fCameraVariable>,
}
#[repr(C, align(8))]
pub struct FRotator3dCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<URotator3dCameraVariable>,
}
#[repr(C, align(8))]
pub struct FTransform3fCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UTransform3fCameraVariable>,
}
#[repr(C, align(8))]
pub struct FTransform3dCameraVariableReference {
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UTransform3dCameraVariable>,
}
#[repr(C, align(4))]
pub struct FCameraVariableSetterHandle {
    pub value: u32,
    pub serial_number: u32,
}
#[repr(C, align(8))]
pub struct FCustomCameraNodeBlendableParameter {
    pub parameter_name: FName,
    pub parameter_type: ECameraVariableType,
    pub blendable_struct_type: UPtr<UScriptStruct>,
    pub override_variable_id: FCameraVariableID,
    pub override_variable: UPtr<UCameraVariableAsset>,
}
#[repr(C, align(8))]
pub struct FCustomCameraNodeDataParameter {
    pub parameter_name: FName,
    pub parameter_type: ECameraContextDataType,
    pub parameter_container_type: ECameraContextDataContainerType,
    pub parameter_type_object: UPtr<UObject>,
    pub override_data_id: FCameraContextDataID,
}
#[repr(C, align(8))]
pub struct FCustomCameraNodeParameters {
    pub blendable_parameters: TArray<FCustomCameraNodeBlendableParameter>,
    pub data_parameters: TArray<FCustomCameraNodeDataParameter>,
}
#[repr(C, align(8))]
pub struct FCameraRotatorCurve {
    pub curves: FRichCurve,
}
#[repr(C, align(8))]
pub struct FCameraSingleCurve {
    pub curve: FRichCurve,
}
#[repr(C, align(8))]
pub struct FCameraVectorCurve {
    pub curves: FRichCurve,
}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorEvaluationParams {
    pub delta_time: f32,
    pub evaluation_context_owner: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorActivateParams {
    pub evaluation_context_owner: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorDeactivateParams {
    pub evaluation_context_owner: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FCameraDirectorStateTreeEvaluationData {
    pub active_camera_rigs: TArray<UPtr<UCameraRigAsset>>,
    pub active_camera_rig_proxies: TArray<UPtr<UCameraRigProxyAsset>>,
}
#[repr(C, align(8))]
pub struct FGameplayCamerasStateTreeTask {}
#[repr(C, align(8))]
pub struct FGameplayCamerasStateTreeCondition {}
#[repr(C, align(8))]
pub struct FGameplayCamerasActivateCameraRigTaskInstanceData {
    pub camera_rig: UPtr<UCameraRigAsset>,
}
#[repr(C, align(8))]
pub struct FGameplayCamerasActivateCameraRigTask {
    pub b_run_once: bool,
}
#[repr(C, align(8))]
pub struct FGameplayCamerasActivateCameraRigViaProxyTaskInstanceData {
    pub camera_rig_proxy: UPtr<UCameraRigProxyAsset>,
}
#[repr(C, align(8))]
pub struct FGameplayCamerasActivateCameraRigViaProxyTask {
    pub b_run_once: bool,
}
#[repr(C, align(8))]
pub struct FBlueprintCameraEvaluationDataRef {}
#[repr(C, align(8))]
pub struct FBlueprintCameraPose {
    pub location: FVector,
    pub rotation: FRotator,
    pub target_distance: f64,
    pub field_of_view: f32,
    pub focal_length: f32,
    pub orthographic_width: f32,
    pub aperture: f32,
    pub shutter_speed: f32,
    pub focus_distance: f32,
    pub sensor_width: f32,
    pub sensor_height: f32,
    pub sensor_horizontal_offset: f32,
    pub sensor_vertical_offset: f32,
    pub iso: f32,
    pub squeeze_factor: f32,
    pub overscan: f32,
    pub diaphragm_blade_count: i32,
    pub near_clipping_plane: f32,
    pub far_clipping_plane: f32,
    pub physical_camera_blend_weight: f32,
    pub enable_physical_camera: bool,
    pub constrain_aspect_ratio: bool,
    pub override_aspect_ratio_axis_constraint: bool,
    pub aspect_ratio_axis_constraint: EAspectRatioAxisConstraint,
    pub projection_mode: ECameraProjectionMode,
}
#[repr(C, align(4))]
pub struct FPerlinNoiseData {
    pub amplitude: f32,
    pub frequency: f32,
}
#[repr(C, align(8))]
pub struct FCameraActorAttachmentInfo {
    pub actor: UPtr<AActor>,
    pub socket_name: FName,
    pub bone_name: FName,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FSplineOrbitControlPoint {
    pub location_offset: FVector3d,
    pub target_offset: FVector3d,
    pub rotation_offset: FRotator3d,
    pub pitch_angle: f32,
}
#[repr(C, align(8))]
pub struct FCameraActorTargetInfo {
    pub actor: UPtr<AActor>,
    pub socket_name: FName,
    pub bone_name: FName,
    pub target_shape: ECameraTargetShape,
    pub target_size: f32,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FCameraFramingZone {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
#[repr(C, align(8))]
pub struct FCameraFramingZoneParameter {
    pub value: FCameraFramingZone,
    pub variable_id: FCameraVariableID,
}
#[repr(C, align(1))]
pub struct FCameraRigInputSlotParameters {
    pub b_is_accumulated: bool,
    pub b_is_pre_blended: bool,
}
#[repr(C, align(8))]
pub struct FCameraParameterClamping {
    pub min_value: f64,
    pub max_value: f64,
    pub b_clamp_min: bool,
    pub b_clamp_max: bool,
}
#[repr(C, align(8))]
pub struct FCameraParameterNormalization {
    pub max_value: f64,
    pub b_normalize: bool,
}
pub struct UHasCameraBuildStatus {}
pub struct IHasCameraBuildStatus {}
pub struct UCameraRigInstanceFunctions {}
pub struct UAssetReferenceCameraNode {}
pub struct IAssetReferenceCameraNode {}
pub struct UObjectTreeGraphObject {}
pub struct IObjectTreeGraphObject {}
pub struct UObjectTreeGraphRootObject {}
pub struct IObjectTreeGraphRootObject {}
pub struct UBaseCameraObject {
    pub interface: FCameraObjectInterface,
    pub allocation_info: FCameraObjectAllocationInfo,
    pub default_parameters: FInstancedPropertyBag,
    pub parameter_definitions: TArray<FCameraObjectInterfaceParameterDefinition>,
}
pub struct UCameraNode {
    pub b_is_enabled: bool,
    pub graph_node_pos: FIntVector2,
    pub graph_node_comment: FString,
    pub graph_node_pos_x_deprecated: i32,
    pub graph_node_pos_y_deprecated: i32,
}
pub struct UBlendCameraNode {}
pub struct UBlendStackCameraNode {
    pub blend_stack_type: ECameraBlendStackType,
    pub layer: ECameraRigLayer,
}
pub struct UBlendStackRootCameraNode {
    pub blend: UPtr<UBlendCameraNode>,
    pub root_node: UPtr<UCameraNode>,
}
pub struct UCameraAsset {
    pub camera_director: UPtr<UCameraDirector>,
    pub enter_transitions: TArray<UPtr<UCameraRigTransition>>,
    pub exit_transitions: TArray<UPtr<UCameraRigTransition>>,
    pub build_status: ECameraBuildStatus,
    pub default_parameters: FInstancedPropertyBag,
    pub parameter_definitions: TArray<FCameraObjectInterfaceParameterDefinition>,
    pub allocation_info: FCameraAssetAllocationInfo,
    pub camera_rigs_deprecated: TArray<UPtr<UCameraRigAsset>>,
    pub transition_graph_node_pos: FIntVector2,
    pub transition_graph_node_comment: FString,
    pub all_shared_transitions_objects: TArray<UPtr<UObject>>,
}
pub struct UCameraDirector {
    pub camera_rig_proxy_redirect_table: FCameraRigProxyRedirectTable,
    pub camera_rig_proxy_table_deprecated: UPtr<UCameraRigProxyTable>,
}
pub struct UCameraObjectInterfaceParameterBase {
    pub interface_parameter_name: FString,
    pub target: UPtr<UCameraNode>,
    pub target_property_name: FName,
    pub b_has_graph_node: bool,
    pub guid: FGuid,
    pub graph_node_pos: FIntVector2,
}
pub struct UCameraObjectInterfaceBlendableParameter {
    pub parameter_type: ECameraVariableType,
    pub blendable_struct_type: UPtr<UScriptStruct>,
    pub b_is_pre_blended: bool,
    pub private_variable_id: FCameraVariableID,
    pub private_variable_deprecated: UPtr<UCameraVariableAsset>,
}
pub struct UCameraObjectInterfaceDataParameter {
    pub data_type: ECameraContextDataType,
    pub data_container_type: ECameraContextDataContainerType,
    pub data_type_object: UPtr<UObject>,
    pub private_data_id: FCameraContextDataID,
}
pub struct UCameraRigAsset {
    pub root_node: UPtr<UCameraNode>,
    pub gameplay_tags: FGameplayTagContainer,
    pub enter_transitions: TArray<UPtr<UCameraRigTransition>>,
    pub exit_transitions: TArray<UPtr<UCameraRigTransition>>,
    pub initial_orientation: ECameraRigInitialOrientation,
    pub build_status: ECameraBuildStatus,
    pub guid: FGuid,
    pub node_graph_node_pos: FIntVector2,
    pub transition_graph_node_pos: FIntVector2,
    pub node_graph_node_comment: FString,
    pub transition_graph_node_comment: FString,
    pub all_node_tree_objects: TArray<UPtr<UObject>>,
    pub all_transitions_objects: TArray<UPtr<UObject>>,
    pub graph_node_pos_x_deprecated: i32,
    pub graph_node_pos_y_deprecated: i32,
}
pub struct UCombinedCameraRigsCameraNode {
    pub camera_rig_references: TArray<FCameraRigAssetReference>,
}
pub struct UCameraRigProxyAsset {
    pub guid: FGuid,
}
pub struct UCameraRigProxyTable {
    pub entries: TArray<FCameraRigProxyRedirectTableEntry>,
}
pub struct UCameraRigTransitionCondition {
    pub graph_node_pos: FIntVector2,
    pub graph_node_comment: FString,
    pub graph_node_pos_x_deprecated: i32,
    pub graph_node_pos_y_deprecated: i32,
}
pub struct UCameraRigTransition {
    pub conditions: TArray<UPtr<UCameraRigTransitionCondition>>,
    pub blend: UPtr<UBlendCameraNode>,
    pub initial_orientation: ECameraRigInitialOrientation,
    pub b_override_initial_orientation: bool,
    pub b_allow_camera_rig_merging: bool,
    pub graph_node_pos: FIntVector2,
    pub graph_node_comment: FString,
    pub graph_node_pos_x_deprecated: i32,
    pub graph_node_pos_y_deprecated: i32,
}
pub struct UCameraShakeAsset {
    pub root_node: UPtr<UShakeCameraNode>,
    pub blend_in: UPtr<USimpleFixedTimeBlendCameraNode>,
    pub blend_out: UPtr<USimpleFixedTimeBlendCameraNode>,
    pub b_is_single_instance: bool,
    pub build_status: ECameraBuildStatus,
    pub guid: FGuid,
    pub graph_node_pos: FIntVector2,
    pub graph_node_comment: FString,
    pub all_node_objects: TArray<UPtr<UObject>>,
}
pub struct UCameraValueInterpolator {}
pub struct UPopValueInterpolator {}
pub struct UCameraVariableAsset {
    pub display_name: FString,
    pub b_auto_reset: bool,
    pub b_is_pre_blended: bool,
    pub b_is_private: bool,
    pub guid: FGuid,
}
pub struct UBooleanCameraVariable {
    pub b_default_value: bool,
}
pub struct UInteger32CameraVariable {
    pub default_value: i32,
}
pub struct UFloatCameraVariable {
    pub default_value: f32,
}
pub struct UDoubleCameraVariable {
    pub default_value: f64,
}
pub struct UVector2fCameraVariable {
    pub default_value: FVector2f,
}
pub struct UVector2dCameraVariable {
    pub default_value: FVector2D,
}
pub struct UVector3fCameraVariable {
    pub default_value: FVector3f,
}
pub struct UVector3dCameraVariable {
    pub default_value: FVector3d,
}
pub struct UVector4fCameraVariable {
    pub default_value: FVector4f,
}
pub struct UVector4dCameraVariable {
    pub default_value: FVector4d,
}
pub struct URotator3fCameraVariable {
    pub default_value: FRotator3f,
}
pub struct URotator3dCameraVariable {
    pub default_value: FRotator3d,
}
pub struct UTransform3fCameraVariable {
    pub default_value: FTransform3f,
}
pub struct UTransform3dCameraVariable {
    pub default_value: FTransform3d,
}
pub struct UCameraVariableCollection {
    pub variables: TArray<UPtr<UCameraVariableAsset>>,
}
pub struct URootCameraNode {}
pub struct UDefaultRootCameraNode {
    pub base_layer: UPtr<UBlendStackCameraNode>,
    pub main_layer: UPtr<UBlendStackCameraNode>,
    pub global_layer: UPtr<UBlendStackCameraNode>,
    pub visual_layer: UPtr<UBlendStackCameraNode>,
}
pub struct UCustomCameraNodeParameterProvider {}
pub struct ICustomCameraNodeParameterProvider {}
pub struct UObjectTreeGraphComment {
    pub comment_text: FString,
    pub comment_color: FLinearColor,
    pub graph_node_pos: FIntVector2,
    pub graph_node_size: FIntVector2,
}
pub struct UShakeCameraNode {}
pub struct UBlueprintCameraDirectorEvaluator {}
pub struct UBlueprintCameraDirector {
    pub camera_director_evaluator_class: TSubclassOf<UBlueprintCameraDirectorEvaluator>,
}
pub struct UCameraDirectorStateTreeSchema {
    pub context_data_descs: TArray<FStateTreeExternalDataDesc>,
}
pub struct UPriorityQueueCameraDirector {}
pub struct USingleCameraDirector {
    pub camera_rig: UPtr<UCameraRigAsset>,
}
pub struct UStateTreeCameraDirector {
    pub state_tree_reference: FStateTreeReference,
}
pub struct UActivateCameraRigFunctions {}
pub struct UCameraComponentCameraNode {}
pub struct UCalcCameraActorCameraNode {}
pub struct UBlueprintCameraEvaluationDataFunctionLibrary {}
pub struct UBlueprintCameraVariableTableFunctionLibrary {}
pub struct UBlueprintCameraContextDataTableFunctionLibrary {}
pub struct UBlueprintCameraPoseFunctionLibrary {}
pub struct UCameraRigParameterInterop {}
pub struct UCameraRigParameterInteropLibrary {}
pub struct UControllerGameplayCameraEvaluationComponent {
    pub camera_system_host: TScriptInterface<IGameplayCameraSystemHost>,
}
pub struct AGameplayCameraActorBase {}
pub struct AGameplayCameraActor {
    pub camera_component: UPtr<UGameplayCameraComponent>,
}
pub struct UGameplayCameraComponentBase {
    pub auto_activate_for_player: EAutoReceiveInput,
    pub b_set_control_rotation_when_view_target: bool,
    pub b_run_in_editor: bool,
    pub editor_preview_camera_rig_index: i32,
    pub output_camera_component: UPtr<UCineCameraComponent>,
}
pub struct UGameplayCameraComponent {
    pub camera_reference: FCameraAssetReference,
    pub cached_parameter_overrides: FInstancedPropertyBag,
    pub camera_deprecated: UPtr<UCameraAsset>,
}
pub struct UGameplayCameraParameterSetterComponent {
    pub camera_rig_reference: FCameraRigAssetReference,
    pub blend_in_time: f32,
    pub blend_out_time: f32,
    pub blend_type: ECameraVariableSetterBlendType,
}
pub struct AGameplayCameraRigActor {
    pub camera_rig_component: UPtr<UGameplayCameraRigComponent>,
}
pub struct UGameplayCameraRigComponent {
    pub camera_rig_reference: FCameraRigAssetReference,
    pub generated_camera_asset: UPtr<UCameraAsset>,
    pub cached_parameter_overrides: FInstancedPropertyBag,
}
pub struct AGameplayCamerasPlayerCameraManager {
    pub view_rotation_mode: EGameplayCamerasViewRotationMode,
    pub b_override_view_rotation_mode: bool,
    pub original_camera_manager: UPtr<APlayerCameraManager>,
}
pub struct AGameplayCameraSystemActor {
    pub camera_system_component: UPtr<UGameplayCameraSystemComponent>,
}
pub struct UGameplayCameraSystemComponent {
    pub auto_activate_for_player: EAutoReceiveInput,
    pub b_set_player_controller_rotation: bool,
    pub weak_player_controller: TWeakObjectPtr<APlayerController>,
    pub editor_sprite_texture_scale: f32,
}
pub struct UGameplayControlRotationComponent {
    pub axis_actions: TArray<UPtr<UInputAction>>,
    pub axis_action_angular_speed_threshold: f32,
    pub axis_action_magnitude_threshold: f32,
    pub auto_activate_for_player: EAutoReceiveInput,
    pub player_controller: UPtr<APlayerController>,
    pub camera_system_host: TScriptInterface<IGameplayCameraSystemHost>,
}
pub struct UGameplayCameraSystemHost {}
pub struct IGameplayCameraSystemHost {}
pub struct USimpleBlendCameraNode {}
pub struct UViewTargetTransitionParamsBlendCameraNode {
    pub transition_params: FViewTargetTransitionParams,
}
pub struct UGameplayCamerasSettings {
    pub b_auto_build_in_pie: bool,
    pub default_view_rotation_mode: EGameplayCamerasViewRotationMode,
    pub combined_camera_rig_num_threshold: i32,
    pub default_ik_aiming_angle_tolerance: f64,
    pub default_ik_aiming_distance_tolerance: f64,
    pub default_ik_aiming_max_iterations: u8,
    pub default_ik_aiming_min_distance: f64,
}
pub struct UMovieSceneCameraFramingZonePropertySystem {}
pub struct UMovieSceneCameraFramingZoneSection {
    pub left_margin_curve: FMovieSceneDoubleChannel,
    pub top_margin_curve: FMovieSceneDoubleChannel,
    pub right_margin_curve: FMovieSceneDoubleChannel,
    pub bottom_margin_curve: FMovieSceneDoubleChannel,
}
pub struct UMovieSceneCameraFramingZoneTrack {}
pub struct UAttachToActorCameraNode {
    pub attachment: FCameraActorAttachmentInfo,
    pub attachment_data_id: FCameraContextDataID,
    pub attach_to_location: FBooleanCameraParameter,
    pub attach_to_rotation: FBooleanCameraParameter,
}
pub struct UAttachToActorGroupCameraNode {
    pub attachments: TArray<FCameraActorAttachmentInfo>,
    pub attachments_data_id: FCameraContextDataID,
}
pub struct UAttachToPlayerPawnCameraNode {
    pub attach_to_location: FBooleanCameraParameter,
    pub attach_to_rotation: FBooleanCameraParameter,
    pub socket_name: FName,
    pub bone_name: FName,
}
pub struct USimpleFixedTimeBlendCameraNode {
    pub blend_time: FFloatCameraParameter,
}
pub struct ULinearBlendCameraNode {}
pub struct ULocationRotationBlendCameraNode {
    pub location_blend: UPtr<USimpleBlendCameraNode>,
    pub rotation_blend: UPtr<USimpleBlendCameraNode>,
    pub other_blend: UPtr<USimpleBlendCameraNode>,
}
pub struct UOrbitBlendCameraNode {
    pub driving_blend: UPtr<USimpleBlendCameraNode>,
}
pub struct UPopBlendCameraNode {}
pub struct USmoothBlendCameraNode {
    pub blend_type: ESmoothCameraBlendType,
}
pub struct UCollisionPushCameraNode {
    pub safe_position: ECollisionSafePosition,
    pub custom_safe_position: FVector3dCameraVariableReference,
    pub safe_position_offset: FVector3dCameraParameter,
    pub safe_position_offset_space: ECollisionSafePositionOffsetSpace,
    pub enable_collision: FBooleanCameraVariableReference,
    pub collision_sphere_radius: FFloatCameraParameter,
    pub collision_channel: ECollisionChannel,
    pub push_interpolator: UPtr<UCameraValueInterpolator>,
    pub pull_interpolator: UPtr<UCameraValueInterpolator>,
    pub b_run_async_collision: bool,
}
pub struct UOcclusionMaterialCameraNode {
    pub occlusion_transparency_material: UPtr<UMaterialInterface>,
    pub occlusion_sphere_radius: FFloatCameraParameter,
    pub occlusion_channel: ECollisionChannel,
    pub occlusion_target_position: ECameraNodeOriginPosition,
    pub occlusion_target_offset_space: ECameraNodeSpace,
    pub occlusion_target_offset: FVector3dCameraParameter,
}
pub struct UArrayCameraNode {
    pub children: TArray<UPtr<UCameraNode>>,
}
pub struct UAutoFocusCameraNode {
    pub enable_auto_focus: FBooleanCameraVariableReference,
    pub auto_focus_damping_factor: FFloatCameraParameter,
}
pub struct UBodyParametersCameraNode {
    pub shutter_speed: FFloatCameraParameter,
    pub iso: FFloatCameraParameter,
}
pub struct UBoomArmCameraNode {
    pub boom_offset: FVector3dCameraParameter,
    pub boom_length_interpolator: UPtr<UCameraValueInterpolator>,
    pub max_forward_interpolation_factor: FDoubleCameraParameter,
    pub max_backward_interpolation_factor: FDoubleCameraParameter,
    pub input_slot: UPtr<UInput2DCameraNode>,
}
pub struct UCameraRigCameraNode {
    pub camera_rig_reference: FCameraRigAssetReference,
}
pub struct UClippingPlanesCameraNode {
    pub near_plane: FDoubleCameraParameter,
    pub far_plane: FDoubleCameraParameter,
}
pub struct UDampenPositionCameraNode {
    pub forward_damping_factor: FFloatCameraParameter,
    pub lateral_damping_factor: FFloatCameraParameter,
    pub vertical_damping_factor: FFloatCameraParameter,
    pub dampen_space: ECameraNodeSpace,
}
pub struct UDampenRotationCameraNode {
    pub yaw_damping_factor: FFloatCameraParameter,
    pub pitch_damping_factor: FFloatCameraParameter,
    pub roll_damping_factor: FFloatCameraParameter,
}
pub struct UFieldOfViewCameraNode {
    pub field_of_view: FFloatCameraParameter,
}
pub struct UFilmbackCameraNode {
    pub sensor_width: FFloatCameraParameter,
    pub sensor_height: FFloatCameraParameter,
    pub sensor_horizontal_offset: FFloatCameraParameter,
    pub sensor_vertical_offset: FFloatCameraParameter,
    pub overscan: FFloatCameraParameter,
    pub constrain_aspect_ratio: FBooleanCameraParameter,
    pub override_aspect_ratio_axis_constraint: FBooleanCameraParameter,
    pub aspect_ratio_axis_constraint: EAspectRatioAxisConstraint,
}
pub struct ULensParametersCameraNode {
    pub focal_length: FFloatCameraParameter,
    pub focus_distance: FFloatCameraParameter,
    pub aperture: FFloatCameraParameter,
    pub enable_physical_camera: FBooleanCameraParameter,
}
pub struct UOffsetCameraNode {
    pub translation_offset: FVector3dCameraParameter,
    pub rotation_offset: FRotator3dCameraParameter,
    pub offset_space: ECameraNodeSpace,
}
pub struct UOrthographicCameraNode {
    pub enable_orthographic_mode: FBooleanCameraParameter,
    pub orthographic_width: FFloatCameraParameter,
}
pub struct UPostProcessCameraNode {
    pub post_process_settings: FPostProcessSettings,
}
pub struct USetLocationCameraNode {
    pub location: FVector3dCameraParameter,
    pub offset_space: ECameraNodeSpace,
}
pub struct USetRotationCameraNode {
    pub rotation: FRotator3dCameraParameter,
    pub offset_space: ECameraNodeSpace,
}
pub struct USplineFieldOfViewCameraNode {
    pub spline_input: FFloatCameraParameter,
    pub field_of_view_spline: FCameraSingleCurve,
}
pub struct USplineOffsetCameraNode {
    pub spline_input: FFloatCameraParameter,
    pub translation_offset_spline: FCameraVectorCurve,
    pub rotation_offset_spline: FCameraRotatorCurve,
    pub offset_space: ECameraNodeSpace,
}
pub struct USplineOrbitCameraNode {
    pub location_offset_spline: FCameraVectorCurve,
    pub target_offset_spline: FCameraVectorCurve,
    pub rotation_offset_spline: FCameraRotatorCurve,
    pub location_offset_multiplier: FFloatCameraParameter,
    pub target_offset_space: ECameraNodeSpace,
    pub input_slot: UPtr<UInput2DCameraNode>,
}
pub struct UTargetRayCastCameraNode {
    pub trace_channel: ECollisionChannel,
    pub auto_focus: FBooleanCameraParameter,
}
pub struct UBaseFramingCameraNode {
    pub target_location: FVector3dCameraVariableReference,
    pub target_infos: TArray<FCameraActorTargetInfo>,
    pub target_infos_data_id: FCameraContextDataID,
    pub set_target_distance: FBooleanCameraParameter,
    pub initialize_with_ideal_framing: FBooleanCameraParameter,
    pub ideal_framing_location: FVector2dCameraParameter,
    pub reframe_damping_factor: FFloatCameraParameter,
    pub low_reframe_damping_factor: FFloatCameraParameter,
    pub reengage_time: FFloatCameraParameter,
    pub disengage_time: FFloatCameraParameter,
    pub target_movement_anticipation_time: FFloatCameraParameter,
    pub dead_zone: FCameraFramingZoneParameter,
    pub soft_zone: FCameraFramingZoneParameter,
    pub target_info_deprecated: FCameraActorTargetInfo,
}
pub struct UDollyFramingCameraNode {
    pub can_move_laterally: FBooleanCameraParameter,
    pub can_move_vertically: FBooleanCameraParameter,
}
pub struct UPanningFramingCameraNode {
    pub can_pan_laterally: FBooleanCameraParameter,
    pub can_pan_vertically: FBooleanCameraParameter,
}
pub struct UInput2DCameraNode {}
pub struct UAutoRotateInput2DCameraNode {
    pub direction: ECameraAutoRotateDirection,
    pub direction_vector: FVector3dCameraVariableReference,
    pub wait_time: FFloatCameraParameter,
    pub deactivation_threshold: FFloatCameraParameter,
    pub interpolator: UPtr<UCameraValueInterpolator>,
    pub freeze_control_rotation: FBooleanCameraParameter,
    pub enable_auto_rotate: FBooleanCameraParameter,
    pub auto_rotate_yaw: FBooleanCameraParameter,
    pub auto_rotate_pitch: FBooleanCameraParameter,
    pub input_node: UPtr<UInput2DCameraNode>,
}
pub struct UInput1DCameraNode {}
pub struct UCameraRigInput1DSlot {
    pub clamp: FCameraParameterClamping,
    pub normalize: FCameraParameterNormalization,
    pub revert_axis: FBooleanCameraParameter,
    pub speed: FDoubleCameraParameter,
    pub built_in_variable: EBuiltInDoubleCameraVariable,
    pub custom_variable: FDoubleCameraVariableReference,
    pub b_is_pre_blended: bool,
    pub input_slot_parameters_deprecated: FCameraRigInputSlotParameters,
    pub speed_variable_id: FCameraVariableID,
    pub variable_id: FCameraVariableID,
}
pub struct UCameraRigInput2DSlot {
    pub clamp_x: FCameraParameterClamping,
    pub clamp_y: FCameraParameterClamping,
    pub normalize_x: FCameraParameterNormalization,
    pub normalize_y: FCameraParameterNormalization,
    pub revert_axis_x: FBooleanCameraParameter,
    pub revert_axis_y: FBooleanCameraParameter,
    pub speed: FVector2dCameraParameter,
    pub built_in_variable: EBuiltInVector2dCameraVariable,
    pub custom_variable: FVector2dCameraVariableReference,
    pub b_is_pre_blended: bool,
    pub input_slot_parameters_deprecated: FCameraRigInputSlotParameters,
    pub speed_variable_id: FCameraVariableID,
    pub variable_id: FCameraVariableID,
}
pub struct UDrivenControlRotationCameraNode {}
pub struct UInputAccumulator2DCameraNode {
    pub input_slot: UPtr<UInput2DCameraNode>,
}
pub struct UInputAxisBinding2DCameraNode {
    pub axis_actions: TArray<UPtr<UInputAction>>,
    pub multiplier: FVector2dCameraParameter,
    pub b_is_accumulated: bool,
}
pub struct URawInputAxisBinding2DCameraNode {
    pub axis_actions: TArray<UPtr<UInputAction>>,
    pub multiplier: FVector2dCameraParameter,
}
pub struct UCameraShakeCameraNode {
    pub camera_shake_reference: FCameraShakeAssetReference,
    pub evaluation_mode: ECameraShakeEvaluationMode,
}
pub struct UCompositeShakeCameraNode {
    pub shakes: TArray<UPtr<UShakeCameraNode>>,
}
pub struct UEnvelopeShakeCameraNode {
    pub ease_in_time: FFloatCameraParameter,
    pub ease_out_time: FFloatCameraParameter,
    pub total_time: FFloatCameraParameter,
    pub shake: UPtr<UShakeCameraNode>,
}
pub struct UPerlinNoiseLocationShakeCameraNode {
    pub amplitude_multiplier: FFloatCameraParameter,
    pub frequency_multiplier: FFloatCameraParameter,
    pub octaves: FInteger32CameraParameter,
    pub x: FPerlinNoiseData,
    pub y: FPerlinNoiseData,
    pub z: FPerlinNoiseData,
}
pub struct UPerlinNoiseRotationShakeCameraNode {
    pub amplitude_multiplier: FFloatCameraParameter,
    pub frequency_multiplier: FFloatCameraParameter,
    pub octaves: FInteger32CameraParameter,
    pub yaw: FPerlinNoiseData,
    pub pitch: FPerlinNoiseData,
    pub roll: FPerlinNoiseData,
}
pub struct UBlueprintCameraNodeEvaluator {
    pub b_is_first_frame: bool,
    pub b_is_active_camera_rig: bool,
    pub evaluation_context_owner: UPtr<UObject>,
    pub camera_data: FBlueprintCameraEvaluationDataRef,
    pub camera_pose: FBlueprintCameraPose,
    pub variable_table: FBlueprintCameraEvaluationDataRef,
}
pub struct UBlueprintCameraNode {
    pub camera_node_evaluator_template: UPtr<UBlueprintCameraNodeEvaluator>,
    pub camera_node_evaluator_overrides: FCustomCameraNodeParameters,
    pub camera_node_evaluator_class_deprecated: TSubclassOf<
        UBlueprintCameraNodeEvaluator,
    >,
}
pub struct UCameraShakeServiceCameraNode {}
pub struct UUpdateTrackerCameraNode {
    pub double_parameter: FDoubleCameraParameter,
    pub vector_parameter: FVector3dCameraParameter,
}
pub struct UFixedTestCameraDirector {
    pub camera_rigs: TArray<UPtr<UCameraRigAsset>>,
    pub camera_rig_names: TArray<FName>,
}
pub struct UIsCameraRigTransitionCondition {
    pub previous_camera_rig: UPtr<UCameraRigAsset>,
    pub next_camera_rig: UPtr<UCameraRigAsset>,
}
pub struct UGameplayTagTransitionCondition {
    pub previous_gameplay_tag_query: FGameplayTagQuery,
    pub next_gameplay_tag_query: FGameplayTagQuery,
}
pub struct UAccelerationDecelerationValueInterpolator {
    pub acceleration: f32,
    pub max_speed: f32,
    pub deceleration: f32,
}
pub struct UCriticalDamperValueInterpolator {
    pub damping_factor: f32,
}
pub struct UDoubleIIRValueInterpolator {
    pub primary_speed: f32,
    pub intermediate_speed: f32,
    pub b_use_fixed_step: bool,
}
pub struct UIIRValueInterpolator {
    pub speed: f32,
    pub b_use_fixed_step: bool,
}
