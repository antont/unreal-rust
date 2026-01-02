#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FSequencerViewModelScriptingStruct {
    pub ty: FName,
    __padding_end: [u8; 20],
}
impl FSequencerViewModelScriptingStruct {}
#[repr(C, align(8))]
pub struct USequencerOutlinerScriptingObject {
    __padding_end: [u8; 88],
}
impl USequencerOutlinerScriptingObject {}
#[repr(C, align(8))]
pub struct USequencerScriptingLayer {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub outliner: UPtr<USequencerOutlinerScriptingObject>,
}
impl USequencerScriptingLayer {}
#[repr(C, align(8))]
pub struct USequencerViewModelStructExtensions {
    __padding_end: [u8; 48],
}
impl USequencerViewModelStructExtensions {}
#[repr(transparent)]
pub struct FSequencerOutlinerScriptingObject_OnSelectionChanged {
    _opague: u8,
}
