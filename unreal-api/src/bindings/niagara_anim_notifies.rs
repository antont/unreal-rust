#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FCurveParameterPair {
    pub anim_curve_name: FName,
    pub user_variable_name: FName,
}
impl FCurveParameterPair {}
#[repr(C, align(8))]
pub struct UAnimNotifyState_TimedNiagaraEffect {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub template: UPtr<crate::bindings::niagara::UNiagaraSystem>,
    pub socket_name: FName,
    pub location_offset: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FRotator,
    #[doc(hidden)]
    __padding_152: [u8; 24],
    pub b_apply_rate_scale_as_time_dilation: bool,
    pub b_destroy_at_end: bool,
    __padding_end: [u8; 6],
}
impl UAnimNotifyState_TimedNiagaraEffect {}
#[repr(C, align(8))]
pub struct UAnimNotifyState_TimedNiagaraEffectAdvanced {
    #[doc(hidden)]
    __padding_161: [u8; 161],
    pub b_apply_rate_scale_to_progress: bool,
    __padding_end: [u8; 118],
}
impl UAnimNotifyState_TimedNiagaraEffectAdvanced {}
#[repr(C, align(16))]
pub struct UAnimNotify_PlayNiagaraEffect {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub template: UPtr<crate::bindings::niagara::UNiagaraSystem>,
    pub location_offset: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FRotator,
    #[doc(hidden)]
    __padding_192: [u8; 72],
    pub flags_192: u8,
    pub socket_name: FName,
}
impl UAnimNotify_PlayNiagaraEffect {}
#[repr(transparent)]
pub struct ENiagaraAnimNotifyProgressType(pub u8);
impl ENiagaraAnimNotifyProgressType {
    pub const NONE: ENiagaraAnimNotifyProgressType = ENiagaraAnimNotifyProgressType(0);
    pub const FORWARD: ENiagaraAnimNotifyProgressType = ENiagaraAnimNotifyProgressType(
        1,
    );
    pub const REVERSE: ENiagaraAnimNotifyProgressType = ENiagaraAnimNotifyProgressType(
        2,
    );
}
