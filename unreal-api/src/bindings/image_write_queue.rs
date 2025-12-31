#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FImageWriteOptions {
    pub format: EDesiredImageFormat,
    pub on_complete: FImageWriteOptions_OnComplete,
    pub compression_quality: i32,
    pub b_overwrite_file: bool,
    pub b_async: bool,
}
pub struct UImageWriteBlueprintLibrary {}
pub struct FImageWriteOptions_OnComplete;
pub struct FExportToDisk_OnComplete;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDesiredImageFormat(pub u8);
impl EDesiredImageFormat {
    pub const PNG: EDesiredImageFormat = EDesiredImageFormat(0);
    pub const JPG: EDesiredImageFormat = EDesiredImageFormat(1);
    pub const BMP: EDesiredImageFormat = EDesiredImageFormat(2);
    pub const EXR: EDesiredImageFormat = EDesiredImageFormat(3);
}
