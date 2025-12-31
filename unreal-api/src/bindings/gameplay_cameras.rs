#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub parameter_guid: crate::bindings::core_u_object::FGuid,
    pub parameter_type: ECameraObjectInterfaceParameterType,
    pub variable_id: FCameraVariableID,
    pub variable_type: ECameraVariableType,
    pub blendable_struct_type: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub data_id: FCameraContextDataID,
    pub data_type: ECameraContextDataType,
    pub data_container_type: ECameraContextDataContainerType,
    pub data_type_object: UPtr<crate::bindings::core_u_object::UObject>,
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
    pub data_type_object: UPtr<crate::bindings::core_u_object::UObject>,
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
    pub blendable_struct_type: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub b_is_private: bool,
    pub b_is_input: bool,
    pub b_auto_reset: bool,
    pub variable_name: FString,
}
#[repr(C, align(4))]
pub struct FCameraObjectInterfaceParameterMetaData {
    pub parameter_guid: crate::bindings::core_u_object::FGuid,
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
    pub overriden_property_i_ds: TArray<crate::bindings::core_u_object::FGuid>,
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
    pub parameter_override_guids_deprecated: TArray<
        crate::bindings::core_u_object::FGuid,
    >,
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
    pub value: crate::bindings::core_u_object::FVector2f,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector2fCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector2dCameraParameter {
    pub value: crate::bindings::core_u_object::FVector2D,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector2dCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector3fCameraParameter {
    pub value: crate::bindings::core_u_object::FVector3f,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector3fCameraVariable>,
}
#[repr(C, align(8))]
pub struct FVector3dCameraParameter {
    pub value: crate::bindings::core_u_object::FVector,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector3dCameraVariable>,
}
#[repr(C, align(16))]
pub struct FVector4fCameraParameter {
    pub value: crate::bindings::core_u_object::FVector4f,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector4fCameraVariable>,
}
#[repr(C, align(16))]
pub struct FVector4dCameraParameter {
    pub value: crate::bindings::core_u_object::FVector4,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UVector4dCameraVariable>,
}
#[repr(C, align(8))]
pub struct FRotator3fCameraParameter {
    pub value: crate::bindings::core_u_object::FRotator3f,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<URotator3fCameraVariable>,
}
#[repr(C, align(8))]
pub struct FRotator3dCameraParameter {
    pub value: crate::bindings::core_u_object::FRotator,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<URotator3dCameraVariable>,
}
#[repr(C, align(16))]
pub struct FTransform3fCameraParameter {
    pub value: crate::bindings::core_u_object::FTransform3f,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UTransform3fCameraVariable>,
}
#[repr(C, align(16))]
pub struct FTransform3dCameraParameter {
    pub value: crate::bindings::core_u_object::FTransform,
    pub variable_id: FCameraVariableID,
    pub variable: UPtr<UTransform3dCameraVariable>,
}
#[repr(C, align(8))]
pub struct FCameraPose {
    pub location: crate::bindings::core_u_object::FVector3d,
    pub rotation: crate::bindings::core_u_object::FRotator3d,
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
    pub aspect_ratio_axis_constraint: crate::bindings::engine::EAspectRatioAxisConstraint,
    pub projection_mode: crate::bindings::engine::ECameraProjectionMode,
}
#[repr(C, align(8))]
pub struct FCameraRigParameterOverrideBase {
    pub interface_parameter_guid: crate::bindings::core_u_object::FGuid,
    pub private_variable_guid: crate::bindings::core_u_object::FGuid,
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
    pub parameter_override_guids_deprecated: TArray<
        crate::bindings::core_u_object::FGuid,
    >,
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
    pub blendable_struct_type: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub override_variable_id: FCameraVariableID,
    pub override_variable: UPtr<UCameraVariableAsset>,
}
#[repr(C, align(8))]
pub struct FCustomCameraNodeDataParameter {
    pub parameter_name: FName,
    pub parameter_type: ECameraContextDataType,
    pub parameter_container_type: ECameraContextDataContainerType,
    pub parameter_type_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub override_data_id: FCameraContextDataID,
}
#[repr(C, align(8))]
pub struct FCustomCameraNodeParameters {
    pub blendable_parameters: TArray<FCustomCameraNodeBlendableParameter>,
    pub data_parameters: TArray<FCustomCameraNodeDataParameter>,
}
#[repr(C, align(8))]
pub struct FCameraRotatorCurve {
    pub curves: crate::bindings::engine::FRichCurve,
}
#[repr(C, align(8))]
pub struct FCameraSingleCurve {
    pub curve: crate::bindings::engine::FRichCurve,
}
#[repr(C, align(8))]
pub struct FCameraVectorCurve {
    pub curves: crate::bindings::engine::FRichCurve,
}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorEvaluationParams {
    pub delta_time: f32,
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorActivateParams {
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorDeactivateParams {
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
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
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
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
    pub aspect_ratio_axis_constraint: crate::bindings::engine::EAspectRatioAxisConstraint,
    pub projection_mode: crate::bindings::engine::ECameraProjectionMode,
}
#[repr(C, align(4))]
pub struct FPerlinNoiseData {
    pub amplitude: f32,
    pub frequency: f32,
}
#[repr(C, align(8))]
pub struct FCameraActorAttachmentInfo {
    pub actor: UPtr<crate::bindings::engine::AActor>,
    pub socket_name: FName,
    pub bone_name: FName,
    pub weight: f32,
}
#[repr(C, align(8))]
pub struct FSplineOrbitControlPoint {
    pub location_offset: crate::bindings::core_u_object::FVector3d,
    pub target_offset: crate::bindings::core_u_object::FVector3d,
    pub rotation_offset: crate::bindings::core_u_object::FRotator3d,
    pub pitch_angle: f32,
}
#[repr(C, align(8))]
pub struct FCameraActorTargetInfo {
    pub actor: UPtr<crate::bindings::engine::AActor>,
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
    pub default_parameters: crate::bindings::core_u_object::FInstancedPropertyBag,
    pub parameter_definitions: TArray<FCameraObjectInterfaceParameterDefinition>,
}
pub struct UCameraNode {
    pub b_is_enabled: bool,
    pub graph_node_pos: crate::bindings::core_u_object::FIntVector2,
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
    pub default_parameters: crate::bindings::core_u_object::FInstancedPropertyBag,
    pub parameter_definitions: TArray<FCameraObjectInterfaceParameterDefinition>,
    pub allocation_info: FCameraAssetAllocationInfo,
    pub camera_rigs_deprecated: TArray<UPtr<UCameraRigAsset>>,
    pub transition_graph_node_pos: crate::bindings::core_u_object::FIntVector2,
    pub transition_graph_node_comment: FString,
    pub all_shared_transitions_objects: TArray<
        UPtr<crate::bindings::core_u_object::UObject>,
    >,
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
    pub guid: crate::bindings::core_u_object::FGuid,
    pub graph_node_pos: crate::bindings::core_u_object::FIntVector2,
}
pub struct UCameraObjectInterfaceBlendableParameter {
    pub parameter_type: ECameraVariableType,
    pub blendable_struct_type: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub b_is_pre_blended: bool,
    pub private_variable_id: FCameraVariableID,
    pub private_variable_deprecated: UPtr<UCameraVariableAsset>,
}
pub struct UCameraObjectInterfaceDataParameter {
    pub data_type: ECameraContextDataType,
    pub data_container_type: ECameraContextDataContainerType,
    pub data_type_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub private_data_id: FCameraContextDataID,
}
pub struct UCameraRigAsset {
    pub root_node: UPtr<UCameraNode>,
    pub gameplay_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub enter_transitions: TArray<UPtr<UCameraRigTransition>>,
    pub exit_transitions: TArray<UPtr<UCameraRigTransition>>,
    pub initial_orientation: ECameraRigInitialOrientation,
    pub build_status: ECameraBuildStatus,
    pub guid: crate::bindings::core_u_object::FGuid,
    pub node_graph_node_pos: crate::bindings::core_u_object::FIntVector2,
    pub transition_graph_node_pos: crate::bindings::core_u_object::FIntVector2,
    pub node_graph_node_comment: FString,
    pub transition_graph_node_comment: FString,
    pub all_node_tree_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub all_transitions_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub graph_node_pos_x_deprecated: i32,
    pub graph_node_pos_y_deprecated: i32,
}
pub struct UCombinedCameraRigsCameraNode {
    pub camera_rig_references: TArray<FCameraRigAssetReference>,
}
pub struct UCameraRigProxyAsset {
    pub guid: crate::bindings::core_u_object::FGuid,
}
pub struct UCameraRigProxyTable {
    pub entries: TArray<FCameraRigProxyRedirectTableEntry>,
}
pub struct UCameraRigTransitionCondition {
    pub graph_node_pos: crate::bindings::core_u_object::FIntVector2,
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
    pub graph_node_pos: crate::bindings::core_u_object::FIntVector2,
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
    pub guid: crate::bindings::core_u_object::FGuid,
    pub graph_node_pos: crate::bindings::core_u_object::FIntVector2,
    pub graph_node_comment: FString,
    pub all_node_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
pub struct UCameraValueInterpolator {}
pub struct UPopValueInterpolator {}
pub struct UCameraVariableAsset {
    pub display_name: FString,
    pub b_auto_reset: bool,
    pub b_is_pre_blended: bool,
    pub b_is_private: bool,
    pub guid: crate::bindings::core_u_object::FGuid,
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
    pub default_value: crate::bindings::core_u_object::FVector2f,
}
pub struct UVector2dCameraVariable {
    pub default_value: crate::bindings::core_u_object::FVector2D,
}
pub struct UVector3fCameraVariable {
    pub default_value: crate::bindings::core_u_object::FVector3f,
}
pub struct UVector3dCameraVariable {
    pub default_value: crate::bindings::core_u_object::FVector3d,
}
pub struct UVector4fCameraVariable {
    pub default_value: crate::bindings::core_u_object::FVector4f,
}
pub struct UVector4dCameraVariable {
    pub default_value: crate::bindings::core_u_object::FVector4d,
}
pub struct URotator3fCameraVariable {
    pub default_value: crate::bindings::core_u_object::FRotator3f,
}
pub struct URotator3dCameraVariable {
    pub default_value: crate::bindings::core_u_object::FRotator3d,
}
pub struct UTransform3fCameraVariable {
    pub default_value: crate::bindings::core_u_object::FTransform3f,
}
pub struct UTransform3dCameraVariable {
    pub default_value: crate::bindings::core_u_object::FTransform3d,
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
    pub comment_color: crate::bindings::core_u_object::FLinearColor,
    pub graph_node_pos: crate::bindings::core_u_object::FIntVector2,
    pub graph_node_size: crate::bindings::core_u_object::FIntVector2,
}
pub struct UShakeCameraNode {}
pub struct UBlueprintCameraDirectorEvaluator {}
pub struct UBlueprintCameraDirector {
    pub camera_director_evaluator_class: TSubclassOf<UBlueprintCameraDirectorEvaluator>,
}
pub struct UCameraDirectorStateTreeSchema {
    pub context_data_descs: TArray<
        crate::bindings::state_tree_module::FStateTreeExternalDataDesc,
    >,
}
pub struct UPriorityQueueCameraDirector {}
pub struct USingleCameraDirector {
    pub camera_rig: UPtr<UCameraRigAsset>,
}
pub struct UStateTreeCameraDirector {
    pub state_tree_reference: crate::bindings::state_tree_module::FStateTreeReference,
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
    pub auto_activate_for_player: crate::bindings::engine::EAutoReceiveInput,
    pub b_set_control_rotation_when_view_target: bool,
    pub b_run_in_editor: bool,
    pub editor_preview_camera_rig_index: i32,
    pub output_camera_component: UPtr<
        crate::bindings::cinematic_camera::UCineCameraComponent,
    >,
}
pub struct UGameplayCameraComponent {
    pub camera_reference: FCameraAssetReference,
    pub cached_parameter_overrides: crate::bindings::core_u_object::FInstancedPropertyBag,
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
    pub cached_parameter_overrides: crate::bindings::core_u_object::FInstancedPropertyBag,
}
pub struct AGameplayCamerasPlayerCameraManager {
    pub view_rotation_mode: EGameplayCamerasViewRotationMode,
    pub b_override_view_rotation_mode: bool,
    pub original_camera_manager: UPtr<crate::bindings::engine::APlayerCameraManager>,
}
pub struct AGameplayCameraSystemActor {
    pub camera_system_component: UPtr<UGameplayCameraSystemComponent>,
}
pub struct UGameplayCameraSystemComponent {
    pub auto_activate_for_player: crate::bindings::engine::EAutoReceiveInput,
    pub b_set_player_controller_rotation: bool,
    pub weak_player_controller: TWeakObjectPtr<
        crate::bindings::engine::APlayerController,
    >,
    pub editor_sprite_texture_scale: f32,
}
pub struct UGameplayControlRotationComponent {
    pub axis_actions: TArray<UPtr<crate::bindings::enhanced_input::UInputAction>>,
    pub axis_action_angular_speed_threshold: f32,
    pub axis_action_magnitude_threshold: f32,
    pub auto_activate_for_player: crate::bindings::engine::EAutoReceiveInput,
    pub player_controller: UPtr<crate::bindings::engine::APlayerController>,
    pub camera_system_host: TScriptInterface<IGameplayCameraSystemHost>,
}
pub struct UGameplayCameraSystemHost {}
pub struct IGameplayCameraSystemHost {}
pub struct USimpleBlendCameraNode {}
pub struct UViewTargetTransitionParamsBlendCameraNode {
    pub transition_params: crate::bindings::engine::FViewTargetTransitionParams,
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
    pub left_margin_curve: crate::bindings::movie_scene::FMovieSceneDoubleChannel,
    pub top_margin_curve: crate::bindings::movie_scene::FMovieSceneDoubleChannel,
    pub right_margin_curve: crate::bindings::movie_scene::FMovieSceneDoubleChannel,
    pub bottom_margin_curve: crate::bindings::movie_scene::FMovieSceneDoubleChannel,
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
    pub collision_channel: crate::bindings::engine::ECollisionChannel,
    pub push_interpolator: UPtr<UCameraValueInterpolator>,
    pub pull_interpolator: UPtr<UCameraValueInterpolator>,
    pub b_run_async_collision: bool,
}
pub struct UOcclusionMaterialCameraNode {
    pub occlusion_transparency_material: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub occlusion_sphere_radius: FFloatCameraParameter,
    pub occlusion_channel: crate::bindings::engine::ECollisionChannel,
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
    pub aspect_ratio_axis_constraint: crate::bindings::engine::EAspectRatioAxisConstraint,
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
    pub post_process_settings: crate::bindings::engine::FPostProcessSettings,
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
    pub trace_channel: crate::bindings::engine::ECollisionChannel,
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
    pub axis_actions: TArray<UPtr<crate::bindings::enhanced_input::UInputAction>>,
    pub multiplier: FVector2dCameraParameter,
    pub b_is_accumulated: bool,
}
pub struct URawInputAxisBinding2DCameraNode {
    pub axis_actions: TArray<UPtr<crate::bindings::enhanced_input::UInputAction>>,
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
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
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
    pub previous_gameplay_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
    pub next_gameplay_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraObjectInterfaceParameterType(pub u8);
impl ECameraObjectInterfaceParameterType {
    pub const BLENDABLE: ECameraObjectInterfaceParameterType = ECameraObjectInterfaceParameterType(
        0,
    );
    pub const DATA: ECameraObjectInterfaceParameterType = ECameraObjectInterfaceParameterType(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraVariableType(pub i32);
impl ECameraVariableType {
    pub const BOOLEAN: ECameraVariableType = ECameraVariableType(0);
    pub const INTEGER32: ECameraVariableType = ECameraVariableType(1);
    pub const FLOAT: ECameraVariableType = ECameraVariableType(2);
    pub const DOUBLE: ECameraVariableType = ECameraVariableType(3);
    pub const VECTOR2F: ECameraVariableType = ECameraVariableType(4);
    pub const VECTOR2D: ECameraVariableType = ECameraVariableType(5);
    pub const VECTOR3F: ECameraVariableType = ECameraVariableType(6);
    pub const VECTOR3D: ECameraVariableType = ECameraVariableType(7);
    pub const VECTOR4F: ECameraVariableType = ECameraVariableType(8);
    pub const VECTOR4D: ECameraVariableType = ECameraVariableType(9);
    pub const ROTATOR3F: ECameraVariableType = ECameraVariableType(10);
    pub const ROTATOR3D: ECameraVariableType = ECameraVariableType(11);
    pub const TRANSFORM3F: ECameraVariableType = ECameraVariableType(12);
    pub const TRANSFORM3D: ECameraVariableType = ECameraVariableType(13);
    pub const BLENDABLE_STRUCT: ECameraVariableType = ECameraVariableType(14);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraContextDataType(pub i32);
impl ECameraContextDataType {
    pub const NAME: ECameraContextDataType = ECameraContextDataType(0);
    pub const STRING: ECameraContextDataType = ECameraContextDataType(1);
    pub const ENUM: ECameraContextDataType = ECameraContextDataType(2);
    pub const STRUCT: ECameraContextDataType = ECameraContextDataType(3);
    pub const OBJECT: ECameraContextDataType = ECameraContextDataType(4);
    pub const CLASS: ECameraContextDataType = ECameraContextDataType(5);
    pub const COUNT: ECameraContextDataType = ECameraContextDataType(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraContextDataContainerType(pub i32);
impl ECameraContextDataContainerType {
    pub const NONE: ECameraContextDataContainerType = ECameraContextDataContainerType(0);
    pub const ARRAY: ECameraContextDataContainerType = ECameraContextDataContainerType(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraRigLayer(pub u8);
impl ECameraRigLayer {
    pub const NONE: ECameraRigLayer = ECameraRigLayer(0);
    pub const BASE: ECameraRigLayer = ECameraRigLayer(1);
    pub const MAIN: ECameraRigLayer = ECameraRigLayer(2);
    pub const GLOBAL: ECameraRigLayer = ECameraRigLayer(3);
    pub const VISUAL: ECameraRigLayer = ECameraRigLayer(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraTargetShape(pub u8);
impl ECameraTargetShape {
    pub const POINT: ECameraTargetShape = ECameraTargetShape(0);
    pub const AUTOMATIC_BOUNDS: ECameraTargetShape = ECameraTargetShape(1);
    pub const MANUAL_BOUNDS: ECameraTargetShape = ECameraTargetShape(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraEvaluationDataCondition(pub u8);
impl ECameraEvaluationDataCondition {
    pub const ACTIVE_CAMERA_RIG: ECameraEvaluationDataCondition = ECameraEvaluationDataCondition(
        0,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGameplayCameraComponentActivationMode(pub u8);
impl EGameplayCameraComponentActivationMode {
    pub const PUSH: EGameplayCameraComponentActivationMode = EGameplayCameraComponentActivationMode(
        0,
    );
    pub const PUSH_AND_INSERT: EGameplayCameraComponentActivationMode = EGameplayCameraComponentActivationMode(
        1,
    );
    pub const INSERT_OR_PUSH: EGameplayCameraComponentActivationMode = EGameplayCameraComponentActivationMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraBlendStackType(pub i32);
impl ECameraBlendStackType {
    pub const ISOLATED_TRANSIENT: ECameraBlendStackType = ECameraBlendStackType(0);
    pub const ADDITIVE_PERSISTENT: ECameraBlendStackType = ECameraBlendStackType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraBuildStatus(pub u8);
impl ECameraBuildStatus {
    pub const CLEAN: ECameraBuildStatus = ECameraBuildStatus(0);
    pub const CLEAN_WITH_WARNINGS: ECameraBuildStatus = ECameraBuildStatus(1);
    pub const WITH_ERRORS: ECameraBuildStatus = ECameraBuildStatus(2);
    pub const DIRTY: ECameraBuildStatus = ECameraBuildStatus(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraRigInitialOrientation(pub i32);
impl ECameraRigInitialOrientation {
    pub const NONE: ECameraRigInitialOrientation = ECameraRigInitialOrientation(0);
    pub const CONTEXT_YAW_PITCH: ECameraRigInitialOrientation = ECameraRigInitialOrientation(
        1,
    );
    pub const PREVIOUS_YAW_PITCH: ECameraRigInitialOrientation = ECameraRigInitialOrientation(
        2,
    );
    pub const PREVIOUS_ABSOLUTE_TARGET: ECameraRigInitialOrientation = ECameraRigInitialOrientation(
        3,
    );
    pub const PREVIOUS_RELATIVE_TARGET: ECameraRigInitialOrientation = ECameraRigInitialOrientation(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraVariableSetterBlendType(pub u8);
impl ECameraVariableSetterBlendType {
    pub const NONE: ECameraVariableSetterBlendType = ECameraVariableSetterBlendType(0);
    pub const LINEAR: ECameraVariableSetterBlendType = ECameraVariableSetterBlendType(1);
    pub const SMOOTH_STEP: ECameraVariableSetterBlendType = ECameraVariableSetterBlendType(
        2,
    );
    pub const SMOOTHER_STEP: ECameraVariableSetterBlendType = ECameraVariableSetterBlendType(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGameplayCamerasViewRotationMode(pub i32);
impl EGameplayCamerasViewRotationMode {
    pub const NONE: EGameplayCamerasViewRotationMode = EGameplayCamerasViewRotationMode(
        0,
    );
    pub const PREVIEW_UPDATE: EGameplayCamerasViewRotationMode = EGameplayCamerasViewRotationMode(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESmoothCameraBlendType(pub i32);
impl ESmoothCameraBlendType {
    pub const SMOOTH_STEP: ESmoothCameraBlendType = ESmoothCameraBlendType(0);
    pub const SMOOTHER_STEP: ESmoothCameraBlendType = ESmoothCameraBlendType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECollisionSafePosition(pub u8);
impl ECollisionSafePosition {
    pub const ACTIVE_CONTEXT: ECollisionSafePosition = ECollisionSafePosition(0);
    pub const OWNING_CONTEXT: ECollisionSafePosition = ECollisionSafePosition(1);
    pub const PIVOT: ECollisionSafePosition = ECollisionSafePosition(2);
    pub const PAWN: ECollisionSafePosition = ECollisionSafePosition(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECollisionSafePositionOffsetSpace(pub u8);
impl ECollisionSafePositionOffsetSpace {
    pub const ACTIVE_CONTEXT: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        0,
    );
    pub const OWNING_CONTEXT: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        1,
    );
    pub const PIVOT: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        2,
    );
    pub const CAMERA_POSE: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        3,
    );
    pub const PAWN: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraNodeOriginPosition(pub u8);
impl ECameraNodeOriginPosition {
    pub const CAMERA_POSE: ECameraNodeOriginPosition = ECameraNodeOriginPosition(0);
    pub const ACTIVE_CONTEXT: ECameraNodeOriginPosition = ECameraNodeOriginPosition(1);
    pub const OWNING_CONTEXT: ECameraNodeOriginPosition = ECameraNodeOriginPosition(2);
    pub const PIVOT: ECameraNodeOriginPosition = ECameraNodeOriginPosition(3);
    pub const PAWN: ECameraNodeOriginPosition = ECameraNodeOriginPosition(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraNodeSpace(pub u8);
impl ECameraNodeSpace {
    pub const CAMERA_POSE: ECameraNodeSpace = ECameraNodeSpace(0);
    pub const ACTIVE_CONTEXT: ECameraNodeSpace = ECameraNodeSpace(1);
    pub const OWNING_CONTEXT: ECameraNodeSpace = ECameraNodeSpace(2);
    pub const PIVOT: ECameraNodeSpace = ECameraNodeSpace(3);
    pub const PAWN: ECameraNodeSpace = ECameraNodeSpace(4);
    pub const WORLD: ECameraNodeSpace = ECameraNodeSpace(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraAutoRotateDirection(pub i32);
impl ECameraAutoRotateDirection {
    pub const FACING: ECameraAutoRotateDirection = ECameraAutoRotateDirection(0);
    pub const MOVEMENT: ECameraAutoRotateDirection = ECameraAutoRotateDirection(1);
    pub const MOVEMENT_OR_FACING: ECameraAutoRotateDirection = ECameraAutoRotateDirection(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBuiltInDoubleCameraVariable(pub i32);
impl EBuiltInDoubleCameraVariable {
    pub const NONE: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(0);
    pub const YAW: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(1);
    pub const PITCH: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(2);
    pub const ROLL: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(3);
    pub const ZOOM: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBuiltInVector2dCameraVariable(pub i32);
impl EBuiltInVector2dCameraVariable {
    pub const NONE: EBuiltInVector2dCameraVariable = EBuiltInVector2dCameraVariable(0);
    pub const YAW_PITCH: EBuiltInVector2dCameraVariable = EBuiltInVector2dCameraVariable(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECameraShakeEvaluationMode(pub u8);
impl ECameraShakeEvaluationMode {
    pub const INLINE: ECameraShakeEvaluationMode = ECameraShakeEvaluationMode(0);
    pub const VISUAL_LAYER: ECameraShakeEvaluationMode = ECameraShakeEvaluationMode(1);
}
