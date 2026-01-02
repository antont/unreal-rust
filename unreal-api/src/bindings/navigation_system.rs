#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UCrowdManagerBase {
    __padding_end: [u8; 48],
}
impl UCrowdManagerBase {}
#[repr(C, align(8))]
pub struct UBaseGeneratedNavLinksProxy {
    __padding_end: [u8; 72],
}
impl UBaseGeneratedNavLinksProxy {}
#[repr(C, align(8))]
pub struct UNavigationQueryFilter {
    __padding_end: [u8; 80],
}
impl UNavigationQueryFilter {}
#[repr(C, align(8))]
pub struct ANavigationGraphNode {
    __padding_end: [u8; 1136],
}
impl ANavigationGraphNode {}
#[repr(C, align(16))]
pub struct UNavigationGraphNodeComponent {
    __padding_end: [u8; 704],
}
impl UNavigationGraphNodeComponent {}
pub struct UNavigationPathGenerator {}
pub struct INavigationPathGenerator {}
pub struct UNavLinkCustomInterface {}
pub struct INavLinkCustomInterface {}
pub struct UNavLinkHostInterface {}
pub struct INavLinkHostInterface {}
#[repr(C, align(8))]
pub struct UNavLinkTrivial {
    __padding_end: [u8; 88],
}
impl UNavLinkTrivial {}
pub struct UNavNodeInterface {}
pub struct INavNodeInterface {}
#[repr(C, align(8))]
pub struct ANavigationData {
    __padding_end: [u8; 1760],
}
impl ANavigationData {}
#[repr(C, align(8))]
pub struct AAbstractNavData {
    __padding_end: [u8; 1760],
}
impl AAbstractNavData {}
#[repr(C, align(8))]
pub struct UNavArea {
    __padding_end: [u8; 80],
}
impl UNavArea {}
#[repr(C, align(8))]
pub struct UNavAreaMeta {
    __padding_end: [u8; 80],
}
impl UNavAreaMeta {}
#[repr(C, align(8))]
pub struct UNavAreaMeta_SwitchByAgent {
    __padding_end: [u8; 208],
}
impl UNavAreaMeta_SwitchByAgent {}
#[repr(C, align(8))]
pub struct UNavArea_Default {
    __padding_end: [u8; 80],
}
impl UNavArea_Default {}
#[repr(C, align(8))]
pub struct UNavArea_LowHeight {
    __padding_end: [u8; 80],
}
impl UNavArea_LowHeight {}
#[repr(C, align(8))]
pub struct UNavArea_Null {
    __padding_end: [u8; 80],
}
impl UNavArea_Null {}
#[repr(C, align(8))]
pub struct UNavArea_Obstacle {
    __padding_end: [u8; 80],
}
impl UNavArea_Obstacle {}
#[repr(C, align(8))]
pub struct UNavCollision {
    __padding_end: [u8; 312],
}
impl UNavCollision {}
#[repr(C, align(8))]
pub struct URecastFilter_UseDefaultArea {
    __padding_end: [u8; 80],
}
impl URecastFilter_UseDefaultArea {}
#[repr(C, align(8))]
pub struct ANavigationGraph {
    __padding_end: [u8; 1760],
}
impl ANavigationGraph {}
#[repr(C, align(8))]
pub struct UNavigationInvokerComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub tile_generation_radius: f32,
    pub tile_removal_radius: f32,
    __padding_end: [u8; 8],
}
impl UNavigationInvokerComponent {}
#[repr(C, align(8))]
pub struct UNavigationObjectRepository {
    __padding_end: [u8; 392],
}
impl UNavigationObjectRepository {}
#[repr(C, align(8))]
pub struct UNavigationPath {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub path_points: TArray<crate::bindings::core_u_object::FVector>,
    pub recalculate_on_invalidation: crate::bindings::engine::ENavigationOptionFlag,
    __padding_end: [u8; 71],
}
impl UNavigationPath {}
#[repr(C, align(16))]
pub struct UNavigationSystemV1 {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub default_agent_name: FName,
    pub crowd_manager_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    __padding_end: [u8; 5712],
}
impl UNavigationSystemV1 {}
#[repr(C, align(8))]
pub struct UNavigationSystemModuleConfig {
    __padding_end: [u8; 120],
}
impl UNavigationSystemModuleConfig {}
#[repr(C, align(8))]
pub struct ANavigationTestingActor {
    #[doc(hidden)]
    __padding_1280: [u8; 1280],
    pub projected_location: crate::bindings::core_u_object::FVector,
    pub flags_1304: u8,
    #[doc(hidden)]
    __padding_1360: [u8; 55],
    pub flags_1360: u8,
    pub pathfinding_time: f32,
    pub path_cost: f64,
    pub pathfinding_steps: i32,
    __padding_end: [u8; 148],
}
impl ANavigationTestingActor {}
#[repr(C, align(16))]
pub struct UNavLinkComponent {
    #[doc(hidden)]
    __padding_1512: [u8; 1512],
    pub links: TArray<crate::bindings::engine::FNavigationLink>,
    __padding_end: [u8; 8],
}
impl UNavLinkComponent {}
#[repr(C, align(8))]
pub struct UNavRelevantComponent {
    __padding_end: [u8; 320],
}
impl UNavRelevantComponent {}
#[repr(C, align(8))]
pub struct UNavLinkCustomComponent {
    __padding_end: [u8; 592],
}
impl UNavLinkCustomComponent {}
#[repr(C, align(16))]
pub struct UNavLinkRenderingComponent {
    __padding_end: [u8; 1504],
}
impl UNavLinkRenderingComponent {}
#[repr(C, align(8))]
pub struct ANavMeshBoundsVolume {
    __padding_end: [u8; 1216],
}
impl ANavMeshBoundsVolume {}
#[repr(C, align(16))]
pub struct UNavMeshRenderingComponent {
    __padding_end: [u8; 1760],
}
impl UNavMeshRenderingComponent {}
#[repr(C, align(16))]
pub struct UNavTestRenderingComponent {
    __padding_end: [u8; 1840],
}
impl UNavTestRenderingComponent {}
#[repr(C, align(8))]
pub struct ARecastNavMesh {
    __padding_end: [u8; 2480],
}
impl ARecastNavMesh {}
#[repr(C, align(8))]
pub struct URecastNavMeshDataChunk {
    __padding_end: [u8; 80],
}
impl URecastNavMeshDataChunk {}
#[repr(C, align(16))]
pub struct UNavModifierComponent {
    #[doc(hidden)]
    __padding_320: [u8; 320],
    pub area_class: TSubclassOf<UNavArea>,
    pub area_class_to_replace: TSubclassOf<UNavArea>,
    __padding_end: [u8; 176],
}
impl UNavModifierComponent {}
#[repr(C, align(8))]
pub struct ANavModifierVolume {
    #[doc(hidden)]
    __padding_1216: [u8; 1216],
    pub area_class: TSubclassOf<UNavArea>,
    pub area_class_to_replace: TSubclassOf<UNavArea>,
    __padding_end: [u8; 24],
}
impl ANavModifierVolume {}
#[repr(C, align(8))]
pub struct ANavSystemConfigOverride {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub navigation_system_config: UPtr<crate::bindings::engine::UNavigationSystemConfig>,
    pub override_policy: ENavSystemOverridePolicy,
    pub flags_1153: u8,
    __padding_end: [u8; 6],
}
impl ANavSystemConfigOverride {}
#[repr(C, align(16))]
pub struct USplineNavModifierComponent {
    __padding_end: [u8; 688],
}
impl USplineNavModifierComponent {}
#[repr(transparent)]
pub struct FNavigationPath_PathUpdatedNotifier {
    _opague: u8,
}
#[repr(transparent)]
pub struct FNavigationSystemV1_OnNavDataRegisteredEvent {
    _opague: u8,
}
#[repr(transparent)]
pub struct FNavigationSystemV1_OnNavigationGenerationFinishedDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct EHeightFieldRenderMode(pub u8);
impl EHeightFieldRenderMode {
    pub const SOLID: EHeightFieldRenderMode = EHeightFieldRenderMode(0);
    pub const WALKABLE: EHeightFieldRenderMode = EHeightFieldRenderMode(1);
}
#[repr(transparent)]
pub struct ERuntimeGenerationType(pub u8);
impl ERuntimeGenerationType {
    pub const STATIC: ERuntimeGenerationType = ERuntimeGenerationType(0);
    pub const DYNAMIC_MODIFIERS_ONLY: ERuntimeGenerationType = ERuntimeGenerationType(1);
    pub const DYNAMIC: ERuntimeGenerationType = ERuntimeGenerationType(2);
    pub const LEGACY_GENERATION: ERuntimeGenerationType = ERuntimeGenerationType(3);
}
#[repr(transparent)]
pub struct ENavCostDisplay(pub u8);
impl ENavCostDisplay {
    pub const TOTAL_COST: ENavCostDisplay = ENavCostDisplay(0);
    pub const HEURISTIC_ONLY: ENavCostDisplay = ENavCostDisplay(1);
    pub const REAL_COST_ONLY: ENavCostDisplay = ENavCostDisplay(2);
}
#[repr(transparent)]
pub struct ENavigationLedgeSlopeFilterMode(pub u8);
impl ENavigationLedgeSlopeFilterMode {
    pub const RECAST: ENavigationLedgeSlopeFilterMode = ENavigationLedgeSlopeFilterMode(
        0,
    );
    pub const NONE: ENavigationLedgeSlopeFilterMode = ENavigationLedgeSlopeFilterMode(1);
    pub const USE_STEP_HEIGHT_FROM_AGENT_MAX_SLOPE: ENavigationLedgeSlopeFilterMode = ENavigationLedgeSlopeFilterMode(
        2,
    );
}
#[repr(transparent)]
pub struct ERecastPartitioning(pub u8);
impl ERecastPartitioning {
    pub const MONOTONE: ERecastPartitioning = ERecastPartitioning(0);
    pub const WATERSHED: ERecastPartitioning = ERecastPartitioning(1);
    pub const CHUNKY_MONOTONE: ERecastPartitioning = ERecastPartitioning(2);
}
#[repr(transparent)]
pub struct ENavSystemOverridePolicy(pub u8);
impl ENavSystemOverridePolicy {
    pub const OVERRIDE: ENavSystemOverridePolicy = ENavSystemOverridePolicy(0);
    pub const APPEND: ENavSystemOverridePolicy = ENavSystemOverridePolicy(1);
    pub const SKIP: ENavSystemOverridePolicy = ENavSystemOverridePolicy(2);
}
#[repr(transparent)]
pub struct ESubdivisionLOD(pub i32);
impl ESubdivisionLOD {
    pub const LOW: ESubdivisionLOD = ESubdivisionLOD(0);
    pub const MEDIUM: ESubdivisionLOD = ESubdivisionLOD(1);
    pub const HIGH: ESubdivisionLOD = ESubdivisionLOD(2);
    pub const ULTRA: ESubdivisionLOD = ESubdivisionLOD(3);
}
