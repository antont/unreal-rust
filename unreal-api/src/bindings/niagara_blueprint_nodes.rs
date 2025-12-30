#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UK2Node_DataChannelBase {
    pub ignored_variables: TSet<FGuid>,
    pub data_channel_version: FGuid,
    pub data_channel: UPtr<UNiagaraDataChannelAsset>,
}
pub struct UK2Node_WriteDataChannel_WithContext {}
pub struct UK2Node_ReadDataChannel_WithContext {}
pub struct UK2Node_DataChannelGetNum_WithContext {}
pub struct UK2Node_WriteDataChannel {}
pub struct UK2Node_WriteDataChannelSingle_WithContext {}
pub struct UK2Node_ReadDataChannel {}
pub struct UK2Node_ReadDataChannelSingle_WithContext {}
pub struct UK2Node_DataChannelAccessContextOperation {
    pub show_pin_for_properties: TArray<FOptionalPinFromProperty>,
    pub context_struct: UPtr<UScriptStruct>,
}
pub struct UK2Node_DataChannelAccessContext_Make {}
pub struct UK2Node_DataChannelAccessContext_GetMembers {}
pub struct UK2Node_DataChannelAccessContext_SetMembers {}
pub struct UK2Node_DataChannelAccessContext_Prepare {}
