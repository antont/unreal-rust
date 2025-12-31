#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FSettingsContainerReferenceColumn {
    pub container_name: FName,
}
#[repr(C, align(8))]
pub struct FSettingsCategoryReferenceColumn {
    pub category_name: FName,
}
#[repr(C, align(4))]
pub struct FSettingsNameColumn {
    pub name: FName,
}
#[repr(C, align(1))]
pub struct FSettingsContainerTag {}
#[repr(C, align(1))]
pub struct FSettingsCategoryTag {}
#[repr(C, align(1))]
pub struct FSettingsSectionTag {}
#[repr(C, align(1))]
pub struct FSettingsInactiveSectionTag {}
#[repr(C, align(8))]
pub struct FSettingsContainerReferenceWidgetConstructor {}
#[repr(C, align(8))]
pub struct FSettingsCategoryReferenceWidgetConstructor {}
#[repr(C, align(8))]
pub struct FSettingsSectionWidgetConstructor {}
pub struct UTestSettings {
    pub test_int32: i32,
}
pub struct UTedsSettingsEditorSubsystem {}
pub struct UTedsSettingsFactory {}
