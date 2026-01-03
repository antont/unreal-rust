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
impl FAppleImageUtilsImageConversionResult {}
pub struct IAppleImageInterface {}
#[repr(C, align(8))]
pub struct UAppleImageInterface {
    __padding_end: [u8; 48],
}
impl UAppleImageInterface {}
#[repr(C, align(8))]
pub struct UAppleImageUtilsBaseAsyncTaskBlueprintProxy {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub conversion_result: FAppleImageUtilsImageConversionResult,
    __padding_end: [u8; 8],
}
impl UAppleImageUtilsBaseAsyncTaskBlueprintProxy {}
#[repr(C, align(8))]
pub struct FAppleImageUtilsBaseAsyncTaskBlueprintProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAppleImageUtilsBaseAsyncTaskBlueprintProxy_OnFailure {
    _opague: [u8; 24],
}
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
