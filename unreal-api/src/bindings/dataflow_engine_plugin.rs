#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FCollectionAttributeKey {
    __padding_end: [u8; 32],
}
impl FCollectionAttributeKey {}
#[repr(C, align(8))]
pub struct ADataflowActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub dataflow_component: UPtr<UDataflowComponent>,
}
impl ADataflowActor {}
#[repr(C, align(16))]
pub struct UDataflowComponent {
    __padding_end: [u8; 1840],
}
impl UDataflowComponent {}
