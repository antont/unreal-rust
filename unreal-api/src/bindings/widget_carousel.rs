#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FWidgetCarouselNavigationButtonStyle {
    pub inner_button_style: crate::bindings::slate_core::FButtonStyle,
    pub navigation_button_left_image: crate::bindings::slate_core::FSlateBrush,
    pub navigation_button_right_image: crate::bindings::slate_core::FSlateBrush,
}
#[repr(C, align(16))]
pub struct FWidgetCarouselNavigationBarStyle {
    pub highlight_brush: crate::bindings::slate_core::FSlateBrush,
    pub left_button_style: crate::bindings::slate_core::FButtonStyle,
    pub center_button_style: crate::bindings::slate_core::FButtonStyle,
    pub right_button_style: crate::bindings::slate_core::FButtonStyle,
}
