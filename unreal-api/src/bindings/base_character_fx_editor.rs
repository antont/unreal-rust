#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UBaseCharacterFXEditor {
    pub original_objects_to_edit: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
pub struct UBaseCharacterFXEditorMode {
    pub original_objects_to_edit: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub tool_targets: TArray<
        UPtr<crate::bindings::interactive_tools_framework::UToolTarget>,
    >,
}
pub struct UBaseCharacterFXEditorUISubsystem {}
