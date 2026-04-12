#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct ControlOutputFormat(pub u8);
impl ControlOutputFormat {
    pub const HIERARCHY: ControlOutputFormat = ControlOutputFormat(0);
    pub const LIST: ControlOutputFormat = ControlOutputFormat(1);
    pub const CHILD: ControlOutputFormat = ControlOutputFormat(2);
}
