#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_data_registry_subsystem_not_equal_data_registry_type: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_not_equal_data_registry_id: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_is_valid_data_registry_type: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_is_valid_data_registry_id: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_get_possible_data_registry_id_list: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_get_cached_item_from_lookup_bp: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_get_cached_item_bp: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_find_cached_item_from_lookup_bp: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_find_cached_item_bp: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_evaluate_data_registry_curve: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_equal_equal_data_registry_type: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_equal_equal_data_registry_id: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_conv_data_registry_type_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_conv_data_registry_id_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_data_registry_subsystem_acquire_item_bp: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_data_registry_subsystem_not_equal_data_registry_type: std::ptr::null_mut(),
            u_data_registry_subsystem_not_equal_data_registry_id: std::ptr::null_mut(),
            u_data_registry_subsystem_is_valid_data_registry_type: std::ptr::null_mut(),
            u_data_registry_subsystem_is_valid_data_registry_id: std::ptr::null_mut(),
            u_data_registry_subsystem_get_possible_data_registry_id_list: std::ptr::null_mut(),
            u_data_registry_subsystem_get_cached_item_from_lookup_bp: std::ptr::null_mut(),
            u_data_registry_subsystem_get_cached_item_bp: std::ptr::null_mut(),
            u_data_registry_subsystem_find_cached_item_from_lookup_bp: std::ptr::null_mut(),
            u_data_registry_subsystem_find_cached_item_bp: std::ptr::null_mut(),
            u_data_registry_subsystem_evaluate_data_registry_curve: std::ptr::null_mut(),
            u_data_registry_subsystem_equal_equal_data_registry_type: std::ptr::null_mut(),
            u_data_registry_subsystem_equal_equal_data_registry_id: std::ptr::null_mut(),
            u_data_registry_subsystem_conv_data_registry_type_to_string: std::ptr::null_mut(),
            u_data_registry_subsystem_conv_data_registry_id_to_string: std::ptr::null_mut(),
            u_data_registry_subsystem_acquire_item_bp: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDataRegistrySubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NotEqual_DataRegistryType"),
                &raw mut __FUNCTION_PTRS
                    .u_data_registry_subsystem_not_equal_data_registry_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NotEqual_DataRegistryId"),
                &raw mut __FUNCTION_PTRS
                    .u_data_registry_subsystem_not_equal_data_registry_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidDataRegistryType"),
                &raw mut __FUNCTION_PTRS
                    .u_data_registry_subsystem_is_valid_data_registry_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidDataRegistryId"),
                &raw mut __FUNCTION_PTRS
                    .u_data_registry_subsystem_is_valid_data_registry_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPossibleDataRegistryIdList"),
                &raw mut __FUNCTION_PTRS
                    .u_data_registry_subsystem_get_possible_data_registry_id_list,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCachedItemFromLookupBP"),
                &raw mut __FUNCTION_PTRS
                    .u_data_registry_subsystem_get_cached_item_from_lookup_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCachedItemBP"),
                &raw mut __FUNCTION_PTRS.u_data_registry_subsystem_get_cached_item_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindCachedItemFromLookupBP"),
                &raw mut __FUNCTION_PTRS
                    .u_data_registry_subsystem_find_cached_item_from_lookup_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindCachedItemBP"),
                &raw mut __FUNCTION_PTRS.u_data_registry_subsystem_find_cached_item_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateDataRegistryCurve"),
                &raw mut __FUNCTION_PTRS
                    .u_data_registry_subsystem_evaluate_data_registry_curve,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EqualEqual_DataRegistryType"),
                &raw mut __FUNCTION_PTRS
                    .u_data_registry_subsystem_equal_equal_data_registry_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EqualEqual_DataRegistryId"),
                &raw mut __FUNCTION_PTRS
                    .u_data_registry_subsystem_equal_equal_data_registry_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Conv_DataRegistryTypeToString"),
                &raw mut __FUNCTION_PTRS
                    .u_data_registry_subsystem_conv_data_registry_type_to_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Conv_DataRegistryIdToString"),
                &raw mut __FUNCTION_PTRS
                    .u_data_registry_subsystem_conv_data_registry_id_to_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AcquireItemBP"),
                &raw mut __FUNCTION_PTRS.u_data_registry_subsystem_acquire_item_bp,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FDataRegistryLookup {
    pub(crate) __padding_end: [u8; 32],
}
impl FDataRegistryLookup {}
#[repr(C, align(4))]
pub struct FDataRegistryId {
    pub registry_type: FDataRegistryType,
    pub item_name: FName,
}
impl FDataRegistryId {}
#[repr(C, align(4))]
pub struct FDataRegistryType {
    pub name: FName,
}
impl FDataRegistryType {}
#[repr(C, align(4))]
pub struct FDataRegistryCachePolicy {
    pub b_cache_is_always_volatile: bool,
    pub b_use_curve_table_cache_version: bool,
    pub min_number_kept: i32,
    pub max_number_kept: i32,
    pub force_keep_seconds: f32,
    pub force_release_seconds: f32,
}
impl FDataRegistryCachePolicy {}
#[repr(C, align(8))]
pub struct FSoftDataRegistryOrTable {
    pub(crate) __padding_end: [u8; 72],
}
impl FSoftDataRegistryOrTable {}
#[repr(C, align(8))]
pub struct FDataRegistryOrTableRow {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub data_table_row: crate::bindings::engine::FDataTableRowHandle,
    pub data_registry_id: FDataRegistryId,
}
impl FDataRegistryOrTableRow {}
#[repr(C, align(8))]
pub struct UDataRegistrySettings {
    __padding_end: [u8; 128],
}
impl UDataRegistrySettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataRegistrySettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataRegistrySettings")
            .copied()
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
pub struct UDataRegistry {
    __padding_end: [u8; 216],
}
impl UDataRegistry {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataRegistry")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataRegistry")
            .copied()
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
pub struct UDataRegistrySource {
    __padding_end: [u8; 64],
}
impl UDataRegistrySource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataRegistrySource")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataRegistrySource")
            .copied()
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
pub struct UMetaDataRegistrySource {
    __padding_end: [u8; 280],
}
impl UMetaDataRegistrySource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaDataRegistrySource")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaDataRegistrySource")
            .copied()
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
pub struct UDataRegistrySource_CurveTable {
    __padding_end: [u8; 176],
}
impl UDataRegistrySource_CurveTable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataRegistrySource_CurveTable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataRegistrySource_CurveTable")
            .copied()
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
pub struct UMetaDataRegistrySource_CurveTable {
    __padding_end: [u8; 296],
}
impl UMetaDataRegistrySource_CurveTable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaDataRegistrySource_CurveTable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaDataRegistrySource_CurveTable")
            .copied()
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
pub struct UDataRegistrySource_DataTable {
    __padding_end: [u8; 176],
}
impl UDataRegistrySource_DataTable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataRegistrySource_DataTable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataRegistrySource_DataTable")
            .copied()
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
pub struct UMetaDataRegistrySource_DataTable {
    __padding_end: [u8; 296],
}
impl UMetaDataRegistrySource_DataTable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaDataRegistrySource_DataTable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaDataRegistrySource_DataTable")
            .copied()
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
pub struct UDataRegistrySubsystem {
    __padding_end: [u8; 280],
}
impl UDataRegistrySubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataRegistrySubsystem")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataRegistrySubsystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn not_equal_data_registry_type(
        a: FDataRegistryType,
        b: FDataRegistryType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_not_equal_data_registry_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &a,
                __buffer.add(0).cast::<FDataRegistryType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b,
                __buffer.add(12).cast::<FDataRegistryType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_not_equal_data_registry_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn not_equal_data_registry_id(a: FDataRegistryId, b: FDataRegistryId) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_not_equal_data_registry_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &a,
                __buffer.add(0).cast::<FDataRegistryId>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b,
                __buffer.add(24).cast::<FDataRegistryId>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_not_equal_data_registry_id,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn is_valid_data_registry_type(data_registry_type: FDataRegistryType) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_is_valid_data_registry_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_registry_type,
                __buffer.add(0).cast::<FDataRegistryType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_is_valid_data_registry_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn is_valid_data_registry_id(data_registry_id: FDataRegistryId) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_is_valid_data_registry_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_registry_id,
                __buffer.add(0).cast::<FDataRegistryId>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_is_valid_data_registry_id,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_possible_data_registry_id_list(
        registry_type: FDataRegistryType,
        out_id_list: &mut TArray<FDataRegistryId>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_get_possible_data_registry_id_list,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &registry_type,
                __buffer.add(0).cast::<FDataRegistryType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_id_list,
                __buffer.add(16).cast::<TArray<FDataRegistryId>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_get_possible_data_registry_id_list,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<FDataRegistryId>>().swap(out_id_list);
        }
    }
    pub fn get_cached_item_from_lookup_bp(
        item_id: FDataRegistryId,
        resolved_lookup: &FDataRegistryLookup,
        out_item: &mut crate::bindings::engine::FTableRowBase,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_get_cached_item_from_lookup_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &item_id,
                __buffer.add(0).cast::<FDataRegistryId>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                resolved_lookup,
                __buffer.add(24).cast::<FDataRegistryLookup>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_item,
                __buffer.add(56).cast::<crate::bindings::engine::FTableRowBase>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_get_cached_item_from_lookup_bp,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<crate::bindings::engine::FTableRowBase>()
                .swap(out_item);
        }
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn get_cached_item_bp(
        item_id: FDataRegistryId,
        out_item: &mut crate::bindings::engine::FTableRowBase,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_get_cached_item_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &item_id,
                __buffer.add(0).cast::<FDataRegistryId>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_item,
                __buffer.add(24).cast::<crate::bindings::engine::FTableRowBase>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_get_cached_item_bp,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::engine::FTableRowBase>()
                .swap(out_item);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn find_cached_item_from_lookup_bp(
        item_id: FDataRegistryId,
        resolved_lookup: &FDataRegistryLookup,
        out_result: &mut EDataRegistrySubsystemGetItemResult,
        out_item: &mut crate::bindings::engine::FTableRowBase,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_find_cached_item_from_lookup_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &item_id,
                __buffer.add(0).cast::<FDataRegistryId>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                resolved_lookup,
                __buffer.add(24).cast::<FDataRegistryLookup>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(56).cast::<EDataRegistrySubsystemGetItemResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_item,
                __buffer.add(64).cast::<crate::bindings::engine::FTableRowBase>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_find_cached_item_from_lookup_bp,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<EDataRegistrySubsystemGetItemResult>()
                .swap(out_result);
        }
        unsafe {
            __buffer
                .add(64)
                .cast::<crate::bindings::engine::FTableRowBase>()
                .swap(out_item);
        }
    }
    pub fn find_cached_item_bp(
        item_id: FDataRegistryId,
        out_result: &mut EDataRegistrySubsystemGetItemResult,
        out_item: &mut crate::bindings::engine::FTableRowBase,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_find_cached_item_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &item_id,
                __buffer.add(0).cast::<FDataRegistryId>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(24).cast::<EDataRegistrySubsystemGetItemResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_item,
                __buffer.add(32).cast::<crate::bindings::engine::FTableRowBase>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_find_cached_item_bp,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<EDataRegistrySubsystemGetItemResult>()
                .swap(out_result);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::engine::FTableRowBase>()
                .swap(out_item);
        }
    }
    pub fn evaluate_data_registry_curve(
        item_id: FDataRegistryId,
        input_value: f32,
        default_value: f32,
        out_result: &mut EDataRegistrySubsystemGetItemResult,
        out_value: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_evaluate_data_registry_curve,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &item_id,
                __buffer.add(0).cast::<FDataRegistryId>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_value,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(28).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(32).cast::<EDataRegistrySubsystemGetItemResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(36).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_evaluate_data_registry_curve,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<EDataRegistrySubsystemGetItemResult>()
                .swap(out_result);
        }
        unsafe {
            __buffer.add(36).cast::<f32>().swap(out_value);
        }
    }
    pub fn equal_equal_data_registry_type(
        a: FDataRegistryType,
        b: FDataRegistryType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_equal_equal_data_registry_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &a,
                __buffer.add(0).cast::<FDataRegistryType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b,
                __buffer.add(12).cast::<FDataRegistryType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_equal_equal_data_registry_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn equal_equal_data_registry_id(a: FDataRegistryId, b: FDataRegistryId) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_equal_equal_data_registry_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &a,
                __buffer.add(0).cast::<FDataRegistryId>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b,
                __buffer.add(24).cast::<FDataRegistryId>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_equal_equal_data_registry_id,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn conv_data_registry_type_to_string(
        data_registry_type: FDataRegistryType,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_conv_data_registry_type_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_registry_type,
                __buffer.add(0).cast::<FDataRegistryType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_conv_data_registry_type_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn conv_data_registry_id_to_string(
        data_registry_id: FDataRegistryId,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_conv_data_registry_id_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_registry_id,
                __buffer.add(0).cast::<FDataRegistryId>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_conv_data_registry_id_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn acquire_item_bp(
        item_id: FDataRegistryId,
        acquire_callback: FAcquireItemBP_AcquireCallback,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_acquire_item_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &item_id,
                __buffer.add(0).cast::<FDataRegistryId>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &acquire_callback,
                __buffer.add(24).cast::<FAcquireItemBP_AcquireCallback>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::data_registry::UDataRegistrySubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_registry::__FUNCTION_PTRS
                    .u_data_registry_subsystem_acquire_item_bp,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct FAcquireItemBP_AcquireCallback {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EDataRegistryAcquireStatus(pub u8);
impl EDataRegistryAcquireStatus {
    pub const NOT_STARTED: EDataRegistryAcquireStatus = EDataRegistryAcquireStatus(0);
    pub const WAITING_FOR_INITIAL_ACQUIRE: EDataRegistryAcquireStatus = EDataRegistryAcquireStatus(
        1,
    );
    pub const INITIAL_ACQUIRE_FINISHED: EDataRegistryAcquireStatus = EDataRegistryAcquireStatus(
        2,
    );
    pub const WAITING_FOR_RESOURCES: EDataRegistryAcquireStatus = EDataRegistryAcquireStatus(
        3,
    );
    pub const ACQUIRE_FINISHED: EDataRegistryAcquireStatus = EDataRegistryAcquireStatus(
        4,
    );
    pub const ACQUIRE_ERROR: EDataRegistryAcquireStatus = EDataRegistryAcquireStatus(5);
    pub const DOES_NOT_EXIST: EDataRegistryAcquireStatus = EDataRegistryAcquireStatus(6);
}
#[repr(transparent)]
pub struct EDataRegistrySubsystemGetItemResult(pub u8);
impl EDataRegistrySubsystemGetItemResult {
    pub const FOUND: EDataRegistrySubsystemGetItemResult = EDataRegistrySubsystemGetItemResult(
        0,
    );
    pub const NOT_FOUND: EDataRegistrySubsystemGetItemResult = EDataRegistrySubsystemGetItemResult(
        1,
    );
}
#[repr(transparent)]
pub struct EMetaDataRegistrySourceAssetUsage(pub u8);
impl EMetaDataRegistrySourceAssetUsage {
    pub const NO_ASSETS: EMetaDataRegistrySourceAssetUsage = EMetaDataRegistrySourceAssetUsage(
        0,
    );
    pub const SEARCH_ASSETS: EMetaDataRegistrySourceAssetUsage = EMetaDataRegistrySourceAssetUsage(
        1,
    );
    pub const REGISTER_ASSETS: EMetaDataRegistrySourceAssetUsage = EMetaDataRegistrySourceAssetUsage(
        2,
    );
    pub const SEARCH_AND_REGISTER_ASSETS: EMetaDataRegistrySourceAssetUsage = EMetaDataRegistrySourceAssetUsage(
        3,
    );
}
