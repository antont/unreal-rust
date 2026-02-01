#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_tool_menu_remove_menu_entry_object: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_init_menu: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_add_sub_menu_script: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_add_section_script: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_add_menu_entry_object: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_add_menu_entry: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_add_dynamic_section_script: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_script_unregister_menu_entry: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_script_show_in_toolbar_top_level: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_script_register_menu_entry: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_script_is_visible: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_script_init_entry: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_script_get_tool_tip: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_script_get_label: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_script_get_icon: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_script_get_check_state: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_script_execute: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_script_construct_menu_entry: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_script_can_execute: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_unregister_owner_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_set_section_position: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_set_section_label: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_remove_section: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_remove_menu_entry_object: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_remove_menu: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_remove_entry: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_register_menu: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_refresh_menu_widget: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_refresh_all_widgets: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_is_menu_registered: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_get: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_find_menu: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_find_context: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_extend_menu: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menus_add_menu_entry_object: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_context_extensions_find_by_class: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_context_extensions_debug_log_context_object_classes: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_set_tool_tip: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_set_string_command: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_set_label: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_set_icon: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_make_tool_menu_owner: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_make_string_command: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_make_script_slate_icon: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_init_menu_entry: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_get_tool_tip: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_get_label: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_break_tool_menu_owner: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_break_string_command: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_entry_extensions_break_script_slate_icon: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_section_extensions_set_label: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_section_extensions_get_label: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_section_extensions_add_entry_object: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_section_extensions_add_entry: *mut crate::ffi::UFunctionOpague,
    pub u_tool_menu_section_dynamic_construct_sections: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_tool_menu_remove_menu_entry_object: std::ptr::null_mut(),
            u_tool_menu_init_menu: std::ptr::null_mut(),
            u_tool_menu_add_sub_menu_script: std::ptr::null_mut(),
            u_tool_menu_add_section_script: std::ptr::null_mut(),
            u_tool_menu_add_menu_entry_object: std::ptr::null_mut(),
            u_tool_menu_add_menu_entry: std::ptr::null_mut(),
            u_tool_menu_add_dynamic_section_script: std::ptr::null_mut(),
            u_tool_menu_entry_script_unregister_menu_entry: std::ptr::null_mut(),
            u_tool_menu_entry_script_show_in_toolbar_top_level: std::ptr::null_mut(),
            u_tool_menu_entry_script_register_menu_entry: std::ptr::null_mut(),
            u_tool_menu_entry_script_is_visible: std::ptr::null_mut(),
            u_tool_menu_entry_script_init_entry: std::ptr::null_mut(),
            u_tool_menu_entry_script_get_tool_tip: std::ptr::null_mut(),
            u_tool_menu_entry_script_get_label: std::ptr::null_mut(),
            u_tool_menu_entry_script_get_icon: std::ptr::null_mut(),
            u_tool_menu_entry_script_get_check_state: std::ptr::null_mut(),
            u_tool_menu_entry_script_execute: std::ptr::null_mut(),
            u_tool_menu_entry_script_construct_menu_entry: std::ptr::null_mut(),
            u_tool_menu_entry_script_can_execute: std::ptr::null_mut(),
            u_tool_menus_unregister_owner_by_name: std::ptr::null_mut(),
            u_tool_menus_set_section_position: std::ptr::null_mut(),
            u_tool_menus_set_section_label: std::ptr::null_mut(),
            u_tool_menus_remove_section: std::ptr::null_mut(),
            u_tool_menus_remove_menu_entry_object: std::ptr::null_mut(),
            u_tool_menus_remove_menu: std::ptr::null_mut(),
            u_tool_menus_remove_entry: std::ptr::null_mut(),
            u_tool_menus_register_menu: std::ptr::null_mut(),
            u_tool_menus_refresh_menu_widget: std::ptr::null_mut(),
            u_tool_menus_refresh_all_widgets: std::ptr::null_mut(),
            u_tool_menus_is_menu_registered: std::ptr::null_mut(),
            u_tool_menus_get: std::ptr::null_mut(),
            u_tool_menus_find_menu: std::ptr::null_mut(),
            u_tool_menus_find_context: std::ptr::null_mut(),
            u_tool_menus_extend_menu: std::ptr::null_mut(),
            u_tool_menus_add_menu_entry_object: std::ptr::null_mut(),
            u_tool_menu_context_extensions_find_by_class: std::ptr::null_mut(),
            u_tool_menu_context_extensions_debug_log_context_object_classes: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_set_tool_tip: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_set_string_command: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_set_label: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_set_icon: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_make_tool_menu_owner: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_make_string_command: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_make_script_slate_icon: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_init_menu_entry: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_get_tool_tip: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_get_label: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_break_tool_menu_owner: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_break_string_command: std::ptr::null_mut(),
            u_tool_menu_entry_extensions_break_script_slate_icon: std::ptr::null_mut(),
            u_tool_menu_section_extensions_set_label: std::ptr::null_mut(),
            u_tool_menu_section_extensions_get_label: std::ptr::null_mut(),
            u_tool_menu_section_extensions_add_entry_object: std::ptr::null_mut(),
            u_tool_menu_section_extensions_add_entry: std::ptr::null_mut(),
            u_tool_menu_section_dynamic_construct_sections: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UToolMenu::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveMenuEntryObject"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_remove_menu_entry_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InitMenu"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_init_menu,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSubMenuScript"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_add_sub_menu_script,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSectionScript"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_add_section_script,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddMenuEntryObject"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_add_menu_entry_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddMenuEntry"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_add_menu_entry,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddDynamicSectionScript"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_add_dynamic_section_script,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UToolMenuEntryScript::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnregisterMenuEntry"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_script_unregister_menu_entry,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShowInToolbarTopLevel"),
                &raw mut __FUNCTION_PTRS
                    .u_tool_menu_entry_script_show_in_toolbar_top_level,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterMenuEntry"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_script_register_menu_entry,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsVisible"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_script_is_visible,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InitEntry"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_script_init_entry,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetToolTip"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_script_get_tool_tip,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLabel"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_script_get_label,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetIcon"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_script_get_icon,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCheckState"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_script_get_check_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Execute"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_script_execute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConstructMenuEntry"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_script_construct_menu_entry,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanExecute"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_script_can_execute,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UToolMenus::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnregisterOwnerByName"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_unregister_owner_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSectionPosition"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_set_section_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSectionLabel"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_set_section_label,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveSection"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_remove_section,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveMenuEntryObject"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_remove_menu_entry_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveMenu"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_remove_menu,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveEntry"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_remove_entry,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterMenu"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_register_menu,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RefreshMenuWidget"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_refresh_menu_widget,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RefreshAllWidgets"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_refresh_all_widgets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsMenuRegistered"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_is_menu_registered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Get"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_get,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindMenu"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_find_menu,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindContext"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_find_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExtendMenu"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_extend_menu,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddMenuEntryObject"),
                &raw mut __FUNCTION_PTRS.u_tool_menus_add_menu_entry_object,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UToolMenuContextExtensions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindByClass"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_context_extensions_find_by_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DebugLogContextObjectClasses"),
                &raw mut __FUNCTION_PTRS
                    .u_tool_menu_context_extensions_debug_log_context_object_classes,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UToolMenuEntryExtensions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetToolTip"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_extensions_set_tool_tip,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetStringCommand"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_extensions_set_string_command,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLabel"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_extensions_set_label,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetIcon"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_extensions_set_icon,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeToolMenuOwner"),
                &raw mut __FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_make_tool_menu_owner,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeStringCommand"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_extensions_make_string_command,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeScriptSlateIcon"),
                &raw mut __FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_make_script_slate_icon,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InitMenuEntry"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_extensions_init_menu_entry,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetToolTip"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_extensions_get_tool_tip,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLabel"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_entry_extensions_get_label,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BreakToolMenuOwner"),
                &raw mut __FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_break_tool_menu_owner,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BreakStringCommand"),
                &raw mut __FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_break_string_command,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BreakScriptSlateIcon"),
                &raw mut __FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_break_script_slate_icon,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UToolMenuSectionExtensions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLabel"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_section_extensions_set_label,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLabel"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_section_extensions_get_label,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddEntryObject"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_section_extensions_add_entry_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddEntry"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_section_extensions_add_entry,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UToolMenuSectionDynamic::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConstructSections"),
                &raw mut __FUNCTION_PTRS.u_tool_menu_section_dynamic_construct_sections,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FToolMenuContext {
    pub(crate) __padding_end: [u8; 104],
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
}
impl FToolMenuInsert {}
#[repr(C, align(8))]
pub struct FToolMenuOwner {
    pub(crate) __padding_end: [u8; 16],
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
    pub(crate) __padding_end: [u8; 1916],
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
    pub(crate) __padding_end: [u8; 400],
}
impl FToolMenuSection {}
#[repr(C, align(8))]
pub struct UToolMenu {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub menu_name: FName,
    pub menu_parent: FName,
    pub style_name: FName,
    pub tutorial_highlight_name: FName,
    pub menu_type: crate::bindings::slate::EMultiBoxType,
    #[doc(hidden)]
    pub(crate) __padding_98: [u8; 1],
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
impl UToolMenu {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenu")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UToolMenu").copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn remove_menu_entry_object(&mut self, in_object: UPtr<UToolMenuEntryScript>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_remove_menu_entry_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object,
                __buffer.add(0).cast::<UPtr<UToolMenuEntryScript>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_remove_menu_entry_object,
                __buffer,
            )
        };
    }
    pub fn init_menu(
        &mut self,
        owner: FToolMenuOwner,
        name: FName,
        parent: FName,
        ty: crate::bindings::slate::EMultiBoxType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menu_init_menu,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner,
                __buffer.add(0).cast::<FToolMenuOwner>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(16).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&parent, __buffer.add(28).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ty,
                __buffer.add(40).cast::<crate::bindings::slate::EMultiBoxType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menu_init_menu,
                __buffer,
            )
        };
    }
    pub fn add_sub_menu(
        &mut self,
        owner: FName,
        section_name: FName,
        name: FName,
        label: &FText,
        tool_tip: &FText,
    ) -> UPtr<UToolMenu> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_add_sub_menu_script,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&owner, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(24).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(label, __buffer.add(40).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(tool_tip, __buffer.add(56).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_add_sub_menu_script,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<UPtr<UToolMenu>>().read() }
    }
    pub fn add_section(
        &mut self,
        section_name: FName,
        label: &FText,
        insert_name: FName,
        insert_type: EToolMenuInsertType,
        alignment: EToolMenuSectionAlign,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<46>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_add_section_script,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(label, __buffer.add(16).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &insert_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &insert_type,
                __buffer.add(44).cast::<EToolMenuInsertType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &alignment,
                __buffer.add(45).cast::<EToolMenuSectionAlign>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_add_section_script,
                __buffer,
            )
        };
    }
    pub fn add_menu_entry_object(&mut self, in_object: UPtr<UToolMenuEntryScript>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_add_menu_entry_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object,
                __buffer.add(0).cast::<UPtr<UToolMenuEntryScript>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_add_menu_entry_object,
                __buffer,
            )
        };
    }
    pub fn add_menu_entry(&mut self, section_name: FName, args: &FToolMenuEntry) {
        let mut __stack = crate::core_data::StackAlloc::<2024>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menu_add_menu_entry,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                args,
                __buffer.add(16).cast::<FToolMenuEntry>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menu_add_menu_entry,
                __buffer,
            )
        };
    }
    pub fn add_dynamic_section(
        &mut self,
        section_name: FName,
        object: UPtr<UToolMenuSectionDynamic>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_add_dynamic_section_script,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(16).cast::<UPtr<UToolMenuSectionDynamic>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_add_dynamic_section_script,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UToolMenuContextBase {
    __padding_end: [u8; 48],
}
impl UToolMenuContextBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuContextBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuContextBase")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USlateTabManagerContext {
    __padding_end: [u8; 64],
}
impl USlateTabManagerContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateTabManagerContext")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateTabManagerContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UToolMenuEntryScript {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub data: FToolMenuEntryScriptData,
    __padding_end: [u8; 8],
}
impl UToolMenuEntryScript {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuEntryScript")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuEntryScript")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn unregister_menu_entry(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_unregister_menu_entry,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_unregister_menu_entry,
                __buffer,
            )
        };
    }
    pub fn show_in_toolbar_top_level(&self, context: &FToolMenuContext) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_show_in_toolbar_top_level,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FToolMenuContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_show_in_toolbar_top_level,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<bool>().read() }
    }
    pub fn register_menu_entry(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_register_menu_entry,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_register_menu_entry,
                __buffer,
            )
        };
    }
    pub fn is_visible(&self, context: &FToolMenuContext) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_is_visible,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FToolMenuContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_is_visible,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<bool>().read() }
    }
    pub fn init_entry(
        &mut self,
        owner_name: FName,
        menu: FName,
        section: FName,
        name: FName,
        label: &FText,
        tool_tip: &FText,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_init_entry,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&menu, __buffer.add(12).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&section, __buffer.add(24).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(36).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(label, __buffer.add(48).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(tool_tip, __buffer.add(64).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_init_entry,
                __buffer,
            )
        };
    }
    pub fn get_tool_tip(&self, context: &FToolMenuContext) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_get_tool_tip,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FToolMenuContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_get_tool_tip,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<FText>().read() }
    }
    pub fn get_label(&self, context: &FToolMenuContext) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_get_label,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FToolMenuContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_get_label,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<FText>().read() }
    }
    pub fn get_icon(&self, context: &FToolMenuContext) -> FScriptSlateIcon {
        let mut __stack = crate::core_data::StackAlloc::<140>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_get_icon,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FToolMenuContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_get_icon,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<FScriptSlateIcon>().read() }
    }
    pub fn get_check_state(
        &self,
        context: &FToolMenuContext,
    ) -> crate::bindings::slate_core::ECheckBoxState {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_get_check_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FToolMenuContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_get_check_state,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(104)
                .cast::<crate::bindings::slate_core::ECheckBoxState>()
                .read()
        }
    }
    pub fn execute(&mut self, context: &FToolMenuContext) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_execute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FToolMenuContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_execute,
                __buffer,
            )
        };
    }
    pub fn construct_menu_entry(
        &mut self,
        menu: UPtr<UToolMenu>,
        section_name: FName,
        context: &FToolMenuContext,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_construct_menu_entry,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &menu,
                __buffer.add(0).cast::<UPtr<UToolMenu>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(24).cast::<FToolMenuContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_construct_menu_entry,
                __buffer,
            )
        };
    }
    pub fn can_execute(&self, context: &FToolMenuContext) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_can_execute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FToolMenuContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_script_can_execute,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UToolMenuProfileContext {
    __padding_end: [u8; 64],
}
impl UToolMenuProfileContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuProfileContext")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuProfileContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UToolMenus {
    __padding_end: [u8; 928],
}
impl UToolMenus {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenus")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenus")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn unregister_owner_by_name(&mut self, in_owner_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_unregister_owner_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_owner_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_unregister_owner_by_name,
                __buffer,
            )
        };
    }
    pub fn set_section_position(
        &mut self,
        menu_name: FName,
        section_name: FName,
        other_section_name: FName,
        position_type: EToolMenuInsertType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<37>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_set_section_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &menu_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &other_section_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position_type,
                __buffer.add(36).cast::<EToolMenuInsertType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_set_section_position,
                __buffer,
            )
        };
    }
    pub fn set_section_label(
        &mut self,
        menu_name: FName,
        section_name: FName,
        label: FText,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_set_section_label,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &menu_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&label, __buffer.add(24).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_set_section_label,
                __buffer,
            )
        };
    }
    pub fn remove_section(&mut self, menu_name: FName, section: FName) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_remove_section,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &menu_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&section, __buffer.add(12).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_remove_section,
                __buffer,
            )
        };
    }
    pub fn remove_menu_entry_object(
        menu_entry_object: UPtr<UToolMenuEntryScript>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_remove_menu_entry_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &menu_entry_object,
                __buffer.add(0).cast::<UPtr<UToolMenuEntryScript>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenus::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_remove_menu_entry_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn remove_menu(&mut self, menu_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_remove_menu,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &menu_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_remove_menu,
                __buffer,
            )
        };
    }
    pub fn remove_entry(&mut self, menu_name: FName, section: FName, name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_remove_entry,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &menu_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&section, __buffer.add(12).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(24).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_remove_entry,
                __buffer,
            )
        };
    }
    pub fn register_menu(
        &mut self,
        name: FName,
        parent: FName,
        ty: crate::bindings::slate::EMultiBoxType,
        b_warn_if_already_registered: bool,
    ) -> UPtr<UToolMenu> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_register_menu,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&parent, __buffer.add(12).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ty,
                __buffer.add(24).cast::<crate::bindings::slate::EMultiBoxType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_warn_if_already_registered,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_register_menu,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UToolMenu>>().read() }
    }
    pub fn refresh_menu_widget(&mut self, name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_refresh_menu_widget,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_refresh_menu_widget,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn refresh_all_widgets(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_refresh_all_widgets,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_refresh_all_widgets,
                __buffer,
            )
        };
    }
    pub fn is_menu_registered(&self, name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_is_menu_registered,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_is_menu_registered,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get() -> UPtr<UToolMenus> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_get,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::tool_menus::UToolMenus::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_get,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UToolMenus>>().read() }
    }
    pub fn find_menu(&mut self, name: FName) -> UPtr<UToolMenu> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_find_menu,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_find_menu,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UToolMenu>>().read() }
    }
    pub fn find_context(
        in_context: &FToolMenuContext,
        in_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_find_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_context,
                __buffer.add(0).cast::<FToolMenuContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer
                    .add(104)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenus::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_find_context,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(112)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn extend_menu(&mut self, name: FName) -> UPtr<UToolMenu> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_extend_menu,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS.u_tool_menus_extend_menu,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UToolMenu>>().read() }
    }
    pub fn add_menu_entry_object(menu_entry_object: UPtr<UToolMenuEntryScript>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_add_menu_entry_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &menu_entry_object,
                __buffer.add(0).cast::<UPtr<UToolMenuEntryScript>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenus::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menus_add_menu_entry_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UToolMenuContextExtensions {
    __padding_end: [u8; 48],
}
impl UToolMenuContextExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuContextExtensions")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuContextExtensions")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn find_by_class(
        context: &FToolMenuContext,
        in_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_context_extensions_find_by_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FToolMenuContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer
                    .add(104)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuContextExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_context_extensions_find_by_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(112)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn debug_log_context_object_classes(context: &FToolMenuContext) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_context_extensions_debug_log_context_object_classes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FToolMenuContext>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuContextExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_context_extensions_debug_log_context_object_classes,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UToolMenuEntryExtensions {
    __padding_end: [u8; 48],
}
impl UToolMenuEntryExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuEntryExtensions")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuEntryExtensions")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_tool_tip(target: &mut FToolMenuEntry, tool_tip: &FText) {
        let mut __stack = crate::core_data::StackAlloc::<2024>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_set_tool_tip,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target,
                __buffer.add(0).cast::<FToolMenuEntry>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tool_tip,
                __buffer.add(2008).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_set_tool_tip,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FToolMenuEntry>().swap(target);
        }
    }
    pub fn set_string_command(
        target: &mut FToolMenuEntry,
        ty: EToolMenuStringCommandType,
        custom_type: FName,
        string: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2040>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_set_string_command,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target,
                __buffer.add(0).cast::<FToolMenuEntry>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ty,
                __buffer.add(2008).cast::<EToolMenuStringCommandType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_type,
                __buffer.add(2012).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &string,
                __buffer.add(2024).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_set_string_command,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FToolMenuEntry>().swap(target);
        }
    }
    pub fn set_label(target: &mut FToolMenuEntry, label: &FText) {
        let mut __stack = crate::core_data::StackAlloc::<2024>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_set_label,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target,
                __buffer.add(0).cast::<FToolMenuEntry>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(label, __buffer.add(2008).cast::<FText>(), 1);
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_set_label,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FToolMenuEntry>().swap(target);
        }
    }
    pub fn set_icon(
        target: &mut FToolMenuEntry,
        style_set_name: FName,
        style_name: FName,
        small_style_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2044>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_set_icon,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target,
                __buffer.add(0).cast::<FToolMenuEntry>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &style_set_name,
                __buffer.add(2008).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &style_name,
                __buffer.add(2020).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &small_style_name,
                __buffer.add(2032).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_set_icon,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FToolMenuEntry>().swap(target);
        }
    }
    pub fn make_tool_menu_owner(name: FName) -> FToolMenuOwner {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_make_tool_menu_owner,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_make_tool_menu_owner,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FToolMenuOwner>().read() }
    }
    pub fn make_string_command(
        ty: EToolMenuStringCommandType,
        custom_type: FName,
        string: FString,
    ) -> FToolMenuStringCommand {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_make_string_command,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ty,
                __buffer.add(0).cast::<EToolMenuStringCommandType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_type,
                __buffer.add(4).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &string,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_make_string_command,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FToolMenuStringCommand>().read() }
    }
    pub fn make_script_slate_icon(
        style_set_name: FName,
        style_name: FName,
        small_style_name: FName,
    ) -> FScriptSlateIcon {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_make_script_slate_icon,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &style_set_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &style_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &small_style_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_make_script_slate_icon,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<FScriptSlateIcon>().read() }
    }
    pub fn init_menu_entry(
        in_owner: FName,
        in_name: FName,
        in_label: &FText,
        in_tool_tip: &FText,
        command_type: EToolMenuStringCommandType,
        custom_command_type: FName,
        command_string: FString,
    ) -> FToolMenuEntry {
        let mut __stack = crate::core_data::StackAlloc::<2096>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_init_menu_entry,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_owner, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(12).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(in_label, __buffer.add(24).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tool_tip,
                __buffer.add(40).cast::<FText>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &command_type,
                __buffer.add(56).cast::<EToolMenuStringCommandType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_command_type,
                __buffer.add(60).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &command_string,
                __buffer.add(72).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_init_menu_entry,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<FToolMenuEntry>().read() }
    }
    pub fn get_tool_tip(target: &FToolMenuEntry) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<2024>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_get_tool_tip,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target,
                __buffer.add(0).cast::<FToolMenuEntry>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_get_tool_tip,
                __buffer,
            )
        };
        unsafe { __buffer.add(2008).cast::<FText>().read() }
    }
    pub fn get_label(target: &FToolMenuEntry) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<2024>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_get_label,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target,
                __buffer.add(0).cast::<FToolMenuEntry>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_get_label,
                __buffer,
            )
        };
        unsafe { __buffer.add(2008).cast::<FText>().read() }
    }
    pub fn break_tool_menu_owner(in_value: &FToolMenuOwner, name: &mut FName) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_break_tool_menu_owner,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(0).cast::<FToolMenuOwner>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(name, __buffer.add(16).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_break_tool_menu_owner,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FName>().swap(name);
        }
    }
    pub fn break_string_command(
        in_value: &FToolMenuStringCommand,
        ty: &mut EToolMenuStringCommandType,
        custom_type: &mut FName,
        string: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_break_string_command,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(0).cast::<FToolMenuStringCommand>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                ty,
                __buffer.add(32).cast::<EToolMenuStringCommandType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                custom_type,
                __buffer.add(36).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(string, __buffer.add(48).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_break_string_command,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<EToolMenuStringCommandType>().swap(ty);
        }
        unsafe {
            __buffer.add(36).cast::<FName>().swap(custom_type);
        }
        unsafe {
            __buffer.add(48).cast::<FString>().swap(string);
        }
    }
    pub fn break_script_slate_icon(
        in_value: &FScriptSlateIcon,
        style_set_name: &mut FName,
        style_name: &mut FName,
        small_style_name: &mut FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_break_script_slate_icon,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(0).cast::<FScriptSlateIcon>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                style_set_name,
                __buffer.add(36).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                style_name,
                __buffer.add(48).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                small_style_name,
                __buffer.add(60).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuEntryExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_entry_extensions_break_script_slate_icon,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(36).cast::<FName>().swap(style_set_name);
        }
        unsafe {
            __buffer.add(48).cast::<FName>().swap(style_name);
        }
        unsafe {
            __buffer.add(60).cast::<FName>().swap(small_style_name);
        }
    }
}
#[repr(C, align(8))]
pub struct UToolMenuSectionExtensions {
    __padding_end: [u8; 48],
}
impl UToolMenuSectionExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuSectionExtensions")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuSectionExtensions")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_label(section: &mut FToolMenuSection, label: &FText) {
        let mut __stack = crate::core_data::StackAlloc::<592>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_section_extensions_set_label,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                section,
                __buffer.add(0).cast::<FToolMenuSection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(label, __buffer.add(576).cast::<FText>(), 1);
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_section_extensions_set_label,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FToolMenuSection>().swap(section);
        }
    }
    pub fn get_label(section: &FToolMenuSection) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<592>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_section_extensions_get_label,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                section,
                __buffer.add(0).cast::<FToolMenuSection>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_section_extensions_get_label,
                __buffer,
            )
        };
        unsafe { __buffer.add(576).cast::<FText>().read() }
    }
    pub fn add_entry_object(
        section: &mut FToolMenuSection,
        in_object: UPtr<UToolMenuEntryScript>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<584>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_section_extensions_add_entry_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                section,
                __buffer.add(0).cast::<FToolMenuSection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object,
                __buffer.add(576).cast::<UPtr<UToolMenuEntryScript>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_section_extensions_add_entry_object,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FToolMenuSection>().swap(section);
        }
    }
    pub fn add_entry(section: &mut FToolMenuSection, args: &FToolMenuEntry) {
        let mut __stack = crate::core_data::StackAlloc::<2584>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_section_extensions_add_entry,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                section,
                __buffer.add(0).cast::<FToolMenuSection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                args,
                __buffer.add(576).cast::<FToolMenuEntry>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::tool_menus::UToolMenuSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_section_extensions_add_entry,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FToolMenuSection>().swap(section);
        }
    }
}
#[repr(C, align(8))]
pub struct UToolMenuSectionDynamic {
    __padding_end: [u8; 48],
}
impl UToolMenuSectionDynamic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuSectionDynamic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuSectionDynamic")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn construct_sections(
        &mut self,
        menu: UPtr<UToolMenu>,
        context: &FToolMenuContext,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_section_dynamic_construct_sections,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &menu,
                __buffer.add(0).cast::<UPtr<UToolMenu>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(8).cast::<FToolMenuContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::tool_menus::__FUNCTION_PTRS
                    .u_tool_menu_section_dynamic_construct_sections,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UToolMenuWidgetCollectionContext {
    __padding_end: [u8; 64],
}
impl UToolMenuWidgetCollectionContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuWidgetCollectionContext")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMenuWidgetCollectionContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
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
