#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub filter: TextureFilter,
    pub b_override_tiling: bool,
    pub tiling: TextureAddress,
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
    pub default_material_bake_filter: TextureFilter,
    pub default_material_bake_tiling: TextureAddress,
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
    pub default_material_bake_filter: TextureFilter,
    pub default_material_bake_tiling: TextureAddress,
    pub default_input_bake_settings: TMap<
        EGLTFMaterialPropertyGroup,
        FGLTFOverrideMaterialBakeSettings,
    >,
}
pub struct UGLTFMaterialExportOptions {
    pub proxy: UPtr<UMaterialInterface>,
    pub default: FGLTFOverrideMaterialBakeSettings,
    pub inputs: TMap<EGLTFMaterialPropertyGroup, FGLTFOverrideMaterialBakeSettings>,
}
