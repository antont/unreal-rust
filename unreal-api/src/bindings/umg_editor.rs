#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAssetThumbnailWidgetSettings {
    pub b_force_generic_thumbnail: bool,
    pub b_allow_hint_text: bool,
    pub b_allow_real_time_on_hovered: bool,
    pub b_allow_asset_specific_thumbnail_overlay: bool,
    pub thumbnail_label: EThumbnailLabelType_BlueprintType,
    pub highlighted_text_delegate: FAssetThumbnailWidgetSettings_HighlightedTextDelegate,
    pub hint_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    pub b_override_asset_type_color: bool,
    pub asset_type_color_override: crate::bindings::core_u_object::FLinearColor,
    pub padding: crate::bindings::slate_core::FMargin,
    pub generic_thumbnail_size: i32,
    pub color_strip_orientation: EThumbnailColorStripOrientation_BlueprintType,
    __padding_end: [u8; 7],
}
impl FAssetThumbnailWidgetSettings {}
#[repr(C, align(8))]
pub struct UWidgetBlueprint {
    __padding_end: [u8; 1624],
}
impl UWidgetBlueprint {}
#[repr(C, align(8))]
pub struct UWidgetEditingProjectSettings {
    __padding_end: [u8; 688],
}
impl UWidgetEditingProjectSettings {}
#[repr(C, align(8))]
pub struct UUIComponentWidgetPair {
    __padding_end: [u8; 72],
}
impl UUIComponentWidgetPair {}
#[repr(C, align(8))]
pub struct UWidgetBlueprintToolMenuContext {
    __padding_end: [u8; 64],
}
impl UWidgetBlueprintToolMenuContext {}
#[repr(C, align(8))]
pub struct UWidgetPaletteFavorites {
    __padding_end: [u8; 96],
}
impl UWidgetPaletteFavorites {}
#[repr(C, align(8))]
pub struct UAssetDefinition_WidgetBlueprint {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_WidgetBlueprint {}
#[repr(C, align(8))]
pub struct UAssetDefinition_WidgetBlueprintGeneratedClass {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_WidgetBlueprintGeneratedClass {}
#[repr(C, align(8))]
pub struct UAssetThumbnailWidget {
    #[doc(hidden)]
    __padding_848: [u8; 848],
    pub resolution: crate::bindings::core_u_object::FIntPoint,
    pub thumbnail_settings: FAssetThumbnailWidgetSettings,
    __padding_end: [u8; 32],
}
impl UAssetThumbnailWidget {}
#[repr(C, align(8))]
pub struct UK2Node_WidgetAnimationEvent {
    __padding_end: [u8; 440],
}
impl UK2Node_WidgetAnimationEvent {}
#[repr(C, align(8))]
pub struct UK2Node_CreateDragDropOperation {
    __padding_end: [u8; 232],
}
impl UK2Node_CreateDragDropOperation {}
#[repr(C, align(8))]
pub struct UK2Node_CreateWidget {
    __padding_end: [u8; 232],
}
impl UK2Node_CreateWidget {}
#[repr(C, align(8))]
pub struct UK2Node_PlayAnimation {
    __padding_end: [u8; 240],
}
impl UK2Node_PlayAnimation {}
#[repr(C, align(8))]
pub struct UK2Node_PlayAnimationTimeRange {
    __padding_end: [u8; 240],
}
impl UK2Node_PlayAnimationTimeRange {}
#[repr(C, align(8))]
pub struct UK2Node_PlayAnimation2 {
    __padding_end: [u8; 240],
}
impl UK2Node_PlayAnimation2 {}
#[repr(C, align(8))]
pub struct UK2Node_PlayAnimationTimeRange2 {
    __padding_end: [u8; 240],
}
impl UK2Node_PlayAnimationTimeRange2 {}
#[repr(C, align(8))]
pub struct UUMGEditorProjectSettings {
    __padding_end: [u8; 688],
}
impl UUMGEditorProjectSettings {}
#[repr(C, align(8))]
pub struct UWidgetDesignerSettings {
    __padding_end: [u8; 144],
}
impl UWidgetDesignerSettings {}
#[repr(C, align(8))]
pub struct USlateVectorArtDataFactory {
    __padding_end: [u8; 136],
}
impl USlateVectorArtDataFactory {}
#[repr(C, align(8))]
pub struct UWidgetEditorModeUISubsystem {
    __padding_end: [u8; 56],
}
impl UWidgetEditorModeUISubsystem {}
#[repr(C, align(8))]
pub struct UWidgetBlueprintExtension {
    __padding_end: [u8; 48],
}
impl UWidgetBlueprintExtension {}
#[repr(C, align(8))]
pub struct UUIComponentWidgetBlueprintExtension {
    __padding_end: [u8; 64],
}
impl UUIComponentWidgetBlueprintExtension {}
#[repr(C, align(8))]
pub struct UWidgetSlotPair {
    __padding_end: [u8; 96],
}
impl UWidgetSlotPair {}
#[repr(C, align(8))]
pub struct UWidgetBlueprintFactory {
    __padding_end: [u8; 160],
}
impl UWidgetBlueprintFactory {}
#[repr(C, align(8))]
pub struct UWidgetBlueprintThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UWidgetBlueprintThumbnailRenderer {}
#[repr(C, align(8))]
pub struct UWidgetCompilerRule {
    __padding_end: [u8; 48],
}
impl UWidgetCompilerRule {}
#[repr(C, align(8))]
pub struct UWidgetGraphSchema {
    __padding_end: [u8; 152],
}
impl UWidgetGraphSchema {}
#[repr(C, align(8))]
pub struct FAssetThumbnailWidgetSettings_HighlightedTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetThumbnailSettings_HighlightedTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAssetThumbnailWidget_HighlightedTextDelegate {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EThumbnailLabelType_BlueprintType(pub u8);
impl EThumbnailLabelType_BlueprintType {
    pub const CLASS_NAME: EThumbnailLabelType_BlueprintType = EThumbnailLabelType_BlueprintType(
        0,
    );
    pub const ASSET_NAME: EThumbnailLabelType_BlueprintType = EThumbnailLabelType_BlueprintType(
        1,
    );
    pub const NO_LABEL: EThumbnailLabelType_BlueprintType = EThumbnailLabelType_BlueprintType(
        2,
    );
}
#[repr(transparent)]
pub struct EThumbnailColorStripOrientation_BlueprintType(pub u8);
impl EThumbnailColorStripOrientation_BlueprintType {
    pub const HORIZONTAL_BOTTOM_EDGE: EThumbnailColorStripOrientation_BlueprintType = EThumbnailColorStripOrientation_BlueprintType(
        0,
    );
    pub const VERTICAL_RIGHT_EDGE: EThumbnailColorStripOrientation_BlueprintType = EThumbnailColorStripOrientation_BlueprintType(
        1,
    );
}
#[repr(transparent)]
pub struct EPropertyBindingPermissionLevel(pub u8);
impl EPropertyBindingPermissionLevel {
    pub const ALLOW: EPropertyBindingPermissionLevel = EPropertyBindingPermissionLevel(
        0,
    );
    pub const PREVENT: EPropertyBindingPermissionLevel = EPropertyBindingPermissionLevel(
        1,
    );
    pub const PREVENT_AND_WARN: EPropertyBindingPermissionLevel = EPropertyBindingPermissionLevel(
        2,
    );
    pub const PREVENT_AND_ERROR: EPropertyBindingPermissionLevel = EPropertyBindingPermissionLevel(
        3,
    );
}
#[repr(transparent)]
pub struct EWidgetCompileTimeTickPrediction(pub u8);
impl EWidgetCompileTimeTickPrediction {
    pub const WONT_TICK: EWidgetCompileTimeTickPrediction = EWidgetCompileTimeTickPrediction(
        0,
    );
    pub const ON_DEMAND: EWidgetCompileTimeTickPrediction = EWidgetCompileTimeTickPrediction(
        1,
    );
    pub const WILL_TICK: EWidgetCompileTimeTickPrediction = EWidgetCompileTimeTickPrediction(
        2,
    );
}
#[repr(transparent)]
pub struct EThumbnailPreviewSizeMode(pub u8);
impl EThumbnailPreviewSizeMode {
    pub const MATCH_DESIGNER_MODE: EThumbnailPreviewSizeMode = EThumbnailPreviewSizeMode(
        0,
    );
    pub const FILL_SCREEN: EThumbnailPreviewSizeMode = EThumbnailPreviewSizeMode(1);
    pub const CUSTOM: EThumbnailPreviewSizeMode = EThumbnailPreviewSizeMode(2);
    pub const DESIRED: EThumbnailPreviewSizeMode = EThumbnailPreviewSizeMode(3);
}
#[repr(transparent)]
pub struct EDisplayOnCompile(pub u8);
impl EDisplayOnCompile {
    pub const DO_C_ERRORS_OR_WARNINGS: EDisplayOnCompile = EDisplayOnCompile(0);
    pub const DO_C_ERRORS_ONLY: EDisplayOnCompile = EDisplayOnCompile(1);
    pub const DO_C_WARNINGS_ONLY: EDisplayOnCompile = EDisplayOnCompile(2);
    pub const DO_C_NEVER: EDisplayOnCompile = EDisplayOnCompile(3);
}
