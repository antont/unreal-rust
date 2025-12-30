#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub vector: FVector,
    pub rotation: FRotator,
    pub class: TSubclassOf<AActor>,
    pub sound: UPtr<USoundBase>,
}
#[repr(C, align(8))]
pub struct FUuidViewNode {
    pub name: FString,
    pub id: FGuid,
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
    pub index_pins: TArray<FGuid>,
}
pub struct ARustActor {
    pub entity_component: UPtr<UEntityComponent>,
}
pub struct ARustBindingsActor {}
pub struct ARustGameModeBase {
    pub registered_classes: TArray<TSubclassOf<AActor>>,
}
pub struct UUEdGraphSchema_Rust {}
pub struct URustReflectionLibrary {}
