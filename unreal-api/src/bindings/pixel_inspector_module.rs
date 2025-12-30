#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UPixelInspectorView {
    pub final_color: FLinearColor,
    pub pre_exposure: f32,
    pub scene_color_before_post_processing: FLinearColor,
    pub scene_color_before_tonemap: FLinearColor,
    pub luminance_before_tonemap: f32,
    pub normal: FVector,
    pub per_object_g_buffer_data: f32,
    pub metallic: f32,
    pub specular: f32,
    pub roughness: f32,
    pub material_shading_model: EMaterialShadingModel,
    pub selective_output_mask: i32,
    pub base_color: FLinearColor,
    pub indirect_irradiance: f32,
    pub ambient_occlusion: f32,
    pub sub_surface_color: FLinearColor,
    pub subsurface_profile: FVector,
    pub opacity: f32,
    pub clear_coat: f32,
    pub clear_coat_roughness: f32,
    pub world_normal: FVector,
    pub back_lit: f32,
    pub cloth: f32,
    pub eye_tangent: FVector,
    pub iris_mask: f32,
    pub iris_distance: f32,
}
