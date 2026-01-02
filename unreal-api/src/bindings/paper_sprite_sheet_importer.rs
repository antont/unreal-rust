#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UPaperSpriteSheet {
    __padding_end: [u8; 136],
}
impl UPaperSpriteSheet {}
#[repr(C, align(8))]
pub struct UPaperSpriteSheetImportFactory {
    __padding_end: [u8; 336],
}
impl UPaperSpriteSheetImportFactory {}
#[repr(C, align(8))]
pub struct UPaperSpriteSheetReimportFactory {
    __padding_end: [u8; 368],
}
impl UPaperSpriteSheetReimportFactory {}
