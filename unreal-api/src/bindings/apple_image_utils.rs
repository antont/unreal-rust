#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
pub struct FAppleImageUtilsBaseAsyncTaskBlueprintProxy_OnSuccess;
pub struct FAppleImageUtilsBaseAsyncTaskBlueprintProxy_OnFailure;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextureRotationDirection(pub u8);
impl ETextureRotationDirection {
    pub const NONE: ETextureRotationDirection = ETextureRotationDirection(0);
    pub const LEFT: ETextureRotationDirection = ETextureRotationDirection(1);
    pub const RIGHT: ETextureRotationDirection = ETextureRotationDirection(2);
    pub const DOWN: ETextureRotationDirection = ETextureRotationDirection(3);
    pub const LEFT_MIRRORED: ETextureRotationDirection = ETextureRotationDirection(4);
    pub const RIGHT_MIRRORED: ETextureRotationDirection = ETextureRotationDirection(5);
    pub const DOWN_MIRRORED: ETextureRotationDirection = ETextureRotationDirection(6);
    pub const UP_MIRRORED: ETextureRotationDirection = ETextureRotationDirection(7);
}
