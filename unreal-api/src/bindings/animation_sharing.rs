#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAnimationSharingStateProcessor {
    __padding_end: [u8; 96],
}
impl UAnimationSharingStateProcessor {}
#[repr(C, align(16))]
pub struct UAnimSharingStateInstance {
    #[doc(hidden)]
    __padding_1128: [u8; 1128],
    pub animation_to_play: UPtr<crate::bindings::engine::UAnimSequence>,
    pub permutation_time_offset: f32,
    pub play_rate: f32,
    pub b_state_bool: bool,
    __padding_end: [u8; 23],
}
impl UAnimSharingStateInstance {}
#[repr(C, align(16))]
pub struct UAnimSharingTransitionInstance {
    #[doc(hidden)]
    __padding_1128: [u8; 1128],
    pub from_component: TWeakObjectPtr<crate::bindings::engine::USkeletalMeshComponent>,
    pub to_component: TWeakObjectPtr<crate::bindings::engine::USkeletalMeshComponent>,
    pub blend_time: f32,
    pub b_blend_bool: bool,
    __padding_end: [u8; 3],
}
impl UAnimSharingTransitionInstance {}
#[repr(C, align(16))]
pub struct UAnimSharingAdditiveInstance {
    #[doc(hidden)]
    __padding_1128: [u8; 1128],
    pub base_component: TWeakObjectPtr<crate::bindings::engine::USkeletalMeshComponent>,
    pub additive_animation: TWeakObjectPtr<crate::bindings::engine::UAnimSequence>,
    pub alpha: f32,
    pub b_state_bool: bool,
    __padding_end: [u8; 3],
}
impl UAnimSharingAdditiveInstance {}
#[repr(C, align(8))]
pub struct UAnimSharingInstance {
    __padding_end: [u8; 648],
}
impl UAnimSharingInstance {}
#[repr(C, align(8))]
pub struct UAnimationSharingManager {
    __padding_end: [u8; 584],
}
impl UAnimationSharingManager {}
#[repr(C, align(8))]
pub struct UAnimationSharingSetup {
    __padding_end: [u8; 416],
}
impl UAnimationSharingSetup {}
