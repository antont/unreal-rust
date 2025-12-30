#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UK2Node_CameraRigBase {
    pub camera_rig: UPtr<UCameraRigAsset>,
}
pub struct UK2Node_SingleCameraRigParameterBase {
    pub camera_parameter_name: FString,
    pub camera_parameter_type: EK2Node_CameraParameterType,
    pub blendable_camera_parameter_type: ECameraVariableType,
    pub blendable_struct_type: UPtr<UScriptStruct>,
    pub data_camera_parameter_type: ECameraContextDataType,
    pub data_camera_parameter_container_type: ECameraContextDataContainerType,
    pub data_camera_parameter_type_object: UPtr<UObject>,
}
pub struct UK2Node_GetCameraRigParameter {}
pub struct UK2Node_MultiCameraRigParametersBase {
    pub blendable_parameter_pin_names: TArray<FName>,
    pub data_parameter_pin_names: TArray<FName>,
}
pub struct UK2Node_GetCameraRigParameters {}
pub struct UK2Node_SetCameraRigParameter {}
pub struct UK2Node_SetCameraRigParameters {}
