#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FMassFragment {}
#[repr(C, align(4))]
pub struct FMassDebugLogFragment {
    pub log_owner: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(1))]
pub struct FMassTag {}
#[repr(C, align(1))]
pub struct FMassChunkFragment {}
#[repr(C, align(1))]
pub struct FMassSharedFragment {}
#[repr(C, align(1))]
pub struct FMassConstSharedFragment {}
#[repr(C, align(8))]
pub struct FMassEntityHandle {
    pub index: i32,
    pub serial_number: i32,
}
#[repr(C, align(8))]
pub struct FMassGenericDebugEvent {}
#[repr(C, align(1))]
pub struct FMassRelation {}
#[repr(C, align(8))]
pub struct FMassRelationFragment {}
#[repr(C, align(1))]
pub struct FMassRelationMappingFragment {}
#[repr(C, align(8))]
pub struct FMassEntityView {}
#[repr(C, align(8))]
pub struct FMassObserversMap {
    pub container: TMap<
        UPtr<crate::bindings::core_u_object::UScriptStruct>,
        FMassRuntimePipeline,
    >,
}
#[repr(C, align(8))]
pub struct FMassRuntimePipeline {
    pub processors: TArray<UPtr<UMassProcessor>>,
}
#[repr(C, align(8))]
pub struct FMassObserverManager {
    pub fragment_observers: FMassObserversMap,
    pub tag_observers: FMassObserversMap,
}
#[repr(C, align(8))]
pub struct FMassObserverExecutionContext {}
#[repr(C, align(8))]
pub struct FMassProcessorClassCollection {
    pub class_collection: TArray<TSubclassOf<UMassProcessor>>,
}
#[repr(C, align(8))]
pub struct FMassEntityObserverClassesMap {
    pub container: TMap<
        UPtr<crate::bindings::core_u_object::UScriptStruct>,
        FMassProcessorClassCollection,
    >,
}
#[repr(C, align(8))]
pub struct FMassProcessingPhaseConfig {
    pub phase_name: FName,
    pub phase_group_class: TSubclassOf<UMassCompositeProcessor>,
    pub processor_cd_os: TArray<UPtr<UMassProcessor>>,
    pub phase_processor: UPtr<UMassCompositeProcessor>,
    pub description: FText,
}
#[repr(C, align(1))]
pub struct FProcessorAuxDataBase {}
#[repr(C, align(8))]
pub struct FMassProcessingContext_DEPRECATED {
    pub delta_seconds: f32,
    pub aux_data: crate::bindings::core_u_object::FInstancedStruct,
    pub b_flush_command_buffer: bool,
}
#[repr(C, align(8))]
pub struct FMassProcessorExecutionOrder {
    pub execute_in_group: FName,
    pub execute_before: TArray<FName>,
    pub execute_after: TArray<FName>,
}
#[repr(C, align(1))]
pub struct FMassChildOfRelation {}
#[repr(C, align(8))]
pub struct FMassChildOfFragment {
    pub parent: FMassEntityHandle,
}
pub struct UMassModuleSettings {}
pub struct UMassEntitySettings {
    pub chunk_memory_size: u32,
    pub dump_dependency_graph_file_name: FString,
    pub processing_phases_config: FMassProcessingPhaseConfig,
    pub processor_cd_os: TArray<UPtr<UMassProcessor>>,
}
pub struct UMassSubsystemBase {}
pub struct UMassEntitySubsystem {}
pub struct UMassProcessor {
    pub execution_order: FMassProcessorExecutionOrder,
    pub processing_phase: EMassProcessingPhase,
    pub execution_flags: u8,
    pub flags_114: u8,
    pub activation_state: EActivationState,
    pub execution_priority: i16,
}
pub struct UMassObserverProcessor {
    pub b_auto_register_with_observer_registry: bool,
    pub observed_type: UPtr<crate::bindings::core_u_object::UScriptStruct>,
}
pub struct UMassObserverRegistry {
    pub fragment_observers: FMassEntityObserverClassesMap,
    pub tag_observers: FMassEntityObserverClassesMap,
}
pub struct UMassCompositeProcessor {
    pub child_pipeline: FMassRuntimePipeline,
    pub group_name: FName,
}
pub struct UMassRelationObserver {}
pub struct UMassRelationEntityCreation {}
pub struct UMassRelationEntityGuardDog {}
pub struct UMassRelationEntityDestruction {}
pub struct UMassRelationRoleDestruction {}
pub struct UMassSettings {
    pub module_settings: TMap<FName, UPtr<UMassModuleSettings>>,
}
pub struct UMassTickableSubsystemBase {}
pub struct UMassChildOfRelationEntityCreation {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMassProcessingPhase(pub u8);
impl EMassProcessingPhase {
    pub const PRE_PHYSICS: EMassProcessingPhase = EMassProcessingPhase(0);
    pub const START_PHYSICS: EMassProcessingPhase = EMassProcessingPhase(1);
    pub const DURING_PHYSICS: EMassProcessingPhase = EMassProcessingPhase(2);
    pub const END_PHYSICS: EMassProcessingPhase = EMassProcessingPhase(3);
    pub const POST_PHYSICS: EMassProcessingPhase = EMassProcessingPhase(4);
    pub const FRAME_END: EMassProcessingPhase = EMassProcessingPhase(5);
    pub const MAX: EMassProcessingPhase = EMassProcessingPhase(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EActivationState(pub u8);
impl EActivationState {
    pub const INACTIVE: EActivationState = EActivationState(0);
    pub const ACTIVE: EActivationState = EActivationState(1);
    pub const ONE_SHOT: EActivationState = EActivationState(2);
}
