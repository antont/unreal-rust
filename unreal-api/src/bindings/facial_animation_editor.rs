#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UFacialAnimationBulkImporterSettings {
    pub source_import_path: crate::bindings::core_u_object::FDirectoryPath,
    pub target_import_path: crate::bindings::core_u_object::FDirectoryPath,
    pub curve_node_name: FString,
}
