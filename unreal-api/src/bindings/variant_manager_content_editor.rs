#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UAssetDefinition_LevelVariantSets {}
pub struct ULevelVariantSetsActorFactory {}
pub struct USwitchActorFactory {}
pub struct UVariantManagerFactoryNew {}
pub struct AVariantManagerTestActor {
    pub enum_with_no_default: EVariantManagerTestEnum,
    pub enum_with_second_default: EVariantManagerTestEnum,
    pub captured_byte_property: u8,
    pub captured_int_property: i32,
    pub captured_float_property: f32,
    pub b_captured_bool_property: bool,
    pub captured_object_property: UPtr<UObject>,
    pub captured_interface_property: FScriptInterface,
    pub captured_name_property: FName,
    pub captured_str_property: FString,
    pub captured_text_property: FText,
    pub captured_rotator_property: FRotator,
    pub captured_color_property: FColor,
    pub captured_linear_color_property: FLinearColor,
    pub captured_vector_property: FVector,
    pub captured_quat_property: FQuat,
    pub captured_vector4_property: FVector4,
    pub captured_vector2_d_property: FVector2D,
    pub captured_int_point_property: FIntPoint,
    pub captured_u_object_array_property: TArray<UPtr<UObject>>,
    pub captured_vector_array_property: TArray<FVector>,
}
