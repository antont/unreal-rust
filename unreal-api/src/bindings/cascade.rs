#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UCascadeConfiguration {
    __padding_end: [u8; 256],
}
impl UCascadeConfiguration {}
#[repr(C, align(16))]
pub struct UCascadeParticleSystemComponent {
    __padding_end: [u8; 2352],
}
impl UCascadeParticleSystemComponent {}
