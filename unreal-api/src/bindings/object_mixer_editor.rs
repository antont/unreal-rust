#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FObjectMixerWidgetUserConfig {
    pub default_filter_class: TSubclassOf<UObjectMixerObjectFilter>,
}
impl FObjectMixerWidgetUserConfig {}
#[repr(C, align(8))]
pub struct UObjectMixerEditorSettings {
    __padding_end: [u8; 112],
}
impl UObjectMixerEditorSettings {}
#[repr(C, align(8))]
pub struct UObjectMixerObjectFilter {
    __padding_end: [u8; 48],
}
impl UObjectMixerObjectFilter {}
#[repr(C, align(8))]
pub struct UObjectMixerBlueprintObjectFilter {
    __padding_end: [u8; 48],
}
impl UObjectMixerBlueprintObjectFilter {}
#[repr(C, align(8))]
pub struct UObjectMixerEditorSerializedData {
    __padding_end: [u8; 192],
}
impl UObjectMixerEditorSerializedData {}
#[repr(C, align(8))]
pub struct UObjectMixerBlueprintFilterFactory {
    __padding_end: [u8; 144],
}
impl UObjectMixerBlueprintFilterFactory {}
#[repr(C, align(8))]
pub struct UObjectMixerOutlinerModeEditorConfig {
    __padding_end: [u8; 128],
}
impl UObjectMixerOutlinerModeEditorConfig {}
#[repr(C, align(8))]
pub struct UObjectMixerEditorListMenuContext {
    __padding_end: [u8; 80],
}
impl UObjectMixerEditorListMenuContext {}
#[repr(C, align(8))]
pub struct UObjectMixerEditorUWidget {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub object_mixer_widget_user_config: FObjectMixerWidgetUserConfig,
}
impl UObjectMixerEditorUWidget {}
#[repr(transparent)]
pub struct EListViewColumnType(pub i32);
impl EListViewColumnType {
    pub const BUILT_IN: EListViewColumnType = EListViewColumnType(0);
    pub const PROPERTY_GENERATED: EListViewColumnType = EListViewColumnType(1);
}
#[repr(transparent)]
pub struct EObjectMixerInheritanceInclusionOptions(pub u8);
impl EObjectMixerInheritanceInclusionOptions {
    pub const NONE: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        0,
    );
    pub const INCLUDE_ONLY_IMMEDIATE_PARENT: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        1,
    );
    pub const INCLUDE_ONLY_IMMEDIATE_CHILDREN: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        2,
    );
    pub const INCLUDE_ONLY_IMMEDIATE_PARENT_AND_CHILDREN: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        3,
    );
    pub const INCLUDE_ALL_PARENTS: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        4,
    );
    pub const INCLUDE_ALL_CHILDREN: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        5,
    );
    pub const INCLUDE_ALL_PARENTS_AND_CHILDREN: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        6,
    );
    pub const INCLUDE_ALL_PARENTS_AND_ONLY_IMMEDIATE_CHILDREN: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        7,
    );
    pub const INCLUDE_ONLY_IMMEDIATE_PARENT_AND_ALL_CHILDREN: EObjectMixerInheritanceInclusionOptions = EObjectMixerInheritanceInclusionOptions(
        8,
    );
}
#[repr(transparent)]
pub struct EObjectMixerHybridMode(pub u8);
impl EObjectMixerHybridMode {
    pub const HYBRID_ACTOR: EObjectMixerHybridMode = EObjectMixerHybridMode(0);
    pub const HYBRID_COMPONENT: EObjectMixerHybridMode = EObjectMixerHybridMode(1);
    pub const HYBRID_NONE: EObjectMixerHybridMode = EObjectMixerHybridMode(2);
}
