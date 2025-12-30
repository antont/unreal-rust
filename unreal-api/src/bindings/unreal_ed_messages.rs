#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAssetEditorRequestOpenAsset {
    pub asset_name: FString,
}
#[repr(C, align(8))]
pub struct FFileServerReady {
    pub address_list: TArray<FString>,
    pub instance_id: FGuid,
}
