#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMeshBooleanDataflowNode {
    pub operation: EMeshBooleanOperationEnum,
    pub mesh1: UPtr<UDynamicMesh>,
    pub mesh2: UPtr<UDynamicMesh>,
    pub mesh: UPtr<UDynamicMesh>,
}
