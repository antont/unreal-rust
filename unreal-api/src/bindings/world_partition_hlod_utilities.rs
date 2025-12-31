#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UHLODBuilderCustomHLODActorSettings {}
pub struct UHLODBuilderCustomHLODActor {}
pub struct UHLODBuilderInstancingSettings {
    pub b_disallow_nanite: bool,
    pub instance_filtering_type: EInstanceFilteringType,
    pub minimum_extent: f64,
    pub minimum_area: f64,
    pub minimum_volume: f64,
}
pub struct UHLODBuilderInstancing {}
pub struct UHLODBuilderMeshApproximateSettings {
    pub mesh_approximation_settings: crate::bindings::engine::FMeshApproximationSettings,
    pub hlod_material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
pub struct UHLODBuilderMeshApproximate {}
pub struct UHLODBuilderMeshMergeSettings {
    pub mesh_merge_settings: crate::bindings::engine::FMeshMergingSettings,
    pub hlod_material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
pub struct UHLODBuilderMeshMerge {}
pub struct UHLODBuilderMeshSimplifySettings {
    pub mesh_simplify_settings: crate::bindings::engine::FMeshProxySettings,
    pub hlod_material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
pub struct UHLODBuilderMeshSimplify {}
pub struct UWorldPartitionHLODModifierMeshDestruction {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInstanceFilteringType(pub u8);
impl EInstanceFilteringType {
    pub const FILTER_NONE: EInstanceFilteringType = EInstanceFilteringType(0);
    pub const FILTER_MINIMUM_EXTENT: EInstanceFilteringType = EInstanceFilteringType(1);
    pub const FILTER_MINIMUM_AREA: EInstanceFilteringType = EInstanceFilteringType(2);
    pub const FILTER_MINIMUM_VOLUME: EInstanceFilteringType = EInstanceFilteringType(3);
}
