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
    pub fn sync_browser_to_objects(asset_paths: &TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_SYNC_BROWSER_TO_OBJECTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_paths,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_SYNC_BROWSER_TO_OBJECTS,
                __buffer,
            )
        };
    }
    pub fn set_metadata_tag(
        object: UPtr<crate::bindings::core_u_object::UObject>,
        tag: FName,
        value: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_SET_METADATA_TAG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(8).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(24).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_SET_METADATA_TAG,
                __buffer,
            )
        };
    }
    pub fn save_loaded_assets(
        assets_to_save: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
        b_only_if_is_dirty: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_SAVE_LOADED_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_to_save,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_if_is_dirty,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_SAVE_LOADED_ASSETS,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn save_loaded_asset(
        asset_to_save: UPtr<crate::bindings::core_u_object::UObject>,
        b_only_if_is_dirty: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_SAVE_LOADED_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_save,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_if_is_dirty,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_SAVE_LOADED_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn save_directory(
        directory_path: FString,
        b_only_if_is_dirty: bool,
        b_recursive: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_SAVE_DIRECTORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_if_is_dirty,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_SAVE_DIRECTORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn save_asset(asset_to_save: FString, b_only_if_is_dirty: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_SAVE_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_save,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_if_is_dirty,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_SAVE_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn rename_loaded_asset(
        source_asset: UPtr<crate::bindings::core_u_object::UObject>,
        destination_asset_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_RENAME_LOADED_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_asset_path,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_RENAME_LOADED_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn rename_directory(
        source_directory_path: FString,
        destination_directory_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_RENAME_DIRECTORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_directory_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_directory_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_RENAME_DIRECTORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn rename_asset(
        source_asset_path: FString,
        destination_asset_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_RENAME_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_asset_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_RENAME_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_metadata_tag(
        object: UPtr<crate::bindings::core_u_object::UObject>,
        tag: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_REMOVE_METADATA_TAG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(8).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_REMOVE_METADATA_TAG,
                __buffer,
            )
        };
    }
    pub fn make_directory(directory_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_MAKE_DIRECTORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_MAKE_DIRECTORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn load_blueprint_class(
        asset_path: FString,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_LOAD_BLUEPRINT_CLASS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_LOAD_BLUEPRINT_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn load_asset(
        asset_path: FString,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_LOAD_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_LOAD_ASSET,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn list_assets(
        directory_path: FString,
        b_recursive: bool,
        b_include_folder: bool,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_LIST_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_folder,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_LIST_ASSETS,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FString>>().read() }
    }
    pub fn list_asset_by_tag_value(
        tag_name: FName,
        tag_value: FString,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_LIST_ASSET_BY_TAG_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tag_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_LIST_ASSET_BY_TAG_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<TArray<FString>>().read() }
    }
    pub fn get_tag_values(asset_path: FString) -> TMap<FName, FString> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_GET_TAG_VALUES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_GET_TAG_VALUES,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TMap<FName, FString>>().read() }
    }
    pub fn get_project_root_asset_directory() -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_GET_PROJECT_ROOT_ASSET_DIRECTORY,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_GET_PROJECT_ROOT_ASSET_DIRECTORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_path_name_for_loaded_asset(
        loaded_asset: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_GET_PATH_NAME_FOR_LOADED_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &loaded_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_GET_PATH_NAME_FOR_LOADED_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_package_for_object(
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UPackage> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_GET_PACKAGE_FOR_OBJECT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_GET_PACKAGE_FOR_OBJECT,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::core_u_object::UPackage>>()
                .read()
        }
    }
    pub fn get_metadata_tag_values(
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> TMap<FName, FString> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_GET_METADATA_TAG_VALUES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_GET_METADATA_TAG_VALUES,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TMap<FName, FString>>().read() }
    }
    pub fn get_metadata_tag(
        object: UPtr<crate::bindings::core_u_object::UObject>,
        tag: FName,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_GET_METADATA_TAG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(8).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_GET_METADATA_TAG,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn find_package_referencers_for_asset(
        asset_path: FString,
        b_load_assets_to_confirm: bool,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_FIND_PACKAGE_REFERENCERS_FOR_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_load_assets_to_confirm,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_FIND_PACKAGE_REFERENCERS_FOR_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FString>>().read() }
    }
    pub fn find_asset_data(
        asset_path: FString,
    ) -> crate::bindings::core_u_object::FAssetData {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_FIND_ASSET_DATA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_FIND_ASSET_DATA,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FAssetData>().read()
        }
    }
    pub fn duplicate_loaded_asset(
        source_asset: UPtr<crate::bindings::core_u_object::UObject>,
        destination_asset_path: FString,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DUPLICATE_LOADED_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_asset_path,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DUPLICATE_LOADED_ASSET,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn duplicate_directory(
        source_directory_path: FString,
        destination_directory_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DUPLICATE_DIRECTORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_directory_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_directory_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DUPLICATE_DIRECTORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn duplicate_asset(
        source_asset_path: FString,
        destination_asset_path: FString,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DUPLICATE_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_asset_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DUPLICATE_ASSET,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn does_directory_have_assets(
        directory_path: FString,
        b_recursive: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DOES_DIRECTORY_HAVE_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DOES_DIRECTORY_HAVE_ASSETS,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn does_directory_exist(directory_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DOES_DIRECTORY_EXIST,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DOES_DIRECTORY_EXIST,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn does_asset_exist(asset_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DOES_ASSET_EXIST,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DOES_ASSET_EXIST,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn do_assets_exist(asset_paths: &TArray<FString>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DO_ASSETS_EXIST,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_paths,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DO_ASSETS_EXIST,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn delete_loaded_assets(
        assets_to_delete: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DELETE_LOADED_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_to_delete,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DELETE_LOADED_ASSETS,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn delete_loaded_asset(
        asset_to_delete: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DELETE_LOADED_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_delete,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DELETE_LOADED_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn delete_directory(directory_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DELETE_DIRECTORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DELETE_DIRECTORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn delete_asset(asset_path_to_delete: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DELETE_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path_to_delete,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_DELETE_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn consolidate_assets(
        asset_to_consolidate_to: UPtr<crate::bindings::core_u_object::UObject>,
        assets_to_consolidate: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_CONSOLIDATE_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_consolidate_to,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_to_consolidate,
                __buffer
                    .add(8)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_CONSOLIDATE_ASSETS,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn checkout_loaded_assets(
        assets_to_checkout: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_CHECKOUT_LOADED_ASSETS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_to_checkout,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_CHECKOUT_LOADED_ASSETS,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn checkout_loaded_asset(
        asset_to_checkout: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_CHECKOUT_LOADED_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_checkout,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_CHECKOUT_LOADED_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn checkout_directory(directory_path: FString, b_recursive: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_CHECKOUT_DIRECTORY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_CHECKOUT_DIRECTORY,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn checkout_asset(asset_to_checkout: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_CHECKOUT_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_checkout,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorAssetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_ASSET_LIBRARY_CHECKOUT_ASSET,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
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
    pub fn show_suppressable_warning_dialog(
        title: &FText,
        message: &FText,
        in_ini_setting_name: FString,
        in_ini_setting_file_name_override: FString,
        b_default_value: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<66>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_DIALOG_LIBRARY_SHOW_SUPPRESSABLE_WARNING_DIALOG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(title, __buffer.add(0).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(message, __buffer.add(16).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_ini_setting_name,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_ini_setting_file_name_override,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_default_value,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorDialogLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_DIALOG_LIBRARY_SHOW_SUPPRESSABLE_WARNING_DIALOG,
                __buffer,
            )
        };
        unsafe { __buffer.add(65).cast::<bool>().read() }
    }
    pub fn show_objects_details_view(
        title: &FText,
        in_out_objects: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
        options: &FEditorDialogLibraryObjectDetailsViewOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_DIALOG_LIBRARY_SHOW_OBJECTS_DETAILS_VIEW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(title, __buffer.add(0).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_objects,
                __buffer
                    .add(16)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(32).cast::<FEditorDialogLibraryObjectDetailsViewOptions>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorDialogLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_DIALOG_LIBRARY_SHOW_OBJECTS_DETAILS_VIEW,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn show_object_details_view(
        title: &FText,
        in_out_object: UPtr<crate::bindings::core_u_object::UObject>,
        options: &FEditorDialogLibraryObjectDetailsViewOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_DIALOG_LIBRARY_SHOW_OBJECT_DETAILS_VIEW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(title, __buffer.add(0).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_out_object,
                __buffer.add(16).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(24).cast::<FEditorDialogLibraryObjectDetailsViewOptions>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorDialogLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_DIALOG_LIBRARY_SHOW_OBJECT_DETAILS_VIEW,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn show_message(
        title: &FText,
        message: &FText,
        message_type: crate::bindings::core_u_object::EAppMsgType,
        default_value: crate::bindings::core_u_object::EAppReturnType,
        message_category: crate::bindings::core_u_object::EAppMsgCategory,
    ) -> crate::bindings::core_u_object::EAppReturnType {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_DIALOG_LIBRARY_SHOW_MESSAGE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(title, __buffer.add(0).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(message, __buffer.add(16).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message_type,
                __buffer.add(32).cast::<crate::bindings::core_u_object::EAppMsgType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer
                    .add(33)
                    .cast::<crate::bindings::core_u_object::EAppReturnType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message_category,
                __buffer
                    .add(34)
                    .cast::<crate::bindings::core_u_object::EAppMsgCategory>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorDialogLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_DIALOG_LIBRARY_SHOW_MESSAGE,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(35)
                .cast::<crate::bindings::core_u_object::EAppReturnType>()
                .read()
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
    pub fn by_selection(
        target_array: &TArray<UPtr<crate::bindings::engine::AActor>>,
        filter_type: EEditorScriptingFilterType,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_SELECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_array,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_type,
                __buffer.add(16).cast::<EEditorScriptingFilterType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorFilterLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_SELECTION,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn by_level_name(
        target_array: &TArray<UPtr<crate::bindings::engine::AActor>>,
        level_name: FName,
        filter_type: EEditorScriptingFilterType,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_LEVEL_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_array,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_type,
                __buffer.add(28).cast::<EEditorScriptingFilterType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorFilterLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_LEVEL_NAME,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn by_layer(
        target_array: &TArray<UPtr<crate::bindings::engine::AActor>>,
        layer_name: FName,
        filter_type: EEditorScriptingFilterType,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_LAYER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_array,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &layer_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_type,
                __buffer.add(28).cast::<EEditorScriptingFilterType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorFilterLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_LAYER,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn by_id_name(
        target_array: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
        name_sub_string: FString,
        string_match: EEditorScriptingStringMatchType,
        filter_type: EEditorScriptingFilterType,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_ID_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_array,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &name_sub_string,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &string_match,
                __buffer.add(32).cast::<EEditorScriptingStringMatchType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_type,
                __buffer.add(33).cast::<EEditorScriptingFilterType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorFilterLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_ID_NAME,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn by_class(
        target_array: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
        object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        filter_type: EEditorScriptingFilterType,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_CLASS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_array,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_type,
                __buffer.add(24).cast::<EEditorScriptingFilterType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorFilterLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn by_actor_tag(
        target_array: &TArray<UPtr<crate::bindings::engine::AActor>>,
        tag: FName,
        filter_type: EEditorScriptingFilterType,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_ACTOR_TAG,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_array,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(16).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_type,
                __buffer.add(28).cast::<EEditorScriptingFilterType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorFilterLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_ACTOR_TAG,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn by_actor_label(
        target_array: &TArray<UPtr<crate::bindings::engine::AActor>>,
        name_sub_string: FString,
        string_match: EEditorScriptingStringMatchType,
        filter_type: EEditorScriptingFilterType,
        b_ignore_case: bool,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_ACTOR_LABEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_array,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &name_sub_string,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &string_match,
                __buffer.add(32).cast::<EEditorScriptingStringMatchType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_type,
                __buffer.add(33).cast::<EEditorScriptingFilterType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_ignore_case,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorFilterLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_FILTER_LIBRARY_BY_ACTOR_LABEL,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
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
    pub fn spawn_actor_from_object(
        object_to_use: UPtr<crate::bindings::core_u_object::UObject>,
        location: crate::bindings::core_u_object::FVector,
        rotation: crate::bindings::core_u_object::FRotator,
        b_transient: bool,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SPAWN_ACTOR_FROM_OBJECT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_to_use,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_transient,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SPAWN_ACTOR_FROM_OBJECT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(64).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn spawn_actor_from_class(
        actor_class: TSubclassOf<crate::bindings::engine::AActor>,
        location: crate::bindings::core_u_object::FVector,
        rotation: crate::bindings::core_u_object::FRotator,
        b_transient: bool,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SPAWN_ACTOR_FROM_CLASS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_class,
                __buffer.add(0).cast::<TSubclassOf<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_transient,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SPAWN_ACTOR_FROM_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(64).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn set_selected_level_actors(
        actors_to_select: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SET_SELECTED_LEVEL_ACTORS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_select,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SET_SELECTED_LEVEL_ACTORS,
                __buffer,
            )
        };
    }
    pub fn set_level_viewport_camera_info(
        camera_location: crate::bindings::core_u_object::FVector,
        camera_rotation: crate::bindings::core_u_object::FRotator,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SET_LEVEL_VIEWPORT_CAMERA_INFO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_location,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rotation,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SET_LEVEL_VIEWPORT_CAMERA_INFO,
                __buffer,
            )
        };
    }
    pub fn set_current_level_by_name(level_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SET_CURRENT_LEVEL_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SET_CURRENT_LEVEL_BY_NAME,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn set_actor_selection_state(
        actor: UPtr<crate::bindings::engine::AActor>,
        b_should_be_selected: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SET_ACTOR_SELECTION_STATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_be_selected,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SET_ACTOR_SELECTION_STATE,
                __buffer,
            )
        };
    }
    pub fn select_nothing() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SELECT_NOTHING,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SELECT_NOTHING,
                __buffer,
            )
        };
    }
    pub fn save_current_level() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SAVE_CURRENT_LEVEL,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SAVE_CURRENT_LEVEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn save_all_dirty_levels() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SAVE_ALL_DIRTY_LEVELS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_SAVE_ALL_DIRTY_LEVELS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn replace_selected_actors(in_asset_path: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_REPLACE_SELECTED_ACTORS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_REPLACE_SELECTED_ACTORS,
                __buffer,
            )
        };
    }
    pub fn replace_mesh_components_meshes_on_actors(
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        mesh_to_be_replaced: UPtr<crate::bindings::engine::UStaticMesh>,
        new_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MESHES_ON_ACTORS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh_to_be_replaced,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mesh,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MESHES_ON_ACTORS,
                __buffer,
            )
        };
    }
    pub fn replace_mesh_components_meshes(
        mesh_components: &TArray<UPtr<crate::bindings::engine::UStaticMeshComponent>>,
        mesh_to_be_replaced: UPtr<crate::bindings::engine::UStaticMesh>,
        new_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MESHES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                mesh_components,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::engine::UStaticMeshComponent>>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh_to_be_replaced,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mesh,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MESHES,
                __buffer,
            )
        };
    }
    pub fn replace_mesh_components_materials_on_actors(
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        material_to_be_replaced: UPtr<crate::bindings::engine::UMaterialInterface>,
        new_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MATERIALS_ON_ACTORS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_to_be_replaced,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_material,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MATERIALS_ON_ACTORS,
                __buffer,
            )
        };
    }
    pub fn replace_mesh_components_materials(
        mesh_components: &TArray<UPtr<crate::bindings::engine::UMeshComponent>>,
        material_to_be_replaced: UPtr<crate::bindings::engine::UMaterialInterface>,
        new_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MATERIALS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                mesh_components,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UMeshComponent>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_to_be_replaced,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_material,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_REPLACE_MESH_COMPONENTS_MATERIALS,
                __buffer,
            )
        };
    }
    pub fn pilot_level_actor(actor_to_pilot: UPtr<crate::bindings::engine::AActor>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_PILOT_LEVEL_ACTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_to_pilot,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_PILOT_LEVEL_ACTOR,
                __buffer,
            )
        };
    }
    pub fn new_level_from_template(
        asset_path: FString,
        template_asset_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_NEW_LEVEL_FROM_TEMPLATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &template_asset_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_NEW_LEVEL_FROM_TEMPLATE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn new_level(asset_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_NEW_LEVEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_NEW_LEVEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn merge_static_mesh_actors(
        actors_to_merge: &TArray<UPtr<crate::bindings::engine::AStaticMeshActor>>,
        merge_options: &crate::bindings::static_mesh_editor::FMergeStaticMeshActorsOptions,
        out_merged_actor: &mut UPtr<crate::bindings::engine::AStaticMeshActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<409>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_MERGE_STATIC_MESH_ACTORS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_merge,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::AStaticMeshActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                merge_options,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::static_mesh_editor::FMergeStaticMeshActorsOptions,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_merged_actor,
                __buffer
                    .add(400)
                    .cast::<UPtr<crate::bindings::engine::AStaticMeshActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_MERGE_STATIC_MESH_ACTORS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(400)
                .cast::<UPtr<crate::bindings::engine::AStaticMeshActor>>()
                .swap(out_merged_actor);
        }
        unsafe { __buffer.add(408).cast::<bool>().read() }
    }
    pub fn load_level(asset_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_LOAD_LEVEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_LOAD_LEVEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn join_static_mesh_actors(
        actors_to_join: &TArray<UPtr<crate::bindings::engine::AStaticMeshActor>>,
        join_options: &crate::bindings::static_mesh_editor::FJoinStaticMeshActorsOptions,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_JOIN_STATIC_MESH_ACTORS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_join,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::AStaticMeshActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                join_options,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::static_mesh_editor::FJoinStaticMeshActorsOptions,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_JOIN_STATIC_MESH_ACTORS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn get_selected_level_actors() -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_SELECTED_LEVEL_ACTORS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_SELECTED_LEVEL_ACTORS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_pie_worlds(
        b_include_dedicated_server: bool,
    ) -> TArray<UPtr<crate::bindings::engine::UWorld>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_PIE_WORLDS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_dedicated_server,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_PIE_WORLDS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::UWorld>>>()
                .read()
        }
    }
    pub fn get_level_viewport_camera_info(
        camera_location: &mut crate::bindings::core_u_object::FVector,
        camera_rotation: &mut crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_LEVEL_VIEWPORT_CAMERA_INFO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_location,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_rotation,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_LEVEL_VIEWPORT_CAMERA_INFO,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(camera_location);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FRotator>()
                .swap(camera_rotation);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn get_game_world() -> UPtr<crate::bindings::engine::UWorld> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_GAME_WORLD,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_GAME_WORLD,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>().read() }
    }
    pub fn get_editor_world() -> UPtr<crate::bindings::engine::UWorld> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_EDITOR_WORLD,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_EDITOR_WORLD,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>().read() }
    }
    pub fn get_all_level_actors_components() -> TArray<
        UPtr<crate::bindings::engine::UActorComponent>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_ALL_LEVEL_ACTORS_COMPONENTS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_ALL_LEVEL_ACTORS_COMPONENTS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::UActorComponent>>>()
                .read()
        }
    }
    pub fn get_all_level_actors() -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_ALL_LEVEL_ACTORS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_ALL_LEVEL_ACTORS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_actor_reference(
        path_to_actor: FString,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_ACTOR_REFERENCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_to_actor,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_GET_ACTOR_REFERENCE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn eject_pilot_level_actor() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_EJECT_PILOT_LEVEL_ACTOR,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_EJECT_PILOT_LEVEL_ACTOR,
                __buffer,
            )
        };
    }
    pub fn editor_set_game_view(b_game_view: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_EDITOR_SET_GAME_VIEW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_game_view,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_EDITOR_SET_GAME_VIEW,
                __buffer,
            )
        };
    }
    pub fn editor_play_simulate() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_EDITOR_PLAY_SIMULATE,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_EDITOR_PLAY_SIMULATE,
                __buffer,
            )
        };
    }
    pub fn editor_invalidate_viewports() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_EDITOR_INVALIDATE_VIEWPORTS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_EDITOR_INVALIDATE_VIEWPORTS,
                __buffer,
            )
        };
    }
    pub fn editor_end_play() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_EDITOR_END_PLAY,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_EDITOR_END_PLAY,
                __buffer,
            )
        };
    }
    pub fn destroy_actor(
        actor_to_destroy: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_DESTROY_ACTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_to_destroy,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_DESTROY_ACTOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn create_proxy_mesh_actor(
        actors_to_merge: &TArray<UPtr<crate::bindings::engine::AStaticMeshActor>>,
        merge_options: &crate::bindings::static_mesh_editor::FCreateProxyMeshActorOptions,
        out_merged_actor: &mut UPtr<crate::bindings::engine::AStaticMeshActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<401>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_CREATE_PROXY_MESH_ACTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_merge,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::AStaticMeshActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                merge_options,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::static_mesh_editor::FCreateProxyMeshActorOptions,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_merged_actor,
                __buffer
                    .add(392)
                    .cast::<UPtr<crate::bindings::engine::AStaticMeshActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_CREATE_PROXY_MESH_ACTOR,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(392)
                .cast::<UPtr<crate::bindings::engine::AStaticMeshActor>>()
                .swap(out_merged_actor);
        }
        unsafe { __buffer.add(400).cast::<bool>().read() }
    }
    pub fn convert_actors(
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        actor_class: TSubclassOf<crate::bindings::engine::AActor>,
        static_mesh_package_path: FString,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_CONVERT_ACTORS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_class,
                __buffer.add(16).cast::<TSubclassOf<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh_package_path,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_CONVERT_ACTORS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn clear_actor_selection_set() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_CLEAR_ACTOR_SELECTION_SET,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::editor_scripting_utilities::UEditorLevelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::U_EDITOR_LEVEL_LIBRARY_CLEAR_ACTOR_SELECTION_SET,
                __buffer,
            )
        };
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
    pub fn strip_lod_geometry(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        texture_mask: UPtr<crate::bindings::engine::UTexture2D>,
        threshold: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_STRIP_LOD_GEOMETRY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture_mask,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UTexture2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&threshold, __buffer.add(24).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorSkeletalMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_STRIP_LOD_GEOMETRY,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn set_lod_build_settings(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        build_options: &crate::bindings::engine::FSkeletalMeshBuildSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_SET_LOD_BUILD_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                build_options,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::engine::FSkeletalMeshBuildSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorSkeletalMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_SET_LOD_BUILD_SETTINGS,
                __buffer,
            )
        };
    }
    pub fn rename_socket(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        old_name: FName,
        new_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_RENAME_SOCKET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&old_name, __buffer.add(8).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorSkeletalMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_RENAME_SOCKET,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_lods_remove_lo_ds(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        to_remove_lo_ds: TArray<i32>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_REMOVE_LO_DS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &to_remove_lo_ds,
                __buffer.add(8).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorSkeletalMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_REMOVE_LO_DS,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn reimport_all_custom_lods_reimport_all_custom_lo_ds(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_REIMPORT_ALL_CUSTOM_LO_DS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorSkeletalMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_REIMPORT_ALL_CUSTOM_LO_DS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn regenerate_lod(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        new_lod_count: i32,
        b_regenerate_even_if_imported: bool,
        b_generate_base_lod: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_REGENERATE_LOD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_lod_count,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_regenerate_even_if_imported,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_generate_base_lod,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorSkeletalMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_REGENERATE_LOD,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn import_lod(
        base_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        source_filename: FString,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_IMPORT_LOD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_filename,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorSkeletalMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_IMPORT_LOD,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn get_num_verts(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_GET_NUM_VERTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorSkeletalMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_GET_NUM_VERTS,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn get_lod_count(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_GET_LOD_COUNT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorSkeletalMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_GET_LOD_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_lod_build_settings(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        lod_index: i32,
        out_build_options: &mut crate::bindings::engine::FSkeletalMeshBuildSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_GET_LOD_BUILD_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_build_options,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::engine::FSkeletalMeshBuildSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorSkeletalMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_GET_LOD_BUILD_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(12)
                .cast::<crate::bindings::engine::FSkeletalMeshBuildSettings>()
                .swap(out_build_options);
        }
    }
    pub fn create_physics_asset(
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) -> UPtr<crate::bindings::engine::UPhysicsAsset> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_CREATE_PHYSICS_ASSET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorSkeletalMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_SKELETAL_MESH_LIBRARY_CREATE_PHYSICS_ASSET,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<crate::bindings::engine::UPhysicsAsset>>().read()
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
    pub fn set_lods_with_notification(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        reduction_options: &crate::bindings::static_mesh_editor::FStaticMeshReductionOptions,
        b_apply_changes: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LODS_WITH_NOTIFICATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                reduction_options,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::static_mesh_editor::FStaticMeshReductionOptions,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_changes,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LODS_WITH_NOTIFICATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<i32>().read() }
    }
    pub fn set_lods(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        reduction_options: &crate::bindings::static_mesh_editor::FStaticMeshReductionOptions,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LODS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                reduction_options,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::static_mesh_editor::FStaticMeshReductionOptions,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LODS,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn set_lod_reduction_settings(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        reduction_options: &crate::bindings::engine::FMeshReductionSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LOD_REDUCTION_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                reduction_options,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::engine::FMeshReductionSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LOD_REDUCTION_SETTINGS,
                __buffer,
            )
        };
    }
    pub fn set_lod_from_static_mesh(
        destination_static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        destination_lod_index: i32,
        source_static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        source_lod_index: i32,
        b_reuse_existing_material_slots: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LOD_FROM_STATIC_MESH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_lod_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_static_mesh,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_lod_index,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reuse_existing_material_slots,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LOD_FROM_STATIC_MESH,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn set_lod_build_settings(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        build_options: &crate::bindings::engine::FMeshBuildSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LOD_BUILD_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                build_options,
                __buffer.add(16).cast::<crate::bindings::engine::FMeshBuildSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_LOD_BUILD_SETTINGS,
                __buffer,
            )
        };
    }
    pub fn set_generate_lightmap_uv(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_generate_lightmap_u_vs: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_GENERATE_LIGHTMAP_U_VS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_generate_lightmap_u_vs,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_GENERATE_LIGHTMAP_U_VS,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_convex_decomposition_collisions_with_notification(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        hull_count: i32,
        max_hull_verts: i32,
        hull_precision: i32,
        b_apply_changes: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_CONVEX_DECOMPOSITION_COLLISIONS_WITH_NOTIFICATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&hull_count, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_hull_verts,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hull_precision,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_changes,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_CONVEX_DECOMPOSITION_COLLISIONS_WITH_NOTIFICATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn set_convex_decomposition_collisions(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        hull_count: i32,
        max_hull_verts: i32,
        hull_precision: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_CONVEX_DECOMPOSITION_COLLISIONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&hull_count, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_hull_verts,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hull_precision,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_CONVEX_DECOMPOSITION_COLLISIONS,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn set_allow_cpu_access(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_allow_cpu_access: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_ALLOW_CPU_ACCESS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_cpu_access,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_SET_ALLOW_CPU_ACCESS,
                __buffer,
            )
        };
    }
    pub fn remove_uv_channel(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        uv_channel_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_UV_CHANNEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv_channel_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_UV_CHANNEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_lods(static_mesh: UPtr<crate::bindings::engine::UStaticMesh>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_LODS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_LODS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn remove_collisions_with_notification(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_apply_changes: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_COLLISIONS_WITH_NOTIFICATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_changes,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_COLLISIONS_WITH_NOTIFICATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn remove_collisions(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_COLLISIONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REMOVE_COLLISIONS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn reimport_all_custom_lods_reimport_all_custom_lo_ds(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REIMPORT_ALL_CUSTOM_LO_DS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_REIMPORT_ALL_CUSTOM_LO_DS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_section_collision_enabled(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        section_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_IS_SECTION_COLLISION_ENABLED,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_IS_SECTION_COLLISION_ENABLED,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn insert_uv_channel(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        uv_channel_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_INSERT_UV_CHANNEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv_channel_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_INSERT_UV_CHANNEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn import_lod(
        base_static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        source_filename: FString,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_IMPORT_LOD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_filename,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_IMPORT_LOD,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn has_vertex_colors(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_HAS_VERTEX_COLORS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_HAS_VERTEX_COLORS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn has_instance_vertex_colors(
        static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_HAS_INSTANCE_VERTEX_COLORS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UStaticMeshComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_HAS_INSTANCE_VERTEX_COLORS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_simple_collision_count(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_SIMPLE_COLLISION_COUNT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_SIMPLE_COLLISION_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_num_uv_channels(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_NUM_UV_CHANNELS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_NUM_UV_CHANNELS,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn get_number_verts(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_NUMBER_VERTS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_NUMBER_VERTS,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn get_number_materials(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_NUMBER_MATERIALS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_NUMBER_MATERIALS,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_lod_screen_sizes(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> TArray<f32> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_SCREEN_SIZES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_SCREEN_SIZES,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<f32>>().read() }
    }
    pub fn get_lod_reduction_settings(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        out_reduction_options: &mut crate::bindings::engine::FMeshReductionSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_REDUCTION_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_reduction_options,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::engine::FMeshReductionSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_REDUCTION_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(12)
                .cast::<crate::bindings::engine::FMeshReductionSettings>()
                .swap(out_reduction_options);
        }
    }
    pub fn get_lod_count(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_COUNT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_lod_build_settings(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        out_build_options: &mut crate::bindings::engine::FMeshBuildSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_BUILD_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_build_options,
                __buffer.add(16).cast::<crate::bindings::engine::FMeshBuildSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_LOD_BUILD_SETTINGS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::FMeshBuildSettings>()
                .swap(out_build_options);
        }
    }
    pub fn get_convex_collision_count(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_CONVEX_COLLISION_COUNT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_CONVEX_COLLISION_COUNT,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_collision_complexity(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> crate::bindings::physics_core::ECollisionTraceFlag {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_COLLISION_COMPLEXITY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GET_COLLISION_COMPLEXITY,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::physics_core::ECollisionTraceFlag>()
                .read()
        }
    }
    pub fn generate_planar_uv_channel(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        uv_channel_index: i32,
        position: &crate::bindings::core_u_object::FVector,
        orientation: &crate::bindings::core_u_object::FRotator,
        tiling: &crate::bindings::core_u_object::FVector2D,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GENERATE_PLANAR_UV_CHANNEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv_channel_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orientation,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tiling,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GENERATE_PLANAR_UV_CHANNEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn generate_cylindrical_uv_channel(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        uv_channel_index: i32,
        position: &crate::bindings::core_u_object::FVector,
        orientation: &crate::bindings::core_u_object::FRotator,
        tiling: &crate::bindings::core_u_object::FVector2D,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GENERATE_CYLINDRICAL_UV_CHANNEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv_channel_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orientation,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tiling,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GENERATE_CYLINDRICAL_UV_CHANNEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn generate_box_uv_channel(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        uv_channel_index: i32,
        position: &crate::bindings::core_u_object::FVector,
        orientation: &crate::bindings::core_u_object::FRotator,
        size: &crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GENERATE_BOX_UV_CHANNEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv_channel_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orientation,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                size,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_GENERATE_BOX_UV_CHANNEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn enable_section_collision(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_collision_enabled: bool,
        lod_index: i32,
        section_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ENABLE_SECTION_COLLISION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_collision_enabled,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ENABLE_SECTION_COLLISION,
                __buffer,
            )
        };
    }
    pub fn enable_section_cast_shadow(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_cast_shadow: bool,
        lod_index: i32,
        section_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ENABLE_SECTION_CAST_SHADOW,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_cast_shadow,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ENABLE_SECTION_CAST_SHADOW,
                __buffer,
            )
        };
    }
    pub fn bulk_set_convex_decomposition_collisions_with_notification(
        static_meshes: &TArray<UPtr<crate::bindings::engine::UStaticMesh>>,
        hull_count: i32,
        max_hull_verts: i32,
        hull_precision: i32,
        b_apply_changes: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<30>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_BULK_SET_CONVEX_DECOMPOSITION_COLLISIONS_WITH_NOTIFICATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                static_meshes,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UStaticMesh>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hull_count,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_hull_verts,
                __buffer.add(20).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hull_precision,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_changes,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_BULK_SET_CONVEX_DECOMPOSITION_COLLISIONS_WITH_NOTIFICATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(29).cast::<bool>().read() }
    }
    pub fn bulk_set_convex_decomposition_collisions(
        static_meshes: &TArray<UPtr<crate::bindings::engine::UStaticMesh>>,
        hull_count: i32,
        max_hull_verts: i32,
        hull_precision: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_BULK_SET_CONVEX_DECOMPOSITION_COLLISIONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                static_meshes,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UStaticMesh>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hull_count,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_hull_verts,
                __buffer.add(20).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hull_precision,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_BULK_SET_CONVEX_DECOMPOSITION_COLLISIONS,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn add_uv_channel(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ADD_UV_CHANNEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ADD_UV_CHANNEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn add_simple_collisions_with_notification(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        shape_type: crate::bindings::static_mesh_editor::EScriptCollisionShapeType,
        b_apply_changes: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ADD_SIMPLE_COLLISIONS_WITH_NOTIFICATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &shape_type,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::static_mesh_editor::EScriptCollisionShapeType,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_changes,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ADD_SIMPLE_COLLISIONS_WITH_NOTIFICATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn add_simple_collisions(
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        shape_type: crate::bindings::static_mesh_editor::EScriptCollisionShapeType,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ADD_SIMPLE_COLLISIONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &shape_type,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::static_mesh_editor::EScriptCollisionShapeType,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::editor_scripting_utilities::UDEPRECATED_EditorStaticMeshLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::editor_scripting_utilities::UDEPRECATED_EDITOR_STATIC_MESH_LIBRARY_ADD_SIMPLE_COLLISIONS,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<i32>().read() }
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
