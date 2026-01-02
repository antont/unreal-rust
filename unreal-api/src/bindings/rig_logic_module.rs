#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FCoordinateSystem {
    pub x_axis: EDirection,
    pub y_axis: EDirection,
    pub z_axis: EDirection,
}
impl FCoordinateSystem {}
#[repr(C, align(4))]
pub struct FMeshBlendShapeChannelMapping {
    pub mesh_index: i32,
    pub blend_shape_channel_index: i32,
}
impl FMeshBlendShapeChannelMapping {}
#[repr(C, align(4))]
pub struct FTextureCoordinate {
    pub u: f32,
    pub v: f32,
}
impl FTextureCoordinate {}
#[repr(C, align(4))]
pub struct FVertexLayout {
    pub position: i32,
    pub texture_coordinate: i32,
    pub normal: i32,
}
impl FVertexLayout {}
#[repr(C, align(8))]
pub struct FAnimNode_RigLogic {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub anim_sequence: crate::bindings::engine::FPoseLink,
    pub cache_anim_curve_names: bool,
    __padding_end: [u8; 79],
}
impl FAnimNode_RigLogic {}
#[repr(C, align(4))]
pub struct FRigLogicConfiguration {
    pub calculation_type: ERigLogicCalculationType,
    pub load_joints: bool,
    pub load_blend_shapes: bool,
    pub load_animated_maps: bool,
    pub load_machine_learned_behavior: bool,
    pub load_rbf_behavior: bool,
    pub load_twist_swing_behavior: bool,
    pub translation_type: ERigLogicTranslationType,
    pub rotation_type: ERigLogicRotationType,
    pub rotation_order: ERigLogicRotationOrder,
    pub scale_type: ERigLogicScaleType,
    pub translation_pruning_threshold: f32,
    pub rotation_pruning_threshold: f32,
    pub scale_pruning_threshold: f32,
}
impl FRigLogicConfiguration {}
#[repr(C, align(8))]
pub struct FRigUnit_RigLogic {
    __padding_end: [u8; 192],
}
impl FRigUnit_RigLogic {}
#[repr(C, align(8))]
pub struct UDEPRECATED_DNAIndexMapping {
    __padding_end: [u8; 48],
}
impl UDEPRECATED_DNAIndexMapping {}
#[repr(C, align(8))]
pub struct UDNAAsset {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub meta_data: TMap<FString, FString>,
    pub rig_logic_configuration: FRigLogicConfiguration,
    __padding_end: [u8; 152],
}
impl UDNAAsset {}
#[repr(C, align(8))]
pub struct USkelMeshDNAUtils {
    __padding_end: [u8; 48],
}
impl USkelMeshDNAUtils {}
#[repr(transparent)]
pub struct EDirection(pub u8);
impl EDirection {
    pub const LEFT: EDirection = EDirection(0);
    pub const RIGHT: EDirection = EDirection(1);
    pub const UP: EDirection = EDirection(2);
    pub const DOWN: EDirection = EDirection(3);
    pub const FRONT: EDirection = EDirection(4);
    pub const BACK: EDirection = EDirection(5);
}
#[repr(transparent)]
pub struct ERigLogicCalculationType(pub u8);
impl ERigLogicCalculationType {
    pub const SCALAR: ERigLogicCalculationType = ERigLogicCalculationType(0);
    pub const SSE: ERigLogicCalculationType = ERigLogicCalculationType(1);
    pub const AVX: ERigLogicCalculationType = ERigLogicCalculationType(2);
    pub const NEON: ERigLogicCalculationType = ERigLogicCalculationType(3);
    pub const ANY_VECTOR: ERigLogicCalculationType = ERigLogicCalculationType(4);
}
#[repr(transparent)]
pub struct ERigLogicTranslationType(pub u8);
impl ERigLogicTranslationType {
    pub const NONE: ERigLogicTranslationType = ERigLogicTranslationType(0);
    pub const VECTOR: ERigLogicTranslationType = ERigLogicTranslationType(3);
}
#[repr(transparent)]
pub struct ERigLogicRotationType(pub u8);
impl ERigLogicRotationType {
    pub const NONE: ERigLogicRotationType = ERigLogicRotationType(0);
    pub const EULER_ANGLES: ERigLogicRotationType = ERigLogicRotationType(3);
    pub const QUATERNIONS: ERigLogicRotationType = ERigLogicRotationType(4);
}
#[repr(transparent)]
pub struct ERigLogicRotationOrder(pub u8);
impl ERigLogicRotationOrder {
    pub const XYZ: ERigLogicRotationOrder = ERigLogicRotationOrder(0);
    pub const XZY: ERigLogicRotationOrder = ERigLogicRotationOrder(1);
    pub const YXZ: ERigLogicRotationOrder = ERigLogicRotationOrder(2);
    pub const YZX: ERigLogicRotationOrder = ERigLogicRotationOrder(3);
    pub const ZXY: ERigLogicRotationOrder = ERigLogicRotationOrder(4);
    pub const ZYX: ERigLogicRotationOrder = ERigLogicRotationOrder(5);
}
#[repr(transparent)]
pub struct ERigLogicScaleType(pub u8);
impl ERigLogicScaleType {
    pub const NONE: ERigLogicScaleType = ERigLogicScaleType(0);
    pub const VECTOR: ERigLogicScaleType = ERigLogicScaleType(3);
}
