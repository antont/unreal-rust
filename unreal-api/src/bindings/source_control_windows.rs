#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct USourceControlMenuContext {
    pub selected_files: TArray<FString>,
}
pub struct USourceControlSettings {
    pub b_show_asset_type_column: bool,
    pub b_show_asset_last_modified_time_column: bool,
    pub b_show_asset_checked_out_by_column: bool,
}
pub struct USourceControlHistoryWidgetContext {}
pub struct USourceControlSubmitWidgetContext {}
