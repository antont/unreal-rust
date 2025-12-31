#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FNamingTokenValueData {
    pub token_key: FString,
    pub token_namespace: FString,
    pub token_value: FText,
    pub b_was_evaluated: bool,
}
#[repr(C, align(8))]
pub struct FNamingTokenResultData {
    pub original_text: FText,
    pub evaluated_text: FText,
    pub token_values: TArray<FNamingTokenValueData>,
}
#[repr(C, align(8))]
pub struct FNamingTokenData {
    pub token_key: FString,
    pub display_name: FText,
    pub description: FText,
    pub function_name: FName,
}
#[repr(C, align(8))]
pub struct FNamingTokenFilterArgs {
    pub additional_namespaces_to_include: TArray<FString>,
    pub b_include_global: bool,
    pub b_force_case_sensitive: bool,
    pub b_native_only: bool,
}
#[repr(C, align(8))]
pub struct FNamingTokensEvaluationData {
    pub current_date_time: crate::bindings::core_u_object::FDateTime,
    pub contexts: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
pub struct UNamingTokens {
    pub custom_tokens: TArray<FNamingTokenData>,
    pub current_evaluation_data: FNamingTokensEvaluationData,
    pub namespace: FString,
    pub namespace_display_name: FText,
    pub test_token_input: FText,
    pub test_token_result: FText,
}
pub struct UGlobalNamingTokens {}
pub struct UNamingTokensEngineSubsystem {
    pub cached_naming_tokens: TMap<FString, UPtr<UNamingTokens>>,
}
