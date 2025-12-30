#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FCurveParameterPair {
    pub anim_curve_name: FName,
    pub user_variable_name: FName,
}
pub struct UAnimNotifyState_TimedNiagaraEffect {
    pub template: UPtr<UNiagaraSystem>,
    pub socket_name: FName,
    pub location_offset: FVector,
    pub rotation_offset: FRotator,
    pub scale: FVector,
    pub b_apply_rate_scale_as_time_dilation: bool,
    pub b_destroy_at_end: bool,
}
pub struct UAnimNotifyState_TimedNiagaraEffectAdvanced {
    pub notify_progress_type: ENiagaraAnimNotifyProgressType,
    pub b_apply_rate_scale_to_progress: bool,
    pub notify_progress_user_parameter: FName,
    pub anim_curves: TArray<FCurveParameterPair>,
    pub b_enable_normalized_notify_progress_deprecated: bool,
}
pub struct UAnimNotify_PlayNiagaraEffect {
    pub template: UPtr<UNiagaraSystem>,
    pub location_offset: FVector,
    pub rotation_offset: FRotator,
    pub scale: FVector,
    pub b_absolute_scale: bool,
    pub flags_192: u8,
    pub socket_name: FName,
}
