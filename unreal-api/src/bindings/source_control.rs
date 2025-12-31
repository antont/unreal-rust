#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FSourceControlState {
    pub filename: FString,
    pub b_is_valid: bool,
    pub b_is_unknown: bool,
    pub b_can_check_in: bool,
    pub b_can_check_out: bool,
    pub b_is_checked_out: bool,
    pub b_is_current: bool,
    pub b_is_source_controlled: bool,
    pub b_is_added: bool,
    pub b_is_deleted: bool,
    pub b_is_ignored: bool,
    pub b_can_edit: bool,
    pub b_can_delete: bool,
    pub b_is_modified: bool,
    pub b_can_add: bool,
    pub b_is_conflicted: bool,
    pub b_can_revert: bool,
    pub b_is_checked_out_other: bool,
    pub checked_out_other: FString,
    pub b_is_checked_out_in_other_branch: bool,
    pub b_is_modified_in_other_branch: bool,
    pub previous_user: FString,
}
pub struct USourceControlHelpers {}
pub struct USourceControlPreferences {
    pub b_enable_validation_tag: bool,
    pub b_should_delete_new_files_on_revert: bool,
    pub b_enable_uncontrolled_changelists: bool,
    pub collection_changelist_tags: TArray<FString>,
    pub specific_collection_changelist_tags: TMap<FName, FString>,
    pub b_requires_revision_control_to_rename_localizable_assets: bool,
}
pub struct FAsyncQueryFileState_FileStateCallback;
pub struct FAsyncQueryFileStates_FileStateCallback;
