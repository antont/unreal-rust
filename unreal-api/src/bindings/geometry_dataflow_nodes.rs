#![allow(clippy::all)]
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
pub struct EMeshBooleanOperationEnum(pub u8);
impl EMeshBooleanOperationEnum {
    pub const DATAFLOW_MESH_BOOLEAN_UNION: EMeshBooleanOperationEnum = EMeshBooleanOperationEnum(
        0,
    );
    pub const DATAFLOW_MESH_BOOLEAN_INTERSECT: EMeshBooleanOperationEnum = EMeshBooleanOperationEnum(
        1,
    );
    pub const DATAFLOW_MESH_BOOLEAN_DIFFERENCE: EMeshBooleanOperationEnum = EMeshBooleanOperationEnum(
        2,
    );
}
