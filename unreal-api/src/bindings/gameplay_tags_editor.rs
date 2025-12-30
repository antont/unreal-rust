#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UAssetDefinition_GameplayTagAssetBase {}
pub struct UGameplayTagSearchFilter {}
pub struct UGameplayTagsK2Node_LiteralGameplayTag {}
pub struct UGameplayTagsK2Node_MultiCompareBase {
    pub number_of_pins: i32,
    pub pin_names: TArray<FName>,
}
pub struct UGameplayTagsK2Node_MultiCompareGameplayTagAssetInterface {}
pub struct UGameplayTagsK2Node_MultiCompareGameplayTagAssetInterfaceSingleTags {}
pub struct UGameplayTagsK2Node_MultiCompareGameplayTagContainer {}
pub struct UGameplayTagsK2Node_MultiCompareGameplayTagContainerSingleTags {}
pub struct UGameplayTagsK2Node_SwitchGameplayTag {
    pub pin_tags: TArray<FGameplayTag>,
    pub pin_names: TArray<FName>,
}
pub struct UGameplayTagsK2Node_SwitchGameplayTagContainer {
    pub pin_containers: TArray<FGameplayTagContainer>,
    pub pin_names: TArray<FName>,
}
