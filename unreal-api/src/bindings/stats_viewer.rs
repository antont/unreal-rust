#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UCookerStats {
    pub assets: TArray<TWeakObjectPtr<crate::bindings::core_u_object::UObject>>,
    pub size_before: f32,
    pub size_after: f32,
    pub path: FString,
}
pub struct ULightingBuildInfo {
    pub object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub lighting_time: f32,
    pub unmapped_texels_percentage: f32,
    pub unmapped_texels_memory: f32,
    pub total_texel_memory: f32,
    pub level_name: FString,
}
pub struct UPrimitiveStats {
    pub object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub actors: TArray<TWeakObjectPtr<crate::bindings::engine::AActor>>,
    pub ty: FString,
    pub count: i32,
    pub sections: i32,
    pub hw_instances: i32,
    pub inst_sections: i32,
    pub triangles: i32,
    pub inst_triangles: i32,
    pub resource_size: f32,
    pub vertex_color_mem: f32,
    pub inst_vertex_color_mem: f32,
    pub lights_lm: i32,
    pub lights_other: f32,
    pub lights_total: f32,
    pub obj_light_cost: f32,
    pub light_map_data: f32,
    pub lmsm_resolution: f32,
    pub radius_min: f64,
    pub radius_max: f64,
    pub radius_avg: f64,
}
pub struct UShaderCookerStats {
    pub name: FString,
    pub platform: FString,
    pub category: FString,
    pub compiled: i32,
    pub cooked: i32,
    pub permutations: i32,
    pub compile_time: f32,
    pub path: FString,
}
pub struct UStaticMeshLightingInfo {
    pub static_mesh_actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub static_mesh: TWeakObjectPtr<crate::bindings::engine::UStaticMesh>,
    pub level_name: FString,
    pub texture_mapping: FString,
    pub b_texture_mapping: bool,
    pub b_has_lightmap_tex_coords: bool,
    pub static_lighting_resolution: i32,
    pub texture_light_map_memory_usage: f32,
    pub vertex_light_map_memory_usage: f32,
    pub light_map_light_count: i32,
    pub texture_shadow_map_memory_usage: f32,
    pub vertex_shadow_map_memory_usage: f32,
    pub shadow_map_light_count: i32,
    pub lightmap_texture_names: TArray<FString>,
}
pub struct UTextureStats {
    pub texture: TWeakObjectPtr<crate::bindings::engine::UTexture>,
    pub actors: TArray<TWeakObjectPtr<crate::bindings::engine::AActor>>,
    pub ty: FString,
    pub virtual_: FString,
    pub max_dim: crate::bindings::core_u_object::FVector2D,
    pub current_dim: crate::bindings::core_u_object::FVector2D,
    pub format: crate::bindings::core_u_object::EPixelFormat,
    pub group: crate::bindings::engine::TextureGroup,
    pub combined_lod_bias: i32,
    pub texture_lod_bias: i32,
    pub current_kb: f32,
    pub fully_loaded_kb: f32,
    pub num_uses: i32,
    pub last_time_rendered: f32,
    pub path: FString,
}
