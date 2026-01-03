#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FCurveEditorBakeFilterRange {
    pub min: crate::bindings::core_u_object::FFrameNumber,
    pub max: crate::bindings::core_u_object::FFrameNumber,
}
impl FCurveEditorBakeFilterRange {}
#[repr(C, align(4))]
pub struct FGaussianParams {
    pub kernel_width: i32,
}
impl FGaussianParams {}
#[repr(C, align(4))]
pub struct FSmartReduceParams {
    pub tolerance_percentage: f32,
    pub sample_rate: crate::bindings::core_u_object::FFrameRate,
}
impl FSmartReduceParams {}
#[repr(C, align(8))]
pub struct UCurveEditorCopyableCurveKeys {
    __padding_end: [u8; 144],
}
impl UCurveEditorCopyableCurveKeys {}
#[repr(C, align(8))]
pub struct UCurveEditorCopyBuffer {
    __padding_end: [u8; 80],
}
impl UCurveEditorCopyBuffer {}
pub struct ICurveEditorKeyProxy {}
#[repr(C, align(8))]
pub struct UCurveEditorKeyProxy {
    __padding_end: [u8; 48],
}
impl UCurveEditorKeyProxy {}
#[repr(C, align(8))]
pub struct UCurveEditorTransactionObject {
    __padding_end: [u8; 64],
}
impl UCurveEditorTransactionObject {}
#[repr(C, align(8))]
pub struct URichCurveKeyProxy {
    __padding_end: [u8; 120],
}
impl URichCurveKeyProxy {}
#[repr(C, align(8))]
pub struct UCurveEditorSettings {
    __padding_end: [u8; 264],
}
impl UCurveEditorSettings {}
#[repr(C, align(8))]
pub struct UCurveEditorFilterBase {
    __padding_end: [u8; 48],
}
impl UCurveEditorFilterBase {}
#[repr(C, align(8))]
pub struct UCurveEditorBakeFilter {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub bake_interval_in_seconds: f32,
    pub bake_interval: crate::bindings::core_u_object::FFrameNumber,
    pub b_custom_range_override: bool,
    pub custom_range_min_in_seconds: f32,
    pub custom_range_max_in_seconds: f32,
    pub custom_range: FCurveEditorBakeFilterRange,
    __padding_end: [u8; 20],
}
impl UCurveEditorBakeFilter {}
#[repr(C, align(8))]
pub struct UCurveEditorEulerFilter {
    __padding_end: [u8; 48],
}
impl UCurveEditorEulerFilter {}
#[repr(C, align(8))]
pub struct UCurveEditorGaussianFilter {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub gaussian_params: FGaussianParams,
    __padding_end: [u8; 4],
}
impl UCurveEditorGaussianFilter {}
#[repr(C, align(8))]
pub struct UCurveEditorReduceFilter {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub tolerance: f32,
    pub b_try_remove_user_set_tangent_keys: bool,
    pub sample_rate: crate::bindings::core_u_object::FFrameRate,
}
impl UCurveEditorReduceFilter {}
#[repr(C, align(8))]
pub struct UCurveEditorSmartReduceFilter {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub smart_reduce_params: FSmartReduceParams,
    __padding_end: [u8; 4],
}
impl UCurveEditorSmartReduceFilter {}
#[repr(C, align(8))]
pub struct UCurveEditorSmartSnapFilter {
    __padding_end: [u8; 48],
}
impl UCurveEditorSmartSnapFilter {}
#[repr(transparent)]
pub struct ECurveEditorPanningMouseButton(pub u8);
impl ECurveEditorPanningMouseButton {
    pub const RIGHT: ECurveEditorPanningMouseButton = ECurveEditorPanningMouseButton(0);
    pub const ALT_MIDDLE: ECurveEditorPanningMouseButton = ECurveEditorPanningMouseButton(
        1,
    );
    pub const RIGHT_OR_ALT_MIDDLE: ECurveEditorPanningMouseButton = ECurveEditorPanningMouseButton(
        2,
    );
}
#[repr(transparent)]
pub struct ECurveEditorTangentVisibility(pub u8);
impl ECurveEditorTangentVisibility {
    pub const ALL_TANGENTS: ECurveEditorTangentVisibility = ECurveEditorTangentVisibility(
        0,
    );
    pub const SELECTED_KEYS: ECurveEditorTangentVisibility = ECurveEditorTangentVisibility(
        1,
    );
    pub const NO_TANGENTS: ECurveEditorTangentVisibility = ECurveEditorTangentVisibility(
        2,
    );
    pub const USER_TANGENTS: ECurveEditorTangentVisibility = ECurveEditorTangentVisibility(
        3,
    );
    pub const NUM: ECurveEditorTangentVisibility = ECurveEditorTangentVisibility(4);
}
#[repr(transparent)]
pub struct ECurveEditorZoomPosition(pub u8);
impl ECurveEditorZoomPosition {
    pub const CURRENT_TIME: ECurveEditorZoomPosition = ECurveEditorZoomPosition(0);
    pub const MOUSE_POSITION: ECurveEditorZoomPosition = ECurveEditorZoomPosition(1);
}
#[repr(transparent)]
pub struct ECurveEditorSnapAxis(pub u8);
impl ECurveEditorSnapAxis {
    pub const CESA_NONE: ECurveEditorSnapAxis = ECurveEditorSnapAxis(0);
    pub const CESA_X: ECurveEditorSnapAxis = ECurveEditorSnapAxis(1);
    pub const CESA_Y: ECurveEditorSnapAxis = ECurveEditorSnapAxis(2);
}
