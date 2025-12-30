#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FGameplayTagContainerNetSerializerSerializationHelper {
    pub gameplay_tags: TArray<FGameplayTag>,
}
#[repr(C, align(4))]
pub struct FGameplayTag {
    pub tag_name: FName,
}
#[repr(C, align(8))]
pub struct FGameplayTagContainer {
    pub gameplay_tags: TArray<FGameplayTag>,
    pub parent_tags: TArray<FGameplayTag>,
}
#[repr(C, align(1))]
pub struct FGameplayTagCreationWidgetHelper {}
#[repr(C, align(8))]
pub struct FGameplayTagQuery {
    pub token_stream_version: i32,
    pub tag_dictionary: TArray<FGameplayTag>,
    pub query_token_stream: TArray<u8>,
    pub user_description: FString,
    pub auto_description: FString,
}
#[repr(C, align(8))]
pub struct FGameplayTagContainerNetSerializerConfig {}
#[repr(C, align(8))]
pub struct FGameplayTagNetSerializerConfig {}
#[repr(C, align(4))]
pub struct FGameplayTagRedirect {
    pub old_tag_name: FName,
    pub new_tag_name: FName,
}
#[repr(C, align(8))]
pub struct FGameplayTagTableRow {
    pub tag: FName,
    pub dev_comment: FString,
}
#[repr(C, align(8))]
pub struct FRestrictedGameplayTagTableRow {
    pub b_allow_non_restricted_children: bool,
}
#[repr(C, align(8))]
pub struct FGameplayTagSource {
    pub source_name: FName,
    pub source_type: EGameplayTagSourceType,
    pub source_tag_list: UPtr<UGameplayTagsList>,
    pub source_restricted_tag_list: UPtr<URestrictedGameplayTagsList>,
}
#[repr(C, align(8))]
pub struct FGameplayTagNode {}
#[repr(C, align(8))]
pub struct FGameplayTagCategoryRemap {
    pub base_category: FString,
    pub remap_categories: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FRestrictedConfigInfo {
    pub restricted_config_name: FString,
    pub owners: TArray<FString>,
}
pub struct UBlueprintGameplayTagLibrary {}
pub struct UGameplayTagAssetInterface {}
pub struct IGameplayTagAssetInterface {}
pub struct UEditableGameplayTagQuery {
    pub user_description: FString,
    pub root_expression: UPtr<UEditableGameplayTagQueryExpression>,
    pub tag_query_export_text_helper: FGameplayTagQuery,
}
pub struct UEditableGameplayTagQueryExpression {}
pub struct UEditableGameplayTagQueryExpression_AnyTagsMatch {
    pub tags: FGameplayTagContainer,
}
pub struct UEditableGameplayTagQueryExpression_AllTagsMatch {
    pub tags: FGameplayTagContainer,
}
pub struct UEditableGameplayTagQueryExpression_NoTagsMatch {
    pub tags: FGameplayTagContainer,
}
pub struct UEditableGameplayTagQueryExpression_AnyTagsExactMatch {
    pub tags: FGameplayTagContainer,
}
pub struct UEditableGameplayTagQueryExpression_AllTagsExactMatch {
    pub tags: FGameplayTagContainer,
}
pub struct UEditableGameplayTagQueryExpression_AnyExprMatch {
    pub expressions: TArray<UPtr<UEditableGameplayTagQueryExpression>>,
}
pub struct UEditableGameplayTagQueryExpression_AllExprMatch {
    pub expressions: TArray<UPtr<UEditableGameplayTagQueryExpression>>,
}
pub struct UEditableGameplayTagQueryExpression_NoExprMatch {
    pub expressions: TArray<UPtr<UEditableGameplayTagQueryExpression>>,
}
pub struct UGameplayTagsManager {
    pub tag_sources: TMap<FName, FGameplayTagSource>,
    pub gameplay_tag_tables: TArray<UPtr<UDataTable>>,
}
pub struct UGameplayTagsList {
    pub config_file_name: FString,
    pub gameplay_tag_redirects: TArray<FGameplayTagRedirect>,
    pub gameplay_tag_list: TArray<FGameplayTagTableRow>,
}
pub struct URestrictedGameplayTagsList {
    pub config_file_name: FString,
    pub restricted_gameplay_tag_list: TArray<FRestrictedGameplayTagTableRow>,
}
pub struct UGameplayTagsSettings {
    pub import_tags_from_config: bool,
    pub warn_on_invalid_tags: bool,
    pub clear_invalid_tags: bool,
    pub allow_editor_tag_unloading: bool,
    pub allow_game_tag_unloading: bool,
    pub fast_replication: bool,
    pub b_dynamic_replication: bool,
    pub invalid_tag_characters: FString,
    pub category_remapping: TArray<FGameplayTagCategoryRemap>,
    pub gameplay_tag_table_list: TArray<FSoftObjectPath>,
    pub commonly_replicated_tags: TArray<FName>,
    pub num_bits_for_container_size: i32,
    pub net_index_first_bit_segment: i32,
    pub restricted_config_files: TArray<FRestrictedConfigInfo>,
    pub restricted_tag_list: FString,
    pub new_tag_source: FString,
    pub cleanup_unused_tags: FString,
}
pub struct UGameplayTagsDeveloperSettings {
    pub developer_config_name: FString,
    pub favorite_tag_source: FName,
}
