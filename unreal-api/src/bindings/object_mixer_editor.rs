#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FObjectMixerCollectionObjectData {
    pub object_path: FSoftObjectPath,
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
    pub parent_class: TSubclassOf<UObject>,
}
pub struct UObjectMixerOutlinerModeEditorConfig {
    pub browsers: TMap<FName, FObjectMixerOutlinerModeConfig>,
}
pub struct UObjectMixerEditorListMenuContext {}
pub struct UObjectMixerEditorUWidget {
    pub object_mixer_widget_user_config: FObjectMixerWidgetUserConfig,
}
