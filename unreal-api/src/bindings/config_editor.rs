#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UPropertyConfigFileDisplayRow {
    pub config_file_name: FString,
    pub external_property: TFieldPath<FProperty>,
    pub b_is_file_writable: bool,
}
pub struct UConfigHierarchyPropertyView {
    pub edit_property: TFieldPath<FProperty>,
    pub config_file_property_objects: TArray<UPtr<UPropertyConfigFileDisplayRow>>,
}
