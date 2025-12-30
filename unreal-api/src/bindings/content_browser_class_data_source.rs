#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FContentBrowserCompiledClassDataFilter {
    pub valid_classes: TSet<TSubclassOf<UObject>>,
    pub valid_folders: TSet<FName>,
}
pub struct UContentBrowserClassDataSource {}
