#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FDetailsSectionSelection {
    pub section_names: TSet<FName>,
}
#[repr(C, align(8))]
pub struct FDetailsViewConfig {
    pub b_show_favorites_category: bool,
    pub b_show_all_advanced: bool,
    pub b_show_hidden_properties_while_playing: bool,
    pub b_show_all_children_if_category_matches: bool,
    pub b_show_only_keyable: bool,
    pub b_show_only_animated: bool,
    pub b_show_only_modified: bool,
    pub b_show_sections: bool,
    pub value_column_width: f32,
    pub selected_sections: TMap<FName, FDetailsSectionSelection>,
}
#[repr(C, align(4))]
pub struct FPropertyEditorPermissionTestStructA {
    pub a_one: i32,
    pub a_two: i32,
}
#[repr(C, align(4))]
pub struct FPropertyEditorPermissionTestStructB {
    pub b_one: i32,
    pub b_two: i32,
}
#[repr(C, align(4))]
pub struct FPropertyEditorPermissionTestStructC {
    pub c_one: i32,
    pub c_two: i32,
}
pub struct UDetailRowMenuContext {}
pub struct UDetailRowMenuContextPrivate {}
pub struct UDetailsViewPropertyHandleTestValueClass {}
pub struct UDetailsViewPropertyHandleTestClass {
    pub test_value_soft_ptr: TSoftObjectPtr<UDetailsViewPropertyHandleTestValueClass>,
    pub test_value_soft_ptr_array: TArray<
        TSoftObjectPtr<UDetailsViewPropertyHandleTestValueClass>,
    >,
}
pub struct UDetailsConfig {
    pub views: TMap<FName, FDetailsViewConfig>,
}
pub struct UEditConditionTestObject {
    pub bool_property: bool,
    pub enum_property: EditConditionTestEnum,
    pub byte_enum_property: EditConditionByteEnum,
    pub double_property: f64,
    pub integer_property: i32,
    pub flags_76: u8,
    pub u_object_ptr: UPtr<UObject>,
    pub soft_class_ptr: TSoftObjectPtr<UClass>,
    pub weak_object_ptr: TWeakObjectPtr<UObject>,
}
pub struct UPropertyEditorSinglePropertyTestClass {
    pub vector: FVector,
}
