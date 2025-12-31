#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FClothCollisionPrim_Sphere {
    pub bone_index: i32,
    pub radius: f32,
    pub local_position: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FClothCollisionPrim_SphereConnection {
    pub sphere_indices: i32,
    pub one_sided_plane_normal: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FClothCollisionPrim_ConvexFace {
    pub plane: crate::bindings::core_u_object::FPlane,
    pub indices: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FClothCollisionPrim_Convex {
    pub planes_deprecated: TArray<crate::bindings::core_u_object::FPlane>,
    pub faces: TArray<FClothCollisionPrim_ConvexFace>,
    pub surface_points: TArray<crate::bindings::core_u_object::FVector>,
    pub bone_index: i32,
}
#[repr(C, align(16))]
pub struct FClothCollisionPrim_Box {
    pub local_position: crate::bindings::core_u_object::FVector,
    pub local_rotation: crate::bindings::core_u_object::FQuat,
    pub half_extents: crate::bindings::core_u_object::FVector,
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
    pub asset_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UClothingSimulationFactory {}
pub struct UClothingInteractor {}
pub struct UClothingSimulationInteractor {
    pub clothing_interactors: TMap<FName, UPtr<UClothingInteractor>>,
}
pub struct UClothPhysicalMeshDataBase_Legacy {
    pub vertices: TArray<crate::bindings::core_u_object::FVector3f>,
    pub normals: TArray<crate::bindings::core_u_object::FVector3f>,
    pub vertex_colors: TArray<crate::bindings::core_u_object::FColor>,
    pub indices: TArray<u32>,
    pub inverse_masses: TArray<f32>,
    pub bone_data: TArray<FClothVertBoneData>,
    pub num_fixed_verts: i32,
    pub max_bone_weights: i32,
    pub self_collision_indices: TArray<u32>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EClothingTeleportMode(pub u8);
impl EClothingTeleportMode {
    pub const NONE: EClothingTeleportMode = EClothingTeleportMode(0);
    pub const TELEPORT: EClothingTeleportMode = EClothingTeleportMode(1);
    pub const TELEPORT_AND_RESET: EClothingTeleportMode = EClothingTeleportMode(2);
    pub const HARD_RESET: EClothingTeleportMode = EClothingTeleportMode(3);
}
