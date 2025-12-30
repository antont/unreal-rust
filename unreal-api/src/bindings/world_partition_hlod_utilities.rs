#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub mesh_approximation_settings: FMeshApproximationSettings,
    pub hlod_material: UPtr<UMaterialInterface>,
}
pub struct UHLODBuilderMeshApproximate {}
pub struct UHLODBuilderMeshMergeSettings {
    pub mesh_merge_settings: FMeshMergingSettings,
    pub hlod_material: UPtr<UMaterialInterface>,
}
pub struct UHLODBuilderMeshMerge {}
pub struct UHLODBuilderMeshSimplifySettings {
    pub mesh_simplify_settings: FMeshProxySettings,
    pub hlod_material: UPtr<UMaterialInterface>,
}
pub struct UHLODBuilderMeshSimplify {}
pub struct UWorldPartitionHLODModifierMeshDestruction {}
