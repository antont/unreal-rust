#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FSolverTrailingFilterSettings {
    pub filter_enabled: bool,
    pub min_mass: f32,
    pub min_speed: f32,
    pub min_volume: f32,
}
impl FSolverTrailingFilterSettings {}
#[repr(C, align(4))]
pub struct FSolverBreakingFilterSettings {
    pub filter_enabled: bool,
    pub min_mass: f32,
    pub min_speed: f32,
    pub min_volume: f32,
}
impl FSolverBreakingFilterSettings {}
#[repr(C, align(4))]
pub struct FSolverCollisionFilterSettings {
    pub filter_enabled: bool,
    pub min_mass: f32,
    pub min_speed: f32,
    pub min_impulse: f32,
}
impl FSolverCollisionFilterSettings {}
#[repr(C, align(8))]
pub struct FClosestPhysicsObjectResult {
    pub(crate) __padding_end: [u8; 40],
}
impl FClosestPhysicsObjectResult {}
#[repr(C, align(4))]
pub struct FSolverRemovalFilterSettings {
    pub filter_enabled: bool,
    pub min_mass: f32,
    pub min_volume: f32,
}
impl FSolverRemovalFilterSettings {}
#[repr(transparent)]
pub struct EGeometryCollectionCacheType(pub u8);
impl EGeometryCollectionCacheType {
    pub const NONE: EGeometryCollectionCacheType = EGeometryCollectionCacheType(0);
    pub const RECORD: EGeometryCollectionCacheType = EGeometryCollectionCacheType(1);
    pub const PLAY: EGeometryCollectionCacheType = EGeometryCollectionCacheType(2);
    pub const RECORD_AND_PLAY: EGeometryCollectionCacheType = EGeometryCollectionCacheType(
        3,
    );
}
#[repr(transparent)]
pub struct EClusterUnionMethod(pub u8);
impl EClusterUnionMethod {
    pub const POINT_IMPLICIT: EClusterUnionMethod = EClusterUnionMethod(0);
    pub const DELAUNAY_TRIANGULATION: EClusterUnionMethod = EClusterUnionMethod(1);
    pub const MINIMAL_SPANNING_SUBSET_DELAUNAY_TRIANGULATION: EClusterUnionMethod = EClusterUnionMethod(
        2,
    );
    pub const POINT_IMPLICIT_AUGMENTED_WITH_MINIMAL_DELAUNAY: EClusterUnionMethod = EClusterUnionMethod(
        3,
    );
    pub const BOUNDS_OVERLAP_FILTERED_DELAUNAY_TRIANGULATION: EClusterUnionMethod = EClusterUnionMethod(
        4,
    );
    pub const NONE: EClusterUnionMethod = EClusterUnionMethod(5);
}
#[repr(transparent)]
pub struct ECollisionTypeEnum(pub u8);
impl ECollisionTypeEnum {
    pub const CHAOS_VOLUMETRIC: ECollisionTypeEnum = ECollisionTypeEnum(0);
    pub const CHAOS_SURFACE_VOLUMETRIC: ECollisionTypeEnum = ECollisionTypeEnum(1);
}
#[repr(transparent)]
pub struct EImplicitTypeEnum(pub u8);
impl EImplicitTypeEnum {
    pub const CHAOS_IMPLICIT_BOX: EImplicitTypeEnum = EImplicitTypeEnum(0);
    pub const CHAOS_IMPLICIT_SPHERE: EImplicitTypeEnum = EImplicitTypeEnum(1);
    pub const CHAOS_IMPLICIT_CAPSULE: EImplicitTypeEnum = EImplicitTypeEnum(2);
    pub const CHAOS_IMPLICIT_LEVEL_SET: EImplicitTypeEnum = EImplicitTypeEnum(3);
    pub const CHAOS_IMPLICIT_NONE: EImplicitTypeEnum = EImplicitTypeEnum(4);
    pub const CHAOS_IMPLICIT_CONVEX: EImplicitTypeEnum = EImplicitTypeEnum(5);
}
#[repr(transparent)]
pub struct EChaosThreadingMode(pub u8);
impl EChaosThreadingMode {
    pub const DEDICATED_THREAD: EChaosThreadingMode = EChaosThreadingMode(0);
    pub const TASK_GRAPH: EChaosThreadingMode = EChaosThreadingMode(1);
    pub const SINGLE_THREAD: EChaosThreadingMode = EChaosThreadingMode(2);
    pub const NUM: EChaosThreadingMode = EChaosThreadingMode(3);
    pub const INVALID: EChaosThreadingMode = EChaosThreadingMode(4);
}
#[repr(transparent)]
pub struct EChaosSolverTickMode(pub u8);
impl EChaosSolverTickMode {
    pub const FIXED: EChaosSolverTickMode = EChaosSolverTickMode(0);
    pub const VARIABLE: EChaosSolverTickMode = EChaosSolverTickMode(1);
    pub const VARIABLE_CAPPED: EChaosSolverTickMode = EChaosSolverTickMode(2);
    pub const VARIABLE_CAPPED_WITH_TARGET: EChaosSolverTickMode = EChaosSolverTickMode(
        3,
    );
}
#[repr(transparent)]
pub struct EChaosBufferMode(pub u8);
impl EChaosBufferMode {
    pub const DOUBLE: EChaosBufferMode = EChaosBufferMode(0);
    pub const TRIPLE: EChaosBufferMode = EChaosBufferMode(1);
    pub const NUM: EChaosBufferMode = EChaosBufferMode(2);
    pub const INVALID: EChaosBufferMode = EChaosBufferMode(3);
}
#[repr(transparent)]
pub struct EGenerateConvexMethod(pub u8);
impl EGenerateConvexMethod {
    pub const EXTERNAL_COLLISION: EGenerateConvexMethod = EGenerateConvexMethod(0);
    pub const COMPUTED_FROM_GEOMETRY: EGenerateConvexMethod = EGenerateConvexMethod(1);
    pub const INTERSECT_EXTERNAL_WITH_COMPUTED: EGenerateConvexMethod = EGenerateConvexMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EAllowConvexMergeMethod(pub u8);
impl EAllowConvexMergeMethod {
    pub const BY_PROXIMITY: EAllowConvexMergeMethod = EAllowConvexMergeMethod(0);
    pub const ANY: EAllowConvexMergeMethod = EAllowConvexMergeMethod(1);
}
#[repr(transparent)]
pub struct EConvexHullProximityFilter(pub u8);
impl EConvexHullProximityFilter {
    pub const NONE: EConvexHullProximityFilter = EConvexHullProximityFilter(0);
    pub const BOUNDING_BOX: EConvexHullProximityFilter = EConvexHullProximityFilter(1);
}
#[repr(transparent)]
pub struct EGeometryCollectionPhysicsTypeEnum(pub u8);
impl EGeometryCollectionPhysicsTypeEnum {
    pub const CHAOS_ANGULAR_VELOCITY: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        0,
    );
    pub const CHAOS_DYNAMIC_STATE: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        1,
    );
    pub const CHAOS_LINEAR_VELOCITY: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        2,
    );
    pub const CHAOS_INITIAL_ANGULAR_VELOCITY: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        3,
    );
    pub const CHAOS_INITIAL_LINEAR_VELOCITY: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        4,
    );
    pub const CHAOS_COLLISION_GROUP: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        5,
    );
    pub const CHAOS_LINEAR_FORCE: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        6,
    );
    pub const CHAOS_ANGULAR_TORQUE: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        7,
    );
    pub const CHAOS_DISABLE_THRESHOLD: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        8,
    );
    pub const CHAOS_SLEEPING_THRESHOLD: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        9,
    );
    pub const CHAOS_EXTERNAL_CLUSTER_STRAIN: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        10,
    );
    pub const CHAOS_INTERNAL_CLUSTER_STRAIN: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        11,
    );
    pub const CHAOS_LINEAR_IMPULSE: EGeometryCollectionPhysicsTypeEnum = EGeometryCollectionPhysicsTypeEnum(
        12,
    );
}
#[repr(transparent)]
pub struct EDamageModelTypeEnum(pub u8);
impl EDamageModelTypeEnum {
    pub const CHAOS_DAMAGE_MODEL_USER_DEFINED_DAMAGE_THRESHOLD: EDamageModelTypeEnum = EDamageModelTypeEnum(
        0,
    );
    pub const CHAOS_DAMAGE_MODEL_MATERIAL_STRENGTH_AND_CONNECTIVITY_DAMAGE_THRESHOLD: EDamageModelTypeEnum = EDamageModelTypeEnum(
        1,
    );
}
#[repr(transparent)]
pub struct EFieldPhysicsType(pub u8);
impl EFieldPhysicsType {
    pub const FIELD_NONE: EFieldPhysicsType = EFieldPhysicsType(0);
    pub const FIELD_DYNAMIC_STATE: EFieldPhysicsType = EFieldPhysicsType(1);
    pub const FIELD_LINEAR_FORCE: EFieldPhysicsType = EFieldPhysicsType(2);
    pub const FIELD_EXTERNAL_CLUSTER_STRAIN: EFieldPhysicsType = EFieldPhysicsType(3);
    pub const FIELD_KILL: EFieldPhysicsType = EFieldPhysicsType(4);
    pub const FIELD_LINEAR_VELOCITY: EFieldPhysicsType = EFieldPhysicsType(5);
    pub const FIELD_ANGULAR_VELOCIY: EFieldPhysicsType = EFieldPhysicsType(6);
    pub const FIELD_ANGULAR_TORQUE: EFieldPhysicsType = EFieldPhysicsType(7);
    pub const FIELD_INTERNAL_CLUSTER_STRAIN: EFieldPhysicsType = EFieldPhysicsType(8);
    pub const FIELD_DISABLE_THRESHOLD: EFieldPhysicsType = EFieldPhysicsType(9);
    pub const FIELD_SLEEPING_THRESHOLD: EFieldPhysicsType = EFieldPhysicsType(10);
    pub const FIELD_POSITION_STATIC: EFieldPhysicsType = EFieldPhysicsType(11);
    pub const FIELD_POSITION_ANIMATED: EFieldPhysicsType = EFieldPhysicsType(12);
    pub const FIELD_POSITION_TARGET: EFieldPhysicsType = EFieldPhysicsType(13);
    pub const FIELD_DYNAMIC_CONSTRAINT: EFieldPhysicsType = EFieldPhysicsType(14);
    pub const FIELD_COLLISION_GROUP: EFieldPhysicsType = EFieldPhysicsType(15);
    pub const FIELD_ACTIVATE_DISABLED: EFieldPhysicsType = EFieldPhysicsType(16);
    pub const FIELD_INITIAL_LINEAR_VELOCITY: EFieldPhysicsType = EFieldPhysicsType(17);
    pub const FIELD_INITIAL_ANGULAR_VELOCITY: EFieldPhysicsType = EFieldPhysicsType(18);
    pub const FIELD_LINEAR_IMPULSE: EFieldPhysicsType = EFieldPhysicsType(19);
}
#[repr(transparent)]
pub struct EFieldResolutionType(pub u8);
impl EFieldResolutionType {
    pub const FIELD_RESOLUTION_MINIMAL: EFieldResolutionType = EFieldResolutionType(0);
    pub const FIELD_RESOLUTION_DISABLED_PARENTS: EFieldResolutionType = EFieldResolutionType(
        1,
    );
}
#[repr(transparent)]
pub struct EFieldFilterType(pub u8);
impl EFieldFilterType {
    pub const FIELD_FILTER_DYNAMIC: EFieldFilterType = EFieldFilterType(0);
    pub const FIELD_FILTER_KINEMATIC: EFieldFilterType = EFieldFilterType(1);
    pub const FIELD_FILTER_STATIC: EFieldFilterType = EFieldFilterType(2);
    pub const FIELD_FILTER_ALL: EFieldFilterType = EFieldFilterType(3);
    pub const FIELD_FILTER_SLEEPING: EFieldFilterType = EFieldFilterType(4);
    pub const FIELD_FILTER_DISABLED: EFieldFilterType = EFieldFilterType(5);
}
#[repr(transparent)]
pub struct EFieldObjectType(pub u8);
impl EFieldObjectType {
    pub const FIELD_OBJECT_RIGID: EFieldObjectType = EFieldObjectType(0);
    pub const FIELD_OBJECT_CLOTH: EFieldObjectType = EFieldObjectType(1);
    pub const FIELD_OBJECT_DESTRUCTION: EFieldObjectType = EFieldObjectType(2);
    pub const FIELD_OBJECT_CHARACTER: EFieldObjectType = EFieldObjectType(3);
    pub const FIELD_OBJECT_ALL: EFieldObjectType = EFieldObjectType(4);
}
#[repr(transparent)]
pub struct EFieldPositionType(pub u8);
impl EFieldPositionType {
    pub const FIELD_POSITION_CENTER_OF_MASS: EFieldPositionType = EFieldPositionType(0);
    pub const FIELD_POSITION_PIVOT_POINT: EFieldPositionType = EFieldPositionType(1);
}
#[repr(transparent)]
pub struct ESetMaskConditionType(pub u8);
impl ESetMaskConditionType {
    pub const FIELD_SET_ALWAYS: ESetMaskConditionType = ESetMaskConditionType(0);
    pub const FIELD_SET_IFF_NOT_INTERIOR: ESetMaskConditionType = ESetMaskConditionType(
        1,
    );
    pub const FIELD_SET_IFF_NOT_EXTERIOR: ESetMaskConditionType = ESetMaskConditionType(
        2,
    );
}
#[repr(transparent)]
pub struct EWaveFunctionType(pub u8);
impl EWaveFunctionType {
    pub const FIELD_WAVE_COSINE: EWaveFunctionType = EWaveFunctionType(0);
    pub const FIELD_WAVE_GAUSSIAN: EWaveFunctionType = EWaveFunctionType(1);
    pub const FIELD_WAVE_FALLOFF: EWaveFunctionType = EWaveFunctionType(2);
    pub const FIELD_WAVE_DECAY: EWaveFunctionType = EWaveFunctionType(3);
}
#[repr(transparent)]
pub struct EFieldFalloffType(pub u8);
impl EFieldFalloffType {
    pub const FIELD_FALL_OFF_NONE: EFieldFalloffType = EFieldFalloffType(0);
    pub const FIELD_FALLOFF_LINEAR: EFieldFalloffType = EFieldFalloffType(1);
    pub const FIELD_FALLOFF_INVERSE: EFieldFalloffType = EFieldFalloffType(2);
    pub const FIELD_FALLOFF_SQUARED: EFieldFalloffType = EFieldFalloffType(3);
    pub const FIELD_FALLOFF_LOGARITHMIC: EFieldFalloffType = EFieldFalloffType(4);
}
#[repr(transparent)]
pub struct EFieldOperationType(pub u8);
impl EFieldOperationType {
    pub const FIELD_MULTIPLY: EFieldOperationType = EFieldOperationType(0);
    pub const FIELD_DIVIDE: EFieldOperationType = EFieldOperationType(1);
    pub const FIELD_ADD: EFieldOperationType = EFieldOperationType(2);
    pub const FIELD_SUBSTRACT: EFieldOperationType = EFieldOperationType(3);
}
#[repr(transparent)]
pub struct EFieldCullingOperationType(pub u8);
impl EFieldCullingOperationType {
    pub const FIELD_CULLING_INSIDE: EFieldCullingOperationType = EFieldCullingOperationType(
        0,
    );
    pub const FIELD_CULLING_OUTSIDE: EFieldCullingOperationType = EFieldCullingOperationType(
        1,
    );
}
#[repr(transparent)]
pub struct EFieldIntegerType(pub u8);
impl EFieldIntegerType {
    pub const INTEGER_DYNAMIC_STATE: EFieldIntegerType = EFieldIntegerType(0);
    pub const INTEGER_ACTIVATE_DISABLED: EFieldIntegerType = EFieldIntegerType(1);
    pub const INTEGER_COLLISION_GROUP: EFieldIntegerType = EFieldIntegerType(2);
    pub const INTEGER_POSITION_ANIMATED: EFieldIntegerType = EFieldIntegerType(3);
    pub const INTEGER_POSITION_STATIC: EFieldIntegerType = EFieldIntegerType(4);
    pub const INTEGER_TARGET_MAX: EFieldIntegerType = EFieldIntegerType(5);
}
#[repr(transparent)]
pub struct EFieldScalarType(pub u8);
impl EFieldScalarType {
    pub const SCALAR_EXTERNAL_CLUSTER_STRAIN: EFieldScalarType = EFieldScalarType(0);
    pub const SCALAR_KILL: EFieldScalarType = EFieldScalarType(1);
    pub const SCALAR_DISABLE_THRESHOLD: EFieldScalarType = EFieldScalarType(2);
    pub const SCALAR_SLEEPING_THRESHOLD: EFieldScalarType = EFieldScalarType(3);
    pub const SCALAR_INTERNAL_CLUSTER_STRAIN: EFieldScalarType = EFieldScalarType(4);
    pub const SCALAR_DYNAMIC_CONSTRAINT: EFieldScalarType = EFieldScalarType(5);
    pub const SCALAR_TARGET_MAX: EFieldScalarType = EFieldScalarType(6);
}
#[repr(transparent)]
pub struct EFieldVectorType(pub u8);
impl EFieldVectorType {
    pub const VECTOR_LINEAR_FORCE: EFieldVectorType = EFieldVectorType(0);
    pub const VECTOR_LINEAR_VELOCITY: EFieldVectorType = EFieldVectorType(1);
    pub const VECTOR_ANGULAR_VELOCITY: EFieldVectorType = EFieldVectorType(2);
    pub const VECTOR_ANGULAR_TORQUE: EFieldVectorType = EFieldVectorType(3);
    pub const VECTOR_POSITION_TARGET: EFieldVectorType = EFieldVectorType(4);
    pub const VECTOR_INITIAL_LINEAR_VELOCITY: EFieldVectorType = EFieldVectorType(5);
    pub const VECTOR_INITIAL_ANGULAR_VELOCITY: EFieldVectorType = EFieldVectorType(6);
    pub const VECTOR_LINEAR_IMPULSE: EFieldVectorType = EFieldVectorType(7);
    pub const VECTOR_TARGET_MAX: EFieldVectorType = EFieldVectorType(8);
}
#[repr(transparent)]
pub struct EObjectStateTypeEnum(pub u8);
impl EObjectStateTypeEnum {
    pub const CHAOS_NONE: EObjectStateTypeEnum = EObjectStateTypeEnum(0);
    pub const CHAOS_OBJECT_SLEEPING: EObjectStateTypeEnum = EObjectStateTypeEnum(1);
    pub const CHAOS_OBJECT_KINEMATIC: EObjectStateTypeEnum = EObjectStateTypeEnum(2);
    pub const CHAOS_OBJECT_STATIC: EObjectStateTypeEnum = EObjectStateTypeEnum(3);
    pub const CHAOS_OBJECT_DYNAMIC: EObjectStateTypeEnum = EObjectStateTypeEnum(4);
    pub const CHAOS_OBJECT_USER_DEFINED: EObjectStateTypeEnum = EObjectStateTypeEnum(
        100,
    );
}
#[repr(transparent)]
pub struct EInitialVelocityTypeEnum(pub u8);
impl EInitialVelocityTypeEnum {
    pub const CHAOS_INITIAL_VELOCITY_USER_DEFINED: EInitialVelocityTypeEnum = EInitialVelocityTypeEnum(
        0,
    );
    pub const CHAOS_INITIAL_VELOCITY_NONE: EInitialVelocityTypeEnum = EInitialVelocityTypeEnum(
        1,
    );
}
#[repr(transparent)]
pub struct EChaosSoftsSimulationSpace(pub u8);
impl EChaosSoftsSimulationSpace {
    pub const WORLD_SPACE: EChaosSoftsSimulationSpace = EChaosSoftsSimulationSpace(0);
    pub const COMPONENT_SPACE: EChaosSoftsSimulationSpace = EChaosSoftsSimulationSpace(
        1,
    );
    pub const REFERENCE_BONE_SPACE: EChaosSoftsSimulationSpace = EChaosSoftsSimulationSpace(
        2,
    );
}
#[repr(transparent)]
pub struct EConvexOverlapRemoval(pub i32);
impl EConvexOverlapRemoval {
    pub const NONE: EConvexOverlapRemoval = EConvexOverlapRemoval(0);
    pub const ALL: EConvexOverlapRemoval = EConvexOverlapRemoval(1);
    pub const ONLY_CLUSTERS: EConvexOverlapRemoval = EConvexOverlapRemoval(2);
    pub const ONLY_CLUSTERS_VS_CLUSTERS: EConvexOverlapRemoval = EConvexOverlapRemoval(
        3,
    );
}
#[repr(transparent)]
pub struct EProximityMethod(pub i32);
impl EProximityMethod {
    pub const PRECISE: EProximityMethod = EProximityMethod(0);
    pub const CONVEX_HULL: EProximityMethod = EProximityMethod(1);
}
#[repr(transparent)]
pub struct EConnectionContactMethod(pub u8);
impl EConnectionContactMethod {
    pub const NONE: EConnectionContactMethod = EConnectionContactMethod(0);
    pub const CONVEX_HULL_CONTACT_AREA: EConnectionContactMethod = EConnectionContactMethod(
        1,
    );
}
#[repr(transparent)]
pub struct EProximityContactMethod(pub u8);
impl EProximityContactMethod {
    pub const MIN_OVERLAP_IN_PROJECTION_TO_MAJOR_AXES: EProximityContactMethod = EProximityContactMethod(
        0,
    );
    pub const CONVEX_HULL_SHARP_CONTACT: EProximityContactMethod = EProximityContactMethod(
        1,
    );
    pub const CONVEX_HULL_AREA_CONTACT: EProximityContactMethod = EProximityContactMethod(
        2,
    );
}
