#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FBlueprintActionMenuItem {}
#[repr(C, align(8))]
pub struct FBlueprintCompiledData {
    pub intermediate_graphs: TArray<UPtr<UEdGraph>>,
}
#[repr(C, align(8))]
pub struct FBlueprintDragDropMenuItem {}
#[repr(C, align(8))]
pub struct FFavoritedBlueprintPaletteItem {}
#[repr(C, align(8))]
pub struct FBlueprintDependency {
    pub dependency_type: EBPDependencyType,
    pub package_name: FName,
    pub native_object_name: FName,
    pub hash: FString,
}
#[repr(C, align(8))]
pub struct FBlueprintDependencies {
    pub blueprint_dependencies: TArray<FBlueprintDependency>,
}
#[repr(C, align(8))]
pub struct FBPGraphClipboardData {
    pub graph_name: FName,
    pub graph_type: EGraphType,
    pub original_blueprint: TWeakObjectPtr<UBlueprint>,
    pub nodes_string: FString,
}
pub struct UBlueprintEditorToolMenuContext {}
pub struct USSCSEditorMenuContext {}
pub struct UBlueprintCompilerExtension {}
pub struct UBlueprintPaletteFavorites {
    pub custom_favorites: TArray<FString>,
    pub current_favorites: TArray<FFavoritedBlueprintPaletteItem>,
    pub current_profile: FString,
}
pub struct UJsonObjectGraphFunctionLibrary {}
pub struct USCSEditorExtensionContext {}
pub struct USubobjectEditorExtensionContext {}
