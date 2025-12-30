#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FClothConstraintSetup_Legacy {
    pub stiffness: f32,
    pub stiffness_multiplier: f32,
    pub stretch_limit: f32,
    pub compression_limit: f32,
}
#[repr(C, align(8))]
pub struct FClothConfig_Legacy {
    pub wind_method: EClothingWindMethod_Legacy,
    pub vertical_constraint_config: FClothConstraintSetup_Legacy,
    pub horizontal_constraint_config: FClothConstraintSetup_Legacy,
    pub bend_constraint_config: FClothConstraintSetup_Legacy,
    pub shear_constraint_config: FClothConstraintSetup_Legacy,
    pub self_collision_radius: f32,
    pub self_collision_stiffness: f32,
    pub self_collision_cull_scale: f32,
    pub damping: FVector,
    pub friction: f32,
    pub wind_drag_coefficient: f32,
    pub wind_lift_coefficient: f32,
    pub linear_drag: FVector,
    pub angular_drag: FVector,
    pub linear_inertia_scale: FVector,
    pub angular_inertia_scale: FVector,
    pub centrifugal_inertia_scale: FVector,
    pub solver_frequency: f32,
    pub stiffness_frequency: f32,
    pub gravity_scale: f32,
    pub gravity_override: FVector,
    pub b_use_gravity_override: bool,
    pub tether_stiffness: f32,
    pub tether_limit: f32,
    pub collision_thickness: f32,
    pub anim_drive_spring_stiffness: f32,
    pub anim_drive_damper_stiffness: f32,
}
#[repr(C, align(8))]
pub struct FPointWeightMap {
    pub values: TArray<f32>,
    pub name: FName,
    pub current_target: u8,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FClothLODDataCommon {
    pub physical_mesh_data: FClothPhysicalMeshData,
    pub b_use_multiple_influences: bool,
    pub skinning_kernel_radius: f32,
    pub b_smooth_transition: bool,
    pub collision_data_deprecated: FClothCollisionData,
    pub parameter_masks_deprecated: TArray<FClothParameterMask_Legacy>,
    pub point_weight_maps: TArray<FPointWeightMap>,
}
#[repr(C, align(8))]
pub struct FClothParameterMask_Legacy {
    pub mask_name: FName,
    pub current_target: EWeightMapTargetCommon,
    pub max_value_deprecated: f32,
    pub min_value_deprecated: f32,
    pub values: TArray<f32>,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FClothPhysicalMeshData {
    pub vertices: TArray<FVector3f>,
    pub normals: TArray<FVector3f>,
    pub vertex_colors: TArray<FColor>,
    pub indices: TArray<u32>,
    pub weight_maps: TMap<u32, FPointWeightMap>,
    pub inverse_masses: TArray<f32>,
    pub bone_data: TArray<FClothVertBoneData>,
    pub self_collision_vertex_set: TSet<i32>,
    pub euclidean_tethers: FClothTetherData,
    pub geodesic_tethers: FClothTetherData,
    pub max_bone_weights: i32,
    pub num_fixed_verts: i32,
    pub self_collision_indices: TArray<u32>,
    pub max_distances_deprecated: TArray<f32>,
    pub backstop_distances_deprecated: TArray<f32>,
    pub backstop_radiuses_deprecated: TArray<f32>,
    pub anim_drive_multipliers_deprecated: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FClothTetherData {}
pub struct UClothConfigCommon {}
pub struct UClothSharedConfigCommon {}
pub struct UClothingAssetCustomData {}
pub struct UClothingAssetCommon {
    pub physics_asset: UPtr<UPhysicsAsset>,
    pub cloth_configs: TMap<FName, UPtr<UClothConfigBase>>,
    pub cloth_shared_sim_config_deprecated: UPtr<UClothConfigBase>,
    pub cloth_sim_config_deprecated: UPtr<UClothConfigBase>,
    pub chaos_cloth_sim_config_deprecated: UPtr<UClothConfigBase>,
    pub cloth_lod_data_deprecated: TArray<UPtr<UClothLODDataCommon_Legacy>>,
    pub lod_data: TArray<FClothLODDataCommon>,
    pub lod_map: TArray<i32>,
    pub used_bone_names: TArray<FName>,
    pub used_bone_indices: TArray<i32>,
    pub reference_bone_index: i32,
    pub cloth_config_deprecated: FClothConfig_Legacy,
}
pub struct UClothLODDataCommon_Legacy {
    pub physical_mesh_data_deprecated: UPtr<UClothPhysicalMeshDataBase_Legacy>,
    pub cloth_physical_mesh_data: FClothPhysicalMeshData,
    pub collision_data: FClothCollisionData,
    pub parameter_masks: TArray<FPointWeightMap>,
}
