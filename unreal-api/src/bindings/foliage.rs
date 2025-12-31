#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FFoliageVertexColorChannelMask {
    pub flags_0: u8,
    pub mask_threshold: f32,
    pub flags_8: u8,
}
#[repr(C, align(8))]
pub struct FFoliageDensityFalloff {
    pub b_use_falloff_curve: bool,
    pub falloff_curve: crate::bindings::engine::FRuntimeFloatCurve,
}
#[repr(C, align(8))]
pub struct FFoliageTypeObject {
    pub foliage_type_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub type_instance: UPtr<UFoliageType>,
    pub b_is_asset: bool,
    pub ty_deprecated: TSubclassOf<UFoliageType_InstancedStaticMesh>,
}
#[repr(C, align(16))]
pub struct FProceduralFoliageInstance {
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub location: crate::bindings::core_u_object::FVector,
    pub age: f32,
    pub normal: crate::bindings::core_u_object::FVector,
    pub scale: f32,
    pub ty: UPtr<UFoliageType>,
}
pub struct UFoliageInstancedStaticMeshComponent {
    pub on_instance_take_point_damage: FFoliageInstancedStaticMeshComponent_OnInstanceTakePointDamage,
    pub on_instance_take_radial_damage: FFoliageInstancedStaticMeshComponent_OnInstanceTakeRadialDamage,
    pub b_enable_discard_on_load: bool,
    pub foliage_hidden_editor_views: u64,
    pub generation_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UFoliageType {
    pub update_guid: crate::bindings::core_u_object::FGuid,
    pub density: f32,
    pub density_adjustment_factor: f32,
    pub radius: f32,
    pub b_single_instance_mode_override_radius: bool,
    pub single_instance_mode_radius: f32,
    pub scaling: EFoliageScaling,
    pub scale_x: crate::bindings::core_u_object::FFloatInterval,
    pub scale_y: crate::bindings::core_u_object::FFloatInterval,
    pub scale_z: crate::bindings::core_u_object::FFloatInterval,
    pub vertex_color_mask_by_channel: FFoliageVertexColorChannelMask,
    pub vertex_color_mask_deprecated: FoliageVertexColorMask,
    pub vertex_color_mask_threshold_deprecated: f32,
    pub flags_168: u8,
    pub z_offset: crate::bindings::core_u_object::FFloatInterval,
    pub flags_180: u8,
    pub align_max_angle: f32,
    pub flags_188: u8,
    pub random_pitch_angle: f32,
    pub ground_slope_angle: crate::bindings::core_u_object::FFloatInterval,
    pub height: crate::bindings::core_u_object::FFloatInterval,
    pub landscape_layers: TArray<FName>,
    pub minimum_layer_weight: f32,
    pub exclusion_landscape_layers: TArray<FName>,
    pub minimum_exclusion_layer_weight: f32,
    pub landscape_layer_deprecated: FName,
    pub flags_272: u8,
    pub collision_scale: crate::bindings::core_u_object::FVector,
    pub average_normal_sample_count: i32,
    pub mesh_bounds: crate::bindings::core_u_object::FBoxSphereBounds,
    pub low_bound_origin_radius: crate::bindings::core_u_object::FVector,
    pub mobility: crate::bindings::engine::EComponentMobility,
    pub cull_distance: crate::bindings::core_u_object::FInt32Interval,
    pub flags_404: u8,
    pub flags_408: u8,
    pub flags_412: u8,
    pub shadow_cache_invalidation_behavior: crate::bindings::engine::EShadowCacheInvalidationBehavior,
    pub overridden_light_map_res: i32,
    pub lightmap_type: crate::bindings::engine::ELightmapType,
    pub flags_428: u8,
    pub flags_432: u8,
    pub world_position_offset_disable_distance: i32,
    pub body_instance: crate::bindings::engine::FBodyInstance,
    pub custom_navigable_geometry: crate::bindings::engine::EHasCustomNavigableGeometry,
    pub lighting_channels: crate::bindings::engine::FLightingChannels,
    pub flags_876: u8,
    pub custom_depth_stencil_write_mask: crate::bindings::engine::ERendererStencilMask,
    pub custom_depth_stencil_value: i32,
    pub translucency_sort_priority: i32,
    pub hidden_editor_views: u64,
    pub flags_904: u8,
    pub collision_radius: f32,
    pub shade_radius: f32,
    pub num_steps: i32,
    pub initial_seed_density: f32,
    pub average_spread_distance: f32,
    pub spread_variance: f32,
    pub seeds_per_step: i32,
    pub distribution_seed: i32,
    pub max_initial_seed_offset: f32,
    pub b_can_grow_in_shade: bool,
    pub b_spawns_in_shade: bool,
    pub max_initial_age: f32,
    pub max_age: f32,
    pub overlap_priority: f32,
    pub procedural_scale: crate::bindings::core_u_object::FFloatInterval,
    pub scale_curve: crate::bindings::engine::FRuntimeFloatCurve,
    pub density_falloff: FFoliageDensityFalloff,
    pub change_count: i32,
    pub flags_1252: u8,
    pub flags_1253: u8,
    pub flags_1254: u8,
    pub runtime_virtual_textures: TArray<
        UPtr<crate::bindings::engine::URuntimeVirtualTexture>,
    >,
    pub virtual_texture_cull_mips: i32,
    pub virtual_texture_render_pass_type: crate::bindings::engine::ERuntimeVirtualTextureMainPassType,
    pub flags_1280: u8,
    pub scale_min_x_deprecated: f32,
    pub scale_min_y_deprecated: f32,
    pub scale_min_z_deprecated: f32,
    pub scale_max_x_deprecated: f32,
    pub scale_max_y_deprecated: f32,
    pub scale_max_z_deprecated: f32,
    pub height_min_deprecated: f32,
    pub height_max_deprecated: f32,
    pub z_offset_min_deprecated: f32,
    pub z_offset_max_deprecated: f32,
    pub start_cull_distance_deprecated: i32,
    pub end_cull_distance_deprecated: i32,
    pub flags_1332: u8,
    pub ground_slope_deprecated: f32,
    pub min_ground_slope_deprecated: f32,
    pub min_scale_deprecated: f32,
    pub max_scale_deprecated: f32,
}
pub struct UFoliageType_Actor {
    pub actor_class: TSubclassOf<crate::bindings::engine::AActor>,
    pub b_should_attach_to_base_component: bool,
    pub b_static_mesh_only: bool,
    pub static_mesh_only_component_class: TSubclassOf<
        UFoliageInstancedStaticMeshComponent,
    >,
}
pub struct UFoliageType_InstancedStaticMesh {
    pub mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub override_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub nanite_override_materials: TArray<
        UPtr<crate::bindings::engine::UMaterialInterface>,
    >,
    pub component_class: TSubclassOf<UFoliageInstancedStaticMeshComponent>,
}
pub struct AInstancedFoliageActor {}
pub struct UInteractiveFoliageComponent {}
pub struct UFoliageStatistics {}
pub struct UGrassInstancedStaticMeshComponent {}
pub struct AInteractiveFoliageActor {
    pub capsule_component: UPtr<crate::bindings::engine::UCapsuleComponent>,
    pub touching_actor_entry_position: crate::bindings::core_u_object::FVector,
    pub foliage_velocity: crate::bindings::core_u_object::FVector,
    pub foliage_force: crate::bindings::core_u_object::FVector,
    pub foliage_position: crate::bindings::core_u_object::FVector,
    pub foliage_damage_impulse_scale: f32,
    pub foliage_touch_impulse_scale: f32,
    pub foliage_stiffness: f32,
    pub foliage_stiffness_quadratic: f32,
    pub foliage_damping: f32,
    pub max_damage_impulse: f32,
    pub max_touch_impulse: f32,
    pub max_force: f32,
    pub mass: f32,
}
pub struct AProceduralFoliageBlockingVolume {
    pub procedural_foliage_volume: UPtr<AProceduralFoliageVolume>,
    pub density_falloff: FFoliageDensityFalloff,
}
pub struct UProceduralFoliageComponent {
    pub foliage_spawner: UPtr<UProceduralFoliageSpawner>,
    pub tile_overlap: f32,
    pub b_allow_landscape: bool,
    pub b_allow_bsp: bool,
    pub b_allow_static_mesh: bool,
    pub b_allow_translucent: bool,
    pub b_allow_foliage: bool,
    pub b_show_debug_tiles: bool,
    pub spawning_volume: UPtr<crate::bindings::engine::AVolume>,
    pub procedural_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UProceduralFoliageSpawner {
    pub random_seed: i32,
    pub tile_size: f32,
    pub num_unique_tiles: i32,
    pub minimum_quad_tree_size: f32,
    pub foliage_types: TArray<FFoliageTypeObject>,
    pub b_use_override_foliage_terrain_materials: bool,
    pub override_foliage_terrain_materials: TArray<
        TSoftObjectPtr<crate::bindings::engine::UMaterialInterface>,
    >,
}
pub struct UProceduralFoliageTile {
    pub foliage_spawner: UPtr<UProceduralFoliageSpawner>,
    pub instances_array: TArray<FProceduralFoliageInstance>,
}
pub struct AProceduralFoliageVolume {
    pub procedural_component: UPtr<UProceduralFoliageComponent>,
}
pub struct FFoliageInstancedStaticMeshComponent_OnInstanceTakePointDamage;
pub struct FFoliageInstancedStaticMeshComponent_OnInstanceTakeRadialDamage;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFoliageScaling(pub u8);
impl EFoliageScaling {
    pub const UNIFORM: EFoliageScaling = EFoliageScaling(0);
    pub const FREE: EFoliageScaling = EFoliageScaling(1);
    pub const LOCK_XY: EFoliageScaling = EFoliageScaling(2);
    pub const LOCK_XZ: EFoliageScaling = EFoliageScaling(3);
    pub const LOCK_YZ: EFoliageScaling = EFoliageScaling(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct FoliageVertexColorMask(pub u8);
impl FoliageVertexColorMask {
    pub const FOLIAGEVERTEXCOLORMASK_DISABLED: FoliageVertexColorMask = FoliageVertexColorMask(
        0,
    );
    pub const FOLIAGEVERTEXCOLORMASK_RED: FoliageVertexColorMask = FoliageVertexColorMask(
        1,
    );
    pub const FOLIAGEVERTEXCOLORMASK_GREEN: FoliageVertexColorMask = FoliageVertexColorMask(
        2,
    );
    pub const FOLIAGEVERTEXCOLORMASK_BLUE: FoliageVertexColorMask = FoliageVertexColorMask(
        3,
    );
    pub const FOLIAGEVERTEXCOLORMASK_ALPHA: FoliageVertexColorMask = FoliageVertexColorMask(
        4,
    );
}
