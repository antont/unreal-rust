#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UBaseCharacterFXEditor {
    pub original_objects_to_edit: TArray<UPtr<UObject>>,
}
pub struct UBaseCharacterFXEditorMode {
    pub original_objects_to_edit: TArray<UPtr<UObject>>,
    pub tool_targets: TArray<UPtr<UToolTarget>>,
}
pub struct UBaseCharacterFXEditorUISubsystem {}
