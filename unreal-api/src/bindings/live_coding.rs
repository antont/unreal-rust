#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct ULiveCodingSettings {
    pub b_enabled: bool,
    pub startup: ELiveCodingStartupMode,
    pub b_enable_reinstancing: bool,
    pub b_automatically_compile_new_classes: bool,
    pub b_preload_engine_modules: bool,
    pub b_preload_engine_plugin_modules: bool,
    pub b_preload_project_modules: bool,
    pub b_preload_project_plugin_modules: bool,
    pub preload_named_modules: TArray<FName>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELiveCodingStartupMode(pub u8);
impl ELiveCodingStartupMode {
    pub const AUTOMATIC: ELiveCodingStartupMode = ELiveCodingStartupMode(0);
    pub const AUTOMATIC_BUT_HIDDEN: ELiveCodingStartupMode = ELiveCodingStartupMode(1);
    pub const MANUAL: ELiveCodingStartupMode = ELiveCodingStartupMode(2);
}
