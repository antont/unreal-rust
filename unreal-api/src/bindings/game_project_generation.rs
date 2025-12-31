#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FTemplateReplacement {
    pub extensions: TArray<FString>,
    pub from: FString,
    pub to: FString,
    pub b_case_sensitive: bool,
}
#[repr(C, align(8))]
pub struct FTemplateFolderRename {
    pub from: FString,
    pub to: FString,
}
#[repr(C, align(8))]
pub struct FLocalizedTemplateString {
    pub language: FString,
    pub text: FString,
}
#[repr(C, align(8))]
pub struct FTemplateVariant {
    pub name: FName,
    pub localized_display_names: TArray<FLocalizedTemplateString>,
    pub localized_descriptions: TArray<FLocalizedTemplateString>,
    pub shared_content_packs: TArray<
        crate::bindings::add_content_dialog::FFeaturePackLevelSet,
    >,
}
#[repr(C, align(8))]
pub struct FTemplateCategoryDef {
    pub key: FName,
    pub localized_display_names: TArray<FLocalizedTemplateString>,
    pub localized_descriptions: TArray<FLocalizedTemplateString>,
    pub icon: FString,
    pub is_major_category: bool,
}
pub struct UClassTemplate {
    pub generated_base_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
}
pub struct UPluginClassTemplate {
    pub plugin_name: FString,
}
pub struct UClassTemplateEditorSubsystem {}
pub struct UTemplateProjectDefs {
    pub localized_display_names: TArray<FLocalizedTemplateString>,
    pub localized_descriptions: TArray<FLocalizedTemplateString>,
    pub folders_to_ignore: TArray<FString>,
    pub files_to_ignore: TArray<FString>,
    pub folder_renames: TArray<FTemplateFolderRename>,
    pub filename_replacements: TArray<FTemplateReplacement>,
    pub replacements_in_files: TArray<FTemplateReplacement>,
    pub sort_key: FString,
    pub categories: TArray<FName>,
    pub class_types: FString,
    pub asset_types: FString,
    pub b_allow_project_creation: bool,
    pub b_is_enterprise: bool,
    pub b_is_blank: bool,
    pub b_thumbnail_as_icon: bool,
    pub hidden_settings: TArray<ETemplateSetting>,
    pub packs_to_include: TArray<FString>,
    pub edit_detail_level_preference: crate::bindings::add_content_dialog::EFeaturePackDetailLevel,
    pub shared_content_packs: TArray<
        crate::bindings::add_content_dialog::FFeaturePackLevelSet,
    >,
    pub starter_content_deprecated: FString,
    pub variants: TArray<FTemplateVariant>,
}
pub struct UDefaultTemplateProjectDefs {}
pub struct UTemplateCategories {
    pub categories: TArray<FTemplateCategoryDef>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETemplateSetting(pub i32);
impl ETemplateSetting {
    pub const LANGUAGES: ETemplateSetting = ETemplateSetting(0);
    pub const HARDWARE_TARGET: ETemplateSetting = ETemplateSetting(1);
    pub const GRAPHICS_PRESET: ETemplateSetting = ETemplateSetting(2);
    pub const STARTER_CONTENT: ETemplateSetting = ETemplateSetting(3);
    pub const XR: ETemplateSetting = ETemplateSetting(4);
    pub const RAYTRACING_DEPRECATED: ETemplateSetting = ETemplateSetting(5);
    pub const VARIANTS: ETemplateSetting = ETemplateSetting(6);
    pub const ALL: ETemplateSetting = ETemplateSetting(7);
}
