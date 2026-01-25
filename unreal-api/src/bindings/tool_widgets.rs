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
    pub u_tool_slate_widget_types_function_library_get_action_button_type_names: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_tool_slate_widget_types_function_library_get_action_button_type_names: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UToolSlateWidgetTypesFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActionButtonTypeNames"),
            &raw mut __FUNCTION_PTRS
                .u_tool_slate_widget_types_function_library_get_action_button_type_names,
        );
    }
}
#[repr(C, align(16))]
pub struct FActionButtonStyle {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub button_style: crate::bindings::slate_core::FButtonStyle,
    pub button_content_padding: TOptional<crate::bindings::slate_core::FMargin>,
    pub combo_button_style: crate::bindings::slate_core::FComboButtonStyle,
    pub b_has_down_arrow: bool,
    pub combo_button_content_padding: TOptional<crate::bindings::slate_core::FMargin>,
    pub horizontal_content_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub text_block_style: crate::bindings::slate_core::FTextBlockStyle,
    pub icon_brush: TOptional<crate::bindings::slate_core::FSlateBrush>,
    pub icon_color_and_opacity: TOptional<crate::bindings::slate_core::FSlateColor>,
    pub icon_normal_padding: TOptional<crate::bindings::slate_core::FMargin>,
    pub icon_pressed_padding: TOptional<crate::bindings::slate_core::FMargin>,
    pub action_button_type: FName,
    pub icon_button_style: TOptional<crate::bindings::slate_core::FButtonStyle>,
}
impl FActionButtonStyle {}
#[repr(C, align(8))]
pub struct UFilterBarContext {
    __padding_end: [u8; 96],
}
impl UFilterBarContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFilterBarContext")
            .unwrap()
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
pub struct USidebarButtonMenuContext {
    __padding_end: [u8; 80],
}
impl USidebarButtonMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USidebarButtonMenuContext")
            .unwrap()
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
pub struct UToolSlateWidgetTypesFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UToolSlateWidgetTypesFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolSlateWidgetTypesFunctionLibrary")
            .unwrap()
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
#[repr(transparent)]
pub struct EFilterBarLayout(pub u8);
impl EFilterBarLayout {
    pub const HORIZONTAL: EFilterBarLayout = EFilterBarLayout(0);
    pub const VERTICAL: EFilterBarLayout = EFilterBarLayout(1);
}
