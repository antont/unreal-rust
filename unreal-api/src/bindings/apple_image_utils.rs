#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAppleImageUtilsImageConversionResult {
    pub error: FString,
    pub image_data: TArray<u8>,
}
pub struct UAppleImageInterface {}
pub struct IAppleImageInterface {}
pub struct UAppleImageUtilsBaseAsyncTaskBlueprintProxy {
    pub on_success: FAppleImageUtilsBaseAsyncTaskBlueprintProxy_OnSuccess,
    pub on_failure: FAppleImageUtilsBaseAsyncTaskBlueprintProxy_OnFailure,
    pub conversion_result: FAppleImageUtilsImageConversionResult,
}
