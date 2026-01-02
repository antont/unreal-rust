#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FColorGradingSpinBoxStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub border_brush: crate::bindings::slate_core::FSlateBrush,
    pub active_border_brush: crate::bindings::slate_core::FSlateBrush,
    pub hovered_border_brush: crate::bindings::slate_core::FSlateBrush,
    pub selector_brush: crate::bindings::slate_core::FSlateBrush,
    pub selector_width: f32,
    __padding_end: [u8; 12],
}
impl FColorGradingSpinBoxStyle {}
#[repr(C, align(16))]
pub struct URadialSlider {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub value: f32,
    #[doc(hidden)]
    __padding_744: [u8; 40],
    pub slider_range: crate::bindings::engine::FRuntimeFloatCurve,
    pub value_tags: TArray<f32>,
    pub slider_handle_start_angle: f32,
    pub slider_handle_end_angle: f32,
    pub angular_offset: f32,
    pub hand_start_end_ratio: crate::bindings::core_u_object::FVector2D,
    pub widget_style: crate::bindings::slate_core::FSliderStyle,
    pub slider_bar_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_progress_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_handle_color: crate::bindings::core_u_object::FLinearColor,
    pub center_background_color: crate::bindings::core_u_object::FLinearColor,
    pub locked: bool,
    pub mouse_uses_step: bool,
    pub requires_controller_lock: bool,
    pub step_size: f32,
    pub is_focusable: bool,
    pub use_vertical_drag: bool,
    pub show_slider_handle: bool,
    pub show_slider_hand: bool,
    __padding_end: [u8; 148],
}
impl URadialSlider {}
#[repr(transparent)]
pub struct FRadialSlider_ValueDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct FRadialSlider_OnMouseCaptureBegin {
    _opague: u8,
}
#[repr(transparent)]
pub struct FRadialSlider_OnMouseCaptureEnd {
    _opague: u8,
}
#[repr(transparent)]
pub struct FRadialSlider_OnControllerCaptureBegin {
    _opague: u8,
}
#[repr(transparent)]
pub struct FRadialSlider_OnControllerCaptureEnd {
    _opague: u8,
}
#[repr(transparent)]
pub struct FRadialSlider_OnValueChanged {
    _opague: u8,
}
