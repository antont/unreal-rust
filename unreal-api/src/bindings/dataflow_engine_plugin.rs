#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FCollectionAttributeKey {
    pub attribute: FString,
    pub group: FString,
}
#[repr(C, align(8))]
pub struct FDataflowDynamicMeshArray {
    pub value: TArray<UPtr<UDynamicMesh>>,
}
pub struct ADataflowActor {
    pub dataflow_component: UPtr<UDataflowComponent>,
}
pub struct UDataflowComponent {}
