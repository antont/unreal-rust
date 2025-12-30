#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(1))]
pub struct FActorBrowsingModeConfig {
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
    pub folder_double_click_method: EActorBrowsingFolderDoubleClickMethod,
    pub b_should_update_content_while_in_pie_focused: bool,
    pub b_collapse_outliner_tree_on_new_selection: bool,
}
#[repr(C, align(8))]
pub struct FSceneOutlinerConfig {
    pub column_visibilities: TMap<FName, bool>,
    pub b_should_stack_hierarchy_headers: bool,
}
pub struct USceneOutlinerMenuContext {}
pub struct UActorBrowserConfig {
    pub actor_browsers: TMap<FName, FActorBrowsingModeConfig>,
}
pub struct UOutlinerConfig {
    pub outliners: TMap<FName, FSceneOutlinerConfig>,
}
