#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UConsoleInputBoxMenuContext {
    __padding_end: [u8; 64],
}
impl UConsoleInputBoxMenuContext {}
#[repr(C, align(8))]
pub struct UOutputLogMenuContext {
    __padding_end: [u8; 64],
}
impl UOutputLogMenuContext {}
#[repr(C, align(8))]
pub struct UOutputLogSettings {
    __padding_end: [u8; 144],
}
impl UOutputLogSettings {}
#[repr(transparent)]
pub struct ELogLevelFilter(pub u8);
impl ELogLevelFilter {
    pub const NONE: ELogLevelFilter = ELogLevelFilter(0);
    pub const ENABLED: ELogLevelFilter = ELogLevelFilter(1);
    pub const ALL: ELogLevelFilter = ELogLevelFilter(2);
}
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
