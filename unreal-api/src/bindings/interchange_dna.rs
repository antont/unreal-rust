#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FMeshVertexColorData {
    pub mesh_name: FString,
    pub colors: TArray<crate::bindings::core_u_object::FLinearColor>,
}
impl FMeshVertexColorData {}
#[repr(C, align(8))]
pub struct UDNAMeshVertexColorDataAsset {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub mesh_color_entries: TArray<FMeshVertexColorData>,
}
impl UDNAMeshVertexColorDataAsset {}
#[repr(C, align(8))]
pub struct UMetaHumanInterchangeDnaTranslator {
    __padding_end: [u8; 184],
}
impl UMetaHumanInterchangeDnaTranslator {}
