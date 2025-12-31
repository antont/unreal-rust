#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FObjectMixerCollectionObjectData {
    pub object_path: crate::bindings::core_u_object::FSoftObjectPath,
}
#[repr(C, align(8))]
pub struct FObjectMixerCollectionObjectSet {
    pub collection_name: FName,
    pub collection_objects: TArray<FObjectMixerCollectionObjectData>,
}
#[repr(C, align(4))]
pub struct FObjectMixerColumnData {
    pub column_name: FName,
    pub b_should_be_enabled: bool,
}
#[repr(C, align(8))]
pub struct FObjectMixerSerializationDataPerFilter {
    pub filter_class_name: FName,
    pub serialized_collections: TArray<FObjectMixerCollectionObjectSet>,
    pub serialized_column_data: TSet<FObjectMixerColumnData>,
}
#[repr(C, align(1))]
pub struct FObjectMixerOutlinerModeConfig {
    pub b_hide_temporary_actors: bool,
    pub b_show_only_actors_in_current_level: bool,
    pub b_show_only_actors_in_current_data_layers: bool,
    pub b_show_only_actors_in_current_content_bundle: bool,
    pub b_show_only_selected_actors: bool,
    pub b_hide_actor_components: bool,
    pub b_hide_level_instance_hierarchy: bool,
    pub b_hide_unloaded_actors: bool,
    pub b_hide_empty_folders: bool,
    pub b_always_frame_selection: bool,
}
#[repr(C, align(8))]
pub struct FObjectMixerSceneOutlinerColumnInfo {
    pub property_name: FName,
    pub column_id: FName,
    pub property_display_text: FText,
    pub property_type: EListViewColumnType,
    pub property_category_name: FName,
    pub b_can_be_hidden: bool,
    pub b_is_desired_to_be_shown_by_default: bool,
}
#[repr(C, align(8))]
pub struct FObjectMixerWidgetUserConfig {
    pub default_filter_class: TSubclassOf<UObjectMixerObjectFilter>,
}
pub struct UObjectMixerEditorSettings {
    pub b_sync_selection: bool,
    pub hybrid_row_policy: EObjectMixerHybridMode,
}
pub struct UObjectMixerObjectFilter {}
pub struct UObjectMixerBlueprintObjectFilter {}
pub struct UObjectMixerEditorSerializedData {
    pub serialized_data_per_filter: TSet<FObjectMixerSerializationDataPerFilter>,
}
pub struct UObjectMixerBlueprintFilterFactory {
    pub parent_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
pub struct UObjectMixerOutlinerModeEditorConfig {
    pub browsers: TMap<FName, FObjectMixerOutlinerModeConfig>,
}
pub struct UObjectMixerEditorListMenuContext {}
pub struct UObjectMixerEditorUWidget {
    pub object_mixer_widget_user_config: FObjectMixerWidgetUserConfig,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EListViewColumnType(pub i32);
impl EListViewColumnType {
    pub const BUILT_IN: EListViewColumnType = EListViewColumnType(0);
    pub const PROPERTY_GENERATED: EListViewColumnType = EListViewColumnType(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EObjectMixerHybridMode(pub u8);
impl EObjectMixerHybridMode {
    pub const HYBRID_ACTOR: EObjectMixerHybridMode = EObjectMixerHybridMode(0);
    pub const HYBRID_COMPONENT: EObjectMixerHybridMode = EObjectMixerHybridMode(1);
    pub const HYBRID_NONE: EObjectMixerHybridMode = EObjectMixerHybridMode(2);
}
