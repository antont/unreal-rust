#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FOptimusGraphSchemaAction_NewNode {
    pub node_class: TSubclassOf<crate::bindings::optimus_core::UOptimusNode>,
}
#[repr(C, align(8))]
pub struct FOptimusGraphSchemaAction_NewConstantValueNode {
    pub data_type: crate::bindings::optimus_core::FOptimusDataTypeRef,
}
#[repr(C, align(8))]
pub struct FOptimusGraphSchemaAction_NewDataInterfaceNode {
    pub data_interface_class: TSubclassOf<
        crate::bindings::optimus_core::UOptimusComputeDataInterface,
    >,
}
#[repr(C, align(8))]
pub struct FOptimusGraphSchemaAction_NewLoopTerminalNodes {}
#[repr(C, align(8))]
pub struct FOptimusGraphSchemaAction_NewCommentNode {}
#[repr(C, align(8))]
pub struct FOptimusGraphSchemaAction_NewFunctionReferenceNode {}
#[repr(C, align(8))]
pub struct FOptimusSchemaAction_Graph {}
#[repr(C, align(8))]
pub struct FOptimusSchemaAction_Binding {}
#[repr(C, align(8))]
pub struct FOptimusSchemaAction_Resource {}
#[repr(C, align(8))]
pub struct FOptimusSchemaAction_Variable {}
pub struct UOptimusDeformerFactory {}
pub struct UOptimusEditorGraph {}
pub struct UOptimusEditorGraphNode {
    pub model_node: UPtr<crate::bindings::optimus_core::UOptimusNode>,
}
pub struct UOptimusEditorGraphNode_Comment {}
pub struct UOptimusEditorGraphSchema {}
pub struct UOptimusSourceFactory {}
