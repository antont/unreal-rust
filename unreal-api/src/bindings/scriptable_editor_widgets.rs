#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UPropertyViewBase {
    pub object: TSoftObjectPtr<UObject>,
    pub soft_object_path_deprecated: FSoftObjectPath,
    pub b_auto_load_asset: bool,
    pub on_property_changed: FPropertyViewBase_OnPropertyChanged,
}
pub struct UDetailsView {
    pub b_allow_filtering: bool,
    pub b_allow_favorite_system: bool,
    pub b_show_modified_properties_option: bool,
    pub b_show_keyable_properties_option: bool,
    pub b_show_animated_properties_option: bool,
    pub column_width: f32,
    pub b_show_scroll_bar: bool,
    pub b_force_hidden_property_visibility: bool,
    pub view_identifier: FName,
    pub categories_to_show: TArray<FName>,
    pub properties_to_show: TArray<FName>,
    pub b_show_only_allowed: bool,
}
pub struct USinglePropertyView {
    pub property_name: FName,
    pub name_override: FText,
}
