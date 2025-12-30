#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(1))]
pub struct FVirtualKeyboardOptions {
    pub b_enable_autocorrect: bool,
}
#[repr(C, align(8))]
pub struct FInputChord {
    pub key: FKey,
    pub flags_32: u8,
}
#[repr(C, align(8))]
pub struct FAnchors {
    pub minimum: FVector2D,
    pub maximum: FVector2D,
}
#[repr(C, align(4))]
pub struct FCustomizedToolMenuEntry {
    pub visibility: ECustomizedToolMenuVisibility,
}
#[repr(C, align(4))]
pub struct FCustomizedToolMenuSection {
    pub visibility: ECustomizedToolMenuVisibility,
}
#[repr(C, align(8))]
pub struct FCustomizedToolMenuNameArray {
    pub names: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FToolMenuProfile {
    pub name: FName,
    pub entries: TMap<FName, FCustomizedToolMenuEntry>,
    pub sections: TMap<FName, FCustomizedToolMenuSection>,
    pub suppress_extenders: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FCustomizedToolMenu {
    pub entry_order: TMap<FName, FCustomizedToolMenuNameArray>,
    pub section_order: TArray<FName>,
}
#[repr(C, align(4))]
pub struct FInputPreprocessorRegistrationKey {
    pub ty: EInputPreProcessorType,
    pub priority: i32,
}
#[repr(C, align(2))]
pub struct FCharRange {
    pub first: u16,
    pub last: u16,
}
#[repr(C, align(8))]
pub struct FCharRangeList {
    pub ranges: TArray<FCharRange>,
}
pub struct UToolMenuBase {}
pub struct USlateSettings {
    pub b_explicit_canvas_child_z_order: bool,
}
pub struct UButtonWidgetStyle {
    pub button_style: FButtonStyle,
}
pub struct UCheckBoxWidgetStyle {
    pub check_box_style: FCheckBoxStyle,
}
pub struct UComboBoxWidgetStyle {
    pub combo_box_style: FComboBoxStyle,
}
pub struct UComboButtonWidgetStyle {
    pub combo_button_style: FComboButtonStyle,
}
pub struct UEditableTextBoxWidgetStyle {
    pub editable_text_box_style: FEditableTextBoxStyle,
}
pub struct UEditableTextWidgetStyle {
    pub editable_text_style: FEditableTextStyle,
}
pub struct UProgressWidgetStyle {
    pub progress_bar_style: FProgressBarStyle,
}
pub struct UScrollBarWidgetStyle {
    pub scroll_bar_style: FScrollBarStyle,
}
pub struct UScrollBoxWidgetStyle {
    pub scroll_box_style: FScrollBoxStyle,
}
pub struct USpinBoxWidgetStyle {
    pub spin_box_style: FSpinBoxStyle,
}
pub struct UTextBlockWidgetStyle {
    pub text_block_style: FTextBlockStyle,
}
