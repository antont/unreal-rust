#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UEdGraphNode_Reference {
    __padding_end: [u8; 464],
}
impl UEdGraphNode_Reference {}
#[repr(C, align(8))]
pub struct UEdGraphNode_ReferencedProperties {
    __padding_end: [u8; 256],
}
impl UEdGraphNode_ReferencedProperties {}
#[repr(C, align(16))]
pub struct UEdGraph_ReferenceViewer {
    __padding_end: [u8; 880],
}
impl UEdGraph_ReferenceViewer {}
#[repr(C, align(8))]
pub struct UReferenceViewerSchema {
    __padding_end: [u8; 48],
}
impl UReferenceViewerSchema {}
#[repr(C, align(8))]
pub struct UReferenceViewerSettings {
    __padding_end: [u8; 200],
}
impl UReferenceViewerSettings {}
#[repr(C, align(8))]
pub struct USizeMapSettings {
    __padding_end: [u8; 64],
}
impl USizeMapSettings {}
#[repr(transparent)]
pub struct EEditorOnlyReferenceFilterType(pub i32);
impl EEditorOnlyReferenceFilterType {
    pub const GAME: EEditorOnlyReferenceFilterType = EEditorOnlyReferenceFilterType(0);
    pub const PROPAGATION: EEditorOnlyReferenceFilterType = EEditorOnlyReferenceFilterType(
        1,
    );
    pub const EDITOR_ONLY: EEditorOnlyReferenceFilterType = EEditorOnlyReferenceFilterType(
        2,
    );
}
#[repr(transparent)]
pub struct ESizeMapDependencyType(pub i32);
impl ESizeMapDependencyType {
    pub const ALL: ESizeMapDependencyType = ESizeMapDependencyType(0);
    pub const GAME: ESizeMapDependencyType = ESizeMapDependencyType(1);
    pub const EDITOR_ONLY: ESizeMapDependencyType = ESizeMapDependencyType(2);
}
