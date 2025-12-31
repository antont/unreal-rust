#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FilterState {
    pub filter_path: crate::bindings::core_u_object::FTopLevelAssetPath,
    pub b_is_enabled: bool,
}
pub struct UEdGraphNode_Reference {}
pub struct UEdGraphNode_ReferencedProperties {
    pub referencing_node: UPtr<UEdGraphNode_Reference>,
    pub referenced_node: UPtr<UEdGraphNode_Reference>,
}
pub struct UEdGraph_ReferenceViewer {}
pub struct UReferenceViewerSchema {}
pub struct UReferenceViewerSettings {
    pub b_limit_search_depth: bool,
    pub b_is_show_referencers: bool,
    pub max_search_referencer_depth: i32,
    pub b_is_show_dependencies: bool,
    pub max_search_dependency_depth: i32,
    pub b_limit_search_breadth_deprecated: bool,
    pub max_search_breadth: i32,
    pub b_enable_collection_filter: bool,
    pub b_enable_plugin_filter: bool,
    pub b_is_show_soft_references: bool,
    pub b_is_show_hard_references: bool,
    pub b_is_show_editor_only_references: bool,
    pub editor_only_reference_filter_type: EEditorOnlyReferenceFilterType,
    pub b_is_show_management_references: bool,
    pub b_is_show_searchable_names: bool,
    pub b_is_show_code_packages: bool,
    pub b_is_show_duplicates: bool,
    pub b_is_show_filtered_packages_only: bool,
    pub b_is_compact_mode: bool,
    pub b_is_show_external_referencers: bool,
    pub b_is_show_path: bool,
    pub b_filters_enabled: bool,
    pub b_auto_update_filters: bool,
    pub user_filters: TArray<FilterState>,
    pub custom_filters: TMap<FString, bool>,
    pub b_find_path_enabled: bool,
}
pub struct USizeMapSettings {
    pub size_type: FName,
    pub dependency_type: ESizeMapDependencyType,
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESizeMapDependencyType(pub i32);
impl ESizeMapDependencyType {
    pub const ALL: ESizeMapDependencyType = ESizeMapDependencyType(0);
    pub const GAME: ESizeMapDependencyType = ESizeMapDependencyType(1);
    pub const EDITOR_ONLY: ESizeMapDependencyType = ESizeMapDependencyType(2);
}
