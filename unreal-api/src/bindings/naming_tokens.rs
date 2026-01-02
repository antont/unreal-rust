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
impl UNamingTokens {}
#[repr(C, align(8))]
pub struct UGlobalNamingTokens {
    __padding_end: [u8; 320],
}
impl UGlobalNamingTokens {}
#[repr(C, align(8))]
pub struct UNamingTokensEngineSubsystem {
    __padding_end: [u8; 344],
}
impl UNamingTokensEngineSubsystem {}
