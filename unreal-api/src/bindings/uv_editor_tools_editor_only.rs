#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UUVEditorParameterizeMeshToolBuilder {}
pub struct UUVEditorParameterizeMeshTool {
    pub targets: TArray<UPtr<crate::bindings::uv_editor_tools::UUVEditorToolMeshInput>>,
    pub settings: UPtr<
        crate::bindings::modeling_operators_editor_only::UParameterizeMeshToolProperties,
    >,
    pub uv_atlas_properties: UPtr<
        crate::bindings::modeling_operators_editor_only::UParameterizeMeshToolUVAtlasProperties,
    >,
    pub x_atlas_properties: UPtr<
        crate::bindings::modeling_operators_editor_only::UParameterizeMeshToolXAtlasProperties,
    >,
    pub patch_builder_properties: UPtr<
        crate::bindings::modeling_operators_editor_only::UParameterizeMeshToolPatchBuilderProperties,
    >,
    pub polygroup_layer_properties: UPtr<
        crate::bindings::modeling_components::UPolygroupLayersProperties,
    >,
    pub factories: TArray<
        UPtr<
            crate::bindings::modeling_operators_editor_only::UParameterizeMeshOperatorFactory,
        >,
    >,
}
