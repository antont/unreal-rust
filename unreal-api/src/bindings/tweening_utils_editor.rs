#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(16))]
pub struct FTweenPointStyle {
    pub normal: FSlateBrush,
    pub hovered: FSlateBrush,
    pub pressed: FSlateBrush,
    pub passed_point: FSlateBrush,
    pub hit_test_size: FVector2D,
}
#[repr(C, align(16))]
pub struct FTweenSliderStyle {
    pub bar_dimensions: FVector2D,
    pub bar_brush: FSlateBrush,
    pub normal_slider_button: FSlateBrush,
    pub hovered_slider_button: FSlateBrush,
    pub pressed_slider_button: FSlateBrush,
    pub normal_icon_tint: FSlateColor,
    pub hovered_icon_tint: FSlateColor,
    pub pressed_icon_tint: FSlateColor,
    pub small_point: FTweenPointStyle,
    pub medium_point: FTweenPointStyle,
    pub end_point: FTweenPointStyle,
    pub passed_value_background: FSlateBrush,
    pub icon_padding: FMargin,
}
pub struct UTweeningToolsUserSettings {
    pub preferred_tween_function: TMap<FName, FString>,
}
