#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FSequencerViewModelScriptingStruct {
    pub ty: FName,
}
pub struct USequencerOutlinerScriptingObject {
    pub on_selection_changed: FSequencerOutlinerScriptingObject_OnSelectionChanged,
}
pub struct USequencerScriptingLayer {
    pub outliner: UPtr<USequencerOutlinerScriptingObject>,
}
pub struct USequencerViewModelStructExtensions {}
