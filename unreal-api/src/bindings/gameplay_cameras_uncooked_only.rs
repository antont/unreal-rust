#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UK2Node_CameraRigBase {
    __padding_end: [u8; 200],
}
impl UK2Node_CameraRigBase {}
#[repr(C, align(8))]
pub struct UK2Node_SingleCameraRigParameterBase {
    __padding_end: [u8; 248],
}
impl UK2Node_SingleCameraRigParameterBase {}
#[repr(C, align(8))]
pub struct UK2Node_GetCameraRigParameter {
    __padding_end: [u8; 248],
}
impl UK2Node_GetCameraRigParameter {}
#[repr(C, align(8))]
pub struct UK2Node_MultiCameraRigParametersBase {
    __padding_end: [u8; 232],
}
impl UK2Node_MultiCameraRigParametersBase {}
#[repr(C, align(8))]
pub struct UK2Node_GetCameraRigParameters {
    __padding_end: [u8; 232],
}
impl UK2Node_GetCameraRigParameters {}
#[repr(C, align(8))]
pub struct UK2Node_SetCameraRigParameter {
    __padding_end: [u8; 248],
}
impl UK2Node_SetCameraRigParameter {}
#[repr(C, align(8))]
pub struct UK2Node_SetCameraRigParameters {
    __padding_end: [u8; 232],
}
impl UK2Node_SetCameraRigParameters {}
#[repr(transparent)]
pub struct EK2Node_CameraParameterType(pub u8);
impl EK2Node_CameraParameterType {
    pub const UNKNOWN: EK2Node_CameraParameterType = EK2Node_CameraParameterType(0);
    pub const BLENDABLE: EK2Node_CameraParameterType = EK2Node_CameraParameterType(1);
    pub const DATA: EK2Node_CameraParameterType = EK2Node_CameraParameterType(2);
}
