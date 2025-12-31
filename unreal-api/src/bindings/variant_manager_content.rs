#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FFunctionCaller {
    pub function_name: FName,
    pub function_entry: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub display_order: u32,
}
#[repr(C, align(8))]
pub struct FCapturedPropSegment {
    pub property_name: FString,
    pub property_index: i32,
    pub component_name: FString,
}
#[repr(C, align(8))]
pub struct FVariantDependency {
    pub variant_set: TSoftObjectPtr<UVariantSet>,
    pub variant: TSoftObjectPtr<UVariant>,
    pub b_enabled: bool,
}
pub struct ULevelVariantSets {
    pub director_blueprint: UPtr<crate::bindings::core_u_object::UObject>,
    pub director_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub variant_sets: TArray<UPtr<UVariantSet>>,
}
pub struct ALevelVariantSetsActor {
    pub level_variant_sets: crate::bindings::core_u_object::FSoftObjectPath,
    pub director_instances: TMap<
        TSubclassOf<crate::bindings::core_u_object::UObject>,
        UPtr<ULevelVariantSetsFunctionDirector>,
    >,
}
pub struct ULevelVariantSetsFunctionDirector {}
pub struct UPropertyValue {
    pub properties_deprecated: TArray<TFieldPath<FProperty>>,
    pub property_indices_deprecated: TArray<i32>,
    pub captured_prop_segments: TArray<FCapturedPropSegment>,
    pub full_display_string: FString,
    pub property_setter_name: FName,
    pub property_setter_parameter_defaults: TMap<FString, FString>,
    pub b_has_recorded_data: bool,
    pub leaf_property_class_deprecated: TSubclassOf<
        crate::bindings::core_u_object::UObject,
    >,
    pub value_bytes: TArray<u8>,
    pub prop_category: EPropertyValueCategory,
    pub display_order: u32,
}
pub struct UPropertyValueTransform {}
pub struct UPropertyValueVisibility {}
pub struct UPropertyValueColor {}
pub struct UPropertyValueMaterial {}
pub struct UPropertyValueOption {}
pub struct UPropertyValueSoftObject {}
pub struct ASwitchActor {
    pub scene_component: UPtr<crate::bindings::engine::USceneComponent>,
    pub last_selected_option: i32,
}
pub struct UVariant {
    pub dependencies: TArray<FVariantDependency>,
    pub display_text_deprecated: FText,
    pub object_bindings: TArray<UPtr<UVariantObjectBinding>>,
    pub thumbnail: UPtr<crate::bindings::engine::UTexture2D>,
}
pub struct UVariantObjectBinding {
    pub cached_actor_label: FString,
    pub object_ptr: crate::bindings::core_u_object::FSoftObjectPath,
    pub lazy_object_ptr: TLazyObjectPtr<crate::bindings::core_u_object::UObject>,
    pub captured_properties: TArray<UPtr<UPropertyValue>>,
    pub function_callers: TArray<FFunctionCaller>,
}
pub struct UVariantSet {
    pub display_text_deprecated: FText,
    pub b_expanded: bool,
    pub variants: TArray<UPtr<UVariant>>,
    pub thumbnail: UPtr<crate::bindings::engine::UTexture2D>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EPropertyValueCategory(pub u8);
impl EPropertyValueCategory {
    pub const UNDEFINED: EPropertyValueCategory = EPropertyValueCategory(0);
    pub const GENERIC: EPropertyValueCategory = EPropertyValueCategory(1);
    pub const RELATIVE_LOCATION: EPropertyValueCategory = EPropertyValueCategory(2);
    pub const RELATIVE_ROTATION: EPropertyValueCategory = EPropertyValueCategory(4);
    pub const RELATIVE_SCALE3_D: EPropertyValueCategory = EPropertyValueCategory(8);
    pub const VISIBILITY: EPropertyValueCategory = EPropertyValueCategory(16);
    pub const MATERIAL: EPropertyValueCategory = EPropertyValueCategory(32);
    pub const COLOR: EPropertyValueCategory = EPropertyValueCategory(64);
    pub const OPTION: EPropertyValueCategory = EPropertyValueCategory(128);
}
