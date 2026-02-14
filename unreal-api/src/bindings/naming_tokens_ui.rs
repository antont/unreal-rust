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
    pub u_naming_tokens_editable_text_set_widget_argument_style: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_set_display_token_icon: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_set_display_error_message: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_set_display_border_image: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_set_contexts: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_set_can_display_resolved_text: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_set_background_color: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_on_pre_evaluate_naming_tokens_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_get_tokenized_text: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_get_resolved_text: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_get_display_token_icon: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_get_display_error_message: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_get_display_border_image: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_editable_text_get_can_display_resolved_text: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_naming_tokens_editable_text_set_widget_argument_style: std::ptr::null_mut(),
            u_naming_tokens_editable_text_set_display_token_icon: std::ptr::null_mut(),
            u_naming_tokens_editable_text_set_display_error_message: std::ptr::null_mut(),
            u_naming_tokens_editable_text_set_display_border_image: std::ptr::null_mut(),
            u_naming_tokens_editable_text_set_contexts: std::ptr::null_mut(),
            u_naming_tokens_editable_text_set_can_display_resolved_text: std::ptr::null_mut(),
            u_naming_tokens_editable_text_set_background_color: std::ptr::null_mut(),
            u_naming_tokens_editable_text_on_pre_evaluate_naming_tokens_delegate_signature: std::ptr::null_mut(),
            u_naming_tokens_editable_text_get_tokenized_text: std::ptr::null_mut(),
            u_naming_tokens_editable_text_get_resolved_text: std::ptr::null_mut(),
            u_naming_tokens_editable_text_get_display_token_icon: std::ptr::null_mut(),
            u_naming_tokens_editable_text_get_display_error_message: std::ptr::null_mut(),
            u_naming_tokens_editable_text_get_display_border_image: std::ptr::null_mut(),
            u_naming_tokens_editable_text_get_can_display_resolved_text: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UNamingTokensEditableText::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWidgetArgumentStyle"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_widget_argument_style,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDisplayTokenIcon"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_display_token_icon,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDisplayErrorMessage"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_display_error_message,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDisplayBorderImage"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_display_border_image,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetContexts"),
                &raw mut __FUNCTION_PTRS.u_naming_tokens_editable_text_set_contexts,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCanDisplayResolvedText"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_can_display_resolved_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBackgroundColor"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_background_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "OnPreEvaluateNamingTokens__DelegateSignature",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_editable_text_on_pre_evaluate_naming_tokens_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTokenizedText"),
                &raw mut __FUNCTION_PTRS.u_naming_tokens_editable_text_get_tokenized_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetResolvedText"),
                &raw mut __FUNCTION_PTRS.u_naming_tokens_editable_text_get_resolved_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDisplayTokenIcon"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_display_token_icon,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDisplayErrorMessage"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_display_error_message,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDisplayBorderImage"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_display_border_image,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCanDisplayResolvedText"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_can_display_resolved_text,
            );
        }
    }
}
#[repr(C, align(16))]
pub struct UNamingTokensEditableText {
    #[doc(hidden)]
    pub(crate) __padding_1736: [u8; 1736],
    pub filter_args: crate::bindings::naming_tokens::FNamingTokenFilterArgs,
    #[doc(hidden)]
    pub(crate) __padding_1761: [u8; 1],
    pub b_is_multiline: bool,
    pub b_display_token_icon: bool,
    pub b_display_error_message: bool,
    pub b_display_border_image: bool,
    pub argument_style: crate::bindings::slate_core::FTextBlockStyle,
    pub background_color: crate::bindings::slate_core::FSlateColor,
    pub b_can_display_resolved_text: bool,
    #[doc(hidden)]
    pub(crate) __padding_2704: [u8; 56],
    pub contexts: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
impl UNamingTokensEditableText {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNamingTokensEditableText")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNamingTokensEditableText")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_widget_argument_style(
        &mut self,
        in_widget_style: &crate::bindings::slate_core::FTextBlockStyle,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<848>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_widget_argument_style,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_widget_style,
                __buffer.add(0).cast::<crate::bindings::slate_core::FTextBlockStyle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_widget_argument_style,
                __buffer,
            )
        };
    }
    pub fn set_display_token_icon(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_display_token_icon,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_display_token_icon,
                __buffer,
            )
        };
    }
    pub fn set_display_error_message(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_display_error_message,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_display_error_message,
                __buffer,
            )
        };
    }
    pub fn set_display_border_image(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_display_border_image,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_display_border_image,
                __buffer,
            )
        };
    }
    pub fn set_contexts(
        &mut self,
        in_contexts: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_contexts,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_contexts,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_contexts,
                __buffer,
            )
        };
    }
    pub fn set_can_display_resolved_text(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_can_display_resolved_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_can_display_resolved_text,
                __buffer,
            )
        };
    }
    pub fn set_background_color(
        &mut self,
        in_color: &crate::bindings::slate_core::FSlateColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_background_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(0).cast::<crate::bindings::slate_core::FSlateColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_set_background_color,
                __buffer,
            )
        };
    }
    pub fn get_tokenized_text(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_tokenized_text,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_tokenized_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_resolved_text(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_resolved_text,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_resolved_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_display_token_icon(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_display_token_icon,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_display_token_icon,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_display_error_message(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_display_error_message,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_display_error_message,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_display_border_image(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_display_border_image,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_display_border_image,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_can_display_resolved_text(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_can_display_resolved_text,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens_ui::__FUNCTION_PTRS
                    .u_naming_tokens_editable_text_get_can_display_resolved_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct FNamingTokensEditableText_OnPreEvaluateNamingTokens {
    _opague: [u8; 24],
}
