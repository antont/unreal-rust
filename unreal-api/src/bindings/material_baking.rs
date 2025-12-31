#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FPropertyEntry {
    pub property: crate::bindings::engine::EMaterialProperty,
    pub b_use_custom_size: bool,
    pub custom_size: crate::bindings::core_u_object::FIntPoint,
    pub b_use_constant_value: bool,
    pub constant_value: f32,
}
pub struct UMaterialOptions {
    pub properties: TArray<FPropertyEntry>,
    pub texture_size: crate::bindings::core_u_object::FIntPoint,
    pub lod_indices: TArray<i32>,
    pub b_use_mesh_data: bool,
    pub b_use_specific_uv_index: bool,
    pub texture_coordinate_index: i32,
}
pub struct UAssetBakeOptions {}
pub struct UMaterialMergeOptions {
    pub method: EMaterialBakeMethod,
    pub blend_mode: crate::bindings::engine::EBlendMode,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMaterialBakeMethod(pub u8);
impl EMaterialBakeMethod {
    pub const INDIVIDUAL_MATERIAL: EMaterialBakeMethod = EMaterialBakeMethod(0);
    pub const ATLAS_MATERIAL: EMaterialBakeMethod = EMaterialBakeMethod(1);
    pub const BINNED_MATERIAL: EMaterialBakeMethod = EMaterialBakeMethod(2);
}
