#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FBifrostNodeInfo {
    pub name: FString,
    pub value: FString,
}
pub struct UNodePort {}
pub struct UBrowserBinding {}
