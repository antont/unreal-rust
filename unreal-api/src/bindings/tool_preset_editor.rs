#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UToolPresetUserSettings {
    pub enabled_preset_collections: TSet<
        crate::bindings::core_u_object::FSoftObjectPath,
    >,
    pub b_default_collection_enabled: bool,
}
pub struct UToolPresetProjectSettings {
    pub loaded_preset_collections: TSet<crate::bindings::core_u_object::FSoftObjectPath>,
}
