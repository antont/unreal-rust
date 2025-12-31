#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FPropertyBindingUtilsTest_PropertyStructB {
    pub b: i32,
}
#[repr(C, align(4))]
pub struct FPropertyBindingUtilsTest_PropertyStruct {
    pub a: i32,
    pub b: i32,
    pub struct_b: FPropertyBindingUtilsTest_PropertyStructB,
}
pub struct UPropertyBindingUtilsTest_PropertyObjectInstanced {
    pub a: i32,
    pub instanced_struct: crate::bindings::core_u_object::FInstancedStruct,
}
pub struct UPropertyBindingUtilsTest_PropertyObjectInstancedWithB {
    pub b: i32,
}
pub struct UPropertyBindingUtilsTest_PropertyObject {
    pub instanced_object: UPtr<UPropertyBindingUtilsTest_PropertyObjectInstanced>,
    pub array_of_instanced_objects: TArray<
        UPtr<UPropertyBindingUtilsTest_PropertyObjectInstanced>,
    >,
    pub array_of_ints: TArray<i32>,
    pub instanced_struct: crate::bindings::core_u_object::FInstancedStruct,
    pub array_of_instanced_structs: TArray<
        crate::bindings::core_u_object::FInstancedStruct,
    >,
    pub _struct: FPropertyBindingUtilsTest_PropertyStruct,
    pub array_of_struct: TArray<FPropertyBindingUtilsTest_PropertyStruct>,
}
