#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct IBaseSequencerAnimTool {}
#[repr(C, align(8))]
pub struct UBaseSequencerAnimTool {
    __padding_end: [u8; 48],
}
impl UBaseSequencerAnimTool {}
#[repr(C, align(8))]
pub struct UMotionTrailToolBuilder {
    __padding_end: [u8; 48],
}
impl UMotionTrailToolBuilder {}
#[repr(C, align(16))]
pub struct UMotionTrailTool {
    __padding_end: [u8; 720],
}
impl UMotionTrailTool {}
#[repr(C, align(8))]
pub struct USequencerPivotToolBuilder {
    __padding_end: [u8; 48],
}
impl USequencerPivotToolBuilder {}
#[repr(C, align(16))]
pub struct USequencerPivotTool {
    __padding_end: [u8; 672],
}
impl USequencerPivotTool {}
#[repr(C, align(8))]
pub struct USequencerToolsEditMode {
    __padding_end: [u8; 352],
}
impl USequencerToolsEditMode {}
