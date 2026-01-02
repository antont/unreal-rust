#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FToolMenuContext {
    __padding_end: [u8; 104],
}
impl FToolMenuContext {}
#[repr(C, align(8))]
pub struct FToolDynamicUIAction {
    pub execute_action: FToolDynamicUIAction_ExecuteAction,
    pub can_execute_action: FToolDynamicUIAction_CanExecuteAction,
    pub get_action_check_state: FToolDynamicUIAction_GetActionCheckState,
    pub is_action_visible_delegate: FToolDynamicUIAction_IsActionVisibleDelegate,
}
impl FToolDynamicUIAction {}
#[repr(C, align(8))]
pub struct FToolMenuStringCommand {
    pub ty: EToolMenuStringCommandType,
    pub custom_type: FName,
    pub string: FString,
}
impl FToolMenuStringCommand {}
#[repr(C, align(4))]
pub struct FToolMenuInsert {
    pub name: FName,
    pub position: EToolMenuInsertType,
    pub fallback: EToolMenuInsertFallback,
    __padding_end: [u8; 2],
}
impl FToolMenuInsert {}
#[repr(C, align(8))]
pub struct FToolMenuOwner {
    __padding_end: [u8; 16],
}
impl FToolMenuOwner {}
#[repr(C, align(8))]
pub struct FToolMenuEntry {
    pub name: FName,
    pub owner: FToolMenuOwner,
    pub ty: crate::bindings::slate::EMultiBlockType,
    pub user_interface_action_type: crate::bindings::slate::EUserInterfaceActionType,
    pub tutorial_highlight_name: FName,
    pub insert_position: FToolMenuInsert,
    pub b_should_close_window_after_menu_selection: bool,
    pub script_object: UPtr<UToolMenuEntryScript>,
    pub style_name_override: FName,
    __padding_end: [u8; 1916],
}
impl FToolMenuEntry {}
#[repr(C, align(4))]
pub struct FScriptSlateIcon {
    pub style_set_name: FName,
    pub style_name: FName,
    pub small_style_name: FName,
}
impl FScriptSlateIcon {}
#[repr(C, align(4))]
pub struct FToolMenuEntryScriptDataAdvanced {
    pub tutorial_highlight: FName,
    pub entry_type: crate::bindings::slate::EMultiBlockType,
    pub user_interface_action_type: crate::bindings::slate::EUserInterfaceActionType,
    pub style_name_override: FName,
    pub b_is_sub_menu: bool,
    pub b_open_sub_menu_on_click: bool,
    pub b_should_close_window_after_menu_selection: bool,
    pub b_simple_combo_box: bool,
}
impl FToolMenuEntryScriptDataAdvanced {}
#[repr(C, align(8))]
pub struct FToolMenuEntryScriptData {
    pub menu: FName,
    pub section: FName,
    pub name: FName,
    pub label: FText,
    pub tool_tip: FText,
    pub icon: FScriptSlateIcon,
    pub owner_name: FName,
    pub insert_position: FToolMenuInsert,
    pub advanced: FToolMenuEntryScriptDataAdvanced,
}
impl FToolMenuEntryScriptData {}
#[repr(C, align(8))]
pub struct FToolMenuSection {
    pub name: FName,
    pub owner: FToolMenuOwner,
    pub blocks: TArray<FToolMenuEntry>,
    pub insert_position: FToolMenuInsert,
    pub context: FToolMenuContext,
    pub tool_menu_section_dynamic: UPtr<UToolMenuSectionDynamic>,
    __padding_end: [u8; 400],
}
impl FToolMenuSection {}
#[repr(C, align(8))]
pub struct UToolMenu {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub menu_name: FName,
    pub menu_parent: FName,
    pub style_name: FName,
    pub tutorial_highlight_name: FName,
    pub menu_type: crate::bindings::slate::EMultiBoxType,
    #[doc(hidden)]
    __padding_98: [u8; 1],
    pub b_should_close_window_after_menu_selection: bool,
    pub b_close_self_only: bool,
    pub b_searchable: bool,
    pub b_tool_bar_is_focusable: bool,
    pub b_separate_sections: bool,
    pub b_allow_tool_bar_wrap_button: TOptional<bool>,
    pub b_tool_bar_force_small_icons: bool,
    pub b_prevent_customization: bool,
    pub menu_owner: FToolMenuOwner,
    __padding_end: [u8; 192],
}
impl UToolMenu {}
#[repr(C, align(8))]
pub struct UToolMenuContextBase {
    __padding_end: [u8; 48],
}
impl UToolMenuContextBase {}
#[repr(C, align(8))]
pub struct USlateTabManagerContext {
    __padding_end: [u8; 64],
}
impl USlateTabManagerContext {}
#[repr(C, align(8))]
pub struct UToolMenuEntryScript {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub data: FToolMenuEntryScriptData,
    __padding_end: [u8; 8],
}
impl UToolMenuEntryScript {}
#[repr(C, align(8))]
pub struct UToolMenuProfileContext {
    __padding_end: [u8; 64],
}
impl UToolMenuProfileContext {}
#[repr(C, align(8))]
pub struct UToolMenus {
    __padding_end: [u8; 928],
}
impl UToolMenus {}
#[repr(C, align(8))]
pub struct UToolMenuContextExtensions {
    __padding_end: [u8; 48],
}
impl UToolMenuContextExtensions {}
#[repr(C, align(8))]
pub struct UToolMenuEntryExtensions {
    __padding_end: [u8; 48],
}
impl UToolMenuEntryExtensions {}
#[repr(C, align(8))]
pub struct UToolMenuSectionExtensions {
    __padding_end: [u8; 48],
}
impl UToolMenuSectionExtensions {}
#[repr(C, align(8))]
pub struct UToolMenuSectionDynamic {
    __padding_end: [u8; 48],
}
impl UToolMenuSectionDynamic {}
#[repr(C, align(8))]
pub struct UToolMenuWidgetCollectionContext {
    __padding_end: [u8; 64],
}
impl UToolMenuWidgetCollectionContext {}
#[repr(C, align(8))]
pub struct FToolDynamicUIAction_ExecuteAction {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FToolDynamicUIAction_CanExecuteAction {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FToolDynamicUIAction_GetActionCheckState {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FToolDynamicUIAction_IsActionVisibleDelegate {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EToolMenuStringCommandType(pub u8);
impl EToolMenuStringCommandType {
    pub const COMMAND: EToolMenuStringCommandType = EToolMenuStringCommandType(0);
    pub const PYTHON: EToolMenuStringCommandType = EToolMenuStringCommandType(1);
    pub const CUSTOM: EToolMenuStringCommandType = EToolMenuStringCommandType(2);
}
#[repr(transparent)]
pub struct EToolMenuInsertType(pub u8);
impl EToolMenuInsertType {
    pub const DEFAULT: EToolMenuInsertType = EToolMenuInsertType(0);
    pub const BEFORE: EToolMenuInsertType = EToolMenuInsertType(1);
    pub const AFTER: EToolMenuInsertType = EToolMenuInsertType(2);
    pub const FIRST: EToolMenuInsertType = EToolMenuInsertType(3);
    pub const LAST: EToolMenuInsertType = EToolMenuInsertType(4);
}
#[repr(transparent)]
pub struct EToolMenuInsertFallback(pub u8);
impl EToolMenuInsertFallback {
    pub const NONE: EToolMenuInsertFallback = EToolMenuInsertFallback(0);
    pub const INSERT: EToolMenuInsertFallback = EToolMenuInsertFallback(1);
    pub const LOG: EToolMenuInsertFallback = EToolMenuInsertFallback(2);
}
#[repr(transparent)]
pub struct EToolMenuSectionAlign(pub u8);
impl EToolMenuSectionAlign {
    pub const DEFAULT: EToolMenuSectionAlign = EToolMenuSectionAlign(0);
    pub const FIRST: EToolMenuSectionAlign = EToolMenuSectionAlign(1);
    pub const MIDDLE: EToolMenuSectionAlign = EToolMenuSectionAlign(2);
    pub const LAST: EToolMenuSectionAlign = EToolMenuSectionAlign(3);
}
