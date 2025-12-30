#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UStructViewerProjectSettings {
    pub internal_only_paths: TArray<FDirectoryPath>,
    pub internal_only_structs: TArray<TSoftObjectPtr<UScriptStruct>>,
}
