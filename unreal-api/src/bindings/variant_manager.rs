#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FCapturableProperty {
    pub display_name: FString,
}
pub struct UPropertyTemplateObject {
    pub captured_byte_property: u8,
    pub captured_u_int16_property: u16,
    pub captured_u_int32_property: u32,
    pub captured_u_int64_property: u64,
    pub captured_int8_property: i8,
    pub captured_int16_property: i16,
    pub captured_int_property: i32,
    pub captured_int64_property: i64,
    pub captured_float_property: f32,
    pub captured_double_property: f64,
    pub b_captured_bool_property: bool,
    pub captured_object_property: UPtr<crate::bindings::core_u_object::UObject>,
    pub captured_soft_object_property: TSoftObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
    pub captured_interface_property: FScriptInterface,
    pub captured_name_property: FName,
    pub captured_str_property: FString,
    pub captured_text_property: FText,
    pub captured_vector_property: crate::bindings::core_u_object::FVector,
}
pub struct UVariantManagerBlueprintLibrary {}
