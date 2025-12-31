#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FLevelFolderProps {}
#[repr(C, align(8))]
pub struct FTileLODEntryDetails {
    pub lod_index: i32,
    pub relative_distance: i32,
    pub simplification_details: crate::bindings::engine::FLevelSimplificationDetails,
}
#[repr(C, align(8))]
pub struct FWorldBrowserColumnConfig {
    pub column_visibilities: TMap<FName, bool>,
}
pub struct UEditorLevelFolders {}
pub struct UWorldTileDetails {
    pub b_tile_editable: bool,
    pub package_name: FName,
    pub parent_package_name: FName,
    pub position: crate::bindings::core_u_object::FIntVector,
    pub absolute_position: crate::bindings::core_u_object::FIntVector,
    pub z_order: i32,
    pub b_hide_in_tile_view: bool,
    pub num_lod: i32,
    pub lod1: FTileLODEntryDetails,
    pub lod2: FTileLODEntryDetails,
    pub lod3: FTileLODEntryDetails,
    pub lod4: FTileLODEntryDetails,
}
pub struct UWorldBrowserConfig {
    pub column_config: FWorldBrowserColumnConfig,
}
