#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub anim_sequence: FPoseLink,
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
    pub skel_mesh_component: TWeakObjectPtr<USkeletalMeshComponent>,
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
    pub asset_import_data: UPtr<UAssetImportData>,
    pub dna_file_name: FString,
    pub b_keep_dna_after_initialization: bool,
    pub meta_data: TMap<FString, FString>,
    pub rig_logic_configuration: FRigLogicConfiguration,
}
pub struct USkelMeshDNAUtils {}
