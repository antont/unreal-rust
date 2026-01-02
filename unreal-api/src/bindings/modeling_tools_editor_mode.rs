#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct UModelingSelectionInteraction {
    __padding_end: [u8; 576],
}
impl UModelingSelectionInteraction {}
#[repr(C, align(8))]
pub struct UPathSelectionInteraction {
    __padding_end: [u8; 64],
}
impl UPathSelectionInteraction {}
#[repr(C, align(8))]
pub struct UModelingModeEditableToolPaletteConfig {
    __padding_end: [u8; 136],
}
impl UModelingModeEditableToolPaletteConfig {}
#[repr(C, align(16))]
pub struct UModelingToolsEditorMode {
    __padding_end: [u8; 608],
}
impl UModelingToolsEditorMode {}
#[repr(C, align(8))]
pub struct UModelingToolsEditorModeSettings {
    __padding_end: [u8; 224],
}
impl UModelingToolsEditorModeSettings {}
#[repr(C, align(8))]
pub struct UModelingToolsModeCustomizationSettings {
    __padding_end: [u8; 288],
}
impl UModelingToolsModeCustomizationSettings {}
#[repr(C, align(8))]
pub struct UModelingToolsHostCustomizationAPI {
    __padding_end: [u8; 72],
}
impl UModelingToolsHostCustomizationAPI {}
#[repr(transparent)]
pub struct EModelingModeAssetGenerationLocation(pub i32);
impl EModelingModeAssetGenerationLocation {
    pub const AUTO_GENERATED_WORLD_RELATIVE_ASSET_PATH: EModelingModeAssetGenerationLocation = EModelingModeAssetGenerationLocation(
        0,
    );
    pub const AUTO_GENERATED_GLOBAL_ASSET_PATH: EModelingModeAssetGenerationLocation = EModelingModeAssetGenerationLocation(
        1,
    );
    pub const CURRENT_ASSET_BROWSER_PATH_IF_AVAILABLE: EModelingModeAssetGenerationLocation = EModelingModeAssetGenerationLocation(
        2,
    );
}
#[repr(transparent)]
pub struct EModelingModeAssetGenerationBehavior(pub i32);
impl EModelingModeAssetGenerationBehavior {
    pub const AUTO_GENERATE_AND_AUTOSAVE: EModelingModeAssetGenerationBehavior = EModelingModeAssetGenerationBehavior(
        0,
    );
    pub const AUTO_GENERATE_BUT_DO_NOT_AUTOSAVE: EModelingModeAssetGenerationBehavior = EModelingModeAssetGenerationBehavior(
        1,
    );
    pub const INTERACTIVE_PROMPT_TO_SAVE: EModelingModeAssetGenerationBehavior = EModelingModeAssetGenerationBehavior(
        2,
    );
}
#[repr(transparent)]
pub struct EModelingModeDefaultMeshObjectType(pub i32);
impl EModelingModeDefaultMeshObjectType {
    pub const STATIC_MESH_ASSET: EModelingModeDefaultMeshObjectType = EModelingModeDefaultMeshObjectType(
        0,
    );
    pub const VOLUME_ACTOR: EModelingModeDefaultMeshObjectType = EModelingModeDefaultMeshObjectType(
        1,
    );
    pub const DYNAMIC_MESH_ACTOR: EModelingModeDefaultMeshObjectType = EModelingModeDefaultMeshObjectType(
        2,
    );
}
