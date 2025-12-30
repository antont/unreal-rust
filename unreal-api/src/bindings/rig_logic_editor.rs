#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UDNAAssetImportUI {
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub file_name: FString,
}
pub struct UDNAAssetImportFactory {
    pub import_ui: UPtr<UDNAAssetImportUI>,
    pub original_import_ui: UPtr<UDNAAssetImportUI>,
}
pub struct UDNAImporterLibrary {}
