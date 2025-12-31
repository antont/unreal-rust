#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FEntity {
    pub id: u64,
}
#[repr(C, align(8))]
pub struct FRustEvent {
    pub guid: FString,
    pub event: FDynamicRustComponent,
}
#[repr(C, align(8))]
pub struct FDynamicRustComponent {
    pub fields: TMap<FString, FRustProperty>,
    pub name: FString,
}
#[repr(C, align(8))]
pub struct FRustProperty {
    pub tag: i32,
    pub float: f32,
    pub bool: bool,
    pub vector: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub class: TSubclassOf<crate::bindings::engine::AActor>,
    pub sound: UPtr<crate::bindings::engine::USoundBase>,
}
#[repr(C, align(8))]
pub struct FUuidViewNode {
    pub name: FString,
    pub id: crate::bindings::core_u_object::FGuid,
}
pub struct UAnimNotify_RustEvent {
    pub event: FRustEvent,
}
pub struct UUuid {}
pub struct UEntityComponent {
    pub components: TMap<FString, FDynamicRustComponent>,
}
pub struct UK2Node_GetComponentRust {
    pub selected_node: FUuidViewNode,
    pub index_pins: TArray<crate::bindings::core_u_object::FGuid>,
}
pub struct ARustActor {
    pub entity_component: UPtr<UEntityComponent>,
}
pub struct ARustBindingsActor {}
pub struct ARustGameModeBase {
    pub registered_classes: TArray<TSubclassOf<crate::bindings::engine::AActor>>,
}
pub struct UUEdGraphSchema_Rust {}
pub struct URustReflectionLibrary {}
