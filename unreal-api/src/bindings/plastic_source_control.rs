#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UPlasticSourceControlProjectSettings {
    pub user_name_to_display_name: TMap<FString, FString>,
    pub b_hide_email_domain_in_username: bool,
    pub b_prompt_for_checkout_on_change: bool,
    pub limit_number_of_revisions_in_history: i32,
    pub b_show_branch_repository_column: bool,
    pub b_show_branch_created_by_column: bool,
    pub b_show_branch_date_column: bool,
    pub b_show_branch_comment_column: bool,
    pub b_show_lock_id_column: bool,
    pub b_show_lock_workspace_column: bool,
    pub b_show_lock_date_column: bool,
    pub b_show_lock_destination_branch_column: bool,
    pub b_show_changeset_created_by_column: bool,
    pub b_show_changeset_date_column: bool,
    pub b_show_changeset_comment_column: bool,
    pub b_show_changeset_branch_column: bool,
}
