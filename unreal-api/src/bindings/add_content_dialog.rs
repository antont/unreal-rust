#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct EFeaturePackDetailLevel(pub u8);
impl EFeaturePackDetailLevel {
    pub const STANDARD: EFeaturePackDetailLevel = EFeaturePackDetailLevel(0);
    pub const HIGH: EFeaturePackDetailLevel = EFeaturePackDetailLevel(1);
}
