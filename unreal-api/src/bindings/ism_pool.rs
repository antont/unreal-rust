#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct AISMPoolActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub ism_pool_comp: UPtr<UISMPoolComponent>,
    pub ism_pool_debug_draw_comp: UPtr<UISMPoolDebugDrawComponent>,
}
impl AISMPoolActor {}
#[repr(C, align(16))]
pub struct UISMPoolComponent {
    __padding_end: [u8; 960],
}
impl UISMPoolComponent {}
#[repr(C, align(8))]
pub struct UISMPoolSubSystem {
    __padding_end: [u8; 144],
}
impl UISMPoolSubSystem {}
#[repr(C, align(16))]
pub struct UISMPoolDebugDrawComponent {
    __padding_end: [u8; 1744],
}
impl UISMPoolDebugDrawComponent {}
