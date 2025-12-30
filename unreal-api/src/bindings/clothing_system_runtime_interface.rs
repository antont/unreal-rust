#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FClothCollisionPrim_Sphere {
    pub bone_index: i32,
    pub radius: f32,
    pub local_position: FVector,
}
#[repr(C, align(8))]
pub struct FClothCollisionPrim_SphereConnection {
    pub sphere_indices: i32,
    pub one_sided_plane_normal: FVector,
}
#[repr(C, align(16))]
pub struct FClothCollisionPrim_ConvexFace {
    pub plane: FPlane,
    pub indices: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FClothCollisionPrim_Convex {
    pub planes_deprecated: TArray<FPlane>,
    pub faces: TArray<FClothCollisionPrim_ConvexFace>,
    pub surface_points: TArray<FVector>,
    pub bone_index: i32,
}
#[repr(C, align(16))]
pub struct FClothCollisionPrim_Box {
    pub local_position: FVector,
    pub local_rotation: FQuat,
    pub half_extents: FVector,
    pub bone_index: i32,
}
#[repr(C, align(4))]
pub struct FClothVertBoneData {
    pub num_influences: i32,
    pub bone_indices: u16,
    pub bone_weights: f32,
}
#[repr(C, align(8))]
pub struct FClothCollisionData {
    pub spheres: TArray<FClothCollisionPrim_Sphere>,
    pub sphere_connections: TArray<FClothCollisionPrim_SphereConnection>,
    pub convexes: TArray<FClothCollisionPrim_Convex>,
    pub boxes: TArray<FClothCollisionPrim_Box>,
}
#[repr(C, align(8))]
pub struct FClothingSimulationInstance {
    pub clothing_simulation_interactor: UPtr<UClothingSimulationInteractor>,
    pub clothing_simulation_factory: UPtr<UClothingSimulationFactory>,
}
pub struct UClothConfigBase {}
pub struct UDEPRECATED_ClothSharedSimConfigBase {}
pub struct UClothingAssetBase {
    pub imported_file_path: FString,
    pub asset_guid: FGuid,
}
pub struct UClothingSimulationFactory {}
pub struct UClothingInteractor {}
pub struct UClothingSimulationInteractor {
    pub clothing_interactors: TMap<FName, UPtr<UClothingInteractor>>,
}
pub struct UClothPhysicalMeshDataBase_Legacy {
    pub vertices: TArray<FVector3f>,
    pub normals: TArray<FVector3f>,
    pub vertex_colors: TArray<FColor>,
    pub indices: TArray<u32>,
    pub inverse_masses: TArray<f32>,
    pub bone_data: TArray<FClothVertBoneData>,
    pub num_fixed_verts: i32,
    pub max_bone_weights: i32,
    pub self_collision_indices: TArray<u32>,
}
