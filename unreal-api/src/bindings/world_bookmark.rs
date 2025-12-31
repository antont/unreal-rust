#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FWorldBookmarkCategory {
    pub name: FName,
    pub color: crate::bindings::core_u_object::FColor,
    pub guid: crate::bindings::core_u_object::FGuid,
}
pub struct UAssetDefinition_WorldBookmark {}
pub struct UWorldBookmarkBrowserSettings {
    pub b_show_only_bookmarks_for_current_world: bool,
    pub b_show_only_uncontrolled_bookmarks: bool,
    pub b_show_only_favorite_bookmarks: bool,
    pub b_show_only_last_recently_used_bookmarks: bool,
    pub max_last_recently_used_items: i32,
    pub hidden_columns: TArray<FName>,
    pub view_mode: EWorldBookmarkBrowserViewMode,
}
pub struct UWorldBookmark {
    pub editor_state: crate::bindings::unreal_ed::FEditorStateCollection,
    pub category_guid: crate::bindings::core_u_object::FGuid,
    pub bookmark_guid: crate::bindings::core_u_object::FGuid,
    pub last_loaded_time_stamp_utc: crate::bindings::core_u_object::FDateTime,
    pub b_favorite: bool,
    pub bookmark_asset_path: FString,
}
pub struct UWorldBookmarkEditorSettings {
    pub categories: TArray<FWorldBookmarkCategory>,
}
pub struct UWorldBookmarkEditorPerProjectUserSettings {
    pub b_enable_default_bookmarks: bool,
    pub b_enable_home_bookmark: bool,
    pub home_bookmark: TSoftObjectPtr<UWorldBookmark>,
}
pub struct UWorldBookmarkFactory {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWorldBookmarkBrowserViewMode(pub u8);
impl EWorldBookmarkBrowserViewMode {
    pub const LIST_VIEW: EWorldBookmarkBrowserViewMode = EWorldBookmarkBrowserViewMode(
        0,
    );
    pub const TREE_VIEW: EWorldBookmarkBrowserViewMode = EWorldBookmarkBrowserViewMode(
        1,
    );
}
