#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(16))]
pub struct FWidgetCarouselNavigationButtonStyle {
    pub inner_button_style: FButtonStyle,
    pub navigation_button_left_image: FSlateBrush,
    pub navigation_button_right_image: FSlateBrush,
}
#[repr(C, align(16))]
pub struct FWidgetCarouselNavigationBarStyle {
    pub highlight_brush: FSlateBrush,
    pub left_button_style: FButtonStyle,
    pub center_button_style: FButtonStyle,
    pub right_button_style: FButtonStyle,
}
