#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FGLTFExportMessages {
    pub suggestions: TArray<FString>,
    pub warnings: TArray<FString>,
    pub errors: TArray<FString>,
}
#[repr(C, align(4))]
pub struct FGLTFMaterialBakeSize {
    pub x: i32,
    pub y: i32,
    pub b_auto_detect: bool,
}
#[repr(C, align(4))]
pub struct FGLTFOverrideMaterialBakeSettings {
    pub b_override_size: bool,
    pub size: FGLTFMaterialBakeSize,
    pub b_override_filter: bool,
    pub filter: crate::bindings::engine::TextureFilter,
    pub b_override_tiling: bool,
    pub tiling: crate::bindings::engine::TextureAddress,
}
pub struct UGLTFExporter {}
pub struct UGLTFAnimSequenceExporter {}
pub struct UGLTFLevelExporter {}
pub struct UGLTFLevelSequenceExporter {}
pub struct UGLTFLevelVariantSetsExporter {}
pub struct UGLTFMaterialExporter {}
pub struct UGLTFSkeletalMeshExporter {}
pub struct UGLTFStaticMeshExporter {}
pub struct UGLTFExportOptions {
    pub export_uniform_scale: f32,
    pub b_export_preview_mesh: bool,
    pub b_skip_near_default_values: bool,
    pub b_include_copyright_notice: bool,
    pub b_export_proxy_materials: bool,
    pub b_use_importer_material_mapping: bool,
    pub b_export_unlit_materials: bool,
    pub b_export_clear_coat_materials: bool,
    pub b_export_cloth_materials: bool,
    pub b_export_thin_translucent_materials: bool,
    pub b_export_specular_glossiness_materials: bool,
    pub b_export_emissive_strength: bool,
    pub bake_material_inputs: EGLTFMaterialBakeMode,
    pub default_material_bake_size: FGLTFMaterialBakeSize,
    pub default_material_bake_filter: crate::bindings::engine::TextureFilter,
    pub default_material_bake_tiling: crate::bindings::engine::TextureAddress,
    pub default_input_bake_settings: TMap<
        EGLTFMaterialPropertyGroup,
        FGLTFOverrideMaterialBakeSettings,
    >,
    pub default_level_of_detail: i32,
    pub b_export_source_model: bool,
    pub b_export_vertex_colors: bool,
    pub b_export_vertex_skin_weights: bool,
    pub b_make_skinned_meshes_root: bool,
    pub b_use_mesh_quantization: bool,
    pub b_export_morph_targets: bool,
    pub b_export_level_sequences: bool,
    pub b_export_animation_sequences: bool,
    pub texture_image_format: EGLTFTextureImageFormat,
    pub texture_image_quality: i32,
    pub b_export_texture_transforms: bool,
    pub b_export_lightmaps: bool,
    pub b_adjust_normalmaps: bool,
    pub b_export_hidden_in_game: bool,
    pub b_export_lights: bool,
    pub b_export_cameras: bool,
    pub export_material_variants: EGLTFMaterialVariantMode,
}
pub struct UGLTFProxyOptions {
    pub b_bake_material_inputs: bool,
    pub b_use_thin_translucent_shading_model: bool,
    pub default_material_bake_size: FGLTFMaterialBakeSize,
    pub default_material_bake_filter: crate::bindings::engine::TextureFilter,
    pub default_material_bake_tiling: crate::bindings::engine::TextureAddress,
    pub default_input_bake_settings: TMap<
        EGLTFMaterialPropertyGroup,
        FGLTFOverrideMaterialBakeSettings,
    >,
}
pub struct UGLTFMaterialExportOptions {
    pub proxy: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub default: FGLTFOverrideMaterialBakeSettings,
    pub inputs: TMap<EGLTFMaterialPropertyGroup, FGLTFOverrideMaterialBakeSettings>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGLTFMaterialBakeMode(pub u8);
impl EGLTFMaterialBakeMode {
    pub const DISABLED: EGLTFMaterialBakeMode = EGLTFMaterialBakeMode(0);
    pub const SIMPLE: EGLTFMaterialBakeMode = EGLTFMaterialBakeMode(1);
    pub const USE_MESH_DATA: EGLTFMaterialBakeMode = EGLTFMaterialBakeMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGLTFMaterialPropertyGroup(pub u8);
impl EGLTFMaterialPropertyGroup {
    pub const NONE: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(0);
    pub const BASE_COLOR_OPACITY: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(
        1,
    );
    pub const METALLIC_ROUGHNESS: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(
        2,
    );
    pub const EMISSIVE_COLOR: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(3);
    pub const NORMAL: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(4);
    pub const AMBIENT_OCCLUSION: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(
        5,
    );
    pub const CLEAR_COAT_ROUGHNESS: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(
        6,
    );
    pub const CLEAR_COAT_BOTTOM_NORMAL: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(
        7,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGLTFTextureImageFormat(pub u8);
impl EGLTFTextureImageFormat {
    pub const NONE: EGLTFTextureImageFormat = EGLTFTextureImageFormat(0);
    pub const PNG: EGLTFTextureImageFormat = EGLTFTextureImageFormat(1);
    pub const JPEG: EGLTFTextureImageFormat = EGLTFTextureImageFormat(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGLTFMaterialVariantMode(pub u8);
impl EGLTFMaterialVariantMode {
    pub const NONE: EGLTFMaterialVariantMode = EGLTFMaterialVariantMode(0);
    pub const SIMPLE: EGLTFMaterialVariantMode = EGLTFMaterialVariantMode(1);
    pub const USE_MESH_DATA: EGLTFMaterialVariantMode = EGLTFMaterialVariantMode(2);
}
