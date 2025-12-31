#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FLocalizableMessageParameterInt {
    pub value: i64,
}
#[repr(C, align(8))]
pub struct FLocalizableMessageParameterFloat {
    pub value: f64,
}
#[repr(C, align(8))]
pub struct FLocalizableMessageParameterString {
    pub value: FString,
}
#[repr(C, align(8))]
pub struct FLocalizableMessageParameterMessage {
    pub value: FLocalizableMessage,
}
#[repr(C, align(8))]
pub struct FLocalizableMessage {
    pub key: FString,
    pub default_text: FString,
    pub substitutions: TArray<FLocalizableMessageParameterEntry>,
}
#[repr(C, align(8))]
pub struct FLocalizableMessageParameterEntry {
    pub key: FString,
    pub value: crate::bindings::core_u_object::FInstancedStruct,
}
