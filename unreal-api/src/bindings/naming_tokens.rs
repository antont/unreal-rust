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
    __padding_end: [u8; 7],
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
    __padding_end: [u8; 5],
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
}
