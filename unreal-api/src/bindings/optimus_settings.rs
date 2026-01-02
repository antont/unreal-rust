#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UOptimusSettings {
    __padding_end: [u8; 208],
}
impl UOptimusSettings {}
#[repr(transparent)]
pub struct EOptimusDefaultDeformerMode(pub u8);
impl EOptimusDefaultDeformerMode {
    pub const NEVER: EOptimusDefaultDeformerMode = EOptimusDefaultDeformerMode(0);
    pub const OPT_IN: EOptimusDefaultDeformerMode = EOptimusDefaultDeformerMode(1);
    pub const ALWAYS: EOptimusDefaultDeformerMode = EOptimusDefaultDeformerMode(2);
}
