#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FPreviewableWidgetVariant {
    pub object_path: crate::bindings::core_u_object::FSoftObjectPath,
    __padding_end: [u8; 16],
}
impl FPreviewableWidgetVariant {}
#[repr(C, align(8))]
pub struct UAssetDefinition_WidgetPreview {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_WidgetPreview {}
#[repr(C, align(8))]
pub struct UWidgetPreview {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub widget_type: FPreviewableWidgetVariant,
    pub slot_widget_types: TMap<FName, FPreviewableWidgetVariant>,
    pub b_should_override_widget_size: bool,
    pub overridden_widget_size: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 88],
}
impl UWidgetPreview {}
#[repr(C, align(8))]
pub struct UWidgetPreviewEditor {
    __padding_end: [u8; 72],
}
impl UWidgetPreviewEditor {}
#[repr(C, align(8))]
pub struct UWidgetPreviewFactory {
    __padding_end: [u8; 136],
}
impl UWidgetPreviewFactory {}
