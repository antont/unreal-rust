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
#[repr(C, align(4))]
pub struct FMeshBlendShapeChannelMapping {
    pub mesh_index: i32,
    pub blend_shape_channel_index: i32,
}
#[repr(C, align(4))]
pub struct FTextureCoordinate {
    pub u: f32,
    pub v: f32,
}
#[repr(C, align(4))]
pub struct FVertexLayout {
    pub position: i32,
    pub texture_coordinate: i32,
    pub normal: i32,
}
#[repr(C, align(8))]
pub struct FAnimNode_RigLogic {
    pub anim_sequence: crate::bindings::engine::FPoseLink,
    pub cache_anim_curve_names: bool,
    pub lod_threshold: i32,
}
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
#[repr(C, align(4))]
pub struct FBoneIndexControlAttributeMapping {}
#[repr(C, align(8))]
pub struct FRigUnit_RigLogic_IntArray {
    pub values: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FRigUnit_RigLogic_Data {
    pub skel_mesh_component: TWeakObjectPtr<
        crate::bindings::engine::USkeletalMeshComponent,
    >,
    pub input_curve_indices: TArray<i32>,
    pub neural_net_mask_curve_indices: TArray<i32>,
    pub hierarchy_bone_indices: TArray<i32>,
    pub driver_joints_to_control_attributes_map: TArray<
        FBoneIndexControlAttributeMapping,
    >,
    pub morph_target_curve_indices: TArray<FRigUnit_RigLogic_IntArray>,
    pub blend_shape_indices: TArray<FRigUnit_RigLogic_IntArray>,
    pub curve_element_indices_for_anim_maps: TArray<FRigUnit_RigLogic_IntArray>,
    pub rig_logic_indices_for_anim_maps: TArray<FRigUnit_RigLogic_IntArray>,
    pub current_lod: u32,
}
#[repr(C, align(8))]
pub struct FRigUnit_RigLogic {
    pub data: FRigUnit_RigLogic_Data,
    pub b_is_initialized: bool,
}
pub struct UDEPRECATED_DNAIndexMapping {}
pub struct UDNAAsset {
    pub asset_import_data: UPtr<crate::bindings::engine::UAssetImportData>,
    pub dna_file_name: FString,
    pub b_keep_dna_after_initialization: bool,
    pub meta_data: TMap<FString, FString>,
    pub rig_logic_configuration: FRigLogicConfiguration,
}
pub struct USkelMeshDNAUtils {}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigLogicCalculationType(pub u8);
impl ERigLogicCalculationType {
    pub const SCALAR: ERigLogicCalculationType = ERigLogicCalculationType(0);
    pub const SSE: ERigLogicCalculationType = ERigLogicCalculationType(1);
    pub const AVX: ERigLogicCalculationType = ERigLogicCalculationType(2);
    pub const NEON: ERigLogicCalculationType = ERigLogicCalculationType(3);
    pub const ANY_VECTOR: ERigLogicCalculationType = ERigLogicCalculationType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigLogicTranslationType(pub u8);
impl ERigLogicTranslationType {
    pub const NONE: ERigLogicTranslationType = ERigLogicTranslationType(0);
    pub const VECTOR: ERigLogicTranslationType = ERigLogicTranslationType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigLogicRotationType(pub u8);
impl ERigLogicRotationType {
    pub const NONE: ERigLogicRotationType = ERigLogicRotationType(0);
    pub const EULER_ANGLES: ERigLogicRotationType = ERigLogicRotationType(3);
    pub const QUATERNIONS: ERigLogicRotationType = ERigLogicRotationType(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERigLogicScaleType(pub u8);
impl ERigLogicScaleType {
    pub const NONE: ERigLogicScaleType = ERigLogicScaleType(0);
    pub const VECTOR: ERigLogicScaleType = ERigLogicScaleType(3);
}
