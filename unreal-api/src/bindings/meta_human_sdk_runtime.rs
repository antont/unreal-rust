#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMetaHumanCustomizableBodyPart {
    pub control_rig_class: TSubclassOf<UControlRig>,
    pub control_rig_lod_threshold: i32,
    pub physics_asset: UPtr<UPhysicsAsset>,
    pub rigid_body_lod_threshold: i32,
    pub component_name: FString,
}
pub struct UMetaHumanComponentBase {
    pub body_component_name: FString,
    pub body_type: EMetaHumanBodyType,
    pub b_enable_body_correctives: bool,
    pub face_component_name: FString,
    pub rig_logic_lod_threshold: i32,
    pub b_enable_neck_correctives: bool,
    pub neck_correctives_lod_threshold: i32,
    pub b_enable_neck_proc_control_rig: bool,
    pub neck_proc_control_rig_lod_threshold: i32,
    pub torso: FMetaHumanCustomizableBodyPart,
    pub legs: FMetaHumanCustomizableBodyPart,
    pub feet: FMetaHumanCustomizableBodyPart,
}
pub struct UMetaHumanComponentUE {
    pub post_process_anim_bp: TSoftObjectPtr<UClass>,
}
