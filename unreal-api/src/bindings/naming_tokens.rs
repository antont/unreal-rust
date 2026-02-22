#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_naming_tokens_process_token_template_function: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_on_pre_evaluate: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_on_post_evaluate: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_get_current_date_time: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_engine_subsystem_unregister_global_namespace: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_engine_subsystem_register_global_namespace: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_engine_subsystem_is_global_namespace_registered: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_engine_subsystem_get_naming_tokens_native: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_engine_subsystem_get_naming_tokens: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_engine_subsystem_get_multiple_naming_tokens: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_engine_subsystem_get_global_namespaces: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_engine_subsystem_get_all_namespaces: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_engine_subsystem_evaluate_token_text: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_engine_subsystem_evaluate_token_string: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_engine_subsystem_evaluate_token_list: *mut crate::ffi::UFunctionOpague,
    pub u_naming_tokens_engine_subsystem_clear_cached_naming_tokens: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_naming_tokens_process_token_template_function: std::ptr::null_mut(),
            u_naming_tokens_on_pre_evaluate: std::ptr::null_mut(),
            u_naming_tokens_on_post_evaluate: std::ptr::null_mut(),
            u_naming_tokens_get_current_date_time: std::ptr::null_mut(),
            u_naming_tokens_engine_subsystem_unregister_global_namespace: std::ptr::null_mut(),
            u_naming_tokens_engine_subsystem_register_global_namespace: std::ptr::null_mut(),
            u_naming_tokens_engine_subsystem_is_global_namespace_registered: std::ptr::null_mut(),
            u_naming_tokens_engine_subsystem_get_naming_tokens_native: std::ptr::null_mut(),
            u_naming_tokens_engine_subsystem_get_naming_tokens: std::ptr::null_mut(),
            u_naming_tokens_engine_subsystem_get_multiple_naming_tokens: std::ptr::null_mut(),
            u_naming_tokens_engine_subsystem_get_global_namespaces: std::ptr::null_mut(),
            u_naming_tokens_engine_subsystem_get_all_namespaces: std::ptr::null_mut(),
            u_naming_tokens_engine_subsystem_evaluate_token_text: std::ptr::null_mut(),
            u_naming_tokens_engine_subsystem_evaluate_token_string: std::ptr::null_mut(),
            u_naming_tokens_engine_subsystem_evaluate_token_list: std::ptr::null_mut(),
            u_naming_tokens_engine_subsystem_clear_cached_naming_tokens: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UNamingTokens::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ProcessTokenTemplateFunction"),
                &raw mut __FUNCTION_PTRS.u_naming_tokens_process_token_template_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPreEvaluate"),
                &raw mut __FUNCTION_PTRS.u_naming_tokens_on_pre_evaluate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPostEvaluate"),
                &raw mut __FUNCTION_PTRS.u_naming_tokens_on_post_evaluate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentDateTime"),
                &raw mut __FUNCTION_PTRS.u_naming_tokens_get_current_date_time,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UNamingTokensEngineSubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnregisterGlobalNamespace"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_unregister_global_namespace,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterGlobalNamespace"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_register_global_namespace,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsGlobalNamespaceRegistered"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_is_global_namespace_registered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNamingTokensNative"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_naming_tokens_native,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNamingTokens"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_naming_tokens,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMultipleNamingTokens"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_multiple_naming_tokens,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGlobalNamespaces"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_global_namespaces,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllNamespaces"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_all_namespaces,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateTokenText"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_evaluate_token_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateTokenString"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_evaluate_token_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateTokenList"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_evaluate_token_list,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearCachedNamingTokens"),
                &raw mut __FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_clear_cached_naming_tokens,
            );
        }
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
    pub(crate) __padding_end: [u8; 40],
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
    pub(crate) __padding_end: [u8; 8],
}
impl FNamingTokensEvaluationData {}
#[repr(C, align(8))]
pub struct UNamingTokens {
    #[doc(hidden)]
    pub(crate) __padding_224: [u8; 224],
    pub current_evaluation_data: FNamingTokensEvaluationData,
    __padding_end: [u8; 64],
}
impl UNamingTokens {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNamingTokens")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNamingTokens")
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
    pub fn on_pre_evaluate(&mut self, in_evaluation_data: &FNamingTokensEvaluationData) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_on_pre_evaluate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_evaluation_data,
                __buffer.add(0).cast::<FNamingTokensEvaluationData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_on_pre_evaluate,
                __buffer,
            )
        };
    }
    pub fn on_post_evaluate(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_on_post_evaluate,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_on_post_evaluate,
                __buffer,
            )
        };
    }
    pub fn get_current_date_time(&self) -> crate::bindings::core_u_object::FDateTime {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_get_current_date_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_get_current_date_time,
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGlobalNamingTokens")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGlobalNamingTokens")
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
}
#[repr(C, align(8))]
pub struct UNamingTokensEngineSubsystem {
    __padding_end: [u8; 344],
}
impl UNamingTokensEngineSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNamingTokensEngineSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNamingTokensEngineSubsystem")
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
    pub fn unregister_global_namespace(&mut self, in_namespace: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_unregister_global_namespace,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_unregister_global_namespace,
                __buffer,
            )
        };
        std::mem::forget(in_namespace);
    }
    pub fn register_global_namespace(&mut self, in_namespace: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_register_global_namespace,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_register_global_namespace,
                __buffer,
            )
        };
        std::mem::forget(in_namespace);
    }
    pub fn is_global_namespace_registered(&self, in_namespace: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_is_global_namespace_registered,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_is_global_namespace_registered,
                __buffer,
            )
        };
        std::mem::forget(in_namespace);
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_naming_tokens_native,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_naming_tokens_native,
                __buffer,
            )
        };
        std::mem::forget(in_namespace);
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_naming_tokens,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_naming_tokens,
                __buffer,
            )
        };
        std::mem::forget(in_namespace);
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_multiple_naming_tokens,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_multiple_naming_tokens,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_global_namespaces,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_global_namespaces,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_all_namespaces,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_get_all_namespaces,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_evaluate_token_text,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_evaluate_token_text,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_evaluate_token_string,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_evaluate_token_string,
                __buffer,
            )
        };
        std::mem::forget(in_token_string);
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_evaluate_token_list,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_evaluate_token_list,
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
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_clear_cached_naming_tokens,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::naming_tokens::__FUNCTION_PTRS
                    .u_naming_tokens_engine_subsystem_clear_cached_naming_tokens,
                __buffer,
            )
        };
    }
}
