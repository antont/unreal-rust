#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(1))]
pub struct FTestStructSimpleBase {}
#[repr(C, align(4))]
pub struct FTestStructSimple {
    pub float: f32,
}
#[repr(C, align(4))]
pub struct FTestStructSimpleNonZeroDefault {
    pub float: f32,
}
#[repr(C, align(8))]
pub struct FTestStructComplex {
    pub string: FString,
    pub string_array: TArray<FString>,
}
#[repr(C, align(4))]
pub struct FTestStructSimple1 {
    pub float: f32,
}
#[repr(C, align(4))]
pub struct FTestStructSimple2 {
    pub float: f32,
}
#[repr(C, align(4))]
pub struct FTestStructSimple3 {
    pub float: f32,
}
#[repr(C, align(4))]
pub struct FTestStructSimple4 {
    pub float: f32,
}
#[repr(C, align(4))]
pub struct FTestStructSimple5 {
    pub float: f32,
}
#[repr(C, align(4))]
pub struct FTestStructSimple6 {
    pub float: f32,
}
#[repr(C, align(4))]
pub struct FTestStructSimple7 {
    pub float: f32,
}
#[repr(C, align(4))]
pub struct FTestStructHashable1 {
    pub float: f32,
}
#[repr(C, align(1))]
pub struct FTestStructNonTrivialDestructor {}
#[repr(C, align(8))]
pub struct FTestStructWithSubClassOf {
    pub class_property: TSubclassOf<UBagTestObject1>,
    pub soft_class_property: TSoftObjectPtr<UClass>,
}
pub struct UBagTestObject1 {}
pub struct UBagTestObject2 {}
pub struct UBagTestObject1Derived {}
pub struct UTestObjectWithPropertyBag {
    pub bag: FInstancedPropertyBag,
}
pub struct UTestObjectWithInstanceStruct {
    pub value: FInstancedStruct,
}
