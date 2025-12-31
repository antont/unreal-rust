#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FToolMenuContext {
    pub context_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
#[repr(C, align(8))]
pub struct FToolDynamicUIAction {
    pub execute_action: FToolDynamicUIAction_ExecuteAction,
    pub can_execute_action: FToolDynamicUIAction_CanExecuteAction,
    pub get_action_check_state: FToolDynamicUIAction_GetActionCheckState,
    pub is_action_visible_delegate: FToolDynamicUIAction_IsActionVisibleDelegate,
}
#[repr(C, align(8))]
pub struct FToolMenuStringCommand {
    pub ty: EToolMenuStringCommandType,
    pub custom_type: FName,
    pub string: FString,
}
#[repr(C, align(4))]
pub struct FToolMenuInsert {
    pub name: FName,
    pub position: EToolMenuInsertType,
    pub fallback: EToolMenuInsertFallback,
}
#[repr(C, align(8))]
pub struct FToolMenuOwner {}
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
    pub b_command_is_keybind_only: bool,
}
#[repr(C, align(4))]
pub struct FScriptSlateIcon {
    pub style_set_name: FName,
    pub style_name: FName,
    pub small_style_name: FName,
}
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
#[repr(C, align(8))]
pub struct FToolMenuProfileMap {
    pub menu_profiles: TMap<FName, crate::bindings::slate::FToolMenuProfile>,
}
#[repr(C, align(8))]
pub struct FToolMenuSection {
    pub name: FName,
    pub owner: FToolMenuOwner,
    pub blocks: TArray<FToolMenuEntry>,
    pub insert_position: FToolMenuInsert,
    pub context: FToolMenuContext,
    pub tool_menu_section_dynamic: UPtr<UToolMenuSectionDynamic>,
}
pub struct UToolMenu {
    pub menu_name: FName,
    pub menu_parent: FName,
    pub style_name: FName,
    pub tutorial_highlight_name: FName,
    pub menu_type: crate::bindings::slate::EMultiBoxType,
    pub b_should_cleanup_context_on_destroy: bool,
    pub b_should_close_window_after_menu_selection: bool,
    pub b_close_self_only: bool,
    pub b_searchable: bool,
    pub b_tool_bar_is_focusable: bool,
    pub b_separate_sections: bool,
    pub b_allow_tool_bar_wrap_button: TOptional<bool>,
    pub b_tool_bar_force_small_icons: bool,
    pub b_prevent_customization: bool,
    pub menu_owner: FToolMenuOwner,
    pub context: FToolMenuContext,
    pub sections: TArray<FToolMenuSection>,
    pub sub_menu_parent: UPtr<UToolMenu>,
    pub sub_menu_source_entry_name: FName,
}
pub struct UToolMenuContextBase {}
pub struct USlateTabManagerContext {}
pub struct UToolMenuEntryScript {
    pub data: FToolMenuEntryScriptData,
    pub b_has_registered_menu_entry: bool,
}
pub struct UToolMenuProfileContext {}
pub struct UToolMenus {
    pub customized_menus: TArray<crate::bindings::slate::FCustomizedToolMenu>,
    pub menu_substitutions_during_generate: TMap<FName, FName>,
    pub menus: TMap<FName, UPtr<UToolMenu>>,
    pub menu_profiles: TMap<FName, FToolMenuProfileMap>,
}
pub struct UToolMenuContextExtensions {}
pub struct UToolMenuEntryExtensions {}
pub struct UToolMenuSectionExtensions {}
pub struct UToolMenuSectionDynamic {}
pub struct UToolMenuWidgetCollectionContext {}
pub struct FToolDynamicUIAction_ExecuteAction;
pub struct FToolDynamicUIAction_CanExecuteAction;
pub struct FToolDynamicUIAction_GetActionCheckState;
pub struct FToolDynamicUIAction_IsActionVisibleDelegate;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EToolMenuStringCommandType(pub u8);
impl EToolMenuStringCommandType {
    pub const COMMAND: EToolMenuStringCommandType = EToolMenuStringCommandType(0);
    pub const PYTHON: EToolMenuStringCommandType = EToolMenuStringCommandType(1);
    pub const CUSTOM: EToolMenuStringCommandType = EToolMenuStringCommandType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EToolMenuInsertType(pub u8);
impl EToolMenuInsertType {
    pub const DEFAULT: EToolMenuInsertType = EToolMenuInsertType(0);
    pub const BEFORE: EToolMenuInsertType = EToolMenuInsertType(1);
    pub const AFTER: EToolMenuInsertType = EToolMenuInsertType(2);
    pub const FIRST: EToolMenuInsertType = EToolMenuInsertType(3);
    pub const LAST: EToolMenuInsertType = EToolMenuInsertType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EToolMenuInsertFallback(pub u8);
impl EToolMenuInsertFallback {
    pub const NONE: EToolMenuInsertFallback = EToolMenuInsertFallback(0);
    pub const INSERT: EToolMenuInsertFallback = EToolMenuInsertFallback(1);
    pub const LOG: EToolMenuInsertFallback = EToolMenuInsertFallback(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EToolMenuSectionAlign(pub u8);
impl EToolMenuSectionAlign {
    pub const DEFAULT: EToolMenuSectionAlign = EToolMenuSectionAlign(0);
    pub const FIRST: EToolMenuSectionAlign = EToolMenuSectionAlign(1);
    pub const MIDDLE: EToolMenuSectionAlign = EToolMenuSectionAlign(2);
    pub const LAST: EToolMenuSectionAlign = EToolMenuSectionAlign(3);
}
