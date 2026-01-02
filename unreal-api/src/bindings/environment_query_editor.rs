#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UActorFactoryEnvironmentQuery {
    __padding_end: [u8; 144],
}
impl UActorFactoryEnvironmentQuery {}
#[repr(C, align(8))]
pub struct UAssetDefinition_EnvironmentQuery {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_EnvironmentQuery {}
#[repr(C, align(8))]
pub struct UEdGraphSchema_EnvironmentQuery {
    __padding_end: [u8; 48],
}
impl UEdGraphSchema_EnvironmentQuery {}
#[repr(C, align(8))]
pub struct UEnvironmentQueryFactory {
    __padding_end: [u8; 136],
}
impl UEnvironmentQueryFactory {}
#[repr(C, align(8))]
pub struct UEnvironmentQueryGraph {
    __padding_end: [u8; 200],
}
impl UEnvironmentQueryGraph {}
#[repr(C, align(8))]
pub struct UEnvironmentQueryGraphNode {
    __padding_end: [u8; 344],
}
impl UEnvironmentQueryGraphNode {}
#[repr(C, align(8))]
pub struct UEnvironmentQueryGraphNode_Option {
    __padding_end: [u8; 376],
}
impl UEnvironmentQueryGraphNode_Option {}
#[repr(C, align(8))]
pub struct UEnvironmentQueryGraphNode_Root {
    __padding_end: [u8; 368],
}
impl UEnvironmentQueryGraphNode_Root {}
#[repr(C, align(8))]
pub struct UEnvironmentQueryGraphNode_Test {
    __padding_end: [u8; 368],
}
impl UEnvironmentQueryGraphNode_Test {}
