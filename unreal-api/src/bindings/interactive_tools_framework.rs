#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FInputRayHit {
    pub b_hit: bool,
    pub hit_depth: f64,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    pub b_has_hit_normal: bool,
    pub hit_identifier: i32,
    #[doc(hidden)]
    __padding_56: [u8; 8],
    pub hit_object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
}
impl FInputRayHit {}
#[repr(C, align(8))]
pub struct FDeviceButtonState {
    pub button: crate::bindings::input_core::FKey,
    pub b_pressed: bool,
    pub b_down: bool,
    pub b_released: bool,
    pub b_double_clicked: bool,
    __padding_end: [u8; 4],
}
impl FDeviceButtonState {}
#[repr(C, align(8))]
pub struct FKeyboardInputDeviceState {
    pub active_key: FDeviceButtonState,
}
impl FKeyboardInputDeviceState {}
#[repr(C, align(8))]
pub struct FMouseInputDeviceState {
    pub left: FDeviceButtonState,
    pub middle: FDeviceButtonState,
    pub right: FDeviceButtonState,
    pub wheel_delta: f32,
    pub position2_d: crate::bindings::core_u_object::FVector2D,
    pub delta2_d: crate::bindings::core_u_object::FVector2D,
    pub world_ray: crate::bindings::core_u_object::FRay,
}
impl FMouseInputDeviceState {}
#[repr(C, align(8))]
pub struct FInputDeviceState {
    pub input_device: EInputDevices,
    pub b_shift_key_down: bool,
    pub b_alt_key_down: bool,
    pub b_ctrl_key_down: bool,
    pub b_cmd_key_down: bool,
    pub keyboard: FKeyboardInputDeviceState,
    pub mouse: FMouseInputDeviceState,
}
impl FInputDeviceState {}
#[repr(C, align(8))]
pub struct FInputDeviceRay {
    pub world_ray: crate::bindings::core_u_object::FRay,
    pub b_has2_d: bool,
    pub screen_position: crate::bindings::core_u_object::FVector2D,
}
impl FInputDeviceRay {}
pub struct IAssetBackedTarget {}
#[repr(C, align(8))]
pub struct UAssetBackedTarget {
    __padding_end: [u8; 48],
}
impl UAssetBackedTarget {}
pub struct IGizmoBaseComponentInterface {}
#[repr(C, align(8))]
pub struct UGizmoBaseComponentInterface {
    __padding_end: [u8; 48],
}
impl UGizmoBaseComponentInterface {}
#[repr(C, align(16))]
pub struct UGizmoBaseComponent {
    __padding_end: [u8; 1568],
}
impl UGizmoBaseComponent {}
#[repr(C, align(8))]
pub struct UInteractiveCommandArguments {
    __padding_end: [u8; 56],
}
impl UInteractiveCommandArguments {}
#[repr(C, align(8))]
pub struct UInteractiveCommandResult {
    __padding_end: [u8; 48],
}
impl UInteractiveCommandResult {}
#[repr(C, align(8))]
pub struct UInteractiveCommand {
    __padding_end: [u8; 48],
}
impl UInteractiveCommand {}
#[repr(C, align(8))]
pub struct UInteractiveGizmoBuilder {
    __padding_end: [u8; 48],
}
impl UInteractiveGizmoBuilder {}
pub struct IToolContextTransactionProvider {}
#[repr(C, align(8))]
pub struct UToolContextTransactionProvider {
    __padding_end: [u8; 48],
}
impl UToolContextTransactionProvider {}
#[repr(C, align(8))]
pub struct AInternalToolFrameworkActor {
    __padding_end: [u8; 1144],
}
impl AInternalToolFrameworkActor {}
pub struct IToolFrameworkComponent {}
#[repr(C, align(8))]
pub struct UToolFrameworkComponent {
    __padding_end: [u8; 48],
}
impl UToolFrameworkComponent {}
pub struct IInteractiveToolCameraFocusAPI {}
#[repr(C, align(8))]
pub struct UInteractiveToolCameraFocusAPI {
    __padding_end: [u8; 48],
}
impl UInteractiveToolCameraFocusAPI {}
pub struct IInteractiveToolNestedAcceptCancelAPI {}
#[repr(C, align(8))]
pub struct UInteractiveToolNestedAcceptCancelAPI {
    __padding_end: [u8; 48],
}
impl UInteractiveToolNestedAcceptCancelAPI {}
pub struct IInteractiveToolShutdownQueryAPI {}
#[repr(C, align(8))]
pub struct UInteractiveToolShutdownQueryAPI {
    __padding_end: [u8; 48],
}
impl UInteractiveToolShutdownQueryAPI {}
pub struct IInteractiveToolExclusiveToolAPI {}
#[repr(C, align(8))]
pub struct UInteractiveToolExclusiveToolAPI {
    __padding_end: [u8; 48],
}
impl UInteractiveToolExclusiveToolAPI {}
pub struct IInteractiveToolEditorGizmoAPI {}
#[repr(C, align(8))]
pub struct UInteractiveToolEditorGizmoAPI {
    __padding_end: [u8; 48],
}
impl UInteractiveToolEditorGizmoAPI {}
pub struct IInteractiveToolManageGeometrySelectionAPI {}
#[repr(C, align(8))]
pub struct UInteractiveToolManageGeometrySelectionAPI {
    __padding_end: [u8; 48],
}
impl UInteractiveToolManageGeometrySelectionAPI {}
pub struct IMaterialProvider {}
#[repr(C, align(8))]
pub struct UMaterialProvider {
    __padding_end: [u8; 48],
}
impl UMaterialProvider {}
pub struct IMeshDescriptionCommitter {}
#[repr(C, align(8))]
pub struct UMeshDescriptionCommitter {
    __padding_end: [u8; 48],
}
impl UMeshDescriptionCommitter {}
pub struct IMeshDescriptionProvider {}
#[repr(C, align(8))]
pub struct UMeshDescriptionProvider {
    __padding_end: [u8; 48],
}
impl UMeshDescriptionProvider {}
#[repr(C, align(8))]
pub struct UGizmoBaseVec2ParameterSource {
    __padding_end: [u8; 88],
}
impl UGizmoBaseVec2ParameterSource {}
#[repr(C, align(8))]
pub struct UGizmoLocalVec2ParameterSource {
    __padding_end: [u8; 136],
}
impl UGizmoLocalVec2ParameterSource {}
pub struct IPhysicsDataSource {}
#[repr(C, align(8))]
pub struct UPhysicsDataSource {
    __padding_end: [u8; 48],
}
impl UPhysicsDataSource {}
pub struct ISceneComponentBackedTarget {}
#[repr(C, align(8))]
pub struct USceneComponentBackedTarget {
    __padding_end: [u8; 48],
}
impl USceneComponentBackedTarget {}
pub struct IPrimitiveComponentBackedTarget {}
#[repr(C, align(8))]
pub struct UPrimitiveComponentBackedTarget {
    __padding_end: [u8; 48],
}
impl UPrimitiveComponentBackedTarget {}
pub struct ISkeletalMeshBackedTarget {}
#[repr(C, align(8))]
pub struct USkeletalMeshBackedTarget {
    __padding_end: [u8; 48],
}
impl USkeletalMeshBackedTarget {}
pub struct IStaticMeshBackedTarget {}
#[repr(C, align(8))]
pub struct UStaticMeshBackedTarget {
    __padding_end: [u8; 48],
}
impl UStaticMeshBackedTarget {}
#[repr(C, align(8))]
pub struct UInputBehavior {
    __padding_end: [u8; 88],
}
impl UInputBehavior {}
#[repr(C, align(16))]
pub struct UAnyButtonInputBehavior {
    __padding_end: [u8; 144],
}
impl UAnyButtonInputBehavior {}
#[repr(C, align(16))]
pub struct UClickDragInputBehavior {
    __padding_end: [u8; 320],
}
impl UClickDragInputBehavior {}
#[repr(C, align(16))]
pub struct ULocalClickDragInputBehavior {
    __padding_end: [u8; 560],
}
impl ULocalClickDragInputBehavior {}
#[repr(C, align(16))]
pub struct USingleClickInputBehavior {
    __padding_end: [u8; 304],
}
impl USingleClickInputBehavior {}
#[repr(C, align(16))]
pub struct UDoubleClickInputBehavior {
    __padding_end: [u8; 304],
}
impl UDoubleClickInputBehavior {}
#[repr(C, align(16))]
pub struct ULocalDoubleClickInputBehavior {
    __padding_end: [u8; 464],
}
impl ULocalDoubleClickInputBehavior {}
#[repr(C, align(8))]
pub struct UKeyAsModifierInputBehavior {
    __padding_end: [u8; 192],
}
impl UKeyAsModifierInputBehavior {}
#[repr(C, align(8))]
pub struct UMouseButtonAsModifierInputBehavior {
    __padding_end: [u8; 192],
}
impl UMouseButtonAsModifierInputBehavior {}
#[repr(C, align(16))]
pub struct UKeyInputBehavior {
    __padding_end: [u8; 304],
}
impl UKeyInputBehavior {}
#[repr(C, align(16))]
pub struct UMouseHoverBehavior {
    __padding_end: [u8; 256],
}
impl UMouseHoverBehavior {}
#[repr(C, align(16))]
pub struct ULocalMouseHoverBehavior {
    __padding_end: [u8; 496],
}
impl ULocalMouseHoverBehavior {}
#[repr(C, align(16))]
pub struct UMouseWheelInputBehavior {
    __padding_end: [u8; 304],
}
impl UMouseWheelInputBehavior {}
#[repr(C, align(16))]
pub struct UMouseWheelModifierInputBehavior {
    __padding_end: [u8; 304],
}
impl UMouseWheelModifierInputBehavior {}
#[repr(C, align(16))]
pub struct UMultiClickSequenceInputBehavior {
    __padding_end: [u8; 352],
}
impl UMultiClickSequenceInputBehavior {}
#[repr(C, align(16))]
pub struct ULocalSingleClickInputBehavior {
    __padding_end: [u8; 464],
}
impl ULocalSingleClickInputBehavior {}
#[repr(C, align(16))]
pub struct USingleClickOrDragInputBehavior {
    __padding_end: [u8; 384],
}
impl USingleClickOrDragInputBehavior {}
#[repr(C, align(16))]
pub struct USingleKeyCaptureBehavior {
    __padding_end: [u8; 320],
}
impl USingleKeyCaptureBehavior {}
#[repr(C, align(8))]
pub struct UTwoAxisPropertyEditInputBehavior {
    __padding_end: [u8; 152],
}
impl UTwoAxisPropertyEditInputBehavior {}
#[repr(C, align(16))]
pub struct ULocalTwoAxisPropertyEditInputBehavior {
    __padding_end: [u8; 624],
}
impl ULocalTwoAxisPropertyEditInputBehavior {}
pub struct IWidgetBaseBehavior {}
#[repr(C, align(8))]
pub struct UWidgetBaseBehavior {
    __padding_end: [u8; 48],
}
impl UWidgetBaseBehavior {}
#[repr(C, align(8))]
pub struct UAxisAngleGizmoBuilder {
    __padding_end: [u8; 48],
}
impl UAxisAngleGizmoBuilder {}
#[repr(C, align(8))]
pub struct UInteractiveGizmo {
    __padding_end: [u8; 64],
}
impl UInteractiveGizmo {}
#[repr(C, align(16))]
pub struct UAxisAngleGizmo {
    __padding_end: [u8; 448],
}
impl UAxisAngleGizmo {}
#[repr(C, align(8))]
pub struct UAxisPositionGizmoBuilder {
    __padding_end: [u8; 48],
}
impl UAxisPositionGizmoBuilder {}
#[repr(C, align(16))]
pub struct UAxisPositionGizmo {
    __padding_end: [u8; 432],
}
impl UAxisPositionGizmo {}
#[repr(C, align(8))]
pub struct UGizmoConstantAxisSource {
    __padding_end: [u8; 104],
}
impl UGizmoConstantAxisSource {}
#[repr(C, align(8))]
pub struct UGizmoConstantFrameAxisSource {
    __padding_end: [u8; 152],
}
impl UGizmoConstantFrameAxisSource {}
#[repr(C, align(8))]
pub struct UGizmoWorldAxisSource {
    __padding_end: [u8; 88],
}
impl UGizmoWorldAxisSource {}
#[repr(C, align(8))]
pub struct UGizmoComponentAxisSource {
    __padding_end: [u8; 72],
}
impl UGizmoComponentAxisSource {}
#[repr(C, align(8))]
pub struct UBrushStampIndicatorBuilder {
    __padding_end: [u8; 48],
}
impl UBrushStampIndicatorBuilder {}
#[repr(C, align(8))]
pub struct UBrushStampIndicator {
    __padding_end: [u8; 224],
}
impl UBrushStampIndicator {}
#[repr(C, align(8))]
pub struct AGizmoActor {
    __padding_end: [u8; 1144],
}
impl AGizmoActor {}
#[repr(C, align(8))]
pub struct ACombinedTransformGizmoActor {
    __padding_end: [u8; 1336],
}
impl ACombinedTransformGizmoActor {}
#[repr(C, align(16))]
pub struct UCombinedTransformGizmoBuilder {
    __padding_end: [u8; 208],
}
impl UCombinedTransformGizmoBuilder {}
#[repr(C, align(16))]
pub struct UCombinedTransformGizmo {
    __padding_end: [u8; 1376],
}
impl UCombinedTransformGizmo {}
#[repr(C, align(16))]
pub struct UTransformProxy {
    __padding_end: [u8; 496],
}
impl UTransformProxy {}
#[repr(C, align(16))]
pub struct UComponentBoundTransformProxy {
    __padding_end: [u8; 512],
}
impl UComponentBoundTransformProxy {}
#[repr(C, align(16))]
pub struct UPlanePositionGizmo {
    __padding_end: [u8; 640],
}
impl UPlanePositionGizmo {}
#[repr(C, align(16))]
pub struct UFreePositionSubGizmo {
    __padding_end: [u8; 656],
}
impl UFreePositionSubGizmo {}
#[repr(C, align(16))]
pub struct UFreeRotationSubGizmo {
    __padding_end: [u8; 256],
}
impl UFreeRotationSubGizmo {}
#[repr(C, align(16))]
pub struct UGizmoArrowComponent {
    __padding_end: [u8; 1600],
}
impl UGizmoArrowComponent {}
#[repr(C, align(16))]
pub struct UGizmoBoxComponent {
    __padding_end: [u8; 1648],
}
impl UGizmoBoxComponent {}
#[repr(C, align(16))]
pub struct UGizmoCircleComponent {
    __padding_end: [u8; 1600],
}
impl UGizmoCircleComponent {}
#[repr(C, align(8))]
pub struct UGizmoElementBase {
    __padding_end: [u8; 344],
}
impl UGizmoElementBase {}
#[repr(C, align(8))]
pub struct UGizmoElementLineBase {
    __padding_end: [u8; 472],
}
impl UGizmoElementLineBase {}
#[repr(C, align(8))]
pub struct UGizmoElementCircleBase {
    __padding_end: [u8; 664],
}
impl UGizmoElementCircleBase {}
#[repr(C, align(8))]
pub struct UGizmoElementArc {
    __padding_end: [u8; 672],
}
impl UGizmoElementArc {}
#[repr(C, align(8))]
pub struct UGizmoElementGroupBase {
    __padding_end: [u8; 496],
}
impl UGizmoElementGroupBase {}
#[repr(C, align(8))]
pub struct UGizmoElementArrow {
    __padding_end: [u8; 632],
}
impl UGizmoElementArrow {}
#[repr(C, align(8))]
pub struct UGizmoElementArrowHead {
    __padding_end: [u8; 624],
}
impl UGizmoElementArrowHead {}
#[repr(C, align(8))]
pub struct UGizmoElementBox {
    __padding_end: [u8; 440],
}
impl UGizmoElementBox {}
#[repr(C, align(8))]
pub struct UGizmoElementCircle {
    __padding_end: [u8; 672],
}
impl UGizmoElementCircle {}
#[repr(C, align(8))]
pub struct UGizmoElementCone {
    __padding_end: [u8; 408],
}
impl UGizmoElementCone {}
#[repr(C, align(8))]
pub struct UGizmoElementCylinder {
    __padding_end: [u8; 416],
}
impl UGizmoElementCylinder {}
#[repr(C, align(8))]
pub struct UGizmoElementGroup {
    __padding_end: [u8; 496],
}
impl UGizmoElementGroup {}
#[repr(C, align(16))]
pub struct UGizmoElementHitTarget {
    __padding_end: [u8; 128],
}
impl UGizmoElementHitTarget {}
#[repr(C, align(16))]
pub struct UGizmoElementHitMultiTarget {
    __padding_end: [u8; 128],
}
impl UGizmoElementHitMultiTarget {}
#[repr(C, align(8))]
pub struct UGizmoElementLineStrip {
    __padding_end: [u8; 584],
}
impl UGizmoElementLineStrip {}
#[repr(C, align(8))]
pub struct UGizmoElementRectangle {
    __padding_end: [u8; 560],
}
impl UGizmoElementRectangle {}
#[repr(C, align(8))]
pub struct UGizmoElementSphere {
    __padding_end: [u8; 376],
}
impl UGizmoElementSphere {}
#[repr(C, align(8))]
pub struct UGizmoElementTorus {
    __padding_end: [u8; 760],
}
impl UGizmoElementTorus {}
#[repr(C, align(8))]
pub struct UGizmoElementTriangleList {
    __padding_end: [u8; 448],
}
impl UGizmoElementTriangleList {}
pub struct IGizmoTransformSource {}
#[repr(C, align(8))]
pub struct UGizmoTransformSource {
    __padding_end: [u8; 48],
}
impl UGizmoTransformSource {}
pub struct IGizmoAxisSource {}
#[repr(C, align(8))]
pub struct UGizmoAxisSource {
    __padding_end: [u8; 48],
}
impl UGizmoAxisSource {}
pub struct IGizmoClickTarget {}
#[repr(C, align(8))]
pub struct UGizmoClickTarget {
    __padding_end: [u8; 48],
}
impl UGizmoClickTarget {}
pub struct IGizmoClickMultiTarget {}
#[repr(C, align(8))]
pub struct UGizmoClickMultiTarget {
    __padding_end: [u8; 48],
}
impl UGizmoClickMultiTarget {}
pub struct IGizmoRenderTarget {}
#[repr(C, align(8))]
pub struct UGizmoRenderTarget {
    __padding_end: [u8; 48],
}
impl UGizmoRenderTarget {}
pub struct IGizmoRenderMultiTarget {}
#[repr(C, align(8))]
pub struct UGizmoRenderMultiTarget {
    __padding_end: [u8; 48],
}
impl UGizmoRenderMultiTarget {}
pub struct IGizmoStateTarget {}
#[repr(C, align(8))]
pub struct UGizmoStateTarget {
    __padding_end: [u8; 48],
}
impl UGizmoStateTarget {}
pub struct IGizmoFloatParameterSource {}
#[repr(C, align(8))]
pub struct UGizmoFloatParameterSource {
    __padding_end: [u8; 48],
}
impl UGizmoFloatParameterSource {}
pub struct IGizmoVec2ParameterSource {}
#[repr(C, align(8))]
pub struct UGizmoVec2ParameterSource {
    __padding_end: [u8; 48],
}
impl UGizmoVec2ParameterSource {}
#[repr(C, align(16))]
pub struct UGizmoLineHandleComponent {
    __padding_end: [u8; 1632],
}
impl UGizmoLineHandleComponent {}
#[repr(C, align(16))]
pub struct UGizmoRectangleComponent {
    __padding_end: [u8; 1648],
}
impl UGizmoRectangleComponent {}
#[repr(C, align(16))]
pub struct USimpleLambdaInteractiveGizmoBuilder {
    __padding_end: [u8; 96],
}
impl USimpleLambdaInteractiveGizmoBuilder {}
#[repr(C, align(16))]
pub struct UGizmoViewContext {
    __padding_end: [u8; 768],
}
impl UGizmoViewContext {}
#[repr(C, align(16))]
pub struct UGizmoLambdaHitTarget {
    __padding_end: [u8; 304],
}
impl UGizmoLambdaHitTarget {}
#[repr(C, align(16))]
pub struct UGizmoComponentHitTarget {
    __padding_end: [u8; 304],
}
impl UGizmoComponentHitTarget {}
#[repr(C, align(8))]
pub struct AIntervalGizmoActor {
    __padding_end: [u8; 1192],
}
impl AIntervalGizmoActor {}
#[repr(C, align(16))]
pub struct UIntervalGizmoBuilder {
    __padding_end: [u8; 160],
}
impl UIntervalGizmoBuilder {}
#[repr(C, align(16))]
pub struct UIntervalGizmo {
    __padding_end: [u8; 560],
}
impl UIntervalGizmo {}
#[repr(C, align(8))]
pub struct UGizmoBaseFloatParameterSource {
    __padding_end: [u8; 88],
}
impl UGizmoBaseFloatParameterSource {}
#[repr(C, align(8))]
pub struct UGizmoAxisIntervalParameterSource {
    __padding_end: [u8; 112],
}
impl UGizmoAxisIntervalParameterSource {}
#[repr(C, align(8))]
pub struct UGizmoLocalFloatParameterSource {
    __padding_end: [u8; 104],
}
impl UGizmoLocalFloatParameterSource {}
#[repr(C, align(16))]
pub struct UGizmoAxisTranslationParameterSource {
    __padding_end: [u8; 384],
}
impl UGizmoAxisTranslationParameterSource {}
#[repr(C, align(16))]
pub struct UGizmoPlaneTranslationParameterSource {
    __padding_end: [u8; 528],
}
impl UGizmoPlaneTranslationParameterSource {}
#[repr(C, align(16))]
pub struct UGizmoAxisRotationParameterSource {
    __padding_end: [u8; 384],
}
impl UGizmoAxisRotationParameterSource {}
#[repr(C, align(16))]
pub struct UGizmoUniformScaleParameterSource {
    __padding_end: [u8; 480],
}
impl UGizmoUniformScaleParameterSource {}
#[repr(C, align(16))]
pub struct UGizmoAxisScaleParameterSource {
    __padding_end: [u8; 400],
}
impl UGizmoAxisScaleParameterSource {}
#[repr(C, align(16))]
pub struct UGizmoPlaneScaleParameterSource {
    __padding_end: [u8; 576],
}
impl UGizmoPlaneScaleParameterSource {}
#[repr(C, align(8))]
pub struct UPlanePositionGizmoBuilder {
    __padding_end: [u8; 48],
}
impl UPlanePositionGizmoBuilder {}
#[repr(C, align(16))]
pub struct URepositionableTransformGizmoBuilder {
    __padding_end: [u8; 208],
}
impl URepositionableTransformGizmoBuilder {}
#[repr(C, align(16))]
pub struct URepositionableTransformGizmo {
    __padding_end: [u8; 1504],
}
impl URepositionableTransformGizmo {}
#[repr(C, align(8))]
pub struct UScalableSphereGizmoBuilder {
    __padding_end: [u8; 48],
}
impl UScalableSphereGizmoBuilder {}
#[repr(C, align(16))]
pub struct UScalableSphereGizmo {
    __padding_end: [u8; 256],
}
impl UScalableSphereGizmo {}
#[repr(C, align(16))]
pub struct UScalableSphereGizmoInputBehavior {
    __padding_end: [u8; 224],
}
impl UScalableSphereGizmoInputBehavior {}
#[repr(C, align(8))]
pub struct USimpleSingleClickGizmo {
    __padding_end: [u8; 136],
}
impl USimpleSingleClickGizmo {}
#[repr(C, align(8))]
pub struct UGizmoNilStateTarget {
    __padding_end: [u8; 56],
}
impl UGizmoNilStateTarget {}
#[repr(C, align(16))]
pub struct UGizmoLambdaStateTarget {
    __padding_end: [u8; 160],
}
impl UGizmoLambdaStateTarget {}
#[repr(C, align(8))]
pub struct UGizmoObjectModifyStateTarget {
    __padding_end: [u8; 96],
}
impl UGizmoObjectModifyStateTarget {}
#[repr(C, align(16))]
pub struct UGizmoTransformChangeStateTarget {
    __padding_end: [u8; 320],
}
impl UGizmoTransformChangeStateTarget {}
#[repr(C, align(8))]
pub struct UCombinedTransformGizmoContextObject {
    __padding_end: [u8; 104],
}
impl UCombinedTransformGizmoContextObject {}
#[repr(C, align(8))]
pub struct UGizmoBaseTransformSource {
    __padding_end: [u8; 88],
}
impl UGizmoBaseTransformSource {}
#[repr(C, align(8))]
pub struct UGizmoComponentWorldTransformSource {
    __padding_end: [u8; 104],
}
impl UGizmoComponentWorldTransformSource {}
#[repr(C, align(16))]
pub struct UGizmoScaledTransformSource {
    __padding_end: [u8; 208],
}
impl UGizmoScaledTransformSource {}
#[repr(C, align(8))]
pub struct UGizmoTransformProxyTransformSource {
    __padding_end: [u8; 136],
}
impl UGizmoTransformProxyTransformSource {}
#[repr(C, align(8))]
pub struct UGizmoScaledAndUnscaledTransformSources {
    __padding_end: [u8; 120],
}
impl UGizmoScaledAndUnscaledTransformSources {}
#[repr(C, align(16))]
pub struct UViewAdjustedStaticMeshGizmoComponent {
    __padding_end: [u8; 2000],
}
impl UViewAdjustedStaticMeshGizmoComponent {}
#[repr(C, align(8))]
pub struct UInteractiveToolPropertySet {
    __padding_end: [u8; 184],
}
impl UInteractiveToolPropertySet {}
#[repr(C, align(8))]
pub struct UBrushBaseProperties {
    __padding_end: [u8; 216],
}
impl UBrushBaseProperties {}
#[repr(C, align(16))]
pub struct UBrushAdjusterInputBehavior {
    __padding_end: [u8; 224],
}
impl UBrushAdjusterInputBehavior {}
#[repr(C, align(8))]
pub struct UInteractiveTool {
    __padding_end: [u8; 200],
}
impl UInteractiveTool {}
#[repr(C, align(8))]
pub struct USingleSelectionTool {
    __padding_end: [u8; 216],
}
impl USingleSelectionTool {}
#[repr(C, align(8))]
pub struct UMeshSurfacePointTool {
    __padding_end: [u8; 304],
}
impl UMeshSurfacePointTool {}
#[repr(C, align(8))]
pub struct UBaseBrushTool {
    __padding_end: [u8; 752],
}
impl UBaseBrushTool {}
#[repr(C, align(8))]
pub struct UInteractiveToolBuilder {
    __padding_end: [u8; 48],
}
impl UInteractiveToolBuilder {}
#[repr(C, align(8))]
pub struct UClickDragToolBuilder {
    __padding_end: [u8; 48],
}
impl UClickDragToolBuilder {}
#[repr(C, align(8))]
pub struct UClickDragTool {
    __padding_end: [u8; 208],
}
impl UClickDragTool {}
#[repr(C, align(8))]
pub struct UInteractiveToolWithToolTargetsBuilder {
    __padding_end: [u8; 48],
}
impl UInteractiveToolWithToolTargetsBuilder {}
#[repr(C, align(8))]
pub struct UMeshSurfacePointToolBuilder {
    __padding_end: [u8; 56],
}
impl UMeshSurfacePointToolBuilder {}
#[repr(C, align(8))]
pub struct USingleClickToolBuilder {
    __padding_end: [u8; 48],
}
impl USingleClickToolBuilder {}
#[repr(C, align(8))]
pub struct USingleClickTool {
    __padding_end: [u8; 208],
}
impl USingleClickTool {}
#[repr(C, align(8))]
pub struct UContextObjectStore {
    __padding_end: [u8; 64],
}
impl UContextObjectStore {}
#[repr(C, align(8))]
pub struct UInputBehaviorSet {
    __padding_end: [u8; 64],
}
impl UInputBehaviorSet {}
pub struct IInputBehaviorSource {}
#[repr(C, align(8))]
pub struct UInputBehaviorSource {
    __padding_end: [u8; 48],
}
impl UInputBehaviorSource {}
#[repr(C, align(16))]
pub struct ULocalInputBehaviorSource {
    __padding_end: [u8; 112],
}
impl ULocalInputBehaviorSource {}
#[repr(C, align(8))]
pub struct UInputRouter {
    __padding_end: [u8; 440],
}
impl UInputRouter {}
#[repr(C, align(8))]
pub struct UInteractionMechanic {
    __padding_end: [u8; 56],
}
impl UInteractionMechanic {}
#[repr(C, align(8))]
pub struct UInteractiveGizmoManager {
    __padding_end: [u8; 200],
}
impl UInteractiveGizmoManager {}
#[repr(C, align(8))]
pub struct UInteractiveToolManager {
    __padding_end: [u8; 496],
}
impl UInteractiveToolManager {}
#[repr(C, align(16))]
pub struct UInteractiveToolsContext {
    __padding_end: [u8; 688],
}
impl UInteractiveToolsContext {}
#[repr(C, align(8))]
pub struct UMultiSelectionTool {
    __padding_end: [u8; 224],
}
impl UMultiSelectionTool {}
#[repr(C, align(8))]
pub struct USceneSnappingManager {
    __padding_end: [u8; 48],
}
impl USceneSnappingManager {}
#[repr(C, align(8))]
pub struct USelectionSet {
    __padding_end: [u8; 80],
}
impl USelectionSet {}
#[repr(C, align(8))]
pub struct UMeshSelectionSet {
    __padding_end: [u8; 144],
}
impl UMeshSelectionSet {}
#[repr(C, align(8))]
pub struct UToolsContextCursorAPI {
    __padding_end: [u8; 80],
}
impl UToolsContextCursorAPI {}
#[repr(C, align(16))]
pub struct UToolTargetManager {
    __padding_end: [u8; 128],
}
impl UToolTargetManager {}
#[repr(C, align(8))]
pub struct UToolTarget {
    __padding_end: [u8; 48],
}
impl UToolTarget {}
#[repr(C, align(8))]
pub struct UPrimitiveComponentToolTarget {
    __padding_end: [u8; 64],
}
impl UPrimitiveComponentToolTarget {}
#[repr(C, align(8))]
pub struct UToolTargetFactory {
    __padding_end: [u8; 48],
}
impl UToolTargetFactory {}
#[repr(C, align(8))]
pub struct UPrimitiveComponentToolTargetFactory {
    __padding_end: [u8; 48],
}
impl UPrimitiveComponentToolTargetFactory {}
#[repr(C, align(8))]
pub struct USceneComponentToolTarget {
    __padding_end: [u8; 64],
}
impl USceneComponentToolTarget {}
#[repr(C, align(8))]
pub struct USceneComponentToolTargetFactory {
    __padding_end: [u8; 48],
}
impl USceneComponentToolTargetFactory {}
#[repr(transparent)]
pub struct EInputDevices(pub i32);
impl EInputDevices {
    pub const NONE: EInputDevices = EInputDevices(0);
    pub const KEYBOARD: EInputDevices = EInputDevices(1);
    pub const MOUSE: EInputDevices = EInputDevices(2);
    pub const GAMEPAD: EInputDevices = EInputDevices(4);
    pub const OCULUS_TOUCH: EInputDevices = EInputDevices(8);
    pub const HTC_VIVE_WANDS: EInputDevices = EInputDevices(16);
    pub const ANY_SPATIAL_DEVICE: EInputDevices = EInputDevices(24);
    pub const TABLET_FINGERS: EInputDevices = EInputDevices(1024);
}
#[repr(transparent)]
pub struct EToolContextCoordinateSystem(pub u8);
impl EToolContextCoordinateSystem {
    pub const WORLD: EToolContextCoordinateSystem = EToolContextCoordinateSystem(0);
    pub const LOCAL: EToolContextCoordinateSystem = EToolContextCoordinateSystem(1);
    pub const SCREEN: EToolContextCoordinateSystem = EToolContextCoordinateSystem(2);
}
#[repr(transparent)]
pub struct EToolContextTransformGizmoMode(pub u8);
impl EToolContextTransformGizmoMode {
    pub const NO_GIZMO: EToolContextTransformGizmoMode = EToolContextTransformGizmoMode(
        0,
    );
    pub const TRANSLATION: EToolContextTransformGizmoMode = EToolContextTransformGizmoMode(
        1,
    );
    pub const ROTATION: EToolContextTransformGizmoMode = EToolContextTransformGizmoMode(
        2,
    );
    pub const SCALE: EToolContextTransformGizmoMode = EToolContextTransformGizmoMode(3);
    pub const COMBINED: EToolContextTransformGizmoMode = EToolContextTransformGizmoMode(
        8,
    );
}
#[repr(transparent)]
pub struct EGizmoElementState(pub u8);
impl EGizmoElementState {
    pub const NONE: EGizmoElementState = EGizmoElementState(0);
    pub const VISIBLE: EGizmoElementState = EGizmoElementState(2);
    pub const HITTABLE: EGizmoElementState = EGizmoElementState(4);
    pub const VISIBLE_AND_HITTABLE: EGizmoElementState = EGizmoElementState(6);
}
#[repr(transparent)]
pub struct EGizmoElementInteractionState(pub i32);
impl EGizmoElementInteractionState {
    pub const NONE: EGizmoElementInteractionState = EGizmoElementInteractionState(0);
    pub const HOVERING: EGizmoElementInteractionState = EGizmoElementInteractionState(1);
    pub const INTERACTING: EGizmoElementInteractionState = EGizmoElementInteractionState(
        2,
    );
    pub const SELECTED: EGizmoElementInteractionState = EGizmoElementInteractionState(3);
    pub const SUBDUED: EGizmoElementInteractionState = EGizmoElementInteractionState(4);
    pub const MAX: EGizmoElementInteractionState = EGizmoElementInteractionState(5);
}
#[repr(transparent)]
pub struct EGizmoElementViewDependentType(pub i32);
impl EGizmoElementViewDependentType {
    pub const NONE: EGizmoElementViewDependentType = EGizmoElementViewDependentType(0);
    pub const AXIS: EGizmoElementViewDependentType = EGizmoElementViewDependentType(1);
    pub const PLANE: EGizmoElementViewDependentType = EGizmoElementViewDependentType(2);
}
#[repr(transparent)]
pub struct EGizmoElementViewAlignType(pub i32);
impl EGizmoElementViewAlignType {
    pub const NONE: EGizmoElementViewAlignType = EGizmoElementViewAlignType(0);
    pub const POINT_ONLY: EGizmoElementViewAlignType = EGizmoElementViewAlignType(1);
    pub const POINT_EYE: EGizmoElementViewAlignType = EGizmoElementViewAlignType(2);
    pub const POINT_SCREEN: EGizmoElementViewAlignType = EGizmoElementViewAlignType(3);
    pub const AXIAL: EGizmoElementViewAlignType = EGizmoElementViewAlignType(4);
}
#[repr(transparent)]
pub struct EGizmoElementPartialType(pub i32);
impl EGizmoElementPartialType {
    pub const NONE: EGizmoElementPartialType = EGizmoElementPartialType(0);
    pub const PARTIAL: EGizmoElementPartialType = EGizmoElementPartialType(1);
    pub const PARTIAL_VIEW_DEPENDENT: EGizmoElementPartialType = EGizmoElementPartialType(
        2,
    );
}
#[repr(transparent)]
pub struct EGizmoElementArrowHeadType(pub i32);
impl EGizmoElementArrowHeadType {
    pub const NONE: EGizmoElementArrowHeadType = EGizmoElementArrowHeadType(0);
    pub const CONE: EGizmoElementArrowHeadType = EGizmoElementArrowHeadType(1);
    pub const CUBE: EGizmoElementArrowHeadType = EGizmoElementArrowHeadType(2);
    pub const SPHERE: EGizmoElementArrowHeadType = EGizmoElementArrowHeadType(3);
}
