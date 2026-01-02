#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UPropertyViewBase {
    __padding_end: [u8; 832],
}
impl UPropertyViewBase {}
#[repr(C, align(8))]
pub struct UDetailsView {
    __padding_end: [u8; 928],
}
impl UDetailsView {}
#[repr(C, align(8))]
pub struct USinglePropertyView {
    __padding_end: [u8; 880],
}
impl USinglePropertyView {}
#[repr(transparent)]
pub struct FPropertyViewBase_OnPropertyChanged {
    _opague: u8,
}
