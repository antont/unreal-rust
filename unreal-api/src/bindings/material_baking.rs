#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FPropertyEntry {
    pub property: EMaterialProperty,
    pub b_use_custom_size: bool,
    pub custom_size: FIntPoint,
    pub b_use_constant_value: bool,
    pub constant_value: f32,
}
pub struct UMaterialOptions {
    pub properties: TArray<FPropertyEntry>,
    pub texture_size: FIntPoint,
    pub lod_indices: TArray<i32>,
    pub b_use_mesh_data: bool,
    pub b_use_specific_uv_index: bool,
    pub texture_coordinate_index: i32,
}
pub struct UAssetBakeOptions {}
pub struct UMaterialMergeOptions {
    pub method: EMaterialBakeMethod,
    pub blend_mode: EBlendMode,
}
