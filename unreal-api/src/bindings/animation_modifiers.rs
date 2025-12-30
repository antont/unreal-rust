#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UAnimationModifierSettings {
    pub default_animation_modifiers: TArray<TSubclassOf<UAnimationModifier>>,
    pub b_apply_animation_modifiers_on_import: bool,
}
pub struct UAnimationModifier {
    pub b_reapply_post_owner_change: bool,
    pub revision_guid: FGuid,
    pub applied_guid_deprecated: FGuid,
    pub stored_native_revision: i32,
    pub previously_applied_modifier_deprecated: UPtr<UAnimationModifier>,
}
pub struct UAnimationModifiersAssetUserData {
    pub animation_modifier_instances: TArray<UPtr<UAnimationModifier>>,
    pub applied_modifiers: TMap<FSoftObjectPath, UPtr<UAnimationModifier>>,
}
