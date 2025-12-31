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
pub struct UDNAMeshVertexColorDataAsset {
    pub mesh_color_entries: TArray<FMeshVertexColorData>,
}
pub struct UMetaHumanInterchangeDnaTranslator {}
