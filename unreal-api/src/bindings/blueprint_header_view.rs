#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UBlueprintHeaderViewSettings {
    __padding_end: [u8; 224],
}
impl UBlueprintHeaderViewSettings {}
#[repr(transparent)]
pub struct EHeaderViewSortMethod(pub u8);
impl EHeaderViewSortMethod {
    pub const NONE: EHeaderViewSortMethod = EHeaderViewSortMethod(0);
    pub const SORT_BY_ACCESS_SPECIFIER: EHeaderViewSortMethod = EHeaderViewSortMethod(1);
    pub const SORT_FOR_OPTIMAL_PADDING: EHeaderViewSortMethod = EHeaderViewSortMethod(2);
}
