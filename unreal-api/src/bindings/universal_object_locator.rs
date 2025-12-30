#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FDirectPathObjectLocator {
    pub path: FSoftObjectPath,
}
#[repr(C, align(8))]
pub struct FSubObjectLocator {
    pub path_within_context: FString,
}
#[repr(C, align(8))]
pub struct FUniversalObjectLocator {
    pub fragments: TArray<FUniversalObjectLocatorFragment>,
}
#[repr(C, align(8))]
pub struct FUniversalObjectLocatorFragment {}
#[repr(C, align(1))]
pub struct FUniversalObjectLocatorEmptyPayload {}
