#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAutomationTestPreset {
    pub id: FName,
    pub name: FText,
    pub enabled_tests: TArray<FString>,
}
