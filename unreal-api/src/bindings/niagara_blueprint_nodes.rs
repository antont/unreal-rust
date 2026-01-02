#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UK2Node_DataChannelBase {
    __padding_end: [u8; 456],
}
impl UK2Node_DataChannelBase {}
#[repr(C, align(8))]
pub struct UK2Node_WriteDataChannel_WithContext {
    __padding_end: [u8; 456],
}
impl UK2Node_WriteDataChannel_WithContext {}
#[repr(C, align(8))]
pub struct UK2Node_ReadDataChannel_WithContext {
    __padding_end: [u8; 456],
}
impl UK2Node_ReadDataChannel_WithContext {}
#[repr(C, align(8))]
pub struct UK2Node_DataChannelGetNum_WithContext {
    __padding_end: [u8; 456],
}
impl UK2Node_DataChannelGetNum_WithContext {}
#[repr(C, align(8))]
pub struct UK2Node_WriteDataChannel {
    __padding_end: [u8; 456],
}
impl UK2Node_WriteDataChannel {}
#[repr(C, align(8))]
pub struct UK2Node_WriteDataChannelSingle_WithContext {
    __padding_end: [u8; 456],
}
impl UK2Node_WriteDataChannelSingle_WithContext {}
#[repr(C, align(8))]
pub struct UK2Node_ReadDataChannel {
    __padding_end: [u8; 456],
}
impl UK2Node_ReadDataChannel {}
#[repr(C, align(8))]
pub struct UK2Node_ReadDataChannelSingle_WithContext {
    __padding_end: [u8; 456],
}
impl UK2Node_ReadDataChannelSingle_WithContext {}
#[repr(C, align(8))]
pub struct UK2Node_DataChannelAccessContextOperation {
    #[doc(hidden)]
    __padding_352: [u8; 352],
    pub context_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    __padding_end: [u8; 16],
}
impl UK2Node_DataChannelAccessContextOperation {}
#[repr(C, align(8))]
pub struct UK2Node_DataChannelAccessContext_Make {
    __padding_end: [u8; 376],
}
impl UK2Node_DataChannelAccessContext_Make {}
#[repr(C, align(8))]
pub struct UK2Node_DataChannelAccessContext_GetMembers {
    __padding_end: [u8; 376],
}
impl UK2Node_DataChannelAccessContext_GetMembers {}
#[repr(C, align(8))]
pub struct UK2Node_DataChannelAccessContext_SetMembers {
    __padding_end: [u8; 376],
}
impl UK2Node_DataChannelAccessContext_SetMembers {}
#[repr(C, align(8))]
pub struct UK2Node_DataChannelAccessContext_Prepare {
    __padding_end: [u8; 376],
}
impl UK2Node_DataChannelAccessContext_Prepare {}
