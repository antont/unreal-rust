#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(16))]
pub struct FPIEPreviewWindowStyle {
    pub screen_rotation_button_style: FButtonStyle,
    pub quarter_mobile_content_scale_factor_button_style: FButtonStyle,
    pub half_mobile_content_scale_factor_button_style: FButtonStyle,
    pub full_mobile_content_scale_factor_button_style: FButtonStyle,
}
pub struct UPIEPreviewSettings {
    pub window_pos_x: i32,
    pub window_pos_y: i32,
    pub window_scaling_factor: f32,
}
