#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FActorLayer {
    pub name: FName,
}
impl FActorLayer {}
#[repr(C, align(8))]
pub struct ULayersBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl ULayersBlueprintLibrary {}
