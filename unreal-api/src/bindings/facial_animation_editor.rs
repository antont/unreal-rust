#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UFacialAnimationBulkImporterSettings {
    pub source_import_path: FDirectoryPath,
    pub target_import_path: FDirectoryPath,
    pub curve_node_name: FString,
}
