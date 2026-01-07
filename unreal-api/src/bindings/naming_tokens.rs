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
pub static mut U_NAMING_TOKENS_PROCESS_TOKEN_TEMPLATE_FUNCTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ON_PRE_EVALUATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ON_POST_EVALUATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_GET_CURRENT_DATE_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_UNREGISTER_GLOBAL_NAMESPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_REGISTER_GLOBAL_NAMESPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_IS_GLOBAL_NAMESPACE_REGISTERED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_NAMING_TOKENS_NATIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_NAMING_TOKENS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_MULTIPLE_NAMING_TOKENS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_GLOBAL_NAMESPACES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_ALL_NAMESPACES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_EVALUATE_TOKEN_TEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_EVALUATE_TOKEN_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_EVALUATE_TOKEN_LIST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_CLEAR_CACHED_NAMING_TOKENS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNamingTokens::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ProcessTokenTemplateFunction"),
            &raw mut U_NAMING_TOKENS_PROCESS_TOKEN_TEMPLATE_FUNCTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPreEvaluate"),
            &raw mut U_NAMING_TOKENS_ON_PRE_EVALUATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPostEvaluate"),
            &raw mut U_NAMING_TOKENS_ON_POST_EVALUATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentDateTime"),
            &raw mut U_NAMING_TOKENS_GET_CURRENT_DATE_TIME,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNamingTokensEngineSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterGlobalNamespace"),
            &raw mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_UNREGISTER_GLOBAL_NAMESPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterGlobalNamespace"),
            &raw mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_REGISTER_GLOBAL_NAMESPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsGlobalNamespaceRegistered"),
            &raw mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_IS_GLOBAL_NAMESPACE_REGISTERED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNamingTokensNative"),
            &raw mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_NAMING_TOKENS_NATIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNamingTokens"),
            &raw mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_NAMING_TOKENS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMultipleNamingTokens"),
            &raw mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_MULTIPLE_NAMING_TOKENS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGlobalNamespaces"),
            &raw mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_GLOBAL_NAMESPACES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllNamespaces"),
            &raw mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_ALL_NAMESPACES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateTokenText"),
            &raw mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_EVALUATE_TOKEN_TEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateTokenString"),
            &raw mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_EVALUATE_TOKEN_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateTokenList"),
            &raw mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_EVALUATE_TOKEN_LIST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearCachedNamingTokens"),
            &raw mut U_NAMING_TOKENS_ENGINE_SUBSYSTEM_CLEAR_CACHED_NAMING_TOKENS,
        );
    }
}
#[repr(C, align(8))]
pub struct FNamingTokenValueData {
    pub token_key: FString,
    pub token_namespace: FString,
    pub token_value: FText,
    pub b_was_evaluated: bool,
}
impl FNamingTokenValueData {}
#[repr(C, align(8))]
pub struct FNamingTokenResultData {
    pub original_text: FText,
    pub evaluated_text: FText,
    pub token_values: TArray<FNamingTokenValueData>,
}
impl FNamingTokenResultData {}
#[repr(C, align(8))]
pub struct FNamingTokenData {
    pub token_key: FString,
    pub display_name: FText,
    pub description: FText,
    __padding_end: [u8; 40],
}
impl FNamingTokenData {}
#[repr(C, align(8))]
pub struct FNamingTokenFilterArgs {
    pub additional_namespaces_to_include: TArray<FString>,
    pub b_include_global: bool,
    pub b_force_case_sensitive: bool,
    pub b_native_only: bool,
}
impl FNamingTokenFilterArgs {}
#[repr(C, align(8))]
pub struct FNamingTokensEvaluationData {
    pub current_date_time: crate::bindings::core_u_object::FDateTime,
    pub contexts: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    __padding_end: [u8; 8],
}
impl FNamingTokensEvaluationData {}
#[repr(C, align(8))]
pub struct UNamingTokens {
    #[doc(hidden)]
    __padding_224: [u8; 224],
    pub current_evaluation_data: FNamingTokensEvaluationData,
    __padding_end: [u8; 64],
}
impl UNamingTokens {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNamingTokens")
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
    pub fn get_current_date_time(&self) -> crate::bindings::core_u_object::FDateTime {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_GET_CURRENT_DATE_TIME,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::U_NAMING_TOKENS_GET_CURRENT_DATE_TIME,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FDateTime>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UGlobalNamingTokens {
    __padding_end: [u8; 320],
}
impl UGlobalNamingTokens {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGlobalNamingTokens")
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
pub struct UNamingTokensEngineSubsystem {
    __padding_end: [u8; 344],
}
impl UNamingTokensEngineSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNamingTokensEngineSubsystem")
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
    pub fn unregister_global_namespace(&mut self, in_namespace: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_UNREGISTER_GLOBAL_NAMESPACE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_namespace,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_UNREGISTER_GLOBAL_NAMESPACE,
                __buffer,
            )
        };
    }
    pub fn register_global_namespace(&mut self, in_namespace: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_REGISTER_GLOBAL_NAMESPACE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_namespace,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_REGISTER_GLOBAL_NAMESPACE,
                __buffer,
            )
        };
    }
    pub fn is_global_namespace_registered(&self, in_namespace: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_IS_GLOBAL_NAMESPACE_REGISTERED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_namespace,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_IS_GLOBAL_NAMESPACE_REGISTERED,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_naming_tokens_native(
        &self,
        in_namespace: FString,
    ) -> UPtr<UNamingTokens> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_NAMING_TOKENS_NATIVE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_namespace,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_NAMING_TOKENS_NATIVE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UNamingTokens>>().read() }
    }
    pub fn get_naming_tokens(&self, in_namespace: FString) -> UPtr<UNamingTokens> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_NAMING_TOKENS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_namespace,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_NAMING_TOKENS,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UNamingTokens>>().read() }
    }
    pub fn get_multiple_naming_tokens(
        &self,
        in_namespaces: &TArray<FString>,
    ) -> TArray<UPtr<UNamingTokens>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_MULTIPLE_NAMING_TOKENS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_namespaces,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_MULTIPLE_NAMING_TOKENS,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<UPtr<UNamingTokens>>>().read() }
    }
    pub fn get_global_namespaces(&self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_GLOBAL_NAMESPACES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_GLOBAL_NAMESPACES,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_all_namespaces(&self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_ALL_NAMESPACES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_GET_ALL_NAMESPACES,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn evaluate_token_text(
        &mut self,
        in_token_text: &FText,
        in_filter: &FNamingTokenFilterArgs,
        in_contexts: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> FNamingTokenResultData {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_EVALUATE_TOKEN_TEXT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_token_text,
                __buffer.add(0).cast::<FText>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_filter,
                __buffer.add(16).cast::<FNamingTokenFilterArgs>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_contexts,
                __buffer
                    .add(40)
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
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_EVALUATE_TOKEN_TEXT,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<FNamingTokenResultData>().read() }
    }
    pub fn evaluate_token_string(
        &mut self,
        in_token_string: FString,
        in_filter: &FNamingTokenFilterArgs,
        in_contexts: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> FNamingTokenResultData {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_EVALUATE_TOKEN_STRING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_token_string,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_filter,
                __buffer.add(16).cast::<FNamingTokenFilterArgs>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_contexts,
                __buffer
                    .add(40)
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
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_EVALUATE_TOKEN_STRING,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<FNamingTokenResultData>().read() }
    }
    pub fn evaluate_token_list(
        &mut self,
        in_token_list: &TArray<FString>,
        in_filter: &FNamingTokenFilterArgs,
        in_contexts: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> TArray<FNamingTokenValueData> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_EVALUATE_TOKEN_LIST,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_token_list,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_filter,
                __buffer.add(16).cast::<FNamingTokenFilterArgs>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_contexts,
                __buffer
                    .add(40)
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
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_EVALUATE_TOKEN_LIST,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<TArray<FNamingTokenValueData>>().read() }
    }
    pub fn clear_cached_naming_tokens(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_CLEAR_CACHED_NAMING_TOKENS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::U_NAMING_TOKENS_ENGINE_SUBSYSTEM_CLEAR_CACHED_NAMING_TOKENS,
                __buffer,
            )
        };
    }
}
