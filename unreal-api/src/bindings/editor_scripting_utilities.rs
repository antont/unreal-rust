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
pub static mut U_EDITOR_ASSET_LIBRARY_SYNC_BROWSER_TO_OBJECTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_SET_METADATA_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_SAVE_LOADED_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_SAVE_LOADED_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_SAVE_DIRECTORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_SAVE_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_RENAME_LOADED_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_RENAME_DIRECTORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_RENAME_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_REMOVE_METADATA_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_MAKE_DIRECTORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_LOAD_BLUEPRINT_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_LOAD_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_LIST_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_LIST_ASSET_BY_TAG_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_GET_TAG_VALUES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_GET_PROJECT_ROOT_ASSET_DIRECTORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_GET_PATH_NAME_FOR_LOADED_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_GET_PACKAGE_FOR_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_GET_METADATA_TAG_VALUES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_GET_METADATA_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_FIND_PACKAGE_REFERENCERS_FOR_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_FIND_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_DUPLICATE_LOADED_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_DUPLICATE_DIRECTORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_DUPLICATE_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_DOES_DIRECTORY_HAVE_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_DOES_DIRECTORY_EXIST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_DOES_ASSET_EXIST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_DO_ASSETS_EXIST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_DELETE_LOADED_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_DELETE_LOADED_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_DELETE_DIRECTORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_DELETE_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_CONSOLIDATE_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_CHECKOUT_LOADED_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_CHECKOUT_LOADED_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_CHECKOUT_DIRECTORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_ASSET_LIBRARY_CHECKOUT_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_DIALOG_LIBRARY_SHOW_SUPPRESSABLE_WARNING_DIALOG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_DIALOG_LIBRARY_SHOW_OBJECTS_DETAILS_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_DIALOG_LIBRARY_SHOW_OBJECT_DETAILS_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_DIALOG_LIBRARY_SHOW_MESSAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_FILTER_LIBRARY_BY_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_FILTER_LIBRARY_BY_LEVEL_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_FILTER_LIBRARY_BY_LAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_FILTER_LIBRARY_BY_ID_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_FILTER_LIBRARY_BY_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_FILTER_LIBRARY_BY_ACTOR_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_FILTER_LIBRARY_BY_ACTOR_LABEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_SPAWN_ACTOR_FROM_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_SPAWN_ACTOR_FROM_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_SET_SELECTED_LEVEL_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_SET_LEVEL_VIEWPORT_CAMERA_INFO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_SET_CURRENT_LEVEL_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_SET_ACTOR_SELECTION_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_SELECT_NOTHING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_SAVE_CURRENT_LEVEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_SAVE_ALL_DIRTY_LEVELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_REPLACE_SELECTED_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MESHES_ON_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MESHES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MATERIALS_ON_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MATERIALS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_PILOT_LEVEL_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_NEW_LEVEL_FROM_TEMPLATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_NEW_LEVEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_MERGE_STATIC_MESH_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_LOAD_LEVEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_JOIN_STATIC_MESH_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_GET_SELECTED_LEVEL_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_GET_PIE_WORLDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_GET_LEVEL_VIEWPORT_CAMERA_INFO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_GET_GAME_WORLD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_GET_EDITOR_WORLD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_GET_ALL_LEVEL_ACTORS_COMPONENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_GET_ALL_LEVEL_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_GET_ACTOR_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_EJECT_PILOT_LEVEL_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_EDITOR_SET_GAME_VIEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_EDITOR_PLAY_SIMULATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_EDITOR_INVALIDATE_VIEWPORTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_EDITOR_END_PLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_DESTROY_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_CREATE_PROXY_MESH_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_CONVERT_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_EDITOR_LEVEL_LIBRARY_CLEAR_ACTOR_SELECTION_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_STRIP_LOD_GEOMETRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_SET_LOD_BUILD_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_RENAME_SOCKET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_REMOVE_LO_DS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_REIMPORT_ALL_CUSTOM_LO_DS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_REGENERATE_LOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_IMPORT_LOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_GET_NUM_VERTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_GET_LOD_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_GET_LOD_BUILD_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_CREATE_PHYSICS_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LODS_WITH_NOTIFICATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LODS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LOD_REDUCTION_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LOD_FROM_STATIC_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LOD_BUILD_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_GENERATE_LIGHTMAP_U_VS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_CONVEX_DECOMPOSITION_COLLISIONS_WITH_NOTIFICATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_CONVEX_DECOMPOSITION_COLLISIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_ALLOW_CPU_ACCESS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_UV_CHANNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_LODS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_COLLISIONS_WITH_NOTIFICATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_COLLISIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REIMPORT_ALL_CUSTOM_LO_DS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_IS_SECTION_COLLISION_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_INSERT_UV_CHANNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_IMPORT_LOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_HAS_VERTEX_COLORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_HAS_INSTANCE_VERTEX_COLORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_SIMPLE_COLLISION_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_NUM_UV_CHANNELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_NUMBER_VERTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_NUMBER_MATERIALS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_SCREEN_SIZES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_REDUCTION_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_BUILD_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_CONVEX_COLLISION_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_COLLISION_COMPLEXITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GENERATE_PLANAR_UV_CHANNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GENERATE_CYLINDRICAL_UV_CHANNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GENERATE_BOX_UV_CHANNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ENABLE_SECTION_COLLISION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ENABLE_SECTION_CAST_SHADOW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_BULK_SET_CONVEX_DECOMPOSITION_COLLISIONS_WITH_NOTIFICATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_BULK_SET_CONVEX_DECOMPOSITION_COLLISIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ADD_UV_CHANNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ADD_SIMPLE_COLLISIONS_WITH_NOTIFICATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ADD_SIMPLE_COLLISIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorAssetLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SyncBrowserToObjects"),
            &raw mut U_EDITOR_ASSET_LIBRARY_SYNC_BROWSER_TO_OBJECTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMetadataTag"),
            &raw mut U_EDITOR_ASSET_LIBRARY_SET_METADATA_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveLoadedAssets"),
            &raw mut U_EDITOR_ASSET_LIBRARY_SAVE_LOADED_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveLoadedAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_SAVE_LOADED_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveDirectory"),
            &raw mut U_EDITOR_ASSET_LIBRARY_SAVE_DIRECTORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_SAVE_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameLoadedAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_RENAME_LOADED_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameDirectory"),
            &raw mut U_EDITOR_ASSET_LIBRARY_RENAME_DIRECTORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_RENAME_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveMetadataTag"),
            &raw mut U_EDITOR_ASSET_LIBRARY_REMOVE_METADATA_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeDirectory"),
            &raw mut U_EDITOR_ASSET_LIBRARY_MAKE_DIRECTORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadBlueprintClass"),
            &raw mut U_EDITOR_ASSET_LIBRARY_LOAD_BLUEPRINT_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_LOAD_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ListAssets"),
            &raw mut U_EDITOR_ASSET_LIBRARY_LIST_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ListAssetByTagValue"),
            &raw mut U_EDITOR_ASSET_LIBRARY_LIST_ASSET_BY_TAG_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTagValues"),
            &raw mut U_EDITOR_ASSET_LIBRARY_GET_TAG_VALUES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetProjectRootAssetDirectory"),
            &raw mut U_EDITOR_ASSET_LIBRARY_GET_PROJECT_ROOT_ASSET_DIRECTORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPathNameForLoadedAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_GET_PATH_NAME_FOR_LOADED_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPackageForObject"),
            &raw mut U_EDITOR_ASSET_LIBRARY_GET_PACKAGE_FOR_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetadataTagValues"),
            &raw mut U_EDITOR_ASSET_LIBRARY_GET_METADATA_TAG_VALUES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetadataTag"),
            &raw mut U_EDITOR_ASSET_LIBRARY_GET_METADATA_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindPackageReferencersForAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_FIND_PACKAGE_REFERENCERS_FOR_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindAssetData"),
            &raw mut U_EDITOR_ASSET_LIBRARY_FIND_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateLoadedAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_DUPLICATE_LOADED_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateDirectory"),
            &raw mut U_EDITOR_ASSET_LIBRARY_DUPLICATE_DIRECTORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_DUPLICATE_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesDirectoryHaveAssets"),
            &raw mut U_EDITOR_ASSET_LIBRARY_DOES_DIRECTORY_HAVE_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesDirectoryExist"),
            &raw mut U_EDITOR_ASSET_LIBRARY_DOES_DIRECTORY_EXIST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesAssetExist"),
            &raw mut U_EDITOR_ASSET_LIBRARY_DOES_ASSET_EXIST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoAssetsExist"),
            &raw mut U_EDITOR_ASSET_LIBRARY_DO_ASSETS_EXIST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteLoadedAssets"),
            &raw mut U_EDITOR_ASSET_LIBRARY_DELETE_LOADED_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteLoadedAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_DELETE_LOADED_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteDirectory"),
            &raw mut U_EDITOR_ASSET_LIBRARY_DELETE_DIRECTORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_DELETE_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConsolidateAssets"),
            &raw mut U_EDITOR_ASSET_LIBRARY_CONSOLIDATE_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckoutLoadedAssets"),
            &raw mut U_EDITOR_ASSET_LIBRARY_CHECKOUT_LOADED_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckoutLoadedAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_CHECKOUT_LOADED_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckoutDirectory"),
            &raw mut U_EDITOR_ASSET_LIBRARY_CHECKOUT_DIRECTORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckoutAsset"),
            &raw mut U_EDITOR_ASSET_LIBRARY_CHECKOUT_ASSET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorDialogLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowSuppressableWarningDialog"),
            &raw mut U_EDITOR_DIALOG_LIBRARY_SHOW_SUPPRESSABLE_WARNING_DIALOG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowObjectsDetailsView"),
            &raw mut U_EDITOR_DIALOG_LIBRARY_SHOW_OBJECTS_DETAILS_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowObjectDetailsView"),
            &raw mut U_EDITOR_DIALOG_LIBRARY_SHOW_OBJECT_DETAILS_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowMessage"),
            &raw mut U_EDITOR_DIALOG_LIBRARY_SHOW_MESSAGE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorFilterLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BySelection"),
            &raw mut U_EDITOR_FILTER_LIBRARY_BY_SELECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ByLevelName"),
            &raw mut U_EDITOR_FILTER_LIBRARY_BY_LEVEL_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ByLayer"),
            &raw mut U_EDITOR_FILTER_LIBRARY_BY_LAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ByIDName"),
            &raw mut U_EDITOR_FILTER_LIBRARY_BY_ID_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ByClass"),
            &raw mut U_EDITOR_FILTER_LIBRARY_BY_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ByActorTag"),
            &raw mut U_EDITOR_FILTER_LIBRARY_BY_ACTOR_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ByActorLabel"),
            &raw mut U_EDITOR_FILTER_LIBRARY_BY_ACTOR_LABEL,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorLevelLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnActorFromObject"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_SPAWN_ACTOR_FROM_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnActorFromClass"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_SPAWN_ACTOR_FROM_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSelectedLevelActors"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_SET_SELECTED_LEVEL_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLevelViewportCameraInfo"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_SET_LEVEL_VIEWPORT_CAMERA_INFO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurrentLevelByName"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_SET_CURRENT_LEVEL_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetActorSelectionState"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_SET_ACTOR_SELECTION_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectNothing"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_SELECT_NOTHING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveCurrentLevel"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_SAVE_CURRENT_LEVEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveAllDirtyLevels"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_SAVE_ALL_DIRTY_LEVELS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceSelectedActors"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_REPLACE_SELECTED_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceMeshComponentsMeshesOnActors"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MESHES_ON_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceMeshComponentsMeshes"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MESHES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceMeshComponentsMaterialsOnActors"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MATERIALS_ON_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceMeshComponentsMaterials"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MATERIALS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PilotLevelActor"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_PILOT_LEVEL_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NewLevelFromTemplate"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_NEW_LEVEL_FROM_TEMPLATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NewLevel"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_NEW_LEVEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MergeStaticMeshActors"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_MERGE_STATIC_MESH_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadLevel"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_LOAD_LEVEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("JoinStaticMeshActors"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_JOIN_STATIC_MESH_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedLevelActors"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_GET_SELECTED_LEVEL_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPIEWorlds"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_GET_PIE_WORLDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLevelViewportCameraInfo"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_GET_LEVEL_VIEWPORT_CAMERA_INFO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGameWorld"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_GET_GAME_WORLD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEditorWorld"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_GET_EDITOR_WORLD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllLevelActorsComponents"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_GET_ALL_LEVEL_ACTORS_COMPONENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllLevelActors"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_GET_ALL_LEVEL_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActorReference"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_GET_ACTOR_REFERENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EjectPilotLevelActor"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_EJECT_PILOT_LEVEL_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorSetGameView"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_EDITOR_SET_GAME_VIEW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorPlaySimulate"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_EDITOR_PLAY_SIMULATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorInvalidateViewports"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_EDITOR_INVALIDATE_VIEWPORTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorEndPlay"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_EDITOR_END_PLAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DestroyActor"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_DESTROY_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateProxyMeshActor"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_CREATE_PROXY_MESH_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertActors"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_CONVERT_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearActorSelectionSet"),
            &raw mut U_EDITOR_LEVEL_LIBRARY_CLEAR_ACTOR_SELECTION_SET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDEPRECATED_EditorSkeletalMeshLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StripLODGeometry"),
            &raw mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_STRIP_LOD_GEOMETRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLodBuildSettings"),
            &raw mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_SET_LOD_BUILD_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameSocket"),
            &raw mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_RENAME_SOCKET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveLODs"),
            &raw mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_REMOVE_LO_DS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReimportAllCustomLODs"),
            &raw mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_REIMPORT_ALL_CUSTOM_LO_DS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegenerateLOD"),
            &raw mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_REGENERATE_LOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportLOD"),
            &raw mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_IMPORT_LOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumVerts"),
            &raw mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_GET_NUM_VERTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLODCount"),
            &raw mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_GET_LOD_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLodBuildSettings"),
            &raw mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_GET_LOD_BUILD_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePhysicsAsset"),
            &raw mut UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_CREATE_PHYSICS_ASSET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDEPRECATED_EditorStaticMeshLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLodsWithNotification"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LODS_WITH_NOTIFICATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLods"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LODS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLodReductionSettings"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LOD_REDUCTION_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLodFromStaticMesh"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LOD_FROM_STATIC_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLodBuildSettings"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LOD_BUILD_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGenerateLightmapUVs"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_GENERATE_LIGHTMAP_U_VS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "SetConvexDecompositionCollisionsWithNotification",
            ),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_CONVEX_DECOMPOSITION_COLLISIONS_WITH_NOTIFICATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetConvexDecompositionCollisions"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_CONVEX_DECOMPOSITION_COLLISIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAllowCPUAccess"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_ALLOW_CPU_ACCESS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveUVChannel"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_UV_CHANNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveLods"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_LODS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCollisionsWithNotification"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_COLLISIONS_WITH_NOTIFICATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCollisions"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_COLLISIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReimportAllCustomLODs"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REIMPORT_ALL_CUSTOM_LO_DS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSectionCollisionEnabled"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_IS_SECTION_COLLISION_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InsertUVChannel"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_INSERT_UV_CHANNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportLOD"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_IMPORT_LOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasVertexColors"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_HAS_VERTEX_COLORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasInstanceVertexColors"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_HAS_INSTANCE_VERTEX_COLORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSimpleCollisionCount"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_SIMPLE_COLLISION_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumUVChannels"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_NUM_UV_CHANNELS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumberVerts"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_NUMBER_VERTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumberMaterials"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_NUMBER_MATERIALS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLodScreenSizes"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_SCREEN_SIZES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLodReductionSettings"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_REDUCTION_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLodCount"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLodBuildSettings"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_BUILD_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConvexCollisionCount"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_CONVEX_COLLISION_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollisionComplexity"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_COLLISION_COMPLEXITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GeneratePlanarUVChannel"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GENERATE_PLANAR_UV_CHANNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateCylindricalUVChannel"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GENERATE_CYLINDRICAL_UV_CHANNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateBoxUVChannel"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GENERATE_BOX_UV_CHANNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnableSectionCollision"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ENABLE_SECTION_COLLISION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnableSectionCastShadow"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ENABLE_SECTION_CAST_SHADOW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "BulkSetConvexDecompositionCollisionsWithNotification",
            ),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_BULK_SET_CONVEX_DECOMPOSITION_COLLISIONS_WITH_NOTIFICATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BulkSetConvexDecompositionCollisions"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_BULK_SET_CONVEX_DECOMPOSITION_COLLISIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddUVChannel"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ADD_UV_CHANNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSimpleCollisionsWithNotification"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ADD_SIMPLE_COLLISIONS_WITH_NOTIFICATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSimpleCollisions"),
            &raw mut UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ADD_SIMPLE_COLLISIONS,
        );
    }
}
#[repr(C, align(4))]
pub struct FEditorDialogLibraryObjectDetailsViewOptions {
    pub b_show_object_name: bool,
    pub b_allow_search: bool,
    pub b_allow_resizing: bool,
    pub min_width: i32,
    pub min_height: i32,
    pub value_column_width_ratio: f32,
}
impl FEditorDialogLibraryObjectDetailsViewOptions {}
#[repr(C, align(8))]
pub struct FEditorScriptingJoinStaticMeshActorsOptions_Deprecated {
    pub b_destroy_source_actors: bool,
    pub new_actor_label: FString,
    pub b_rename_components_from_source: bool,
    __padding_end: [u8; 7],
}
impl FEditorScriptingJoinStaticMeshActorsOptions_Deprecated {}
#[repr(C, align(8))]
pub struct FEditorScriptingMergeStaticMeshActorsOptions_Deprecated {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub b_spawn_merged_actor: bool,
    pub base_package_name: FString,
    pub mesh_merging_settings: crate::bindings::engine::FMeshMergingSettings,
}
impl FEditorScriptingMergeStaticMeshActorsOptions_Deprecated {}
#[repr(C, align(8))]
pub struct FEditorScriptingCreateProxyMeshActorOptions_Deprecated {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub b_spawn_merged_actor: bool,
    pub base_package_name: FString,
    pub mesh_proxy_settings: crate::bindings::engine::FMeshProxySettings,
}
impl FEditorScriptingCreateProxyMeshActorOptions_Deprecated {}
#[repr(C, align(4))]
pub struct FEditorScriptingMeshReductionSettings_Deprecated {
    pub percent_triangles: f32,
    pub screen_size: f32,
}
impl FEditorScriptingMeshReductionSettings_Deprecated {}
#[repr(C, align(8))]
pub struct FEditorScriptingMeshReductionOptions_Deprecated {
    pub b_auto_compute_lod_screen_size: bool,
    pub reduction_settings: TArray<FEditorScriptingMeshReductionSettings_Deprecated>,
}
impl FEditorScriptingMeshReductionOptions_Deprecated {}
#[repr(C, align(8))]
pub struct UEditorAssetLibrary {
    __padding_end: [u8; 48],
}
impl UEditorAssetLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorAssetLibrary")
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
pub struct UEditorDialogLibrary {
    __padding_end: [u8; 48],
}
impl UEditorDialogLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorDialogLibrary")
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
pub struct UEditorFilterLibrary {
    __padding_end: [u8; 48],
}
impl UEditorFilterLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorFilterLibrary")
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
pub struct UEditorLevelLibrary {
    __padding_end: [u8; 48],
}
impl UEditorLevelLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorLevelLibrary")
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
pub struct UDEPRECATED_EditorSkeletalMeshLibrary {
    __padding_end: [u8; 48],
}
impl UDEPRECATED_EditorSkeletalMeshLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_EditorSkeletalMeshLibrary")
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
pub struct UDEPRECATED_EditorStaticMeshLibrary {
    __padding_end: [u8; 48],
}
impl UDEPRECATED_EditorStaticMeshLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_EditorStaticMeshLibrary")
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
#[repr(transparent)]
pub struct EEditorScriptingStringMatchType(pub u8);
impl EEditorScriptingStringMatchType {
    pub const CONTAINS: EEditorScriptingStringMatchType = EEditorScriptingStringMatchType(
        0,
    );
    pub const MATCHES_WILDCARD: EEditorScriptingStringMatchType = EEditorScriptingStringMatchType(
        1,
    );
    pub const EXACT_MATCH: EEditorScriptingStringMatchType = EEditorScriptingStringMatchType(
        2,
    );
}
#[repr(transparent)]
pub struct EEditorScriptingFilterType(pub u8);
impl EEditorScriptingFilterType {
    pub const INCLUDE: EEditorScriptingFilterType = EEditorScriptingFilterType(0);
    pub const EXCLUDE: EEditorScriptingFilterType = EEditorScriptingFilterType(1);
}
