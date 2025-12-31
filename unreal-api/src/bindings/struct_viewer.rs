#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UStructViewerProjectSettings {
    pub internal_only_paths: TArray<crate::bindings::core_u_object::FDirectoryPath>,
    pub internal_only_structs: TArray<
        TSoftObjectPtr<crate::bindings::core_u_object::UScriptStruct>,
    >,
}
