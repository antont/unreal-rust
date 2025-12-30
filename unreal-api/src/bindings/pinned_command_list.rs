#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FPinnedCommandListCommand {
    pub name: FName,
    pub binding: FName,
    pub ty: EPinnedCommandListType,
}
#[repr(C, align(8))]
pub struct FPinnedCommandListContext {
    pub name: FName,
    pub commands: TArray<FPinnedCommandListCommand>,
}
pub struct UPinnedCommandListSettings {
    pub contexts: TArray<FPinnedCommandListContext>,
}
