#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FTweenPointStyle {
    pub normal: crate::bindings::slate_core::FSlateBrush,
    pub hovered: crate::bindings::slate_core::FSlateBrush,
    pub pressed: crate::bindings::slate_core::FSlateBrush,
    pub passed_point: crate::bindings::slate_core::FSlateBrush,
    pub hit_test_size: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(16))]
pub struct FTweenSliderStyle {
    pub bar_dimensions: crate::bindings::core_u_object::FVector2D,
    pub bar_brush: crate::bindings::slate_core::FSlateBrush,
    pub normal_slider_button: crate::bindings::slate_core::FSlateBrush,
    pub hovered_slider_button: crate::bindings::slate_core::FSlateBrush,
    pub pressed_slider_button: crate::bindings::slate_core::FSlateBrush,
    pub normal_icon_tint: crate::bindings::slate_core::FSlateColor,
    pub hovered_icon_tint: crate::bindings::slate_core::FSlateColor,
    pub pressed_icon_tint: crate::bindings::slate_core::FSlateColor,
    pub small_point: FTweenPointStyle,
    pub medium_point: FTweenPointStyle,
    pub end_point: FTweenPointStyle,
    pub passed_value_background: crate::bindings::slate_core::FSlateBrush,
    pub icon_padding: crate::bindings::slate_core::FMargin,
}
pub struct UTweeningToolsUserSettings {
    pub preferred_tween_function: TMap<FName, FString>,
}
