#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FStructSerializerNumericTestStruct {
    pub int8: i8,
    pub int16: i16,
    pub int32: i32,
    pub int64: i64,
    pub u_int8: u8,
    pub u_int16: u16,
    pub u_int32: u32,
    pub u_int64: u64,
    pub float: f32,
    pub double: f64,
}
#[repr(C, align(1))]
pub struct FStructSerializerBooleanTestStruct {
    pub bool_false: bool,
    pub bool_true: bool,
    pub flags_2: u8,
}
#[repr(C, align(8))]
pub struct FStructSerializerObjectTestStruct {
    pub raw_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub wrapped_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub sub_class: TSubclassOf<UObjectTest>,
    pub soft_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub raw_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub wrapped_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub weak_object: TWeakObjectPtr<UObjectTest>,
    pub soft_object: TSoftObjectPtr<UObjectTest>,
    pub class_path: crate::bindings::core_u_object::FSoftClassPath,
    pub object_path: crate::bindings::core_u_object::FSoftObjectPath,
}
#[repr(C, align(16))]
pub struct FStructSerializerBuiltinTestStruct {
    pub guid: crate::bindings::core_u_object::FGuid,
    pub name: FName,
    pub string: FString,
    pub text: FText,
    pub datetime: crate::bindings::core_u_object::FDateTime,
    pub timespan: crate::bindings::core_u_object::FTimespan,
    pub vector: crate::bindings::core_u_object::FVector,
    pub vector4: crate::bindings::core_u_object::FVector4,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub quat: crate::bindings::core_u_object::FQuat,
    pub color: crate::bindings::core_u_object::FColor,
}
#[repr(C, align(16))]
pub struct FStructSerializerLWCTypesTest {
    pub vector: crate::bindings::core_u_object::FVector,
    pub vector2_d: crate::bindings::core_u_object::FVector2D,
    pub vector4: crate::bindings::core_u_object::FVector4,
    pub matrix: crate::bindings::core_u_object::FMatrix,
    pub plane: crate::bindings::core_u_object::FPlane,
    pub quat: crate::bindings::core_u_object::FQuat,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub box_: crate::bindings::core_u_object::FBox,
    pub box2_d: crate::bindings::core_u_object::FBox2D,
    pub box_sphere_bounds: crate::bindings::core_u_object::FBoxSphereBounds,
    pub oriented_box: crate::bindings::core_u_object::FOrientedBox,
    pub float: f32,
    pub double: f64,
    pub vector_array: TArray<crate::bindings::core_u_object::FVector>,
    pub str_to_vec: TMap<FString, crate::bindings::core_u_object::FVector>,
    pub vector_set: TSet<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(4))]
pub struct FOrientedBoxFloat {
    pub center: crate::bindings::core_u_object::FVector3f,
    pub axis_x: crate::bindings::core_u_object::FVector3f,
    pub axis_y: crate::bindings::core_u_object::FVector3f,
    pub axis_z: crate::bindings::core_u_object::FVector3f,
    pub extent_x: f32,
    pub extent_y: f32,
    pub extent_z: f32,
}
#[repr(C, align(16))]
pub struct FStructSerializerNonLWCTypesTest {
    pub vector: crate::bindings::core_u_object::FVector3f,
    pub vector2_d: crate::bindings::core_u_object::FVector2f,
    pub vector4: crate::bindings::core_u_object::FVector4f,
    pub matrix: crate::bindings::core_u_object::FMatrix44f,
    pub plane: crate::bindings::core_u_object::FPlane4f,
    pub quat: crate::bindings::core_u_object::FQuat4f,
    pub rotator: crate::bindings::core_u_object::FRotator3f,
    pub transform: crate::bindings::core_u_object::FTransform3f,
    pub box_: crate::bindings::core_u_object::FBox3f,
    pub box2_d: crate::bindings::core_u_object::FBox2f,
    pub box_sphere_bounds: crate::bindings::core_u_object::FBoxSphereBounds3f,
    pub oriented_box: FOrientedBoxFloat,
    pub float: f32,
    pub double: f64,
    pub vector_array: TArray<crate::bindings::core_u_object::FVector3f>,
    pub str_to_vec: TMap<FString, crate::bindings::core_u_object::FVector3f>,
    pub vector_set: TSet<crate::bindings::core_u_object::FVector3f>,
}
#[repr(C, align(8))]
pub struct FStructSerializerByteArray {
    pub dummy1: i32,
    pub byte_array: TArray<u8>,
    pub dummy2: i32,
    pub int8_array: TArray<i8>,
    pub dummy3: i32,
}
#[repr(C, align(8))]
pub struct FStructSerializerArrayTestStruct {
    pub int32_array: TArray<i32>,
    pub byte_array: TArray<u8>,
    pub static_single_element: i32,
    pub static_int32_array: i32,
    pub static_float_array: f32,
    pub vector_array: TArray<crate::bindings::core_u_object::FVector>,
    pub struct_array: TArray<FStructSerializerBuiltinTestStruct>,
}
#[repr(C, align(8))]
pub struct FStructSerializerMapTestStruct {
    pub int_to_str: TMap<i32, FString>,
    pub str_to_str: TMap<FString, FString>,
    pub str_to_vec: TMap<FString, crate::bindings::core_u_object::FVector>,
    pub str_to_struct: TMap<FString, FStructSerializerBuiltinTestStruct>,
}
#[repr(C, align(8))]
pub struct FStructSerializerSetTestStruct {
    pub str_set: TSet<FString>,
    pub int_set: TSet<i32>,
    pub name_set: TSet<FName>,
    pub struct_set: TSet<FStructSerializerBuiltinTestStruct>,
}
#[repr(C, align(16))]
pub struct FStructSerializerOptionalTestStruct {
    pub str_optional: TOptional<FString>,
    pub str_optional_unset: TOptional<FString>,
    pub int_optional: TOptional<i32>,
    pub int_optional_unset: TOptional<i32>,
    pub name_optional: TOptional<FName>,
    pub name_optional_unset: TOptional<FName>,
    pub struct_optional: TOptional<FStructSerializerBuiltinTestStruct>,
    pub struct_optional_unset: TOptional<FStructSerializerBuiltinTestStruct>,
}
#[repr(C, align(16))]
pub struct FStructSerializerTestStruct {
    pub numerics: FStructSerializerNumericTestStruct,
    pub booleans: FStructSerializerBooleanTestStruct,
    pub objects: FStructSerializerObjectTestStruct,
    pub builtins: FStructSerializerBuiltinTestStruct,
    pub arrays: FStructSerializerArrayTestStruct,
    pub maps: FStructSerializerMapTestStruct,
    pub sets: FStructSerializerSetTestStruct,
    pub optionals: FStructSerializerOptionalTestStruct,
    pub lwc_types: FStructSerializerLWCTypesTest,
}
pub struct UObjectTest {}
