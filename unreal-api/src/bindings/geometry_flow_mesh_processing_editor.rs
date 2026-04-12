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
pub struct EGeometryFlow_AutoUVMethod(pub u8);
impl EGeometryFlow_AutoUVMethod {
    pub const PATCH_BUILDER: EGeometryFlow_AutoUVMethod = EGeometryFlow_AutoUVMethod(0);
    pub const UV_ATLAS: EGeometryFlow_AutoUVMethod = EGeometryFlow_AutoUVMethod(1);
    pub const X_ATLAS: EGeometryFlow_AutoUVMethod = EGeometryFlow_AutoUVMethod(2);
}
