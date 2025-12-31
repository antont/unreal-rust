#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FOutputLogCategorySettings {
    pub name: FName,
    pub b_enabled: bool,
}
#[repr(C, align(8))]
pub struct FOutputLogFilterSettings {
    pub messages_filter: ELogLevelFilter,
    pub warnings_filter: ELogLevelFilter,
    pub errors_filter: ELogLevelFilter,
    pub filter_text: FText,
    pub categories: TArray<FOutputLogCategorySettings>,
    pub b_select_new_categories: bool,
}
pub struct UConsoleInputBoxMenuContext {}
pub struct UOutputLogMenuContext {}
pub struct UOutputLogSettings {
    pub log_font_size: i32,
    pub log_timestamp_mode: crate::bindings::core_u_object::ELogTimes,
    pub category_colorization_mode: ELogCategoryColorizationMode,
    pub b_cycle_to_output_log_drawer: bool,
    pub b_enable_output_log_word_wrap: bool,
    pub b_enable_output_log_clear_on_pie: bool,
    pub output_log_tab_filter: FOutputLogFilterSettings,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELogLevelFilter(pub u8);
impl ELogLevelFilter {
    pub const NONE: ELogLevelFilter = ELogLevelFilter(0);
    pub const ENABLED: ELogLevelFilter = ELogLevelFilter(1);
    pub const ALL: ELogLevelFilter = ELogLevelFilter(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELogCategoryColorizationMode(pub u8);
impl ELogCategoryColorizationMode {
    pub const NONE: ELogCategoryColorizationMode = ELogCategoryColorizationMode(0);
    pub const COLORIZE_WHOLE_LINE: ELogCategoryColorizationMode = ELogCategoryColorizationMode(
        1,
    );
    pub const COLORIZE_CATEGORY_ONLY: ELogCategoryColorizationMode = ELogCategoryColorizationMode(
        2,
    );
    pub const COLORIZE_CATEGORY_AS_BADGE: ELogCategoryColorizationMode = ELogCategoryColorizationMode(
        3,
    );
}
