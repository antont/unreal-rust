#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UUVEditorParameterizeMeshToolBuilder {}
pub struct UUVEditorParameterizeMeshTool {
    pub targets: TArray<UPtr<UUVEditorToolMeshInput>>,
    pub settings: UPtr<UParameterizeMeshToolProperties>,
    pub uv_atlas_properties: UPtr<UParameterizeMeshToolUVAtlasProperties>,
    pub x_atlas_properties: UPtr<UParameterizeMeshToolXAtlasProperties>,
    pub patch_builder_properties: UPtr<UParameterizeMeshToolPatchBuilderProperties>,
    pub polygroup_layer_properties: UPtr<UPolygroupLayersProperties>,
    pub factories: TArray<UPtr<UParameterizeMeshOperatorFactory>>,
}
