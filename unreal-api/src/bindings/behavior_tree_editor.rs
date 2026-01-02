#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAssetDefinition_BehaviorTree {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_BehaviorTree {}
#[repr(C, align(8))]
pub struct UAssetDefinition_Blackboard {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_Blackboard {}
#[repr(C, align(8))]
pub struct UBehaviorTreeDecoratorGraph {
    __padding_end: [u8; 192],
}
impl UBehaviorTreeDecoratorGraph {}
#[repr(C, align(8))]
pub struct UBehaviorTreeDecoratorGraphNode {
    __padding_end: [u8; 200],
}
impl UBehaviorTreeDecoratorGraphNode {}
#[repr(C, align(8))]
pub struct UBehaviorTreeDecoratorGraphNode_Decorator {
    __padding_end: [u8; 304],
}
impl UBehaviorTreeDecoratorGraphNode_Decorator {}
#[repr(C, align(8))]
pub struct UBehaviorTreeDecoratorGraphNode_Logic {
    __padding_end: [u8; 208],
}
impl UBehaviorTreeDecoratorGraphNode_Logic {}
#[repr(C, align(8))]
pub struct UBehaviorTreeEditorTypes {
    __padding_end: [u8; 48],
}
impl UBehaviorTreeEditorTypes {}
#[repr(C, align(8))]
pub struct UBehaviorTreeFactory {
    __padding_end: [u8; 136],
}
impl UBehaviorTreeFactory {}
#[repr(C, align(8))]
pub struct UBehaviorTreeGraph {
    __padding_end: [u8; 216],
}
impl UBehaviorTreeGraph {}
#[repr(C, align(8))]
pub struct UBehaviorTreeGraphNode {
    __padding_end: [u8; 408],
}
impl UBehaviorTreeGraphNode {}
#[repr(C, align(8))]
pub struct UBehaviorTreeGraphNode_Composite {
    __padding_end: [u8; 408],
}
impl UBehaviorTreeGraphNode_Composite {}
#[repr(C, align(8))]
pub struct UBehaviorTreeGraphNode_CompositeDecorator {
    __padding_end: [u8; 488],
}
impl UBehaviorTreeGraphNode_CompositeDecorator {}
#[repr(C, align(8))]
pub struct UBehaviorTreeGraphNode_Decorator {
    __padding_end: [u8; 408],
}
impl UBehaviorTreeGraphNode_Decorator {}
#[repr(C, align(8))]
pub struct UBehaviorTreeGraphNode_Root {
    __padding_end: [u8; 416],
}
impl UBehaviorTreeGraphNode_Root {}
#[repr(C, align(8))]
pub struct UBehaviorTreeGraphNode_Service {
    __padding_end: [u8; 408],
}
impl UBehaviorTreeGraphNode_Service {}
#[repr(C, align(8))]
pub struct UBehaviorTreeGraphNode_SimpleParallel {
    __padding_end: [u8; 408],
}
impl UBehaviorTreeGraphNode_SimpleParallel {}
#[repr(C, align(8))]
pub struct UBehaviorTreeGraphNode_Task {
    __padding_end: [u8; 408],
}
impl UBehaviorTreeGraphNode_Task {}
#[repr(C, align(8))]
pub struct UBehaviorTreeGraphNode_SubtreeTask {
    __padding_end: [u8; 432],
}
impl UBehaviorTreeGraphNode_SubtreeTask {}
#[repr(C, align(8))]
pub struct UBlackboardDataFactory {
    __padding_end: [u8; 136],
}
impl UBlackboardDataFactory {}
#[repr(C, align(8))]
pub struct UEdGraphSchema_BehaviorTree {
    __padding_end: [u8; 80],
}
impl UEdGraphSchema_BehaviorTree {}
#[repr(C, align(8))]
pub struct UEdGraphSchema_BehaviorTreeDecorator {
    __padding_end: [u8; 64],
}
impl UEdGraphSchema_BehaviorTreeDecorator {}
#[repr(transparent)]
pub struct EDecoratorLogicMode(pub u8);
impl EDecoratorLogicMode {
    pub const SINK: EDecoratorLogicMode = EDecoratorLogicMode(0);
    pub const AND: EDecoratorLogicMode = EDecoratorLogicMode(1);
    pub const OR: EDecoratorLogicMode = EDecoratorLogicMode(2);
    pub const NOT: EDecoratorLogicMode = EDecoratorLogicMode(3);
}
