#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAnimationSetup {
    pub anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
    pub anim_blueprint: TSubclassOf<UAnimSharingStateInstance>,
    pub num_randomized_instances: crate::bindings::core_u_object::FPerPlatformInt,
    pub enabled: crate::bindings::core_u_object::FPerPlatformBool,
}
#[repr(C, align(8))]
pub struct FAnimationStateEntry {
    pub state: u8,
    pub animation_setups: TArray<FAnimationSetup>,
    pub b_on_demand: bool,
    pub b_additive: bool,
    pub blend_time: f32,
    pub b_return_to_previous_state: bool,
    pub b_set_next_state: bool,
    pub next_state: u8,
    pub maximum_number_of_concurrent_instances: crate::bindings::core_u_object::FPerPlatformInt,
    pub wiggle_time_percentage: f32,
    pub b_requires_curves: bool,
}
#[repr(C, align(8))]
pub struct FPerSkeletonAnimationSharingSetup {
    pub skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub blend_anim_blueprint: TSubclassOf<UAnimSharingTransitionInstance>,
    pub additive_anim_blueprint: TSubclassOf<UAnimSharingAdditiveInstance>,
    pub state_processor_class: TSubclassOf<UAnimationSharingStateProcessor>,
    pub b_enable_material_parameter_caching: bool,
    pub animation_states: TArray<FAnimationStateEntry>,
}
#[repr(C, align(8))]
pub struct FAnimationSharingScalability {
    pub use_blend_transitions: crate::bindings::core_u_object::FPerPlatformBool,
    pub blend_significance_value: crate::bindings::core_u_object::FPerPlatformFloat,
    pub maximum_number_concurrent_blends: crate::bindings::core_u_object::FPerPlatformInt,
    pub tick_significance_value: crate::bindings::core_u_object::FPerPlatformFloat,
}
#[repr(C, align(8))]
pub struct FTickAnimationSharingFunction {}
pub struct UAnimationSharingStateProcessor {
    pub animation_state_enum: TSoftObjectPtr<crate::bindings::core_u_object::UEnum>,
}
pub struct UAnimSharingStateInstance {
    pub animation_to_play: UPtr<crate::bindings::engine::UAnimSequence>,
    pub permutation_time_offset: f32,
    pub play_rate: f32,
    pub b_state_bool: bool,
    pub instance: UPtr<UAnimSharingInstance>,
}
pub struct UAnimSharingTransitionInstance {
    pub from_component: TWeakObjectPtr<crate::bindings::engine::USkeletalMeshComponent>,
    pub to_component: TWeakObjectPtr<crate::bindings::engine::USkeletalMeshComponent>,
    pub blend_time: f32,
    pub b_blend_bool: bool,
}
pub struct UAnimSharingAdditiveInstance {
    pub base_component: TWeakObjectPtr<crate::bindings::engine::USkeletalMeshComponent>,
    pub additive_animation: TWeakObjectPtr<crate::bindings::engine::UAnimSequence>,
    pub alpha: f32,
    pub b_state_bool: bool,
}
pub struct UAnimSharingInstance {
    pub registered_actors: TArray<UPtr<crate::bindings::engine::AActor>>,
    pub state_processor: UPtr<UAnimationSharingStateProcessor>,
    pub used_animation_sequences: TArray<UPtr<crate::bindings::engine::UAnimSequence>>,
    pub state_enum: UPtr<crate::bindings::core_u_object::UEnum>,
    pub sharing_actor: UPtr<crate::bindings::engine::AActor>,
}
pub struct UAnimationSharingManager {
    pub skeletons: TArray<UPtr<crate::bindings::engine::USkeleton>>,
    pub per_skeleton_data: TArray<UPtr<UAnimSharingInstance>>,
}
pub struct UAnimationSharingSetup {
    pub skeleton_setups: TArray<FPerSkeletonAnimationSharingSetup>,
    pub scalability_settings: FAnimationSharingScalability,
}
