#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FTranslationChange {
    pub version: FString,
    pub date_and_time: crate::bindings::core_u_object::FDateTime,
    pub source: FString,
    pub translation: FString,
}
#[repr(C, align(8))]
pub struct FTranslationContextInfo {
    pub key: FString,
    pub context: FString,
    pub changes: TArray<FTranslationChange>,
}
pub struct UTranslationPickerSettings {
    pub b_submit_translation_picker_changes_to_localization_service: bool,
}
pub struct UTranslationUnit {
    pub namespace: FString,
    pub key: FString,
    pub source: FString,
    pub translation: FString,
    pub contexts: TArray<FTranslationContextInfo>,
    pub has_been_reviewed: bool,
    pub translation_before_import: FString,
    pub locres_path: FString,
}
