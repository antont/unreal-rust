#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UEditorFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UEditorFunctionLibrary {}
#[repr(C, align(8))]
pub struct UEditorUtilityToolMenuEntry {
    __padding_end: [u8; 240],
}
impl UEditorUtilityToolMenuEntry {}
#[repr(C, align(8))]
pub struct UEditorUtilityToolMenuSection {
    __padding_end: [u8; 64],
}
impl UEditorUtilityToolMenuSection {}
pub struct IEditorUtilityExtension {}
#[repr(C, align(8))]
pub struct UEditorUtilityExtension {
    __padding_end: [u8; 48],
}
impl UEditorUtilityExtension {}
#[repr(C, align(8))]
pub struct UEditorUtilityObject {
    __padding_end: [u8; 64],
}
impl UEditorUtilityObject {}
#[repr(C, align(8))]
pub struct UActorActionUtility {
    __padding_end: [u8; 88],
}
impl UActorActionUtility {}
#[repr(C, align(8))]
pub struct UAssetActionUtility {
    __padding_end: [u8; 112],
}
impl UAssetActionUtility {}
#[repr(C, align(8))]
pub struct UAsyncCaptureScene {
    __padding_end: [u8; 104],
}
impl UAsyncCaptureScene {}
#[repr(C, align(8))]
pub struct UAsyncImageExport {
    __padding_end: [u8; 112],
}
impl UAsyncImageExport {}
#[repr(C, align(8))]
pub struct UAsyncRegisterAndExecuteTask {
    __padding_end: [u8; 80],
}
impl UAsyncRegisterAndExecuteTask {}
#[repr(C, align(8))]
pub struct AEditorUtilityActor {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub b_receives_editor_input: bool,
    __padding_end: [u8; 7],
}
impl AEditorUtilityActor {}
#[repr(C, align(8))]
pub struct UEditorUtilityActorComponent {
    __padding_end: [u8; 240],
}
impl UEditorUtilityActorComponent {}
#[repr(C, align(8))]
pub struct UEditorUtilityBlueprint {
    __padding_end: [u8; 1432],
}
impl UEditorUtilityBlueprint {}
#[repr(C, align(8))]
pub struct UEditorUtilityBlueprintFactory {
    __padding_end: [u8; 160],
}
impl UEditorUtilityBlueprintFactory {}
#[repr(C, align(16))]
pub struct AEditorUtilityCamera {
    __padding_end: [u8; 3136],
}
impl AEditorUtilityCamera {}
#[repr(C, align(8))]
pub struct UEditorUtilityBlueprintAsyncActionBase {
    __padding_end: [u8; 56],
}
impl UEditorUtilityBlueprintAsyncActionBase {}
#[repr(C, align(8))]
pub struct UAsyncEditorDelay {
    __padding_end: [u8; 96],
}
impl UAsyncEditorDelay {}
#[repr(C, align(8))]
pub struct UAsyncEditorWaitForGameWorld {
    __padding_end: [u8; 88],
}
impl UAsyncEditorWaitForGameWorld {}
#[repr(C, align(8))]
pub struct UAsyncEditorOpenMapAndFocusActor {
    __padding_end: [u8; 136],
}
impl UAsyncEditorOpenMapAndFocusActor {}
#[repr(C, align(8))]
pub struct UEditorUtilityLibrary {
    __padding_end: [u8; 48],
}
impl UEditorUtilityLibrary {}
#[repr(C, align(16))]
pub struct UEditorUtilitySubsystem {
    __padding_end: [u8; 688],
}
impl UEditorUtilitySubsystem {}
#[repr(C, align(8))]
pub struct UEditorUtilityTask {
    __padding_end: [u8; 128],
}
impl UEditorUtilityTask {}
#[repr(C, align(8))]
pub struct UEditorUtilityWidget {
    #[doc(hidden)]
    __padding_1296: [u8; 1296],
    pub tab_display_name: FText,
    pub help_text: FString,
    #[doc(hidden)]
    __padding_1329: [u8; 1],
    pub b_auto_run_default_action: bool,
    __padding_end: [u8; 6],
}
impl UEditorUtilityWidget {}
#[repr(C, align(8))]
pub struct UEditorUtilityWidgetBlueprint {
    __padding_end: [u8; 1672],
}
impl UEditorUtilityWidgetBlueprint {}
#[repr(C, align(8))]
pub struct UEditorUtilityWidgetBlueprintFactory {
    __padding_end: [u8; 160],
}
impl UEditorUtilityWidgetBlueprintFactory {}
#[repr(C, align(16))]
pub struct UEditorUtilityButton {
    __padding_end: [u8; 2160],
}
impl UEditorUtilityButton {}
#[repr(C, align(16))]
pub struct UEditorUtilityCheckBox {
    __padding_end: [u8; 3664],
}
impl UEditorUtilityCheckBox {}
#[repr(C, align(16))]
pub struct UEditorUtilityCircularThrobber {
    __padding_end: [u8; 960],
}
impl UEditorUtilityCircularThrobber {}
#[repr(C, align(16))]
pub struct UEditorUtilityComboBoxKey {
    __padding_end: [u8; 8000],
}
impl UEditorUtilityComboBoxKey {}
#[repr(C, align(16))]
pub struct UEditorUtilityComboBoxString {
    __padding_end: [u8; 8128],
}
impl UEditorUtilityComboBoxString {}
#[repr(C, align(16))]
pub struct UEditorUtilityEditableText {
    __padding_end: [u8; 1696],
}
impl UEditorUtilityEditableText {}
#[repr(C, align(16))]
pub struct UEditorUtilityEditableTextBox {
    __padding_end: [u8; 4768],
}
impl UEditorUtilityEditableTextBox {}
#[repr(C, align(16))]
pub struct UEditorUtilityExpandableArea {
    __padding_end: [u8; 1488],
}
impl UEditorUtilityExpandableArea {}
#[repr(C, align(16))]
pub struct UEditorUtilityInputKeySelector {
    __padding_end: [u8; 2816],
}
impl UEditorUtilityInputKeySelector {}
#[repr(C, align(16))]
pub struct UEditorUtilityListView {
    __padding_end: [u8; 5088],
}
impl UEditorUtilityListView {}
#[repr(C, align(16))]
pub struct UEditorUtilityMultiLineEditableText {
    __padding_end: [u8; 1744],
}
impl UEditorUtilityMultiLineEditableText {}
#[repr(C, align(16))]
pub struct UEditorUtilityMultiLineEditableTextBox {
    __padding_end: [u8; 5584],
}
impl UEditorUtilityMultiLineEditableTextBox {}
#[repr(C, align(16))]
pub struct UEditorUtilityProgressBar {
    __padding_end: [u8; 1504],
}
impl UEditorUtilityProgressBar {}
#[repr(C, align(16))]
pub struct UEditorUtilityScrollBar {
    __padding_end: [u8; 2672],
}
impl UEditorUtilityScrollBar {}
#[repr(C, align(16))]
pub struct UEditorUtilityScrollBox {
    __padding_end: [u8; 3760],
}
impl UEditorUtilityScrollBox {}
#[repr(C, align(16))]
pub struct UEditorUtilitySlider {
    __padding_end: [u8; 2224],
}
impl UEditorUtilitySlider {}
#[repr(C, align(16))]
pub struct UEditorUtilitySpinBox {
    __padding_end: [u8; 2592],
}
impl UEditorUtilitySpinBox {}
#[repr(C, align(16))]
pub struct UEditorUtilityThrobber {
    __padding_end: [u8; 928],
}
impl UEditorUtilityThrobber {}
#[repr(C, align(16))]
pub struct UEditorUtilityTreeView {
    __padding_end: [u8; 5216],
}
impl UEditorUtilityTreeView {}
#[repr(C, align(8))]
pub struct UEditorUtilityWidgetProjectSettings {
    __padding_end: [u8; 1176],
}
impl UEditorUtilityWidgetProjectSettings {}
#[repr(C, align(8))]
pub struct UDEPRECATED_GlobalEditorUtilityBase {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub help_text: FString,
    #[doc(hidden)]
    __padding_65: [u8; 1],
    pub b_auto_run_default_action: bool,
    __padding_end: [u8; 54],
}
impl UDEPRECATED_GlobalEditorUtilityBase {}
#[repr(C, align(8))]
pub struct ADEPRECATED_PlacedEditorUtilityBase {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub help_text: FString,
}
impl ADEPRECATED_PlacedEditorUtilityBase {}
#[repr(C, align(8))]
pub struct UToolMenuWidget {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub menu_name: FString,
    pub menu_type: crate::bindings::slate::EMultiBoxType,
    #[doc(hidden)]
    __padding_736: [u8; 20],
    pub full_menu_name: FName,
    __padding_end: [u8; 4],
}
impl UToolMenuWidget {}
#[repr(C, align(8))]
pub struct FAsyncCaptureScene_Complete {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncImageExport_Complete {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncRegisterAndExecuteTask_OnFinished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncEditorDelay_Complete {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncEditorWaitForGameWorld_Complete {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncEditorOpenMapAndFocusActor_Complete {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorUtilitySubsystem_OnBeginPIE {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorUtilitySubsystem_OnEndPIE {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGlobalEditorUtilityBase_OnEachSelectedActor {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGlobalEditorUtilityBase_OnEachSelectedAsset {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ECastToWidgetBlueprintCases(pub u8);
impl ECastToWidgetBlueprintCases {
    pub const CAST_SUCCEEDED: ECastToWidgetBlueprintCases = ECastToWidgetBlueprintCases(
        0,
    );
    pub const CAST_FAILED: ECastToWidgetBlueprintCases = ECastToWidgetBlueprintCases(1);
}
