#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FCameraContextDataID {
    __padding_end: [u8; 4],
}
impl FCameraContextDataID {}
#[repr(C, align(4))]
pub struct FCameraRigInstanceID {
    __padding_end: [u8; 8],
}
impl FCameraRigInstanceID {}
#[repr(C, align(4))]
pub struct FCameraShakeInstanceID {
    __padding_end: [u8; 4],
}
impl FCameraShakeInstanceID {}
#[repr(C, align(8))]
pub struct FBaseCameraObjectReference {
    __padding_end: [u8; 56],
}
impl FBaseCameraObjectReference {}
#[repr(C, align(8))]
pub struct FCameraAssetReference {
    __padding_end: [u8; 56],
}
impl FCameraAssetReference {}
#[repr(C, align(8))]
pub struct FBooleanCameraParameter {
    pub value: bool,
    __padding_end: [u8; 15],
}
impl FBooleanCameraParameter {}
#[repr(C, align(8))]
pub struct FInteger32CameraParameter {
    pub value: i32,
    __padding_end: [u8; 12],
}
impl FInteger32CameraParameter {}
#[repr(C, align(8))]
pub struct FFloatCameraParameter {
    pub value: f32,
    __padding_end: [u8; 12],
}
impl FFloatCameraParameter {}
#[repr(C, align(8))]
pub struct FDoubleCameraParameter {
    pub value: f64,
    __padding_end: [u8; 16],
}
impl FDoubleCameraParameter {}
#[repr(C, align(8))]
pub struct FVector2fCameraParameter {
    pub value: crate::bindings::core_u_object::FVector2f,
    __padding_end: [u8; 16],
}
impl FVector2fCameraParameter {}
#[repr(C, align(8))]
pub struct FVector2dCameraParameter {
    pub value: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 16],
}
impl FVector2dCameraParameter {}
#[repr(C, align(8))]
pub struct FVector3fCameraParameter {
    pub value: crate::bindings::core_u_object::FVector3f,
    __padding_end: [u8; 12],
}
impl FVector3fCameraParameter {}
#[repr(C, align(8))]
pub struct FVector3dCameraParameter {
    pub value: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 16],
}
impl FVector3dCameraParameter {}
#[repr(C, align(16))]
pub struct FVector4fCameraParameter {
    pub value: crate::bindings::core_u_object::FVector4f,
    __padding_end: [u8; 16],
}
impl FVector4fCameraParameter {}
#[repr(C, align(16))]
pub struct FVector4dCameraParameter {
    pub value: crate::bindings::core_u_object::FVector4,
    __padding_end: [u8; 16],
}
impl FVector4dCameraParameter {}
#[repr(C, align(8))]
pub struct FRotator3fCameraParameter {
    pub value: crate::bindings::core_u_object::FRotator3f,
    __padding_end: [u8; 12],
}
impl FRotator3fCameraParameter {}
#[repr(C, align(8))]
pub struct FRotator3dCameraParameter {
    pub value: crate::bindings::core_u_object::FRotator,
    __padding_end: [u8; 16],
}
impl FRotator3dCameraParameter {}
#[repr(C, align(16))]
pub struct FTransform3fCameraParameter {
    pub value: crate::bindings::core_u_object::FTransform3f,
    __padding_end: [u8; 16],
}
impl FTransform3fCameraParameter {}
#[repr(C, align(16))]
pub struct FTransform3dCameraParameter {
    pub value: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 16],
}
impl FTransform3dCameraParameter {}
#[repr(C, align(8))]
pub struct FCameraRigAssetReference {
    __padding_end: [u8; 304],
}
impl FCameraRigAssetReference {}
#[repr(C, align(8))]
pub struct FCameraShakeAssetReference {
    __padding_end: [u8; 64],
}
impl FCameraShakeAssetReference {}
#[repr(C, align(4))]
pub struct FCameraVariableSetterHandle {
    __padding_end: [u8; 8],
}
impl FCameraVariableSetterHandle {}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorEvaluationParams {
    pub delta_time: f32,
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FBlueprintCameraDirectorEvaluationParams {}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorActivateParams {
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FBlueprintCameraDirectorActivateParams {}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorDeactivateParams {
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FBlueprintCameraDirectorDeactivateParams {}
#[repr(C, align(8))]
pub struct FCameraDirectorStateTreeEvaluationData {
    __padding_end: [u8; 32],
}
impl FCameraDirectorStateTreeEvaluationData {}
#[repr(C, align(8))]
pub struct FBlueprintCameraEvaluationDataRef {
    __padding_end: [u8; 24],
}
impl FBlueprintCameraEvaluationDataRef {}
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
    #[doc(hidden)]
    __padding_124: [u8; 4],
    pub enable_physical_camera: bool,
    pub constrain_aspect_ratio: bool,
    #[doc(hidden)]
    __padding_127: [u8; 1],
    pub aspect_ratio_axis_constraint: crate::bindings::engine::EAspectRatioAxisConstraint,
    pub projection_mode: crate::bindings::engine::ECameraProjectionMode,
    __padding_end: [u8; 7],
}
impl FBlueprintCameraPose {}
#[repr(C, align(8))]
pub struct FCameraActorAttachmentInfo {
    pub actor: UPtr<crate::bindings::engine::AActor>,
    pub socket_name: FName,
    pub bone_name: FName,
    pub weight: f32,
    __padding_end: [u8; 4],
}
impl FCameraActorAttachmentInfo {}
#[repr(C, align(8))]
pub struct FCameraActorTargetInfo {
    pub actor: UPtr<crate::bindings::engine::AActor>,
    pub socket_name: FName,
    pub bone_name: FName,
    pub target_shape: ECameraTargetShape,
    pub target_size: f32,
    pub weight: f32,
    __padding_end: [u8; 4],
}
impl FCameraActorTargetInfo {}
#[repr(C, align(8))]
pub struct FCameraFramingZone {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl FCameraFramingZone {}
pub struct UHasCameraBuildStatus {}
pub struct IHasCameraBuildStatus {}
#[repr(C, align(8))]
pub struct UCameraRigInstanceFunctions {
    __padding_end: [u8; 48],
}
impl UCameraRigInstanceFunctions {}
pub struct UAssetReferenceCameraNode {}
pub struct IAssetReferenceCameraNode {}
pub struct UObjectTreeGraphObject {}
pub struct IObjectTreeGraphObject {}
pub struct UObjectTreeGraphRootObject {}
pub struct IObjectTreeGraphRootObject {}
#[repr(C, align(8))]
pub struct UBaseCameraObject {
    __padding_end: [u8; 176],
}
impl UBaseCameraObject {}
#[repr(C, align(8))]
pub struct UCameraNode {
    __padding_end: [u8; 104],
}
impl UCameraNode {}
#[repr(C, align(8))]
pub struct UBlendCameraNode {
    __padding_end: [u8; 104],
}
impl UBlendCameraNode {}
#[repr(C, align(8))]
pub struct UBlendStackCameraNode {
    __padding_end: [u8; 112],
}
impl UBlendStackCameraNode {}
#[repr(C, align(8))]
pub struct UBlendStackRootCameraNode {
    __padding_end: [u8; 120],
}
impl UBlendStackRootCameraNode {}
#[repr(C, align(8))]
pub struct UCameraAsset {
    __padding_end: [u8; 248],
}
impl UCameraAsset {}
#[repr(C, align(8))]
pub struct UCameraDirector {
    __padding_end: [u8; 72],
}
impl UCameraDirector {}
#[repr(C, align(8))]
pub struct UCameraObjectInterfaceParameterBase {
    __padding_end: [u8; 120],
}
impl UCameraObjectInterfaceParameterBase {}
#[repr(C, align(8))]
pub struct UCameraObjectInterfaceBlendableParameter {
    __padding_end: [u8; 152],
}
impl UCameraObjectInterfaceBlendableParameter {}
#[repr(C, align(8))]
pub struct UCameraObjectInterfaceDataParameter {
    __padding_end: [u8; 144],
}
impl UCameraObjectInterfaceDataParameter {}
#[repr(C, align(8))]
pub struct UCameraRigAsset {
    __padding_end: [u8; 400],
}
impl UCameraRigAsset {}
#[repr(C, align(8))]
pub struct UCombinedCameraRigsCameraNode {
    __padding_end: [u8; 120],
}
impl UCombinedCameraRigsCameraNode {}
#[repr(C, align(8))]
pub struct UCameraRigProxyAsset {
    __padding_end: [u8; 64],
}
impl UCameraRigProxyAsset {}
#[repr(C, align(8))]
pub struct UCameraRigProxyTable {
    __padding_end: [u8; 64],
}
impl UCameraRigProxyTable {}
#[repr(C, align(8))]
pub struct UCameraRigTransitionCondition {
    __padding_end: [u8; 88],
}
impl UCameraRigTransitionCondition {}
#[repr(C, align(8))]
pub struct UCameraRigTransition {
    __padding_end: [u8; 120],
}
impl UCameraRigTransition {}
#[repr(C, align(8))]
pub struct UCameraShakeAsset {
    __padding_end: [u8; 288],
}
impl UCameraShakeAsset {}
#[repr(C, align(8))]
pub struct UCameraValueInterpolator {
    __padding_end: [u8; 48],
}
impl UCameraValueInterpolator {}
#[repr(C, align(8))]
pub struct UPopValueInterpolator {
    __padding_end: [u8; 48],
}
impl UPopValueInterpolator {}
#[repr(C, align(8))]
pub struct UCameraVariableAsset {
    __padding_end: [u8; 88],
}
impl UCameraVariableAsset {}
#[repr(C, align(8))]
pub struct UBooleanCameraVariable {
    __padding_end: [u8; 96],
}
impl UBooleanCameraVariable {}
#[repr(C, align(8))]
pub struct UInteger32CameraVariable {
    __padding_end: [u8; 96],
}
impl UInteger32CameraVariable {}
#[repr(C, align(8))]
pub struct UFloatCameraVariable {
    __padding_end: [u8; 96],
}
impl UFloatCameraVariable {}
#[repr(C, align(8))]
pub struct UDoubleCameraVariable {
    __padding_end: [u8; 96],
}
impl UDoubleCameraVariable {}
#[repr(C, align(8))]
pub struct UVector2fCameraVariable {
    __padding_end: [u8; 96],
}
impl UVector2fCameraVariable {}
#[repr(C, align(8))]
pub struct UVector2dCameraVariable {
    __padding_end: [u8; 104],
}
impl UVector2dCameraVariable {}
#[repr(C, align(8))]
pub struct UVector3fCameraVariable {
    __padding_end: [u8; 104],
}
impl UVector3fCameraVariable {}
#[repr(C, align(8))]
pub struct UVector3dCameraVariable {
    __padding_end: [u8; 112],
}
impl UVector3dCameraVariable {}
#[repr(C, align(16))]
pub struct UVector4fCameraVariable {
    __padding_end: [u8; 112],
}
impl UVector4fCameraVariable {}
#[repr(C, align(16))]
pub struct UVector4dCameraVariable {
    __padding_end: [u8; 128],
}
impl UVector4dCameraVariable {}
#[repr(C, align(8))]
pub struct URotator3fCameraVariable {
    __padding_end: [u8; 104],
}
impl URotator3fCameraVariable {}
#[repr(C, align(8))]
pub struct URotator3dCameraVariable {
    __padding_end: [u8; 112],
}
impl URotator3dCameraVariable {}
#[repr(C, align(16))]
pub struct UTransform3fCameraVariable {
    __padding_end: [u8; 144],
}
impl UTransform3fCameraVariable {}
#[repr(C, align(16))]
pub struct UTransform3dCameraVariable {
    __padding_end: [u8; 192],
}
impl UTransform3dCameraVariable {}
#[repr(C, align(8))]
pub struct UCameraVariableCollection {
    __padding_end: [u8; 64],
}
impl UCameraVariableCollection {}
#[repr(C, align(8))]
pub struct URootCameraNode {
    __padding_end: [u8; 104],
}
impl URootCameraNode {}
#[repr(C, align(8))]
pub struct UDefaultRootCameraNode {
    __padding_end: [u8; 136],
}
impl UDefaultRootCameraNode {}
pub struct UCustomCameraNodeParameterProvider {}
pub struct ICustomCameraNodeParameterProvider {}
#[repr(C, align(8))]
pub struct UObjectTreeGraphComment {
    __padding_end: [u8; 104],
}
impl UObjectTreeGraphComment {}
#[repr(C, align(8))]
pub struct UShakeCameraNode {
    __padding_end: [u8; 104],
}
impl UShakeCameraNode {}
#[repr(C, align(8))]
pub struct UBlueprintCameraDirectorEvaluator {
    __padding_end: [u8; 336],
}
impl UBlueprintCameraDirectorEvaluator {}
#[repr(C, align(8))]
pub struct UBlueprintCameraDirector {
    __padding_end: [u8; 80],
}
impl UBlueprintCameraDirector {}
#[repr(C, align(8))]
pub struct UCameraDirectorStateTreeSchema {
    __padding_end: [u8; 64],
}
impl UCameraDirectorStateTreeSchema {}
#[repr(C, align(8))]
pub struct UPriorityQueueCameraDirector {
    __padding_end: [u8; 72],
}
impl UPriorityQueueCameraDirector {}
#[repr(C, align(8))]
pub struct USingleCameraDirector {
    __padding_end: [u8; 80],
}
impl USingleCameraDirector {}
#[repr(C, align(8))]
pub struct UStateTreeCameraDirector {
    __padding_end: [u8; 112],
}
impl UStateTreeCameraDirector {}
#[repr(C, align(8))]
pub struct UActivateCameraRigFunctions {
    __padding_end: [u8; 48],
}
impl UActivateCameraRigFunctions {}
#[repr(C, align(8))]
pub struct UCameraComponentCameraNode {
    __padding_end: [u8; 104],
}
impl UCameraComponentCameraNode {}
#[repr(C, align(8))]
pub struct UCalcCameraActorCameraNode {
    __padding_end: [u8; 104],
}
impl UCalcCameraActorCameraNode {}
#[repr(C, align(8))]
pub struct UBlueprintCameraEvaluationDataFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintCameraEvaluationDataFunctionLibrary {}
#[repr(C, align(8))]
pub struct UBlueprintCameraVariableTableFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintCameraVariableTableFunctionLibrary {}
#[repr(C, align(8))]
pub struct UBlueprintCameraContextDataTableFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintCameraContextDataTableFunctionLibrary {}
#[repr(C, align(8))]
pub struct UBlueprintCameraPoseFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintCameraPoseFunctionLibrary {}
#[repr(C, align(8))]
pub struct UCameraRigParameterInterop {
    __padding_end: [u8; 48],
}
impl UCameraRigParameterInterop {}
#[repr(C, align(8))]
pub struct UCameraRigParameterInteropLibrary {
    __padding_end: [u8; 48],
}
impl UCameraRigParameterInteropLibrary {}
#[repr(C, align(8))]
pub struct UControllerGameplayCameraEvaluationComponent {
    __padding_end: [u8; 288],
}
impl UControllerGameplayCameraEvaluationComponent {}
#[repr(C, align(8))]
pub struct AGameplayCameraActorBase {
    __padding_end: [u8; 1136],
}
impl AGameplayCameraActorBase {}
#[repr(C, align(8))]
pub struct AGameplayCameraActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub camera_component: UPtr<UGameplayCameraComponent>,
}
impl AGameplayCameraActor {}
#[repr(C, align(16))]
pub struct UGameplayCameraComponentBase {
    #[doc(hidden)]
    __padding_688: [u8; 688],
    pub auto_activate_for_player: crate::bindings::engine::EAutoReceiveInput,
    pub b_set_control_rotation_when_view_target: bool,
    __padding_end: [u8; 62],
}
impl UGameplayCameraComponentBase {}
#[repr(C, align(16))]
pub struct UGameplayCameraComponent {
    #[doc(hidden)]
    __padding_744: [u8; 744],
    pub camera_reference: FCameraAssetReference,
    __padding_end: [u8; 32],
}
impl UGameplayCameraComponent {}
#[repr(C, align(8))]
pub struct UGameplayCameraParameterSetterComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub camera_rig_reference: FCameraRigAssetReference,
    pub blend_in_time: f32,
    pub blend_out_time: f32,
    pub blend_type: ECameraVariableSetterBlendType,
    __padding_end: [u8; 23],
}
impl UGameplayCameraParameterSetterComponent {}
#[repr(C, align(8))]
pub struct AGameplayCameraRigActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub camera_rig_component: UPtr<UGameplayCameraRigComponent>,
}
impl AGameplayCameraRigActor {}
#[repr(C, align(16))]
pub struct UGameplayCameraRigComponent {
    #[doc(hidden)]
    __padding_744: [u8; 744],
    pub camera_rig_reference: FCameraRigAssetReference,
    __padding_end: [u8; 40],
}
impl UGameplayCameraRigComponent {}
#[repr(C, align(16))]
pub struct AGameplayCamerasPlayerCameraManager {
    __padding_end: [u8; 11040],
}
impl AGameplayCamerasPlayerCameraManager {}
#[repr(C, align(8))]
pub struct AGameplayCameraSystemActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub camera_system_component: UPtr<UGameplayCameraSystemComponent>,
}
impl AGameplayCameraSystemActor {}
#[repr(C, align(16))]
pub struct UGameplayCameraSystemComponent {
    #[doc(hidden)]
    __padding_688: [u8; 688],
    pub auto_activate_for_player: crate::bindings::engine::EAutoReceiveInput,
    pub b_set_player_controller_rotation: bool,
    __padding_end: [u8; 14],
}
impl UGameplayCameraSystemComponent {}
#[repr(C, align(8))]
pub struct UGameplayControlRotationComponent {
    #[doc(hidden)]
    __padding_264: [u8; 264],
    pub auto_activate_for_player: crate::bindings::engine::EAutoReceiveInput,
    __padding_end: [u8; 47],
}
impl UGameplayControlRotationComponent {}
pub struct UGameplayCameraSystemHost {}
pub struct IGameplayCameraSystemHost {}
#[repr(C, align(8))]
pub struct USimpleBlendCameraNode {
    __padding_end: [u8; 104],
}
impl USimpleBlendCameraNode {}
#[repr(C, align(8))]
pub struct UViewTargetTransitionParamsBlendCameraNode {
    __padding_end: [u8; 120],
}
impl UViewTargetTransitionParamsBlendCameraNode {}
#[repr(C, align(8))]
pub struct UGameplayCamerasSettings {
    __padding_end: [u8; 152],
}
impl UGameplayCamerasSettings {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraFramingZonePropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneCameraFramingZonePropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraFramingZoneSection {
    __padding_end: [u8; 1616],
}
impl UMovieSceneCameraFramingZoneSection {}
#[repr(C, align(8))]
pub struct UMovieSceneCameraFramingZoneTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneCameraFramingZoneTrack {}
#[repr(C, align(8))]
pub struct UAttachToActorCameraNode {
    __padding_end: [u8; 184],
}
impl UAttachToActorCameraNode {}
#[repr(C, align(8))]
pub struct UAttachToActorGroupCameraNode {
    __padding_end: [u8; 128],
}
impl UAttachToActorGroupCameraNode {}
#[repr(C, align(8))]
pub struct UAttachToPlayerPawnCameraNode {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub socket_name: FName,
    pub bone_name: FName,
}
impl UAttachToPlayerPawnCameraNode {}
#[repr(C, align(8))]
pub struct USimpleFixedTimeBlendCameraNode {
    __padding_end: [u8; 120],
}
impl USimpleFixedTimeBlendCameraNode {}
#[repr(C, align(8))]
pub struct ULinearBlendCameraNode {
    __padding_end: [u8; 120],
}
impl ULinearBlendCameraNode {}
#[repr(C, align(8))]
pub struct ULocationRotationBlendCameraNode {
    __padding_end: [u8; 128],
}
impl ULocationRotationBlendCameraNode {}
#[repr(C, align(8))]
pub struct UOrbitBlendCameraNode {
    __padding_end: [u8; 112],
}
impl UOrbitBlendCameraNode {}
#[repr(C, align(8))]
pub struct UPopBlendCameraNode {
    __padding_end: [u8; 104],
}
impl UPopBlendCameraNode {}
#[repr(C, align(8))]
pub struct USmoothBlendCameraNode {
    __padding_end: [u8; 128],
}
impl USmoothBlendCameraNode {}
#[repr(C, align(8))]
pub struct UCollisionPushCameraNode {
    __padding_end: [u8; 240],
}
impl UCollisionPushCameraNode {}
#[repr(C, align(8))]
pub struct UOcclusionMaterialCameraNode {
    __padding_end: [u8; 176],
}
impl UOcclusionMaterialCameraNode {}
#[repr(C, align(8))]
pub struct UArrayCameraNode {
    __padding_end: [u8; 120],
}
impl UArrayCameraNode {}
#[repr(C, align(8))]
pub struct UAutoFocusCameraNode {
    __padding_end: [u8; 136],
}
impl UAutoFocusCameraNode {}
#[repr(C, align(8))]
pub struct UBodyParametersCameraNode {
    __padding_end: [u8; 136],
}
impl UBodyParametersCameraNode {}
#[repr(C, align(8))]
pub struct UBoomArmCameraNode {
    __padding_end: [u8; 208],
}
impl UBoomArmCameraNode {}
#[repr(C, align(8))]
pub struct UCameraRigCameraNode {
    __padding_end: [u8; 424],
}
impl UCameraRigCameraNode {}
#[repr(C, align(8))]
pub struct UClippingPlanesCameraNode {
    __padding_end: [u8; 152],
}
impl UClippingPlanesCameraNode {}
#[repr(C, align(8))]
pub struct UDampenPositionCameraNode {
    __padding_end: [u8; 160],
}
impl UDampenPositionCameraNode {}
#[repr(C, align(8))]
pub struct UDampenRotationCameraNode {
    __padding_end: [u8; 152],
}
impl UDampenRotationCameraNode {}
#[repr(C, align(8))]
pub struct UFieldOfViewCameraNode {
    __padding_end: [u8; 120],
}
impl UFieldOfViewCameraNode {}
#[repr(C, align(8))]
pub struct UFilmbackCameraNode {
    __padding_end: [u8; 224],
}
impl UFilmbackCameraNode {}
#[repr(C, align(8))]
pub struct ULensParametersCameraNode {
    __padding_end: [u8; 168],
}
impl ULensParametersCameraNode {}
#[repr(C, align(8))]
pub struct UOffsetCameraNode {
    __padding_end: [u8; 192],
}
impl UOffsetCameraNode {}
#[repr(C, align(8))]
pub struct UOrthographicCameraNode {
    __padding_end: [u8; 136],
}
impl UOrthographicCameraNode {}
#[repr(C, align(16))]
pub struct UPostProcessCameraNode {
    __padding_end: [u8; 2064],
}
impl UPostProcessCameraNode {}
#[repr(C, align(8))]
pub struct USetLocationCameraNode {
    __padding_end: [u8; 152],
}
impl USetLocationCameraNode {}
#[repr(C, align(8))]
pub struct USetRotationCameraNode {
    __padding_end: [u8; 152],
}
impl USetRotationCameraNode {}
#[repr(C, align(8))]
pub struct USplineFieldOfViewCameraNode {
    __padding_end: [u8; 248],
}
impl USplineFieldOfViewCameraNode {}
#[repr(C, align(8))]
pub struct USplineOffsetCameraNode {
    __padding_end: [u8; 896],
}
impl USplineOffsetCameraNode {}
#[repr(C, align(8))]
pub struct USplineOrbitCameraNode {
    __padding_end: [u8; 1288],
}
impl USplineOrbitCameraNode {}
#[repr(C, align(8))]
pub struct UTargetRayCastCameraNode {
    __padding_end: [u8; 128],
}
impl UTargetRayCastCameraNode {}
#[repr(C, align(8))]
pub struct UBaseFramingCameraNode {
    __padding_end: [u8; 424],
}
impl UBaseFramingCameraNode {}
#[repr(C, align(8))]
pub struct UDollyFramingCameraNode {
    __padding_end: [u8; 456],
}
impl UDollyFramingCameraNode {}
#[repr(C, align(8))]
pub struct UPanningFramingCameraNode {
    __padding_end: [u8; 456],
}
impl UPanningFramingCameraNode {}
#[repr(C, align(8))]
pub struct UInput2DCameraNode {
    __padding_end: [u8; 104],
}
impl UInput2DCameraNode {}
#[repr(C, align(8))]
pub struct UAutoRotateInput2DCameraNode {
    __padding_end: [u8; 240],
}
impl UAutoRotateInput2DCameraNode {}
#[repr(C, align(8))]
pub struct UInput1DCameraNode {
    __padding_end: [u8; 104],
}
impl UInput1DCameraNode {}
#[repr(C, align(8))]
pub struct UCameraRigInput1DSlot {
    __padding_end: [u8; 224],
}
impl UCameraRigInput1DSlot {}
#[repr(C, align(8))]
pub struct UCameraRigInput2DSlot {
    __padding_end: [u8; 288],
}
impl UCameraRigInput2DSlot {}
#[repr(C, align(8))]
pub struct UDrivenControlRotationCameraNode {
    __padding_end: [u8; 104],
}
impl UDrivenControlRotationCameraNode {}
#[repr(C, align(8))]
pub struct UInputAccumulator2DCameraNode {
    __padding_end: [u8; 296],
}
impl UInputAccumulator2DCameraNode {}
#[repr(C, align(8))]
pub struct UInputAxisBinding2DCameraNode {
    __padding_end: [u8; 344],
}
impl UInputAxisBinding2DCameraNode {}
#[repr(C, align(8))]
pub struct URawInputAxisBinding2DCameraNode {
    __padding_end: [u8; 152],
}
impl URawInputAxisBinding2DCameraNode {}
#[repr(C, align(8))]
pub struct UCameraShakeCameraNode {
    __padding_end: [u8; 184],
}
impl UCameraShakeCameraNode {}
#[repr(C, align(8))]
pub struct UCompositeShakeCameraNode {
    __padding_end: [u8; 120],
}
impl UCompositeShakeCameraNode {}
#[repr(C, align(8))]
pub struct UEnvelopeShakeCameraNode {
    __padding_end: [u8; 160],
}
impl UEnvelopeShakeCameraNode {}
#[repr(C, align(8))]
pub struct UPerlinNoiseLocationShakeCameraNode {
    __padding_end: [u8; 176],
}
impl UPerlinNoiseLocationShakeCameraNode {}
#[repr(C, align(8))]
pub struct UPerlinNoiseRotationShakeCameraNode {
    __padding_end: [u8; 176],
}
impl UPerlinNoiseRotationShakeCameraNode {}
#[repr(C, align(8))]
pub struct UBlueprintCameraNodeEvaluator {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_is_first_frame: bool,
    pub b_is_active_camera_rig: bool,
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
    pub camera_data: FBlueprintCameraEvaluationDataRef,
    pub camera_pose: FBlueprintCameraPose,
    pub variable_table: FBlueprintCameraEvaluationDataRef,
    __padding_end: [u8; 32],
}
impl UBlueprintCameraNodeEvaluator {}
#[repr(C, align(8))]
pub struct UBlueprintCameraNode {
    __padding_end: [u8; 160],
}
impl UBlueprintCameraNode {}
#[repr(C, align(8))]
pub struct UCameraShakeServiceCameraNode {
    __padding_end: [u8; 104],
}
impl UCameraShakeServiceCameraNode {}
#[repr(C, align(8))]
pub struct UUpdateTrackerCameraNode {
    __padding_end: [u8; 168],
}
impl UUpdateTrackerCameraNode {}
#[repr(C, align(8))]
pub struct UFixedTestCameraDirector {
    __padding_end: [u8; 104],
}
impl UFixedTestCameraDirector {}
#[repr(C, align(8))]
pub struct UIsCameraRigTransitionCondition {
    __padding_end: [u8; 104],
}
impl UIsCameraRigTransitionCondition {}
#[repr(C, align(8))]
pub struct UGameplayTagTransitionCondition {
    __padding_end: [u8; 232],
}
impl UGameplayTagTransitionCondition {}
#[repr(C, align(8))]
pub struct UAccelerationDecelerationValueInterpolator {
    __padding_end: [u8; 64],
}
impl UAccelerationDecelerationValueInterpolator {}
#[repr(C, align(8))]
pub struct UCriticalDamperValueInterpolator {
    __padding_end: [u8; 56],
}
impl UCriticalDamperValueInterpolator {}
#[repr(C, align(8))]
pub struct UDoubleIIRValueInterpolator {
    __padding_end: [u8; 64],
}
impl UDoubleIIRValueInterpolator {}
#[repr(C, align(8))]
pub struct UIIRValueInterpolator {
    __padding_end: [u8; 56],
}
impl UIIRValueInterpolator {}
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
#[repr(transparent)]
pub struct ECameraContextDataContainerType(pub i32);
impl ECameraContextDataContainerType {
    pub const NONE: ECameraContextDataContainerType = ECameraContextDataContainerType(0);
    pub const ARRAY: ECameraContextDataContainerType = ECameraContextDataContainerType(
        1,
    );
}
#[repr(transparent)]
pub struct ECameraRigLayer(pub u8);
impl ECameraRigLayer {
    pub const NONE: ECameraRigLayer = ECameraRigLayer(0);
    pub const BASE: ECameraRigLayer = ECameraRigLayer(1);
    pub const MAIN: ECameraRigLayer = ECameraRigLayer(2);
    pub const GLOBAL: ECameraRigLayer = ECameraRigLayer(3);
    pub const VISUAL: ECameraRigLayer = ECameraRigLayer(4);
}
#[repr(transparent)]
pub struct ECameraTargetShape(pub u8);
impl ECameraTargetShape {
    pub const POINT: ECameraTargetShape = ECameraTargetShape(0);
    pub const AUTOMATIC_BOUNDS: ECameraTargetShape = ECameraTargetShape(1);
    pub const MANUAL_BOUNDS: ECameraTargetShape = ECameraTargetShape(2);
}
#[repr(transparent)]
pub struct ECameraEvaluationDataCondition(pub u8);
impl ECameraEvaluationDataCondition {
    pub const ACTIVE_CAMERA_RIG: ECameraEvaluationDataCondition = ECameraEvaluationDataCondition(
        0,
    );
}
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
#[repr(transparent)]
pub struct ECameraBlendStackType(pub i32);
impl ECameraBlendStackType {
    pub const ISOLATED_TRANSIENT: ECameraBlendStackType = ECameraBlendStackType(0);
    pub const ADDITIVE_PERSISTENT: ECameraBlendStackType = ECameraBlendStackType(1);
}
#[repr(transparent)]
pub struct ECameraBuildStatus(pub u8);
impl ECameraBuildStatus {
    pub const CLEAN: ECameraBuildStatus = ECameraBuildStatus(0);
    pub const CLEAN_WITH_WARNINGS: ECameraBuildStatus = ECameraBuildStatus(1);
    pub const WITH_ERRORS: ECameraBuildStatus = ECameraBuildStatus(2);
    pub const DIRTY: ECameraBuildStatus = ECameraBuildStatus(3);
}
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
#[repr(transparent)]
pub struct ESmoothCameraBlendType(pub i32);
impl ESmoothCameraBlendType {
    pub const SMOOTH_STEP: ESmoothCameraBlendType = ESmoothCameraBlendType(0);
    pub const SMOOTHER_STEP: ESmoothCameraBlendType = ESmoothCameraBlendType(1);
}
#[repr(transparent)]
pub struct ECollisionSafePosition(pub u8);
impl ECollisionSafePosition {
    pub const ACTIVE_CONTEXT: ECollisionSafePosition = ECollisionSafePosition(0);
    pub const OWNING_CONTEXT: ECollisionSafePosition = ECollisionSafePosition(1);
    pub const PIVOT: ECollisionSafePosition = ECollisionSafePosition(2);
    pub const PAWN: ECollisionSafePosition = ECollisionSafePosition(3);
}
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
#[repr(transparent)]
pub struct ECameraNodeOriginPosition(pub u8);
impl ECameraNodeOriginPosition {
    pub const CAMERA_POSE: ECameraNodeOriginPosition = ECameraNodeOriginPosition(0);
    pub const ACTIVE_CONTEXT: ECameraNodeOriginPosition = ECameraNodeOriginPosition(1);
    pub const OWNING_CONTEXT: ECameraNodeOriginPosition = ECameraNodeOriginPosition(2);
    pub const PIVOT: ECameraNodeOriginPosition = ECameraNodeOriginPosition(3);
    pub const PAWN: ECameraNodeOriginPosition = ECameraNodeOriginPosition(4);
}
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
#[repr(transparent)]
pub struct ECameraAutoRotateDirection(pub i32);
impl ECameraAutoRotateDirection {
    pub const FACING: ECameraAutoRotateDirection = ECameraAutoRotateDirection(0);
    pub const MOVEMENT: ECameraAutoRotateDirection = ECameraAutoRotateDirection(1);
    pub const MOVEMENT_OR_FACING: ECameraAutoRotateDirection = ECameraAutoRotateDirection(
        2,
    );
}
#[repr(transparent)]
pub struct EBuiltInDoubleCameraVariable(pub i32);
impl EBuiltInDoubleCameraVariable {
    pub const NONE: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(0);
    pub const YAW: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(1);
    pub const PITCH: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(2);
    pub const ROLL: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(3);
    pub const ZOOM: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(4);
}
#[repr(transparent)]
pub struct EBuiltInVector2dCameraVariable(pub i32);
impl EBuiltInVector2dCameraVariable {
    pub const NONE: EBuiltInVector2dCameraVariable = EBuiltInVector2dCameraVariable(0);
    pub const YAW_PITCH: EBuiltInVector2dCameraVariable = EBuiltInVector2dCameraVariable(
        1,
    );
}
#[repr(transparent)]
pub struct ECameraShakeEvaluationMode(pub u8);
impl ECameraShakeEvaluationMode {
    pub const INLINE: ECameraShakeEvaluationMode = ECameraShakeEvaluationMode(0);
    pub const VISUAL_LAYER: ECameraShakeEvaluationMode = ECameraShakeEvaluationMode(1);
}
