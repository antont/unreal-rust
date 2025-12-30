#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FNamedInterface {
    pub interface_name: FName,
    pub interface_object: UPtr<UObject>,
}
#[repr(C, align(8))]
pub struct FNamedInterfaceDef {
    pub interface_name: FName,
    pub interface_class_name: FString,
}
pub struct UNamedInterfaces {
    pub named_interfaces: TArray<FNamedInterface>,
    pub named_interface_defs: TArray<FNamedInterfaceDef>,
}
pub struct UTurnBasedMatchInterface {}
pub struct ITurnBasedMatchInterface {}
