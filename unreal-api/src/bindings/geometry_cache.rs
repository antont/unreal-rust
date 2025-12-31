#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FTrackRenderData {}
#[repr(C, align(4))]
pub struct FGeometryCacheMeshBatchInfo {}
#[repr(C, align(1))]
pub struct FGeometryCacheVertexInfo {}
#[repr(C, align(8))]
pub struct FGeometryCacheMeshData {}
#[repr(C, align(8))]
pub struct FNiagaraGeometryCacheMICOverride {
    pub original_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub replacement_material: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
}
#[repr(C, align(8))]
pub struct FNiagaraGeometryCacheReference {
    pub geometry_cache: UPtr<UGeometryCache>,
    pub geometry_cache_user_param_binding: crate::bindings::niagara::FNiagaraUserParameterBinding,
    pub override_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub mic_override_materials: TArray<FNiagaraGeometryCacheMICOverride>,
}
pub struct UGeometryCache {
    pub asset_import_data: UPtr<crate::bindings::engine::UAssetImportData>,
    pub thumbnail_info: UPtr<crate::bindings::engine::UThumbnailInfo>,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub material_slot_names: TArray<FName>,
    pub tracks: TArray<UPtr<UGeometryCacheTrack>>,
    pub asset_user_data: TArray<UPtr<crate::bindings::engine::UAssetUserData>>,
    pub start_frame: i32,
    pub end_frame: i32,
    pub hash: u64,
}
pub struct AGeometryCacheActor {
    pub geometry_cache_component: UPtr<UGeometryCacheComponent>,
}
pub struct UGeometryCacheCodecBase {
    pub topology_ranges: TArray<i32>,
}
pub struct UGeometryCacheCodecRaw {
    pub dummy_property: i32,
}
pub struct UGeometryCacheCodecV1 {}
pub struct UGeometryCacheComponent {
    pub geometry_cache: UPtr<UGeometryCache>,
    pub b_running: bool,
    pub b_looping: bool,
    pub b_extrapolate_frames: bool,
    pub start_time_offset: f32,
    pub playback_speed: f32,
    pub motion_vector_scale: f32,
    pub num_tracks: i32,
    pub elapsed_time: f32,
    pub duration: f32,
    pub b_manual_tick: bool,
    pub b_override_wireframe_color: bool,
    pub wireframe_override_color: crate::bindings::core_u_object::FLinearColor,
}
pub struct UGeometryCacheTrack {
    pub duration: f32,
}
pub struct UDEPRECATED_GeometryCacheTrack_FlipbookAnimation {
    pub num_mesh_samples: u32,
}
pub struct UGeometryCacheTrackStreamable {
    pub codec: UPtr<UGeometryCacheCodecBase>,
    pub start_sample_time: f32,
}
pub struct UDEPRECATED_GeometryCacheTrack_TransformAnimation {}
pub struct UDEPRECATED_GeometryCacheTrack_TransformGroupAnimation {}
pub struct UNiagaraGeometryCacheRendererProperties {
    pub geometry_caches: TArray<FNiagaraGeometryCacheReference>,
    pub source_mode: crate::bindings::niagara::ENiagaraRendererSourceDataMode,
    pub b_is_looping: bool,
    pub component_count_limit: u32,
    pub position_binding: crate::bindings::niagara::FNiagaraVariableAttributeBinding,
    pub rotation_binding: crate::bindings::niagara::FNiagaraVariableAttributeBinding,
    pub scale_binding: crate::bindings::niagara::FNiagaraVariableAttributeBinding,
    pub elapsed_time_binding: crate::bindings::niagara::FNiagaraVariableAttributeBinding,
    pub enabled_binding: crate::bindings::niagara::FNiagaraVariableAttributeBinding,
    pub array_index_binding: crate::bindings::niagara::FNiagaraVariableAttributeBinding,
    pub renderer_visibility_tag_binding: crate::bindings::niagara::FNiagaraVariableAttributeBinding,
    pub renderer_visibility: i32,
    pub b_assign_components_on_particle_id: bool,
    pub material_parameters: crate::bindings::niagara::FNiagaraRendererMaterialParameters,
}
