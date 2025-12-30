#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FAudioMeterChannelInfo {
    pub meter_value: f32,
    pub peak_value: f32,
    pub clipping_value: f32,
}
#[repr(C, align(16))]
pub struct FAudioMeterWidgetStyle {
    pub meter_value_image: FSlateBrush,
    pub background_image: FSlateBrush,
    pub meter_background_image: FSlateBrush,
    pub meter_value_background_image: FSlateBrush,
    pub meter_peak_image: FSlateBrush,
    pub meter_size: FVector2D,
    pub meter_padding: FVector2D,
    pub meter_value_padding: f32,
    pub peak_value_width: f32,
    pub value_range_db: FVector2D,
    pub b_show_scale: bool,
    pub b_scale_side: bool,
    pub scale_hash_offset: f32,
    pub scale_hash_width: f32,
    pub scale_hash_height: f32,
    pub decibels_per_hash: i32,
    pub font: FSlateFontInfo,
}
#[repr(C, align(8))]
pub struct FAudioMeterDefaultColorWidgetStyle {
    pub meter_background_color: FLinearColor,
    pub meter_value_color: FLinearColor,
    pub meter_peak_color: FLinearColor,
    pub meter_clipping_color: FLinearColor,
    pub meter_scale_color: FLinearColor,
    pub meter_scale_label_color: FLinearColor,
}
