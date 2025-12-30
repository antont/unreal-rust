#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FKeyPosition {
    pub input_value: f64,
    pub output_value: f64,
}
#[repr(C, align(4))]
pub struct FKeyAttributes {
    pub flags_0: u8,
    pub arrive_tangent: f32,
    pub leave_tangent: f32,
    pub interp_mode: ERichCurveInterpMode,
    pub tangent_mode: ERichCurveTangentMode,
    pub tangent_weight_mode: ERichCurveTangentWeightMode,
    pub arrive_tangent_weight: f32,
    pub leave_tangent_weight: f32,
}
#[repr(C, align(8))]
pub struct FCurveEditorZoomScaleConfig {
    pub mouse_wheel_zoom_multiplier: f64,
    pub horizontal_zoom_scale: FRuntimeFloatCurve,
    pub vertical_zoom_scale: FRuntimeFloatCurve,
    pub b_limit_horizontal_zoom_out: bool,
    pub max_horizontal_zoom_out: f64,
    pub b_limit_vertical_zoom_out: bool,
    pub max_vertical_zoom_out: f64,
}
#[repr(C, align(8))]
pub struct FCustomColorForChannel {
    pub object: TSoftObjectPtr<UClass>,
    pub property_name: FString,
    pub color: FLinearColor,
}
#[repr(C, align(8))]
pub struct FCustomColorForSpaceSwitch {
    pub control_name: FString,
    pub color: FLinearColor,
}
#[repr(C, align(4))]
pub struct FCurveEditorBakeFilterRange {
    pub min: FFrameNumber,
    pub max: FFrameNumber,
}
#[repr(C, align(4))]
pub struct FGaussianParams {
    pub kernel_width: i32,
}
#[repr(C, align(4))]
pub struct FSmartReduceParams {
    pub tolerance_percentage: f32,
    pub sample_rate: FFrameRate,
}
pub struct UCurveEditorCopyableCurveKeys {
    pub key_positions: TArray<FKeyPosition>,
    pub key_attributes: TArray<FKeyAttributes>,
    pub short_display_name: FString,
    pub long_display_name: FString,
    pub intention_name: FString,
    pub long_intention_name: FString,
}
pub struct UCurveEditorCopyBuffer {
    pub curves: TArray<UPtr<UCurveEditorCopyableCurveKeys>>,
    pub time_offset: f64,
    pub b_absolute_position: bool,
}
pub struct UCurveEditorKeyProxy {}
pub struct ICurveEditorKeyProxy {}
pub struct UCurveEditorTransactionObject {}
pub struct URichCurveKeyProxy {
    pub value: FRichCurveKey,
}
pub struct UCurveEditorSettings {
    pub panning_mouse_button: ECurveEditorPanningMouseButton,
    pub b_auto_frame_curve_editor: bool,
    pub b_show_bars: bool,
    pub frame_input_padding: i32,
    pub frame_output_padding: i32,
    pub b_show_buffered_curves: bool,
    pub b_show_curve_editor_curve_tool_tips: bool,
    pub tangent_visibility: ECurveEditorTangentVisibility,
    pub zoom_position: ECurveEditorZoomPosition,
    pub snap_axis: ECurveEditorSnapAxis,
    pub b_snap_time_to_selection: bool,
    pub selection_color: FLinearColor,
    pub custom_colors: TArray<FCustomColorForChannel>,
    pub parent_space_custom_color: FLinearColor,
    pub world_space_custom_color: FLinearColor,
    pub control_space_custom_colors: TArray<FCustomColorForSpaceSwitch>,
    pub tree_view_width: f32,
    pub b_scrub_time_start_from_cursor: bool,
    pub marquee_point_sensitivity: f32,
    pub b_show_value_indicators: bool,
}
pub struct UCurveEditorFilterBase {}
pub struct UCurveEditorBakeFilter {
    pub bake_interval_in_seconds: f32,
    pub bake_interval: FFrameNumber,
    pub b_custom_range_override: bool,
    pub custom_range_min_in_seconds: f32,
    pub custom_range_max_in_seconds: f32,
    pub custom_range: FCurveEditorBakeFilterRange,
    pub b_use_seconds: bool,
}
pub struct UCurveEditorEulerFilter {}
pub struct UCurveEditorGaussianFilter {
    pub gaussian_params: FGaussianParams,
}
pub struct UCurveEditorReduceFilter {
    pub tolerance: f32,
    pub b_try_remove_user_set_tangent_keys: bool,
    pub sample_rate: FFrameRate,
}
pub struct UCurveEditorSmartReduceFilter {
    pub smart_reduce_params: FSmartReduceParams,
}
pub struct UCurveEditorSmartSnapFilter {}
