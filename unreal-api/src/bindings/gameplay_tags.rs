#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_blueprint_gameplay_tag_library_remove_gameplay_tag: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_not_equal_tag_tag: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_not_equal_tag_container_tag_container: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_not_equal_gameplay_tag_container: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_not_equal_gameplay_tag: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_matches_tag: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_matches_any_tags: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_make_literal_gameplay_tag_container: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_make_literal_gameplay_tag: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_no_tags: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_any_tags: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_all_tags: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_make_gameplay_tag_query: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_make_gameplay_tag_container_from_tag: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_make_gameplay_tag_container_from_array: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_is_tag_query_empty: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_is_gameplay_tag_valid: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_has_tag: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_has_any_tags: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_has_any_matching_gameplay_tags: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_has_all_tags: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_has_all_matching_gameplay_tags: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_get_tag_name: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_get_owned_gameplay_tags: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_get_num_gameplay_tags_in_container: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_get_debug_string_from_gameplay_tag_container: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_get_debug_string_from_gameplay_tag: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_get_all_actors_of_class_matching_tag_query: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_filter: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_equal_equal_gameplay_tag_container: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_equal_equal_gameplay_tag: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_does_tag_asset_interface_have_tag: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_does_container_match_tag_query: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_conv_object_to_gameplay_tag_asset_interface: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_break_gameplay_tag_container: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_append_gameplay_tag_containers: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_gameplay_tag_library_add_gameplay_tag: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_tag_asset_interface_has_matching_gameplay_tag: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_tag_asset_interface_has_any_matching_gameplay_tags: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_tag_asset_interface_has_all_matching_gameplay_tags: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_tag_asset_interface_bp_get_owned_gameplay_tags: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_blueprint_gameplay_tag_library_remove_gameplay_tag: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_not_equal_tag_tag: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_not_equal_tag_container_tag_container: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_not_equal_gameplay_tag_container: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_not_equal_gameplay_tag: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_matches_tag: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_matches_any_tags: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_make_literal_gameplay_tag_container: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_make_literal_gameplay_tag: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_no_tags: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_any_tags: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_all_tags: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_make_gameplay_tag_query: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_make_gameplay_tag_container_from_tag: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_make_gameplay_tag_container_from_array: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_is_tag_query_empty: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_is_gameplay_tag_valid: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_has_tag: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_has_any_tags: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_has_any_matching_gameplay_tags: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_has_all_tags: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_has_all_matching_gameplay_tags: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_get_tag_name: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_get_owned_gameplay_tags: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_get_num_gameplay_tags_in_container: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_get_debug_string_from_gameplay_tag_container: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_get_debug_string_from_gameplay_tag: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_get_all_actors_of_class_matching_tag_query: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_filter: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_equal_equal_gameplay_tag_container: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_equal_equal_gameplay_tag: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_does_tag_asset_interface_have_tag: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_does_container_match_tag_query: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_conv_object_to_gameplay_tag_asset_interface: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_break_gameplay_tag_container: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_append_gameplay_tag_containers: std::ptr::null_mut(),
            u_blueprint_gameplay_tag_library_add_gameplay_tag: std::ptr::null_mut(),
            u_gameplay_tag_asset_interface_has_matching_gameplay_tag: std::ptr::null_mut(),
            u_gameplay_tag_asset_interface_has_any_matching_gameplay_tags: std::ptr::null_mut(),
            u_gameplay_tag_asset_interface_has_all_matching_gameplay_tags: std::ptr::null_mut(),
            u_gameplay_tag_asset_interface_bp_get_owned_gameplay_tags: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlueprintGameplayTagLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveGameplayTag"),
            &raw mut __FUNCTION_PTRS.u_blueprint_gameplay_tag_library_remove_gameplay_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NotEqual_TagTag"),
            &raw mut __FUNCTION_PTRS.u_blueprint_gameplay_tag_library_not_equal_tag_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NotEqual_TagContainerTagContainer"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_not_equal_tag_container_tag_container,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NotEqual_GameplayTagContainer"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_not_equal_gameplay_tag_container,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NotEqual_GameplayTag"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_not_equal_gameplay_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MatchesTag"),
            &raw mut __FUNCTION_PTRS.u_blueprint_gameplay_tag_library_matches_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MatchesAnyTags"),
            &raw mut __FUNCTION_PTRS.u_blueprint_gameplay_tag_library_matches_any_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeLiteralGameplayTagContainer"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_make_literal_gameplay_tag_container,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeLiteralGameplayTag"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_make_literal_gameplay_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeGameplayTagQuery_MatchNoTags"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_no_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeGameplayTagQuery_MatchAnyTags"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_any_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeGameplayTagQuery_MatchAllTags"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_all_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeGameplayTagQuery"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_make_gameplay_tag_query,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeGameplayTagContainerFromTag"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_make_gameplay_tag_container_from_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeGameplayTagContainerFromArray"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_make_gameplay_tag_container_from_array,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTagQueryEmpty"),
            &raw mut __FUNCTION_PTRS.u_blueprint_gameplay_tag_library_is_tag_query_empty,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsGameplayTagValid"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_is_gameplay_tag_valid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasTag"),
            &raw mut __FUNCTION_PTRS.u_blueprint_gameplay_tag_library_has_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasAnyTags"),
            &raw mut __FUNCTION_PTRS.u_blueprint_gameplay_tag_library_has_any_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasAnyMatchingGameplayTags"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_has_any_matching_gameplay_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasAllTags"),
            &raw mut __FUNCTION_PTRS.u_blueprint_gameplay_tag_library_has_all_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasAllMatchingGameplayTags"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_has_all_matching_gameplay_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTagName"),
            &raw mut __FUNCTION_PTRS.u_blueprint_gameplay_tag_library_get_tag_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOwnedGameplayTags"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_get_owned_gameplay_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumGameplayTagsInContainer"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_get_num_gameplay_tags_in_container,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDebugStringFromGameplayTagContainer"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_get_debug_string_from_gameplay_tag_container,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDebugStringFromGameplayTag"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_get_debug_string_from_gameplay_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllActorsOfClassMatchingTagQuery"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_get_all_actors_of_class_matching_tag_query,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Filter"),
            &raw mut __FUNCTION_PTRS.u_blueprint_gameplay_tag_library_filter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EqualEqual_GameplayTagContainer"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_equal_equal_gameplay_tag_container,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EqualEqual_GameplayTag"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_equal_equal_gameplay_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesTagAssetInterfaceHaveTag"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_does_tag_asset_interface_have_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesContainerMatchTagQuery"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_does_container_match_tag_query,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_ObjectToGameplayTagAssetInterface"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_conv_object_to_gameplay_tag_asset_interface,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BreakGameplayTagContainer"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_break_gameplay_tag_container,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AppendGameplayTagContainers"),
            &raw mut __FUNCTION_PTRS
                .u_blueprint_gameplay_tag_library_append_gameplay_tag_containers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGameplayTag"),
            &raw mut __FUNCTION_PTRS.u_blueprint_gameplay_tag_library_add_gameplay_tag,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameplayTagAssetInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasMatchingGameplayTag"),
            &raw mut __FUNCTION_PTRS
                .u_gameplay_tag_asset_interface_has_matching_gameplay_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasAnyMatchingGameplayTags"),
            &raw mut __FUNCTION_PTRS
                .u_gameplay_tag_asset_interface_has_any_matching_gameplay_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasAllMatchingGameplayTags"),
            &raw mut __FUNCTION_PTRS
                .u_gameplay_tag_asset_interface_has_all_matching_gameplay_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_GetOwnedGameplayTags"),
            &raw mut __FUNCTION_PTRS
                .u_gameplay_tag_asset_interface_bp_get_owned_gameplay_tags,
        );
    }
}
#[repr(C, align(4))]
pub struct FGameplayTag {
    pub(crate) __padding_end: [u8; 12],
}
impl FGameplayTag {}
#[repr(C, align(8))]
pub struct FGameplayTagContainer {
    pub gameplay_tags: TArray<FGameplayTag>,
    pub(crate) __padding_end: [u8; 16],
}
impl FGameplayTagContainer {}
#[repr(C, align(8))]
pub struct FGameplayTagQuery {
    pub(crate) __padding_end: [u8; 72],
}
impl FGameplayTagQuery {}
#[repr(C, align(8))]
pub struct FGameplayTagTableRow {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub tag: FName,
    pub dev_comment: FString,
}
impl FGameplayTagTableRow {}
#[repr(C, align(8))]
pub struct FRestrictedGameplayTagTableRow {
    #[doc(hidden)]
    pub(crate) __padding_40: [u8; 40],
    pub b_allow_non_restricted_children: bool,
}
impl FRestrictedGameplayTagTableRow {}
#[repr(C, align(8))]
pub struct UBlueprintGameplayTagLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintGameplayTagLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintGameplayTagLibrary")
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
    pub fn remove_gameplay_tag(
        tag_container: &mut FGameplayTagContainer,
        tag: FGameplayTag,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_remove_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag,
                __buffer.add(32).cast::<FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_remove_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FGameplayTagContainer>().swap(tag_container);
        }
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn not_equal_tag_tag(a: FGameplayTag, b: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_not_equal_tag_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&a, __buffer.add(0).cast::<FGameplayTag>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b, __buffer.add(16).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_not_equal_tag_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn not_equal_tag_container_tag_container(
        a: FGameplayTagContainer,
        b: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_not_equal_tag_container_tag_container,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &a,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b, __buffer.add(32).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_not_equal_tag_container_tag_container,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn not_equal_gameplay_tag_container(
        a: &FGameplayTagContainer,
        b: &FGameplayTagContainer,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_not_equal_gameplay_tag_container,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                a,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b,
                __buffer.add(32).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_not_equal_gameplay_tag_container,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn not_equal_gameplay_tag(a: FGameplayTag, b: FGameplayTag) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_not_equal_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&a, __buffer.add(0).cast::<FGameplayTag>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b,
                __buffer.add(12).cast::<FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_not_equal_gameplay_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn matches_tag(
        tag_one: FGameplayTag,
        tag_two: FGameplayTag,
        b_exact_match: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_matches_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_one,
                __buffer.add(0).cast::<FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_two,
                __buffer.add(12).cast::<FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exact_match,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_matches_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn matches_any_tags(
        tag_one: FGameplayTag,
        other_container: &FGameplayTagContainer,
        b_exact_match: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_matches_any_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_one,
                __buffer.add(0).cast::<FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                other_container,
                __buffer.add(16).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exact_match,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_matches_any_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(49).cast::<bool>().read() }
    }
    pub fn make_literal_gameplay_tag_container(
        value: FGameplayTagContainer,
    ) -> FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_literal_gameplay_tag_container,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_literal_gameplay_tag_container,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FGameplayTagContainer>().read() }
    }
    pub fn make_literal_gameplay_tag(value: FGameplayTag) -> FGameplayTag {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_literal_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_literal_gameplay_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FGameplayTag>().read() }
    }
    pub fn make_gameplay_tag_query_match_no_tags(
        in_tags: &FGameplayTagContainer,
    ) -> FGameplayTagQuery {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_no_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tags,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_no_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FGameplayTagQuery>().read() }
    }
    pub fn make_gameplay_tag_query_match_any_tags(
        in_tags: &FGameplayTagContainer,
    ) -> FGameplayTagQuery {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_any_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tags,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_any_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FGameplayTagQuery>().read() }
    }
    pub fn make_gameplay_tag_query_match_all_tags(
        in_tags: &FGameplayTagContainer,
    ) -> FGameplayTagQuery {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_all_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tags,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_gameplay_tag_query_match_all_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FGameplayTagQuery>().read() }
    }
    pub fn make_gameplay_tag_query(tag_query: FGameplayTagQuery) -> FGameplayTagQuery {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_gameplay_tag_query,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_query,
                __buffer.add(0).cast::<FGameplayTagQuery>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_gameplay_tag_query,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<FGameplayTagQuery>().read() }
    }
    pub fn make_gameplay_tag_container_from_tag(
        single_tag: FGameplayTag,
    ) -> FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_gameplay_tag_container_from_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &single_tag,
                __buffer.add(0).cast::<FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_gameplay_tag_container_from_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FGameplayTagContainer>().read() }
    }
    pub fn make_gameplay_tag_container_from_array(
        gameplay_tags: &TArray<FGameplayTag>,
    ) -> FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_gameplay_tag_container_from_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_tags,
                __buffer.add(0).cast::<TArray<FGameplayTag>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_make_gameplay_tag_container_from_array,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FGameplayTagContainer>().read() }
    }
    pub fn is_tag_query_empty(tag_query: &FGameplayTagQuery) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_is_tag_query_empty,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_query,
                __buffer.add(0).cast::<FGameplayTagQuery>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_is_tag_query_empty,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn is_gameplay_tag_valid(gameplay_tag: FGameplayTag) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_is_gameplay_tag_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_tag,
                __buffer.add(0).cast::<FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_is_gameplay_tag_valid,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn has_tag(
        tag_container: &FGameplayTagContainer,
        tag: FGameplayTag,
        b_exact_match: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<46>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_has_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag,
                __buffer.add(32).cast::<FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exact_match,
                __buffer.add(44).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_has_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(45).cast::<bool>().read() }
    }
    pub fn has_any_tags(
        tag_container: &FGameplayTagContainer,
        other_container: &FGameplayTagContainer,
        b_exact_match: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<66>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_has_any_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                other_container,
                __buffer.add(32).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exact_match,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_has_any_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(65).cast::<bool>().read() }
    }
    pub fn has_any_matching_gameplay_tags(
        tag_container_interface: TScriptInterface<UGameplayTagAssetInterface>,
        other_container: &FGameplayTagContainer,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_has_any_matching_gameplay_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_container_interface,
                __buffer.add(0).cast::<TScriptInterface<UGameplayTagAssetInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                other_container,
                __buffer.add(16).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_has_any_matching_gameplay_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn has_all_tags(
        tag_container: &FGameplayTagContainer,
        other_container: &FGameplayTagContainer,
        b_exact_match: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<66>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_has_all_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                other_container,
                __buffer.add(32).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exact_match,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_has_all_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(65).cast::<bool>().read() }
    }
    pub fn has_all_matching_gameplay_tags(
        tag_container_interface: TScriptInterface<UGameplayTagAssetInterface>,
        other_container: &FGameplayTagContainer,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_has_all_matching_gameplay_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_container_interface,
                __buffer.add(0).cast::<TScriptInterface<UGameplayTagAssetInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                other_container,
                __buffer.add(16).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_has_all_matching_gameplay_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn get_tag_name(gameplay_tag: &FGameplayTag) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_get_tag_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_tag,
                __buffer.add(0).cast::<FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_get_tag_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_owned_gameplay_tags(
        tag_container_interface: TScriptInterface<UGameplayTagAssetInterface>,
    ) -> FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_get_owned_gameplay_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_container_interface,
                __buffer.add(0).cast::<TScriptInterface<UGameplayTagAssetInterface>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_get_owned_gameplay_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FGameplayTagContainer>().read() }
    }
    pub fn get_num_gameplay_tags_in_container(
        tag_container: &FGameplayTagContainer,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_get_num_gameplay_tags_in_container,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_get_num_gameplay_tags_in_container,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn get_debug_string_from_gameplay_tag_container(
        tag_container: &FGameplayTagContainer,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_get_debug_string_from_gameplay_tag_container,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_get_debug_string_from_gameplay_tag_container,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FString>().read() }
    }
    pub fn get_debug_string_from_gameplay_tag(gameplay_tag: FGameplayTag) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_get_debug_string_from_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_tag,
                __buffer.add(0).cast::<FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_get_debug_string_from_gameplay_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_all_actors_of_class_matching_tag_query(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        actor_class: TSubclassOf<crate::bindings::engine::AActor>,
        gameplay_tag_query: &FGameplayTagQuery,
        out_actors: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_get_all_actors_of_class_matching_tag_query,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_class,
                __buffer.add(8).cast::<TSubclassOf<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_tag_query,
                __buffer.add(16).cast::<FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_actors,
                __buffer.add(88).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_get_all_actors_of_class_matching_tag_query,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(88)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(out_actors);
        }
    }
    pub fn filter(
        tag_container: &FGameplayTagContainer,
        other_container: &FGameplayTagContainer,
        b_exact_match: bool,
    ) -> FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_filter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                other_container,
                __buffer.add(32).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exact_match,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_filter,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<FGameplayTagContainer>().read() }
    }
    pub fn equal_equal_gameplay_tag_container(
        a: &FGameplayTagContainer,
        b: &FGameplayTagContainer,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_equal_equal_gameplay_tag_container,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                a,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b,
                __buffer.add(32).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_equal_equal_gameplay_tag_container,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn equal_equal_gameplay_tag(a: FGameplayTag, b: FGameplayTag) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_equal_equal_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&a, __buffer.add(0).cast::<FGameplayTag>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b,
                __buffer.add(12).cast::<FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_equal_equal_gameplay_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn does_tag_asset_interface_have_tag(
        tag_container_interface: TScriptInterface<UGameplayTagAssetInterface>,
        tag: FGameplayTag,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_does_tag_asset_interface_have_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_container_interface,
                __buffer.add(0).cast::<TScriptInterface<UGameplayTagAssetInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag,
                __buffer.add(16).cast::<FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_does_tag_asset_interface_have_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn does_container_match_tag_query(
        tag_container: &FGameplayTagContainer,
        tag_query: &FGameplayTagQuery,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_does_container_match_tag_query,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_query,
                __buffer.add(32).cast::<FGameplayTagQuery>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_does_container_match_tag_query,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<bool>().read() }
    }
    pub fn conv_object_to_gameplay_tag_asset_interface(
        in_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> TScriptInterface<UGameplayTagAssetInterface> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_conv_object_to_gameplay_tag_asset_interface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_conv_object_to_gameplay_tag_asset_interface,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TScriptInterface<UGameplayTagAssetInterface>>().read()
        }
    }
    pub fn break_gameplay_tag_container(
        gameplay_tag_container: &FGameplayTagContainer,
        gameplay_tags: &mut TArray<FGameplayTag>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_break_gameplay_tag_container,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_tags,
                __buffer.add(32).cast::<TArray<FGameplayTag>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_break_gameplay_tag_container,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<TArray<FGameplayTag>>().swap(gameplay_tags);
        }
    }
    pub fn append_gameplay_tag_containers(
        in_out_tag_container: &mut FGameplayTagContainer,
        in_tag_container: &FGameplayTagContainer,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_append_gameplay_tag_containers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tag_container,
                __buffer.add(32).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_append_gameplay_tag_containers,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FGameplayTagContainer>().swap(in_out_tag_container);
        }
    }
    pub fn add_gameplay_tag(
        tag_container: &mut FGameplayTagContainer,
        tag: FGameplayTag,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_add_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag,
                __buffer.add(32).cast::<FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_tags::UBlueprintGameplayTagLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_blueprint_gameplay_tag_library_add_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FGameplayTagContainer>().swap(tag_container);
        }
    }
}
pub struct IGameplayTagAssetInterface {}
#[repr(C, align(8))]
pub struct UGameplayTagAssetInterface {
    __padding_end: [u8; 48],
}
impl UGameplayTagAssetInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTagAssetInterface")
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
    pub fn has_matching_gameplay_tag(&self, tag_to_check: FGameplayTag) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_gameplay_tag_asset_interface_has_matching_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_to_check,
                __buffer.add(0).cast::<FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_gameplay_tag_asset_interface_has_matching_gameplay_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn has_any_matching_gameplay_tags(
        &self,
        tag_container: &FGameplayTagContainer,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_gameplay_tag_asset_interface_has_any_matching_gameplay_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_gameplay_tag_asset_interface_has_any_matching_gameplay_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn has_all_matching_gameplay_tags(
        &self,
        tag_container: &FGameplayTagContainer,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_gameplay_tag_asset_interface_has_all_matching_gameplay_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_container,
                __buffer.add(0).cast::<FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_gameplay_tag_asset_interface_has_all_matching_gameplay_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn bp_get_owned_gameplay_tags(&self) -> FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_gameplay_tag_asset_interface_bp_get_owned_gameplay_tags,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_tags::__FUNCTION_PTRS
                    .u_gameplay_tag_asset_interface_bp_get_owned_gameplay_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FGameplayTagContainer>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEditableGameplayTagQuery {
    __padding_end: [u8; 160],
}
impl UEditableGameplayTagQuery {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableGameplayTagQuery")
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
pub struct UEditableGameplayTagQueryExpression {
    __padding_end: [u8; 48],
}
impl UEditableGameplayTagQueryExpression {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableGameplayTagQueryExpression")
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
pub struct UEditableGameplayTagQueryExpression_AnyTagsMatch {
    __padding_end: [u8; 80],
}
impl UEditableGameplayTagQueryExpression_AnyTagsMatch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableGameplayTagQueryExpression_AnyTagsMatch")
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
pub struct UEditableGameplayTagQueryExpression_AllTagsMatch {
    __padding_end: [u8; 80],
}
impl UEditableGameplayTagQueryExpression_AllTagsMatch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableGameplayTagQueryExpression_AllTagsMatch")
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
pub struct UEditableGameplayTagQueryExpression_NoTagsMatch {
    __padding_end: [u8; 80],
}
impl UEditableGameplayTagQueryExpression_NoTagsMatch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableGameplayTagQueryExpression_NoTagsMatch")
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
pub struct UEditableGameplayTagQueryExpression_AnyTagsExactMatch {
    __padding_end: [u8; 80],
}
impl UEditableGameplayTagQueryExpression_AnyTagsExactMatch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableGameplayTagQueryExpression_AnyTagsExactMatch")
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
pub struct UEditableGameplayTagQueryExpression_AllTagsExactMatch {
    __padding_end: [u8; 80],
}
impl UEditableGameplayTagQueryExpression_AllTagsExactMatch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableGameplayTagQueryExpression_AllTagsExactMatch")
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
pub struct UEditableGameplayTagQueryExpression_AnyExprMatch {
    __padding_end: [u8; 64],
}
impl UEditableGameplayTagQueryExpression_AnyExprMatch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableGameplayTagQueryExpression_AnyExprMatch")
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
pub struct UEditableGameplayTagQueryExpression_AllExprMatch {
    __padding_end: [u8; 64],
}
impl UEditableGameplayTagQueryExpression_AllExprMatch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableGameplayTagQueryExpression_AllExprMatch")
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
pub struct UEditableGameplayTagQueryExpression_NoExprMatch {
    __padding_end: [u8; 64],
}
impl UEditableGameplayTagQueryExpression_NoExprMatch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditableGameplayTagQueryExpression_NoExprMatch")
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
pub struct UGameplayTagsManager {
    __padding_end: [u8; 1248],
}
impl UGameplayTagsManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTagsManager")
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
pub struct UGameplayTagsList {
    __padding_end: [u8; 96],
}
impl UGameplayTagsList {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTagsList")
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
pub struct URestrictedGameplayTagsList {
    __padding_end: [u8; 80],
}
impl URestrictedGameplayTagsList {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URestrictedGameplayTagsList")
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
pub struct UGameplayTagsSettings {
    __padding_end: [u8; 256],
}
impl UGameplayTagsSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTagsSettings")
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
pub struct UGameplayTagsDeveloperSettings {
    __padding_end: [u8; 136],
}
impl UGameplayTagsDeveloperSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTagsDeveloperSettings")
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
pub struct EGameplayTagSourceType(pub u8);
impl EGameplayTagSourceType {
    pub const NATIVE: EGameplayTagSourceType = EGameplayTagSourceType(0);
    pub const DEFAULT_TAG_LIST: EGameplayTagSourceType = EGameplayTagSourceType(1);
    pub const TAG_LIST: EGameplayTagSourceType = EGameplayTagSourceType(2);
    pub const RESTRICTED_TAG_LIST: EGameplayTagSourceType = EGameplayTagSourceType(3);
    pub const DATA_TABLE: EGameplayTagSourceType = EGameplayTagSourceType(4);
    pub const INVALID: EGameplayTagSourceType = EGameplayTagSourceType(5);
}
#[repr(transparent)]
pub struct EGameplayContainerMatchType(pub u8);
impl EGameplayContainerMatchType {
    pub const ANY: EGameplayContainerMatchType = EGameplayContainerMatchType(0);
    pub const ALL: EGameplayContainerMatchType = EGameplayContainerMatchType(1);
}
