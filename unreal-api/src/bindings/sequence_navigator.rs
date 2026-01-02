#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UNavigationToolFilterBarContext {
    __padding_end: [u8; 88],
}
impl UNavigationToolFilterBarContext {}
#[repr(C, align(8))]
pub struct UNavigationToolFilterExtension {
    __padding_end: [u8; 48],
}
impl UNavigationToolFilterExtension {}
#[repr(C, align(8))]
pub struct UNavigationToolFilterMenuContext {
    __padding_end: [u8; 88],
}
impl UNavigationToolFilterMenuContext {}
#[repr(C, align(8))]
pub struct UNavigationToolFilterTextExpressionExtension {
    __padding_end: [u8; 48],
}
impl UNavigationToolFilterTextExpressionExtension {}
#[repr(C, align(8))]
pub struct UNavigationToolItemMenuContext {
    __padding_end: [u8; 80],
}
impl UNavigationToolItemMenuContext {}
#[repr(C, align(8))]
pub struct UNavigationToolViewMenuContext {
    __padding_end: [u8; 88],
}
impl UNavigationToolViewMenuContext {}
#[repr(C, align(8))]
pub struct UNavigationToolSettings {
    __padding_end: [u8; 368],
}
impl UNavigationToolSettings {}
#[repr(transparent)]
pub struct ENavigationToolFilterMode(pub u8);
impl ENavigationToolFilterMode {
    pub const NONE: ENavigationToolFilterMode = ENavigationToolFilterMode(0);
    pub const MATCHES_TYPE: ENavigationToolFilterMode = ENavigationToolFilterMode(1);
    pub const CONTAINER_OF_TYPE: ENavigationToolFilterMode = ENavigationToolFilterMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENavigationToolItemViewMode(pub u8);
impl ENavigationToolItemViewMode {
    pub const NONE: ENavigationToolItemViewMode = ENavigationToolItemViewMode(0);
    pub const ITEM_TREE: ENavigationToolItemViewMode = ENavigationToolItemViewMode(1);
    pub const HORIZONTAL_ITEM_LIST: ENavigationToolItemViewMode = ENavigationToolItemViewMode(
        2,
    );
    pub const ALL: ENavigationToolItemViewMode = ENavigationToolItemViewMode(3);
}
