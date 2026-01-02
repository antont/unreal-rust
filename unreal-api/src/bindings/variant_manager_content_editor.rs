#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAssetDefinition_LevelVariantSets {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_LevelVariantSets {}
#[repr(C, align(8))]
pub struct ULevelVariantSetsActorFactory {
    __padding_end: [u8; 144],
}
impl ULevelVariantSetsActorFactory {}
#[repr(C, align(8))]
pub struct USwitchActorFactory {
    __padding_end: [u8; 144],
}
impl USwitchActorFactory {}
#[repr(C, align(8))]
pub struct UVariantManagerFactoryNew {
    __padding_end: [u8; 136],
}
impl UVariantManagerFactoryNew {}
#[repr(C, align(16))]
pub struct AVariantManagerTestActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub enum_with_no_default: EVariantManagerTestEnum,
    pub enum_with_second_default: EVariantManagerTestEnum,
    pub captured_byte_property: u8,
    pub captured_int_property: i32,
    pub captured_float_property: f32,
    pub b_captured_bool_property: bool,
    pub captured_object_property: UPtr<crate::bindings::core_u_object::UObject>,
    pub captured_interface_property: FScriptInterface,
    pub captured_name_property: FName,
    pub captured_str_property: FString,
    pub captured_text_property: FText,
    pub captured_rotator_property: crate::bindings::core_u_object::FRotator,
    pub captured_color_property: crate::bindings::core_u_object::FColor,
    pub captured_linear_color_property: crate::bindings::core_u_object::FLinearColor,
    pub captured_vector_property: crate::bindings::core_u_object::FVector,
    pub captured_quat_property: crate::bindings::core_u_object::FQuat,
    pub captured_vector4_property: crate::bindings::core_u_object::FVector4,
    pub captured_vector2_d_property: crate::bindings::core_u_object::FVector2D,
    pub captured_int_point_property: crate::bindings::core_u_object::FIntPoint,
    pub captured_u_object_array_property: TArray<
        UPtr<crate::bindings::core_u_object::UObject>,
    >,
    pub captured_vector_array_property: TArray<crate::bindings::core_u_object::FVector>,
    __padding_end: [u8; 8],
}
impl AVariantManagerTestActor {}
#[repr(transparent)]
pub struct EVariantManagerTestEnum(pub u8);
impl EVariantManagerTestEnum {
    pub const NONE: EVariantManagerTestEnum = EVariantManagerTestEnum(0);
    pub const FIRST_OPTION: EVariantManagerTestEnum = EVariantManagerTestEnum(1);
    pub const SECOND_OPTION: EVariantManagerTestEnum = EVariantManagerTestEnum(3);
    pub const THIRD_OPTION: EVariantManagerTestEnum = EVariantManagerTestEnum(45);
}
