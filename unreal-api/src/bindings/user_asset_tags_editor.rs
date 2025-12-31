#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FPerTypeFavoriteUserAssetTags {
    pub favorite_user_asset_tags: TSet<FName>,
}
#[repr(C, align(8))]
pub struct FPerTaggedAssetBrowserSavedState {
    pub primary_filter_selection: TArray<FName>,
}
#[repr(C, align(1))]
pub struct FTaggedAssetBrowserConfigurationDataBase {}
#[repr(C, align(8))]
pub struct FTaggedAssetBrowserConfigurationData_Standalone {
    pub b_display_all_section: bool,
    pub standalone_filter_classes: TArray<TSubclassOf<UTaggedAssetBrowserFilterBase>>,
    pub extension_filter_classes: TArray<TSubclassOf<UTaggedAssetBrowserFilterBase>>,
}
#[repr(C, align(1))]
pub struct FTaggedAssetBrowserConfigurationData_Extension {}
#[repr(C, align(16))]
pub struct FTaggedAssetBrowserSectionIconData {
    pub b_use_texture_for_icon: bool,
    pub style_name: FName,
    pub icon: UPtr<crate::bindings::engine::UTexture2D>,
}
#[repr(C, align(4))]
pub struct FPerUserAssetTagProviderViewOptions {
    pub b_enabled: bool,
    pub menu_type: EUserAssetTagProviderMenuType,
}
#[repr(C, align(8))]
pub struct FUserAssetTagProviderViewOptions {
    pub per_provider_view_options: TMap<FName, FPerUserAssetTagProviderViewOptions>,
}
pub struct UAssetDefinition_TaggedAssetBrowserConfiguration {}
pub struct UAssetEditor_TaggedAssetBrowserConfiguration {
    pub object_to_edit: UPtr<crate::bindings::core_u_object::UObject>,
}
pub struct ULocalFavoriteUserAssetTagsConfig {
    pub favorite_user_asset_tags_per_class: TMap<
        crate::bindings::core_u_object::FSoftClassPath,
        FPerTypeFavoriteUserAssetTags,
    >,
    pub max_recent_user_asset_tags: i32,
}
pub struct UProjectUserAssetTagSettings {
    pub user_asset_tags_per_type: TMap<
        crate::bindings::core_u_object::FSoftClassPath,
        FPerTypeFavoriteUserAssetTags,
    >,
}
pub struct UTaggedAssetBrowserConfig {
    pub per_tagged_asset_browser_settings: TMap<FName, FPerTaggedAssetBrowserSavedState>,
    pub b_show_hidden_assets: bool,
    pub b_show_deprecated_assets: bool,
}
pub struct UTaggedAssetBrowserConfiguration {
    pub profile_name: FName,
    pub b_is_extension: bool,
    pub standalone_data: FTaggedAssetBrowserConfigurationData_Standalone,
    pub extension_data: FTaggedAssetBrowserConfigurationData_Extension,
    pub filter_root: UPtr<UTaggedAssetBrowserFilterRoot>,
}
pub struct UTaggedAssetBrowserConfigurationFactory {}
pub struct UTaggedAssetBrowserConfigurationHierarchyViewModel {}
pub struct UTaggedAssetBrowserFilterBase {}
pub struct UTaggedAssetBrowserFilterRoot {}
pub struct UTaggedAssetBrowserSection {
    pub icon_data: FTaggedAssetBrowserSectionIconData,
    pub filters: TArray<UPtr<UTaggedAssetBrowserFilterBase>>,
}
pub struct UTaggedAssetBrowserFilter_All {}
pub struct UTaggedAssetBrowserFilter_UserAssetTag {
    pub user_asset_tag: FName,
}
pub struct UTaggedAssetBrowserFilter_UserAssetTagCollection {
    pub name: FName,
    pub description: FText,
}
pub struct UTaggedAssetBrowserFilter_Recent {}
pub struct UTaggedAssetBrowserFilter_Directories {
    pub directory_paths: TArray<crate::bindings::core_u_object::FDirectoryPath>,
}
pub struct UTaggedAssetBrowserFilter_Class {
    pub classes: TArray<TSubclassOf<crate::bindings::core_u_object::UObject>>,
}
pub struct UUserAssetTagEditorContext {}
pub struct UTaggedAssetBrowserMenuContext {}
pub struct UUserAssetTagProvider {}
pub struct UUserAssetTagProvider_LocalAssetTypeFavorites {}
pub struct UUserAssetTagProvider_Project {}
pub struct UUserAssetTagsEditorConfig {
    pub b_sort_by_alphabet: bool,
    pub provider_view_options: FUserAssetTagProviderViewOptions,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUserAssetTagProviderMenuType(pub i32);
impl EUserAssetTagProviderMenuType {
    pub const SECTION: EUserAssetTagProviderMenuType = EUserAssetTagProviderMenuType(0);
    pub const SUB_MENU: EUserAssetTagProviderMenuType = EUserAssetTagProviderMenuType(1);
}
