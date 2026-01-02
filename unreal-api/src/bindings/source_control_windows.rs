#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct USourceControlMenuContext {
    __padding_end: [u8; 80],
}
impl USourceControlMenuContext {}
#[repr(C, align(8))]
pub struct USourceControlSettings {
    __padding_end: [u8; 56],
}
impl USourceControlSettings {}
#[repr(C, align(8))]
pub struct USourceControlHistoryWidgetContext {
    __padding_end: [u8; 80],
}
impl USourceControlHistoryWidgetContext {}
#[repr(C, align(8))]
pub struct USourceControlSubmitWidgetContext {
    __padding_end: [u8; 80],
}
impl USourceControlSubmitWidgetContext {}
