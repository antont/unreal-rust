#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAnimSequencerController {
    __padding_end: [u8; 120],
}
impl UAnimSequencerController {}
#[repr(C, align(8))]
pub struct UAnimationSequencerDataModel {
    __padding_end: [u8; 488],
}
impl UAnimationSequencerDataModel {}
#[repr(C, align(8))]
pub struct FAnimationSequencerDataModel_ModifiedEventDynamic {
    _opague: [u8; 24],
}
