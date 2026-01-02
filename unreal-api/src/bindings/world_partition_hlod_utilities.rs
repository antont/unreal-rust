#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UHLODBuilderCustomHLODActorSettings {
    __padding_end: [u8; 48],
}
impl UHLODBuilderCustomHLODActorSettings {}
#[repr(C, align(8))]
pub struct UHLODBuilderCustomHLODActor {
    __padding_end: [u8; 72],
}
impl UHLODBuilderCustomHLODActor {}
#[repr(C, align(8))]
pub struct UHLODBuilderInstancingSettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_disallow_nanite: bool,
    pub instance_filtering_type: EInstanceFilteringType,
    pub minimum_extent: f64,
    pub minimum_area: f64,
    pub minimum_volume: f64,
}
impl UHLODBuilderInstancingSettings {}
#[repr(C, align(8))]
pub struct UHLODBuilderInstancing {
    __padding_end: [u8; 72],
}
impl UHLODBuilderInstancing {}
#[repr(C, align(8))]
pub struct UHLODBuilderMeshApproximateSettings {
    __padding_end: [u8; 360],
}
impl UHLODBuilderMeshApproximateSettings {}
#[repr(C, align(8))]
pub struct UHLODBuilderMeshApproximate {
    __padding_end: [u8; 72],
}
impl UHLODBuilderMeshApproximate {}
#[repr(C, align(8))]
pub struct UHLODBuilderMeshMergeSettings {
    __padding_end: [u8; 384],
}
impl UHLODBuilderMeshMergeSettings {}
#[repr(C, align(8))]
pub struct UHLODBuilderMeshMerge {
    __padding_end: [u8; 72],
}
impl UHLODBuilderMeshMerge {}
#[repr(C, align(8))]
pub struct UHLODBuilderMeshSimplifySettings {
    __padding_end: [u8; 376],
}
impl UHLODBuilderMeshSimplifySettings {}
#[repr(C, align(8))]
pub struct UHLODBuilderMeshSimplify {
    __padding_end: [u8; 72],
}
impl UHLODBuilderMeshSimplify {}
#[repr(C, align(8))]
pub struct UWorldPartitionHLODModifierMeshDestruction {
    __padding_end: [u8; 80],
}
impl UWorldPartitionHLODModifierMeshDestruction {}
#[repr(transparent)]
pub struct EInstanceFilteringType(pub u8);
impl EInstanceFilteringType {
    pub const FILTER_NONE: EInstanceFilteringType = EInstanceFilteringType(0);
    pub const FILTER_MINIMUM_EXTENT: EInstanceFilteringType = EInstanceFilteringType(1);
    pub const FILTER_MINIMUM_AREA: EInstanceFilteringType = EInstanceFilteringType(2);
    pub const FILTER_MINIMUM_VOLUME: EInstanceFilteringType = EInstanceFilteringType(3);
}
