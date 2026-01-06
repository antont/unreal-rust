#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub fn initialize() {}
#[repr(C, align(4))]
pub struct FAbcCompressionSettings {
    pub b_merge_meshes: bool,
    pub b_bake_matrix_animation: bool,
    pub base_calculation_type: EBaseCalculationType,
    pub percentage_of_total_bases: f32,
    pub max_number_of_bases: i32,
    pub minimum_number_of_vertex_influence_percentage: f32,
}
impl FAbcCompressionSettings {}
#[repr(C, align(4))]
pub struct FAbcSamplingSettings {
    pub sampling_type: EAlembicSamplingType,
    pub frame_steps: i32,
    pub time_steps: f32,
    pub frame_start: i32,
    pub frame_end: i32,
    pub b_skip_empty: bool,
    __padding_end: [u8; 3],
}
impl FAbcSamplingSettings {}
#[repr(C, align(4))]
pub struct FAbcNormalGenerationSettings {
    pub b_force_one_smoothing_group_per_object: bool,
    pub hard_edge_angle_threshold: f32,
    pub b_recompute_normals: bool,
    pub b_ignore_degenerate_triangles: bool,
    pub b_skip_computing_tangents: bool,
    __padding_end: [u8; 1],
}
impl FAbcNormalGenerationSettings {}
#[repr(C, align(1))]
pub struct FAbcMaterialSettings {
    pub b_create_materials: bool,
    pub b_find_materials: bool,
}
impl FAbcMaterialSettings {}
#[repr(C, align(1))]
pub struct FAbcStaticMeshSettings {
    pub b_merge_meshes: bool,
    pub b_propagate_matrix_transformations: bool,
    pub b_generate_lightmap_u_vs: bool,
}
impl FAbcStaticMeshSettings {}
#[repr(C, align(8))]
pub struct FAbcConversionSettings {
    pub preset: EAbcConversionPreset,
    pub b_flip_u: bool,
    pub b_flip_v: bool,
    pub scale: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FVector,
}
impl FAbcConversionSettings {}
#[repr(C, align(4))]
pub struct FAbcGeometryCacheSettings {
    pub b_flatten_tracks: bool,
    pub b_store_imported_vertex_numbers: bool,
    pub b_apply_constant_topology_optimizations: bool,
    #[doc(hidden)]
    __padding_4: [u8; 1],
    pub motion_vectors: EAbcGeometryCacheMotionVectorsImport,
    pub b_optimize_index_buffers: bool,
    pub compressed_position_precision: f32,
    pub compressed_texture_coordinates_number_of_bits: i32,
}
impl FAbcGeometryCacheSettings {}
#[repr(C, align(8))]
pub struct UAbcAssetImportData {
    __padding_end: [u8; 248],
}
impl UAbcAssetImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbcAssetImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAbcImportSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub import_type: EAlembicImportType,
    pub sampling_settings: FAbcSamplingSettings,
    pub normal_generation_settings: FAbcNormalGenerationSettings,
    pub material_settings: FAbcMaterialSettings,
    pub compression_settings: FAbcCompressionSettings,
    pub static_mesh_settings: FAbcStaticMeshSettings,
    pub geometry_cache_settings: FAbcGeometryCacheSettings,
    pub conversion_settings: FAbcConversionSettings,
    __padding_end: [u8; 8],
}
impl UAbcImportSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbcImportSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAlembicTestCommandlet {
    __padding_end: [u8; 168],
}
impl UAlembicTestCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAlembicTestCommandlet")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(transparent)]
pub struct EBaseCalculationType(pub u8);
impl EBaseCalculationType {
    pub const NONE: EBaseCalculationType = EBaseCalculationType(0);
    pub const PERCENTAGE_BASED: EBaseCalculationType = EBaseCalculationType(1);
    pub const FIXED_NUMBER: EBaseCalculationType = EBaseCalculationType(2);
    pub const NO_COMPRESSION: EBaseCalculationType = EBaseCalculationType(3);
}
#[repr(transparent)]
pub struct EAlembicSamplingType(pub u8);
impl EAlembicSamplingType {
    pub const PER_FRAME: EAlembicSamplingType = EAlembicSamplingType(0);
    pub const PER_X_FRAMES: EAlembicSamplingType = EAlembicSamplingType(1);
    pub const PER_TIME_STEP: EAlembicSamplingType = EAlembicSamplingType(2);
}
#[repr(transparent)]
pub struct EAbcConversionPreset(pub u8);
impl EAbcConversionPreset {
    pub const MAYA: EAbcConversionPreset = EAbcConversionPreset(0);
    pub const MAX: EAbcConversionPreset = EAbcConversionPreset(1);
    pub const CUSTOM: EAbcConversionPreset = EAbcConversionPreset(2);
}
#[repr(transparent)]
pub struct EAbcGeometryCacheMotionVectorsImport(pub u8);
impl EAbcGeometryCacheMotionVectorsImport {
    pub const NO_MOTION_VECTORS: EAbcGeometryCacheMotionVectorsImport = EAbcGeometryCacheMotionVectorsImport(
        0,
    );
    pub const IMPORT_ABC_VELOCITIES_AS_MOTION_VECTORS: EAbcGeometryCacheMotionVectorsImport = EAbcGeometryCacheMotionVectorsImport(
        1,
    );
    pub const CALCULATE_MOTION_VECTORS_DURING_IMPORT: EAbcGeometryCacheMotionVectorsImport = EAbcGeometryCacheMotionVectorsImport(
        2,
    );
}
#[repr(transparent)]
pub struct EAlembicImportType(pub u8);
impl EAlembicImportType {
    pub const STATIC_MESH: EAlembicImportType = EAlembicImportType(0);
    pub const GEOMETRY_CACHE: EAlembicImportType = EAlembicImportType(1);
    pub const SKELETAL: EAlembicImportType = EAlembicImportType(2);
}
