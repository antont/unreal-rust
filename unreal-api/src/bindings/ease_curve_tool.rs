#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UEaseCurveSerializer {
    __padding_end: [u8; 48],
}
impl UEaseCurveSerializer {}
#[repr(C, align(8))]
pub struct UCubicBezierCurveSerializer {
    __padding_end: [u8; 48],
}
impl UCubicBezierCurveSerializer {}
#[repr(C, align(8))]
pub struct UEaseCurve {
    __padding_end: [u8; 264],
}
impl UEaseCurve {}
#[repr(C, align(8))]
pub struct UEaseCurveLibrary {
    __padding_end: [u8; 120],
}
impl UEaseCurveLibrary {}
#[repr(C, align(8))]
pub struct UEaseCurveLibraryFactory {
    __padding_end: [u8; 136],
}
impl UEaseCurveLibraryFactory {}
#[repr(C, align(8))]
pub struct UEaseCurveToolMenuContext {
    __padding_end: [u8; 88],
}
impl UEaseCurveToolMenuContext {}
#[repr(C, align(8))]
pub struct UEaseCurveToolSettings {
    __padding_end: [u8; 208],
}
impl UEaseCurveToolSettings {}
