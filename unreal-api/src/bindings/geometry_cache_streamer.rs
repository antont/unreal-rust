#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UGeometryCacheStreamerSettings {
    #[doc(hidden)]
    __padding_104: [u8; 104],
    pub look_ahead_buffer: f32,
    pub max_memory_allowed: f32,
}
impl UGeometryCacheStreamerSettings {}
