#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct UAudioCurveSourceComponent {
    #[doc(hidden)]
    __padding_3560: [u8; 3560],
    pub curve_source_binding_name: FName,
    pub curve_sync_offset: f32,
    __padding_end: [u8; 40],
}
impl UAudioCurveSourceComponent {}
