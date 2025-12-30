#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UWorldPartitionConvertOptions {
    pub commandlet_class: TSubclassOf<UWorldPartitionConvertCommandlet>,
    pub b_in_place: bool,
    pub b_delete_source_levels: bool,
    pub b_generate_ini: bool,
    pub b_report_only: bool,
    pub b_verbose: bool,
    pub b_skip_stable_guid_validation: bool,
    pub b_only_merge_sub_levels: bool,
    pub b_save_foliage_type_to_content_folder: bool,
}
pub struct UContentBundleEditorSubsystem {
    pub content_bundle_editing_sub_module: UPtr<UContentBundleEditingSubmodule>,
}
pub struct UContentBundleEditorSubsystemModule {}
pub struct UActorEditorContextContentBundleState {
    pub content_bundle_guid: FGuid,
}
pub struct UContentBundleEditingSubmodule {
    pub editing_content_bundles_stack: TArray<FGuid>,
    pub editing_content_bundle_guid: FGuid,
}
pub struct UWorldPartitionHLODEditorSubsystem {}
pub struct UHLODLayerFactory {}
pub struct UWorldPartitionEditorSettings {
    pub commandlet_class: TSubclassOf<UWorldPartitionConvertCommandlet>,
    pub instanced_foliage_grid_size: i32,
    pub minimap_low_quality_world_units_per_pixel_threshold: i32,
    pub b_enable_loading_in_editor: bool,
    pub b_enable_streaming_generation_log_on_pie: bool,
    pub b_show_hlo_ds_in_editor: bool,
    pub b_show_hlo_ds_over_loaded_regions: bool,
    pub b_enable_advanced_hlod_settings: bool,
    pub hlod_min_draw_distance: f64,
    pub hlod_max_draw_distance: f64,
}
pub struct UWorldPartitionEditorState {
    pub loaded_editor_regions: TArray<FBox>,
    pub loaded_editor_location_volumes: TArray<FName>,
}
