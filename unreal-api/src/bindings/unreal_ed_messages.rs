#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAssetEditorRequestOpenAsset {
    pub asset_name: FString,
}
#[repr(C, align(8))]
pub struct FFileServerReady {
    pub address_list: TArray<FString>,
    pub instance_id: crate::bindings::core_u_object::FGuid,
}
