#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct AManipulator {
    __padding_end: [u8; 1160],
}
impl AManipulator {}
#[repr(C, align(16))]
pub struct USplineComponentVisualizerSelectionState {
    __padding_end: [u8; 256],
}
impl USplineComponentVisualizerSelectionState {}
#[repr(C, align(8))]
pub struct USplineGeneratorBase {
    __padding_end: [u8; 192],
}
impl USplineGeneratorBase {}
#[repr(C, align(8))]
pub struct UCircleSplineGenerator {
    __padding_end: [u8; 208],
}
impl UCircleSplineGenerator {}
#[repr(C, align(8))]
pub struct UArcSplineGenerator {
    __padding_end: [u8; 208],
}
impl UArcSplineGenerator {}
#[repr(C, align(8))]
pub struct USquareSplineGenerator {
    __padding_end: [u8; 200],
}
impl USquareSplineGenerator {}
#[repr(C, align(8))]
pub struct UEllipseSplineGenerator {
    __padding_end: [u8; 208],
}
impl UEllipseSplineGenerator {}
#[repr(C, align(8))]
pub struct URectangleSplineGenerator {
    __padding_end: [u8; 208],
}
impl URectangleSplineGenerator {}
#[repr(C, align(8))]
pub struct ULineSplineGenerator {
    __padding_end: [u8; 216],
}
impl ULineSplineGenerator {}
#[repr(transparent)]
pub struct ESelectedTangentHandle(pub i32);
impl ESelectedTangentHandle {
    pub const NONE: ESelectedTangentHandle = ESelectedTangentHandle(0);
    pub const LEAVE: ESelectedTangentHandle = ESelectedTangentHandle(1);
    pub const ARRIVE: ESelectedTangentHandle = ESelectedTangentHandle(2);
}
#[repr(transparent)]
pub struct EShapeAddMode(pub u8);
impl EShapeAddMode {
    pub const APPEND_AFTER: EShapeAddMode = EShapeAddMode(1);
    pub const APPEND_BEFORE: EShapeAddMode = EShapeAddMode(2);
    pub const INSERT_AFTER: EShapeAddMode = EShapeAddMode(4);
    pub const INSERT_BEFORE: EShapeAddMode = EShapeAddMode(8);
}
