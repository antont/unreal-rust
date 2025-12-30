#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub raw_class: TSubclassOf<UObject>,
    pub wrapped_class: TSubclassOf<UObject>,
    pub sub_class: TSubclassOf<UObjectTest>,
    pub soft_class: TSoftObjectPtr<UClass>,
    pub raw_object: UPtr<UObject>,
    pub wrapped_object: UPtr<UObject>,
    pub weak_object: TWeakObjectPtr<UObjectTest>,
    pub soft_object: TSoftObjectPtr<UObjectTest>,
    pub class_path: FSoftClassPath,
    pub object_path: FSoftObjectPath,
}
#[repr(C, align(16))]
pub struct FStructSerializerBuiltinTestStruct {
    pub guid: FGuid,
    pub name: FName,
    pub string: FString,
    pub text: FText,
    pub datetime: FDateTime,
    pub timespan: FTimespan,
    pub vector: FVector,
    pub vector4: FVector4,
    pub rotator: FRotator,
    pub quat: FQuat,
    pub color: FColor,
}
#[repr(C, align(16))]
pub struct FStructSerializerLWCTypesTest {
    pub vector: FVector,
    pub vector2_d: FVector2D,
    pub vector4: FVector4,
    pub matrix: FMatrix,
    pub plane: FPlane,
    pub quat: FQuat,
    pub rotator: FRotator,
    pub transform: FTransform,
    pub box_: FBox,
    pub box2_d: FBox2D,
    pub box_sphere_bounds: FBoxSphereBounds,
    pub oriented_box: FOrientedBox,
    pub float: f32,
    pub double: f64,
    pub vector_array: TArray<FVector>,
    pub str_to_vec: TMap<FString, FVector>,
    pub vector_set: TSet<FVector>,
}
#[repr(C, align(4))]
pub struct FOrientedBoxFloat {
    pub center: FVector3f,
    pub axis_x: FVector3f,
    pub axis_y: FVector3f,
    pub axis_z: FVector3f,
    pub extent_x: f32,
    pub extent_y: f32,
    pub extent_z: f32,
}
#[repr(C, align(16))]
pub struct FStructSerializerNonLWCTypesTest {
    pub vector: FVector3f,
    pub vector2_d: FVector2f,
    pub vector4: FVector4f,
    pub matrix: FMatrix44f,
    pub plane: FPlane4f,
    pub quat: FQuat4f,
    pub rotator: FRotator3f,
    pub transform: FTransform3f,
    pub box_: FBox3f,
    pub box2_d: FBox2f,
    pub box_sphere_bounds: FBoxSphereBounds3f,
    pub oriented_box: FOrientedBoxFloat,
    pub float: f32,
    pub double: f64,
    pub vector_array: TArray<FVector3f>,
    pub str_to_vec: TMap<FString, FVector3f>,
    pub vector_set: TSet<FVector3f>,
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
    pub vector_array: TArray<FVector>,
    pub struct_array: TArray<FStructSerializerBuiltinTestStruct>,
}
#[repr(C, align(8))]
pub struct FStructSerializerMapTestStruct {
    pub int_to_str: TMap<i32, FString>,
    pub str_to_str: TMap<FString, FString>,
    pub str_to_vec: TMap<FString, FVector>,
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
