#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FNavigationToolBuiltInFilterParams {
    pub filter_id: FName,
    pub object_classes: TArray<TSubclassOf<crate::bindings::core_u_object::UObject>>,
    pub filter_mode: ENavigationToolFilterMode,
    pub filter_text: FText,
    pub display_name: FText,
    pub tooltip_text: FText,
    pub override_icon: crate::bindings::slate_core::FSlateBrush,
    pub b_use_override_icon: bool,
    pub b_enabled_by_default: bool,
}
#[repr(C, align(8))]
pub struct FNavigationToolColumnView {
    pub view_name: FText,
    pub visible_columns: TSet<FName>,
}
pub struct UNavigationToolFilterBarContext {}
pub struct UNavigationToolFilterExtension {}
pub struct UNavigationToolFilterMenuContext {}
pub struct UNavigationToolFilterTextExpressionExtension {}
pub struct UNavigationToolItemMenuContext {}
pub struct UNavigationToolViewMenuContext {}
pub struct UNavigationToolSettings {
    pub b_apply_default_column_view: bool,
    pub b_use_short_names: bool,
    pub b_use_muted_hierarchy: bool,
    pub b_auto_expand_to_selection: bool,
    pub b_always_show_lock_state: bool,
    pub item_default_view_mode: ENavigationToolItemViewMode,
    pub item_proxy_view_mode: ENavigationToolItemViewMode,
    pub custom_column_views: TSet<FNavigationToolColumnView>,
    pub enabled_built_in_filters: TSet<FName>,
    pub filter_bars: TMap<FName, crate::bindings::sequencer::FSequencerFilterBarConfig>,
    pub b_auto_expand_nodes_on_filter_pass: bool,
    pub b_use_filter_submenus_for_categories: bool,
    pub b_filter_bar_visible: bool,
    pub last_filter_bar_layout: crate::bindings::tool_widgets::EFilterBarLayout,
    pub last_filter_bar_size_coefficient: f32,
    pub b_sync_selection_to_navigation_tool: bool,
    pub b_sync_selection_to_sequencer: bool,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENavigationToolFilterMode(pub u8);
impl ENavigationToolFilterMode {
    pub const NONE: ENavigationToolFilterMode = ENavigationToolFilterMode(0);
    pub const MATCHES_TYPE: ENavigationToolFilterMode = ENavigationToolFilterMode(1);
    pub const CONTAINER_OF_TYPE: ENavigationToolFilterMode = ENavigationToolFilterMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENavigationToolItemViewMode(pub u8);
impl ENavigationToolItemViewMode {
    pub const NONE: ENavigationToolItemViewMode = ENavigationToolItemViewMode(0);
    pub const ITEM_TREE: ENavigationToolItemViewMode = ENavigationToolItemViewMode(1);
    pub const HORIZONTAL_ITEM_LIST: ENavigationToolItemViewMode = ENavigationToolItemViewMode(
        2,
    );
    pub const ALL: ENavigationToolItemViewMode = ENavigationToolItemViewMode(3);
}
