#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_WIDGET_ARGUMENT_STYLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_DISPLAY_TOKEN_ICON: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_DISPLAY_ERROR_MESSAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_DISPLAY_BORDER_IMAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_CONTEXTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_CAN_DISPLAY_RESOLVED_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_BACKGROUND_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_ON_PRE_EVALUATE_NAMING_TOKENS_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_GET_TOKENIZED_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_GET_RESOLVED_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_GET_DISPLAY_TOKEN_ICON: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_GET_DISPLAY_ERROR_MESSAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_GET_DISPLAY_BORDER_IMAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_EDITABLE_TEXT_GET_CAN_DISPLAY_RESOLVED_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNamingTokensEditableText::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWidgetArgumentStyle"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_WIDGET_ARGUMENT_STYLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDisplayTokenIcon"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_DISPLAY_TOKEN_ICON,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDisplayErrorMessage"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_DISPLAY_ERROR_MESSAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDisplayBorderImage"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_DISPLAY_BORDER_IMAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetContexts"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_CONTEXTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCanDisplayResolvedText"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_CAN_DISPLAY_RESOLVED_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBackgroundColor"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_SET_BACKGROUND_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPreEvaluateNamingTokens__DelegateSignature"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_ON_PRE_EVALUATE_NAMING_TOKENS_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTokenizedText"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_GET_TOKENIZED_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetResolvedText"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_GET_RESOLVED_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayTokenIcon"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_GET_DISPLAY_TOKEN_ICON,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayErrorMessage"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_GET_DISPLAY_ERROR_MESSAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayBorderImage"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_GET_DISPLAY_BORDER_IMAGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCanDisplayResolvedText"),
            &raw mut U_NAMING_TOKENS_EDITABLE_TEXT_GET_CAN_DISPLAY_RESOLVED_TEXT,
        );
    }
}
#[repr(C, align(16))]
pub struct UNamingTokensEditableText {
    #[doc(hidden)]
    __padding_1736: [u8; 1736],
    pub filter_args: crate::bindings::naming_tokens::FNamingTokenFilterArgs,
    #[doc(hidden)]
    __padding_1761: [u8; 1],
    pub b_is_multiline: bool,
    pub b_display_token_icon: bool,
    pub b_display_error_message: bool,
    pub b_display_border_image: bool,
    pub argument_style: crate::bindings::slate_core::FTextBlockStyle,
    pub background_color: crate::bindings::slate_core::FSlateColor,
    pub b_can_display_resolved_text: bool,
    #[doc(hidden)]
    __padding_2704: [u8; 56],
    pub contexts: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
impl UNamingTokensEditableText {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNamingTokensEditableText")
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
pub struct FNamingTokensEditableText_OnPreEvaluateNamingTokens {
    _opague: [u8; 24],
}
