#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FExportedTextWidgetConstructor {}
#[repr(C, align(1))]
pub struct FExportedTextWidgetTag {}
#[repr(C, align(8))]
pub struct FLabelWidgetConstructor {}
#[repr(C, align(8))]
pub struct FPackagePathWidgetConstructor {}
#[repr(C, align(8))]
pub struct FLoadedPackagePathWidgetConstructor {}
#[repr(C, align(8))]
pub struct FSlateColorWidgetConstructor {}
#[repr(C, align(8))]
pub struct FUrlWidgetConstructor {}
#[repr(C, align(8))]
pub struct FWorldWidgetConstructor {}
pub struct UWidgetReferenceColumnUpdateFactory {}
pub struct UExportedTextWidgetFactory {}
pub struct UGeneralWidgetRegistrationFactory {}
pub struct ULabelWidgetFactory {}
pub struct UPackagePathWidgetFactory {}
pub struct USlateColorWidgetFactory {}
pub struct UUrlWidgetFactory {}
pub struct UWorldWidgetFactory {}
