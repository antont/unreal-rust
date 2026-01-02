#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UGraphEditorSettings {
    __padding_end: [u8; 808],
}
impl UGraphEditorSettings {}
#[repr(transparent)]
pub struct EGraphPanningMouseButton(pub u8);
impl EGraphPanningMouseButton {
    pub const RIGHT: EGraphPanningMouseButton = EGraphPanningMouseButton(0);
    pub const MIDDLE: EGraphPanningMouseButton = EGraphPanningMouseButton(1);
    pub const BOTH: EGraphPanningMouseButton = EGraphPanningMouseButton(2);
}
#[repr(transparent)]
pub struct EGraphZoomLimitHandling(pub u8);
impl EGraphZoomLimitHandling {
    pub const DEFAULT: EGraphZoomLimitHandling = EGraphZoomLimitHandling(0);
    pub const ALLOW_LIMIT_BREAK: EGraphZoomLimitHandling = EGraphZoomLimitHandling(1);
    pub const DISALLOW_LIMIT_BREAK: EGraphZoomLimitHandling = EGraphZoomLimitHandling(2);
}
