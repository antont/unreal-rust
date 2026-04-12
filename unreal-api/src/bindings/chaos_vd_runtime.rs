#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct EChaosVDRecordingMode(pub u8);
impl EChaosVDRecordingMode {
    pub const INVALID: EChaosVDRecordingMode = EChaosVDRecordingMode(0);
    pub const LIVE: EChaosVDRecordingMode = EChaosVDRecordingMode(1);
    pub const FILE: EChaosVDRecordingMode = EChaosVDRecordingMode(2);
}
#[repr(transparent)]
pub struct EChaosVDTransportMode(pub u8);
impl EChaosVDTransportMode {
    pub const INVALID: EChaosVDTransportMode = EChaosVDTransportMode(0);
    pub const FILE_SYSTEM: EChaosVDTransportMode = EChaosVDTransportMode(1);
    pub const TRACE_SERVER: EChaosVDTransportMode = EChaosVDTransportMode(2);
    pub const DIRECT: EChaosVDTransportMode = EChaosVDTransportMode(3);
    pub const RELAY: EChaosVDTransportMode = EChaosVDTransportMode(4);
}
#[repr(transparent)]
pub struct EChaosVDAccelerationStructureType(pub u32);
impl EChaosVDAccelerationStructureType {
    pub const BOUNDING_VOLUME: EChaosVDAccelerationStructureType = EChaosVDAccelerationStructureType(
        0,
    );
    pub const AABB_TREE: EChaosVDAccelerationStructureType = EChaosVDAccelerationStructureType(
        1,
    );
    pub const AABB_TREE_BV: EChaosVDAccelerationStructureType = EChaosVDAccelerationStructureType(
        2,
    );
    pub const COLLECTION: EChaosVDAccelerationStructureType = EChaosVDAccelerationStructureType(
        3,
    );
    pub const UNKNOWN: EChaosVDAccelerationStructureType = EChaosVDAccelerationStructureType(
        4,
    );
}
#[repr(transparent)]
pub struct EChaosVDContactPointType(pub i8);
impl EChaosVDContactPointType {
    pub const UNKNOWN: EChaosVDContactPointType = EChaosVDContactPointType(0);
    pub const VERTEX_PLANE: EChaosVDContactPointType = EChaosVDContactPointType(1);
    pub const EDGE_EDGE: EChaosVDContactPointType = EChaosVDContactPointType(2);
    pub const PLANE_VERTEX: EChaosVDContactPointType = EChaosVDContactPointType(3);
    pub const VERTEX_VERTEX: EChaosVDContactPointType = EChaosVDContactPointType(4);
}
#[repr(transparent)]
pub struct EChaosVDContactShapesType(pub i32);
impl EChaosVDContactShapesType {
    pub const UNKNOWN: EChaosVDContactShapesType = EChaosVDContactShapesType(0);
    pub const SPHERE_SPHERE: EChaosVDContactShapesType = EChaosVDContactShapesType(1);
    pub const SPHERE_CAPSULE: EChaosVDContactShapesType = EChaosVDContactShapesType(2);
    pub const SPHERE_BOX: EChaosVDContactShapesType = EChaosVDContactShapesType(3);
    pub const SPHERE_CONVEX: EChaosVDContactShapesType = EChaosVDContactShapesType(4);
    pub const SPHERE_TRI_MESH: EChaosVDContactShapesType = EChaosVDContactShapesType(5);
    pub const SPHERE_HEIGHT_FIELD: EChaosVDContactShapesType = EChaosVDContactShapesType(
        6,
    );
    pub const SPHERE_PLANE: EChaosVDContactShapesType = EChaosVDContactShapesType(7);
    pub const CAPSULE_CAPSULE: EChaosVDContactShapesType = EChaosVDContactShapesType(8);
    pub const CAPSULE_BOX: EChaosVDContactShapesType = EChaosVDContactShapesType(9);
    pub const CAPSULE_CONVEX: EChaosVDContactShapesType = EChaosVDContactShapesType(10);
    pub const CAPSULE_TRI_MESH: EChaosVDContactShapesType = EChaosVDContactShapesType(
        11,
    );
    pub const CAPSULE_HEIGHT_FIELD: EChaosVDContactShapesType = EChaosVDContactShapesType(
        12,
    );
    pub const BOX_BOX: EChaosVDContactShapesType = EChaosVDContactShapesType(13);
    pub const BOX_CONVEX: EChaosVDContactShapesType = EChaosVDContactShapesType(14);
    pub const BOX_TRI_MESH: EChaosVDContactShapesType = EChaosVDContactShapesType(15);
    pub const BOX_HEIGHT_FIELD: EChaosVDContactShapesType = EChaosVDContactShapesType(
        16,
    );
    pub const BOX_PLANE: EChaosVDContactShapesType = EChaosVDContactShapesType(17);
    pub const CONVEX_CONVEX: EChaosVDContactShapesType = EChaosVDContactShapesType(18);
    pub const CONVEX_TRI_MESH: EChaosVDContactShapesType = EChaosVDContactShapesType(19);
    pub const CONVEX_HEIGHT_FIELD: EChaosVDContactShapesType = EChaosVDContactShapesType(
        20,
    );
    pub const GENERIC_CONVEX_CONVEX: EChaosVDContactShapesType = EChaosVDContactShapesType(
        21,
    );
    pub const LEVEL_SET_LEVEL_SET: EChaosVDContactShapesType = EChaosVDContactShapesType(
        22,
    );
    pub const NUM_SHAPES_TYPES: EChaosVDContactShapesType = EChaosVDContactShapesType(
        23,
    );
}
#[repr(transparent)]
pub struct EChaosVDMidPhaseType(pub i8);
impl EChaosVDMidPhaseType {
    pub const GENERIC: EChaosVDMidPhaseType = EChaosVDMidPhaseType(0);
    pub const SHAPE_PAIR: EChaosVDMidPhaseType = EChaosVDMidPhaseType(1);
    pub const SPHERE_APPROXIMATION: EChaosVDMidPhaseType = EChaosVDMidPhaseType(2);
    pub const UNKNOWN: EChaosVDMidPhaseType = EChaosVDMidPhaseType(3);
}
#[repr(transparent)]
pub struct EChaosVDCollisionTraceFlag(pub i32);
impl EChaosVDCollisionTraceFlag {
    pub const USE_DEFAULT: EChaosVDCollisionTraceFlag = EChaosVDCollisionTraceFlag(0);
    pub const USE_SIMPLE_AND_COMPLEX: EChaosVDCollisionTraceFlag = EChaosVDCollisionTraceFlag(
        1,
    );
    pub const USE_SIMPLE_AS_COMPLEX: EChaosVDCollisionTraceFlag = EChaosVDCollisionTraceFlag(
        2,
    );
    pub const USE_COMPLEX_AS_SIMPLE: EChaosVDCollisionTraceFlag = EChaosVDCollisionTraceFlag(
        3,
    );
    pub const MAX: EChaosVDCollisionTraceFlag = EChaosVDCollisionTraceFlag(4);
}
#[repr(transparent)]
pub struct EChaosVDParticleContext(pub i32);
impl EChaosVDParticleContext {
    pub const INVALID: EChaosVDParticleContext = EChaosVDParticleContext(0);
    pub const GAME_THREAD: EChaosVDParticleContext = EChaosVDParticleContext(1);
    pub const PHYSICS_THREAD: EChaosVDParticleContext = EChaosVDParticleContext(2);
}
#[repr(transparent)]
pub struct EChaosVDJointReSimType(pub i32);
impl EChaosVDJointReSimType {
    pub const FULL_RESIM: EChaosVDJointReSimType = EChaosVDJointReSimType(0);
    pub const RESIM_AS_FOLLOWER: EChaosVDJointReSimType = EChaosVDJointReSimType(1);
}
#[repr(transparent)]
pub struct EChaosVDJointSyncType(pub i32);
impl EChaosVDJointSyncType {
    pub const IN_SYNC: EChaosVDJointSyncType = EChaosVDJointSyncType(0);
    pub const HARD_DESYNC: EChaosVDJointSyncType = EChaosVDJointSyncType(1);
}
#[repr(transparent)]
pub struct EChaosVDJointMotionType(pub i32);
impl EChaosVDJointMotionType {
    pub const FREE: EChaosVDJointMotionType = EChaosVDJointMotionType(0);
    pub const LIMITED: EChaosVDJointMotionType = EChaosVDJointMotionType(1);
    pub const LOCKED: EChaosVDJointMotionType = EChaosVDJointMotionType(2);
}
#[repr(transparent)]
pub struct EChaosVDJointForceMode(pub i32);
impl EChaosVDJointForceMode {
    pub const ACCELERATION: EChaosVDJointForceMode = EChaosVDJointForceMode(0);
    pub const FORCE: EChaosVDJointForceMode = EChaosVDJointForceMode(1);
}
#[repr(transparent)]
pub struct EChaosVDPlasticityType(pub i32);
impl EChaosVDPlasticityType {
    pub const FREE: EChaosVDPlasticityType = EChaosVDPlasticityType(0);
    pub const SHRINK: EChaosVDPlasticityType = EChaosVDPlasticityType(1);
    pub const GROW: EChaosVDPlasticityType = EChaosVDPlasticityType(2);
}
#[repr(transparent)]
pub struct EChaosVDObjectStateType(pub i8);
impl EChaosVDObjectStateType {
    pub const UNINITIALIZED: EChaosVDObjectStateType = EChaosVDObjectStateType(0);
    pub const SLEEPING: EChaosVDObjectStateType = EChaosVDObjectStateType(1);
    pub const KINEMATIC: EChaosVDObjectStateType = EChaosVDObjectStateType(2);
    pub const STATIC: EChaosVDObjectStateType = EChaosVDObjectStateType(3);
    pub const DYNAMIC: EChaosVDObjectStateType = EChaosVDObjectStateType(4);
    pub const COUNT: EChaosVDObjectStateType = EChaosVDObjectStateType(5);
}
#[repr(transparent)]
pub struct EChaosVDSleepType(pub u8);
impl EChaosVDSleepType {
    pub const MATERIAL_SLEEP: EChaosVDSleepType = EChaosVDSleepType(0);
    pub const NEVER_SLEEP: EChaosVDSleepType = EChaosVDSleepType(1);
}
#[repr(transparent)]
pub struct EChaosVDKinematicTargetMode(pub i32);
impl EChaosVDKinematicTargetMode {
    pub const NONE: EChaosVDKinematicTargetMode = EChaosVDKinematicTargetMode(0);
    pub const RESET: EChaosVDKinematicTargetMode = EChaosVDKinematicTargetMode(1);
    pub const POSITION: EChaosVDKinematicTargetMode = EChaosVDKinematicTargetMode(2);
    pub const VELOCITY: EChaosVDKinematicTargetMode = EChaosVDKinematicTargetMode(3);
}
#[repr(transparent)]
pub struct EChaosVDParticleType(pub u8);
impl EChaosVDParticleType {
    pub const STATIC: EChaosVDParticleType = EChaosVDParticleType(0);
    pub const KINEMATIC: EChaosVDParticleType = EChaosVDParticleType(1);
    pub const RIGID: EChaosVDParticleType = EChaosVDParticleType(2);
    pub const CLUSTERED: EChaosVDParticleType = EChaosVDParticleType(3);
    pub const STATIC_MESH: EChaosVDParticleType = EChaosVDParticleType(4);
    pub const SKELETAL_MESH: EChaosVDParticleType = EChaosVDParticleType(5);
    pub const GEOMETRY_COLLECTION: EChaosVDParticleType = EChaosVDParticleType(6);
    pub const UNKNOWN: EChaosVDParticleType = EChaosVDParticleType(7);
}
#[repr(transparent)]
pub struct EChaosVDSceneQueryVisitorType(pub i32);
impl EChaosVDSceneQueryVisitorType {
    pub const INVALID: EChaosVDSceneQueryVisitorType = EChaosVDSceneQueryVisitorType(0);
    pub const BROAD_PHASE: EChaosVDSceneQueryVisitorType = EChaosVDSceneQueryVisitorType(
        1,
    );
    pub const NARROW_PHASE: EChaosVDSceneQueryVisitorType = EChaosVDSceneQueryVisitorType(
        2,
    );
}
#[repr(transparent)]
pub struct EChaosVDCollisionQueryHitType(pub i32);
impl EChaosVDCollisionQueryHitType {
    pub const NONE: EChaosVDCollisionQueryHitType = EChaosVDCollisionQueryHitType(0);
    pub const TOUCH: EChaosVDCollisionQueryHitType = EChaosVDCollisionQueryHitType(1);
    pub const BLOCK: EChaosVDCollisionQueryHitType = EChaosVDCollisionQueryHitType(2);
}
#[repr(transparent)]
pub struct EChaosVDSQVisitRejectReason(pub i32);
impl EChaosVDSQVisitRejectReason {
    pub const NONE: EChaosVDSQVisitRejectReason = EChaosVDSQVisitRejectReason(0);
    pub const NO_HIT: EChaosVDSQVisitRejectReason = EChaosVDSQVisitRejectReason(1);
    pub const PRE_FILTER: EChaosVDSQVisitRejectReason = EChaosVDSQVisitRejectReason(2);
    pub const POST_FILTER: EChaosVDSQVisitRejectReason = EChaosVDSQVisitRejectReason(3);
    pub const COLOCATED_HIT_HAS_WORSE_NORMAL: EChaosVDSQVisitRejectReason = EChaosVDSQVisitRejectReason(
        4,
    );
    pub const FAILED_FAST_BOUND_TEST: EChaosVDSQVisitRejectReason = EChaosVDSQVisitRejectReason(
        5,
    );
}
#[repr(transparent)]
pub struct EChaosVDSceneQueryType(pub i32);
impl EChaosVDSceneQueryType {
    pub const INVALID: EChaosVDSceneQueryType = EChaosVDSceneQueryType(0);
    pub const SWEEP: EChaosVDSceneQueryType = EChaosVDSceneQueryType(1);
    pub const OVERLAP: EChaosVDSceneQueryType = EChaosVDSceneQueryType(2);
    pub const RAY_CAST: EChaosVDSceneQueryType = EChaosVDSceneQueryType(3);
}
#[repr(transparent)]
pub struct EChaosVDSceneQueryMode(pub i32);
impl EChaosVDSceneQueryMode {
    pub const SINGLE: EChaosVDSceneQueryMode = EChaosVDSceneQueryMode(0);
    pub const MULTI: EChaosVDSceneQueryMode = EChaosVDSceneQueryMode(1);
    pub const TEST: EChaosVDSceneQueryMode = EChaosVDSceneQueryMode(2);
    pub const INVALID: EChaosVDSceneQueryMode = EChaosVDSceneQueryMode(3);
}
