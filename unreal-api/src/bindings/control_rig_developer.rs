#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAnimGraphNode_ControlRig {
    __padding_end: [u8; 2104],
}
impl UAnimGraphNode_ControlRig {}
pub struct UControlRigAssetInterface {}
pub struct IControlRigAssetInterface {}
#[repr(C, align(8))]
pub struct UControlRigBlueprint {
    #[doc(hidden)]
    __padding_4648: [u8; 4648],
    pub hierarchy: UPtr<crate::bindings::control_rig::URigHierarchy>,
    pub modular_rig_model: crate::bindings::control_rig::FModularRigModel,
    __padding_end: [u8; 288],
}
impl UControlRigBlueprint {}
#[repr(C, align(8))]
pub struct UControlRigSchema {
    __padding_end: [u8; 80],
}
impl UControlRigSchema {}
#[repr(C, align(8))]
pub struct UControlRigGraph {
    __padding_end: [u8; 592],
}
impl UControlRigGraph {}
#[repr(C, align(8))]
pub struct UControlRigGraphNode {
    __padding_end: [u8; 1152],
}
impl UControlRigGraphNode {}
#[repr(C, align(8))]
pub struct UControlRigGraphSchema {
    __padding_end: [u8; 168],
}
impl UControlRigGraphSchema {}
