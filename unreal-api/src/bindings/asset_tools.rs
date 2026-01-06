#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_RENAME_REFERENCING_SOFT_OBJECT_PATHS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_RENAME_ASSETS_WITH_DIALOG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_RENAME_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_OPEN_EDITOR_FOR_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_MIGRATE_PACKAGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_IS_ASSET_READ_ONLY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_IMPORT_ASSET_TASKS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_IMPORT_ASSETS_WITH_DIALOG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_IMPORT_ASSETS_AUTOMATED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_FIND_SOFT_REFERENCES_TO_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_EXPORT_ASSETS_WITH_DIALOG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_EXPORT_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_DUPLICATE_ASSET_WITH_DIALOG_AND_TITLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_DUPLICATE_ASSET_WITH_DIALOG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_DUPLICATE_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_DIFF_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_DIFF_AGAINST_DEPOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_CREATE_UNIQUE_ASSET_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_CREATE_ASSET_WITH_DIALOG_ASYNC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_CREATE_ASSET_WITH_DIALOG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_CREATE_ASSET_ASYNC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_CREATE_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_BEGIN_ADVANCED_COPY_PACKAGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ASSET_TOOLS_HELPERS_GET_ASSET_TOOLS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetTools::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameReferencingSoftObjectPaths"),
            &raw mut U_ASSET_TOOLS_RENAME_REFERENCING_SOFT_OBJECT_PATHS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameAssetsWithDialog"),
            &raw mut U_ASSET_TOOLS_RENAME_ASSETS_WITH_DIALOG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameAssets"),
            &raw mut U_ASSET_TOOLS_RENAME_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenEditorForAssets"),
            &raw mut U_ASSET_TOOLS_OPEN_EDITOR_FOR_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MigratePackages"),
            &raw mut U_ASSET_TOOLS_MIGRATE_PACKAGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAssetReadOnly"),
            &raw mut U_ASSET_TOOLS_IS_ASSET_READ_ONLY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportAssetTasks"),
            &raw mut U_ASSET_TOOLS_IMPORT_ASSET_TASKS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportAssetsWithDialog"),
            &raw mut U_ASSET_TOOLS_IMPORT_ASSETS_WITH_DIALOG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportAssetsAutomated"),
            &raw mut U_ASSET_TOOLS_IMPORT_ASSETS_AUTOMATED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindSoftReferencesToObject"),
            &raw mut U_ASSET_TOOLS_FIND_SOFT_REFERENCES_TO_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportAssetsWithDialog"),
            &raw mut U_ASSET_TOOLS_EXPORT_ASSETS_WITH_DIALOG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportAssets"),
            &raw mut U_ASSET_TOOLS_EXPORT_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateAssetWithDialogAndTitle"),
            &raw mut U_ASSET_TOOLS_DUPLICATE_ASSET_WITH_DIALOG_AND_TITLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateAssetWithDialog"),
            &raw mut U_ASSET_TOOLS_DUPLICATE_ASSET_WITH_DIALOG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateAsset"),
            &raw mut U_ASSET_TOOLS_DUPLICATE_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DiffAssets"),
            &raw mut U_ASSET_TOOLS_DIFF_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DiffAgainstDepot"),
            &raw mut U_ASSET_TOOLS_DIFF_AGAINST_DEPOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateUniqueAssetName"),
            &raw mut U_ASSET_TOOLS_CREATE_UNIQUE_ASSET_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAssetWithDialogAsync"),
            &raw mut U_ASSET_TOOLS_CREATE_ASSET_WITH_DIALOG_ASYNC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAssetWithDialog"),
            &raw mut U_ASSET_TOOLS_CREATE_ASSET_WITH_DIALOG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAssetAsync"),
            &raw mut U_ASSET_TOOLS_CREATE_ASSET_ASYNC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAsset"),
            &raw mut U_ASSET_TOOLS_CREATE_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BeginAdvancedCopyPackages"),
            &raw mut U_ASSET_TOOLS_BEGIN_ADVANCED_COPY_PACKAGES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetToolsHelpers::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetTools"),
            &raw mut U_ASSET_TOOLS_HELPERS_GET_ASSET_TOOLS,
        );
    }
}
#[repr(C, align(8))]
pub struct FAssetRenameData {
    pub asset: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub new_package_path: FString,
    pub new_name: FString,
    __padding_end: [u8; 88],
}
impl FAssetRenameData {}
#[repr(C, align(8))]
pub struct FMigrationOptions {
    pub b_prompt: bool,
    pub b_ignore_dependencies: bool,
    pub asset_conflict: EAssetMigrationConflict,
    pub orphan_folder: FString,
}
impl FMigrationOptions {}
#[repr(C, align(8))]
pub struct UAssetToolsSettings {
    __padding_end: [u8; 120],
}
impl UAssetToolsSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetToolsSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct IAssetTools {}
#[repr(C, align(8))]
pub struct UAssetTools {
    __padding_end: [u8; 48],
}
impl UAssetTools {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetTools")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAssetToolsHelpers {
    __padding_end: [u8; 48],
}
impl UAssetToolsHelpers {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetToolsHelpers")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAdvancedCopyCustomization {
    __padding_end: [u8; 424],
}
impl UAdvancedCopyCustomization {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAdvancedCopyCustomization")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAssetDefinition_AssetTypeActionsProxy {
    __padding_end: [u8; 112],
}
impl UAssetDefinition_AssetTypeActionsProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_AssetTypeActionsProxy")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAssetToolsImpl {
    __padding_end: [u8; 1064],
}
impl UAssetToolsImpl {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetToolsImpl")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct FBeginAdvancedCopyPackages_OnCopyComplete {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCreateAssetAsync_OnComplete {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCreateAssetAsync_OnCancelled {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCreateAssetWithDialogAsync_OnComplete {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCreateAssetWithDialogAsync_OnCancelled {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EAssetMigrationConflict(pub u8);
impl EAssetMigrationConflict {
    pub const SKIP: EAssetMigrationConflict = EAssetMigrationConflict(0);
    pub const OVERWRITE: EAssetMigrationConflict = EAssetMigrationConflict(1);
    pub const CANCEL: EAssetMigrationConflict = EAssetMigrationConflict(2);
}
#[repr(transparent)]
pub struct EAssetRenameResult(pub u8);
impl EAssetRenameResult {
    pub const FAILURE: EAssetRenameResult = EAssetRenameResult(0);
    pub const SUCCESS: EAssetRenameResult = EAssetRenameResult(1);
    pub const PENDING: EAssetRenameResult = EAssetRenameResult(2);
}
#[repr(transparent)]
pub struct EAssetTypeActivationOpenedMethod(pub u8);
impl EAssetTypeActivationOpenedMethod {
    pub const EDIT: EAssetTypeActivationOpenedMethod = EAssetTypeActivationOpenedMethod(
        0,
    );
    pub const VIEW: EAssetTypeActivationOpenedMethod = EAssetTypeActivationOpenedMethod(
        1,
    );
}
