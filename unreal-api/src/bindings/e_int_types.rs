#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct EIntTypes(pub u8);
impl EIntTypes {
    pub const NEW_ENUMERATOR4: EIntTypes = EIntTypes(0);
    pub const NEW_ENUMERATOR5: EIntTypes = EIntTypes(1);
    pub const NEW_ENUMERATOR6: EIntTypes = EIntTypes(2);
    pub const NEW_ENUMERATOR7: EIntTypes = EIntTypes(3);
}
