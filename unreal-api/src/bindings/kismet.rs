#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FBlueprintActionMenuItem {}
#[repr(C, align(8))]
pub struct FBlueprintCompiledData {
    pub intermediate_graphs: TArray<UPtr<crate::bindings::engine::UEdGraph>>,
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
    pub graph_type: crate::bindings::engine::EGraphType,
    pub original_blueprint: TWeakObjectPtr<crate::bindings::engine::UBlueprint>,
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBPDependencyType(pub i32);
impl EBPDependencyType {
    pub const ASSET: EBPDependencyType = EBPDependencyType(0);
    pub const STRUCT: EBPDependencyType = EBPDependencyType(1);
    pub const CLASS: EBPDependencyType = EBPDependencyType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFiBIndexAllPermission(pub i32);
impl EFiBIndexAllPermission {
    pub const NONE: EFiBIndexAllPermission = EFiBIndexAllPermission(0);
    pub const LOAD_ONLY: EFiBIndexAllPermission = EFiBIndexAllPermission(1);
    pub const CHECKOUT_AND_RESAVE: EFiBIndexAllPermission = EFiBIndexAllPermission(2);
}
