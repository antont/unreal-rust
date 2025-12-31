#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FBehaviorTreeSchemaAction_AutoArrange {}
#[repr(C, align(8))]
pub struct FDecoratorSchemaAction_NewNode {
    pub node_template: UPtr<UBehaviorTreeDecoratorGraphNode>,
}
pub struct UAssetDefinition_BehaviorTree {}
pub struct UAssetDefinition_Blackboard {}
pub struct UBehaviorTreeDecoratorGraph {}
pub struct UBehaviorTreeDecoratorGraphNode {}
pub struct UBehaviorTreeDecoratorGraphNode_Decorator {
    pub node_instance: UPtr<crate::bindings::core_u_object::UObject>,
    pub class_data: crate::bindings::ai_graph::FGraphNodeClassData,
}
pub struct UBehaviorTreeDecoratorGraphNode_Logic {
    pub logic_mode: EDecoratorLogicMode,
}
pub struct UBehaviorTreeEditorTypes {}
pub struct UBehaviorTreeFactory {}
pub struct UBehaviorTreeGraph {
    pub mod_counter: i32,
    pub b_is_using_mod_counter: bool,
    pub root_node_class: TSubclassOf<UBehaviorTreeGraphNode_Root>,
}
pub struct UBehaviorTreeGraphNode {
    pub decorators: TArray<UPtr<UBehaviorTreeGraphNode>>,
    pub services: TArray<UPtr<UBehaviorTreeGraphNode>>,
    pub flags_376: u8,
}
pub struct UBehaviorTreeGraphNode_Composite {}
pub struct UBehaviorTreeGraphNode_CompositeDecorator {
    pub bound_graph: UPtr<crate::bindings::engine::UEdGraph>,
    pub composite_name: FString,
    pub flags_432: u8,
    pub parent_node_instance: UPtr<crate::bindings::ai_module::UBTCompositeNode>,
    pub cached_description: FString,
}
pub struct UBehaviorTreeGraphNode_Decorator {}
pub struct UBehaviorTreeGraphNode_Root {
    pub blackboard_asset: UPtr<crate::bindings::ai_module::UBlackboardData>,
}
pub struct UBehaviorTreeGraphNode_Service {}
pub struct UBehaviorTreeGraphNode_SimpleParallel {}
pub struct UBehaviorTreeGraphNode_Task {}
pub struct UBehaviorTreeGraphNode_SubtreeTask {}
pub struct UBlackboardDataFactory {}
pub struct UEdGraphSchema_BehaviorTree {}
pub struct UEdGraphSchema_BehaviorTreeDecorator {
    pub pc_boolean: FString,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDecoratorLogicMode(pub u8);
impl EDecoratorLogicMode {
    pub const SINK: EDecoratorLogicMode = EDecoratorLogicMode(0);
    pub const AND: EDecoratorLogicMode = EDecoratorLogicMode(1);
    pub const OR: EDecoratorLogicMode = EDecoratorLogicMode(2);
    pub const NOT: EDecoratorLogicMode = EDecoratorLogicMode(3);
}
