#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FAudioMeterChannelInfo {
    pub meter_value: f32,
    pub peak_value: f32,
    pub clipping_value: f32,
}
impl FAudioMeterChannelInfo {}
#[repr(C, align(16))]
pub struct FAudioMeterWidgetStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub meter_value_image: crate::bindings::slate_core::FSlateBrush,
    pub background_image: crate::bindings::slate_core::FSlateBrush,
    pub meter_background_image: crate::bindings::slate_core::FSlateBrush,
    pub meter_value_background_image: crate::bindings::slate_core::FSlateBrush,
    pub meter_peak_image: crate::bindings::slate_core::FSlateBrush,
    pub meter_size: crate::bindings::core_u_object::FVector2D,
    pub meter_padding: crate::bindings::core_u_object::FVector2D,
    pub meter_value_padding: f32,
    pub peak_value_width: f32,
    pub value_range_db: crate::bindings::core_u_object::FVector2D,
    pub b_show_scale: bool,
    pub b_scale_side: bool,
    pub scale_hash_offset: f32,
    pub scale_hash_width: f32,
    pub scale_hash_height: f32,
    pub decibels_per_hash: i32,
    pub font: crate::bindings::slate_core::FSlateFontInfo,
}
impl FAudioMeterWidgetStyle {}
#[repr(C, align(8))]
pub struct FAudioMeterDefaultColorWidgetStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub meter_background_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_value_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_peak_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_clipping_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_scale_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_scale_label_color: crate::bindings::core_u_object::FLinearColor,
}
impl FAudioMeterDefaultColorWidgetStyle {}
