#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAISchemaAction_AddComment {}
#[repr(C, align(8))]
pub struct FAISchemaAction_NewNode {
    pub node_template: UPtr<UAIGraphNode>,
}
#[repr(C, align(8))]
pub struct FAISchemaAction_NewSubNode {
    pub node_template: UPtr<UAIGraphNode>,
    pub parent_node: UPtr<UAIGraphNode>,
}
#[repr(C, align(8))]
pub struct FGraphNodeClassData {
    pub asset_name: FString,
    pub generated_class_package: FString,
    pub class_name: FString,
    pub category: FText,
}
pub struct UAIGraph {
    pub graph_version: i32,
}
pub struct UAIGraphNode {
    pub class_data: FGraphNodeClassData,
    pub node_instance: UPtr<crate::bindings::core_u_object::UObject>,
    pub parent_node: UPtr<UAIGraphNode>,
    pub sub_nodes: TArray<UPtr<UAIGraphNode>>,
    pub copy_sub_node_index: i32,
    pub flags_324: u8,
    pub error_message: FString,
}
pub struct UAIGraphSchema {}
pub struct UK2Node_AIMoveTo {}
