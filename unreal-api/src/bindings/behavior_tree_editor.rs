#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub node_instance: UPtr<UObject>,
    pub class_data: FGraphNodeClassData,
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
    pub bound_graph: UPtr<UEdGraph>,
    pub composite_name: FString,
    pub flags_432: u8,
    pub parent_node_instance: UPtr<UBTCompositeNode>,
    pub cached_description: FString,
}
pub struct UBehaviorTreeGraphNode_Decorator {}
pub struct UBehaviorTreeGraphNode_Root {
    pub blackboard_asset: UPtr<UBlackboardData>,
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
