#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FOverlayItem {
    pub start_time: crate::bindings::core_u_object::FTimespan,
    pub end_time: crate::bindings::core_u_object::FTimespan,
    pub text: FString,
    pub position: crate::bindings::core_u_object::FVector2D,
}
impl FOverlayItem {}
#[repr(C, align(8))]
pub struct UOverlays {
    __padding_end: [u8; 48],
}
impl UOverlays {}
#[repr(C, align(8))]
pub struct UBasicOverlays {
    __padding_end: [u8; 72],
}
impl UBasicOverlays {}
#[repr(C, align(8))]
pub struct ULocalizedOverlays {
    __padding_end: [u8; 144],
}
impl ULocalizedOverlays {}
