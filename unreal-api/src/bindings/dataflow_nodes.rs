#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct EDataflowWeightMapOverrideType(pub u8);
impl EDataflowWeightMapOverrideType {
    pub const REPLACE_ALL: EDataflowWeightMapOverrideType = EDataflowWeightMapOverrideType(
        0,
    );
    pub const ADD_DIFFERENCE: EDataflowWeightMapOverrideType = EDataflowWeightMapOverrideType(
        1,
    );
    pub const REPLACE_CHANGED: EDataflowWeightMapOverrideType = EDataflowWeightMapOverrideType(
        2,
    );
}
