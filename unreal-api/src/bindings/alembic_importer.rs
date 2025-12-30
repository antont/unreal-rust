#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UAlembicImportFactory {
    pub import_settings: UPtr<UAbcImportSettings>,
    pub b_show_option: bool,
}
