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
impl FSourceControlState {}
#[repr(C, align(8))]
pub struct USourceControlHelpers {
    __padding_end: [u8; 48],
}
impl USourceControlHelpers {}
#[repr(C, align(8))]
pub struct USourceControlPreferences {
    __padding_end: [u8; 216],
}
impl USourceControlPreferences {}
#[repr(transparent)]
pub struct FAsyncQueryFileState_FileStateCallback {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAsyncQueryFileStates_FileStateCallback {
    _opague: u8,
}
