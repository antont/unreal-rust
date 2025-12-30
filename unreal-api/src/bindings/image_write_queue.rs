#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(16))]
pub struct FImageWriteOptions {
    pub format: EDesiredImageFormat,
    pub on_complete: FImageWriteOptions_OnComplete,
    pub compression_quality: i32,
    pub b_overwrite_file: bool,
    pub b_async: bool,
}
pub struct UImageWriteBlueprintLibrary {}
