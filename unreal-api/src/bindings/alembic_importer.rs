#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UAlembicImportFactory {
    pub import_settings: UPtr<crate::bindings::alembic_library::UAbcImportSettings>,
    pub b_show_option: bool,
}
