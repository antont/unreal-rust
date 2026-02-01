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
    pub u_mass_eqs_blueprint_library_send_signal_to_entity: *mut crate::ffi::UFunctionOpague,
    pub u_mass_eqs_blueprint_library_get_enviroment_query_result_as_entity_info: *mut crate::ffi::UFunctionOpague,
    pub u_mass_eqs_blueprint_library_get_current_entity_position: *mut crate::ffi::UFunctionOpague,
    pub u_mass_eqs_blueprint_library_get_cached_entity_position: *mut crate::ffi::UFunctionOpague,
    pub u_mass_eqs_blueprint_library_entity_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_mass_eqs_blueprint_library_entity_comparison: *mut crate::ffi::UFunctionOpague,
    pub u_mass_eqs_blueprint_library_contains_entity: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_mass_eqs_blueprint_library_send_signal_to_entity: std::ptr::null_mut(),
            u_mass_eqs_blueprint_library_get_enviroment_query_result_as_entity_info: std::ptr::null_mut(),
            u_mass_eqs_blueprint_library_get_current_entity_position: std::ptr::null_mut(),
            u_mass_eqs_blueprint_library_get_cached_entity_position: std::ptr::null_mut(),
            u_mass_eqs_blueprint_library_entity_to_string: std::ptr::null_mut(),
            u_mass_eqs_blueprint_library_entity_comparison: std::ptr::null_mut(),
            u_mass_eqs_blueprint_library_contains_entity: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMassEQSBlueprintLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SendSignalToEntity"),
                &raw mut __FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_send_signal_to_entity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEnviromentQueryResultAsEntityInfo"),
                &raw mut __FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_get_enviroment_query_result_as_entity_info,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentEntityPosition"),
                &raw mut __FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_get_current_entity_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCachedEntityPosition"),
                &raw mut __FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_get_cached_entity_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EntityToString"),
                &raw mut __FUNCTION_PTRS.u_mass_eqs_blueprint_library_entity_to_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EntityComparison"),
                &raw mut __FUNCTION_PTRS.u_mass_eqs_blueprint_library_entity_comparison,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ContainsEntity"),
                &raw mut __FUNCTION_PTRS.u_mass_eqs_blueprint_library_contains_entity,
            );
        }
    }
}
#[repr(C, align(16))]
pub struct FMassEnvQueryEntityInfoBlueprintWrapper {
    pub(crate) __padding_end: [u8; 112],
}
impl FMassEnvQueryEntityInfoBlueprintWrapper {}
#[repr(C, align(8))]
pub struct UMassEnvQueryGenerator {
    __padding_end: [u8; 136],
}
impl UMassEnvQueryGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryGenerator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryGenerator")
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
pub struct UMassEnvQueryGenerator_MassEntityHandles {
    __padding_end: [u8; 208],
}
impl UMassEnvQueryGenerator_MassEntityHandles {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryGenerator_MassEntityHandles")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryGenerator_MassEntityHandles")
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
pub struct UEnvQueryItemType_MassEntityHandle {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_MassEntityHandle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_MassEntityHandle")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_MassEntityHandle")
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
pub struct UMassEQSBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UMassEQSBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEQSBlueprintLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEQSBlueprintLibrary")
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
    pub fn send_signal_to_entity(
        owner: UPtr<crate::bindings::engine::AActor>,
        entity_info: &FMassEnvQueryEntityInfoBlueprintWrapper,
        signal: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<140>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_send_signal_to_entity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                entity_info,
                __buffer.add(16).cast::<FMassEnvQueryEntityInfoBlueprintWrapper>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&signal, __buffer.add(128).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::mass_eqs::UMassEQSBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_send_signal_to_entity,
                __buffer,
            )
        };
    }
    pub fn get_enviroment_query_result_as_entity_info(
        query_instance: UPtr<
            crate::bindings::ai_module::UEnvQueryInstanceBlueprintWrapper,
        >,
    ) -> TArray<FMassEnvQueryEntityInfoBlueprintWrapper> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_get_enviroment_query_result_as_entity_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &query_instance,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::ai_module::UEnvQueryInstanceBlueprintWrapper,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mass_eqs::UMassEQSBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_get_enviroment_query_result_as_entity_info,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FMassEnvQueryEntityInfoBlueprintWrapper>>()
                .read()
        }
    }
    pub fn get_current_entity_position(
        owner: UPtr<crate::bindings::engine::AActor>,
        entity_info: &FMassEnvQueryEntityInfoBlueprintWrapper,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<152>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_get_current_entity_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                entity_info,
                __buffer.add(16).cast::<FMassEnvQueryEntityInfoBlueprintWrapper>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mass_eqs::UMassEQSBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_get_current_entity_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(128).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_cached_entity_position(
        entity_info: &FMassEnvQueryEntityInfoBlueprintWrapper,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_get_cached_entity_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                entity_info,
                __buffer.add(0).cast::<FMassEnvQueryEntityInfoBlueprintWrapper>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mass_eqs::UMassEQSBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_get_cached_entity_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn entity_to_string(
        entity_info: &FMassEnvQueryEntityInfoBlueprintWrapper,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_entity_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                entity_info,
                __buffer.add(0).cast::<FMassEnvQueryEntityInfoBlueprintWrapper>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mass_eqs::UMassEQSBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_entity_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(112).cast::<FString>().read() }
    }
    pub fn entity_comparison(
        a: &FMassEnvQueryEntityInfoBlueprintWrapper,
        b: &FMassEnvQueryEntityInfoBlueprintWrapper,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<225>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_entity_comparison,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                a,
                __buffer.add(0).cast::<FMassEnvQueryEntityInfoBlueprintWrapper>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b,
                __buffer.add(112).cast::<FMassEnvQueryEntityInfoBlueprintWrapper>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mass_eqs::UMassEQSBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_entity_comparison,
                __buffer,
            )
        };
        unsafe { __buffer.add(224).cast::<bool>().read() }
    }
    pub fn contains_entity(
        entity_list: &TArray<FMassEnvQueryEntityInfoBlueprintWrapper>,
        entity_info: &FMassEnvQueryEntityInfoBlueprintWrapper,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<129>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_contains_entity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                entity_list,
                __buffer
                    .add(0)
                    .cast::<TArray<FMassEnvQueryEntityInfoBlueprintWrapper>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                entity_info,
                __buffer.add(16).cast::<FMassEnvQueryEntityInfoBlueprintWrapper>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mass_eqs::UMassEQSBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_eqs::__FUNCTION_PTRS
                    .u_mass_eqs_blueprint_library_contains_entity,
                __buffer,
            )
        };
        unsafe { __buffer.add(128).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMassEQSSubsystem {
    __padding_end: [u8; 328],
}
impl UMassEQSSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEQSSubsystem")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEQSSubsystem")
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
pub struct IMassEQSRequestInterface {}
#[repr(C, align(8))]
pub struct UMassEQSRequestInterface {
    __padding_end: [u8; 48],
}
impl UMassEQSRequestInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEQSRequestInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEQSRequestInterface")
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
pub struct UMassEnvQueryProcessorBase {
    __padding_end: [u8; 256],
}
impl UMassEnvQueryProcessorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryProcessorBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryProcessorBase")
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
#[repr(C, align(16))]
pub struct UMassEnvQueryGeneratorProcessor_MassEntityHandles {
    __padding_end: [u8; 1136],
}
impl UMassEnvQueryGeneratorProcessor_MassEntityHandles {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryGeneratorProcessor_MassEntityHandles")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryGeneratorProcessor_MassEntityHandles")
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
#[repr(C, align(16))]
pub struct UMassEnvQueryTestProcessor_MassEntityTags {
    __padding_end: [u8; 1136],
}
impl UMassEnvQueryTestProcessor_MassEntityTags {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryTestProcessor_MassEntityTags")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryTestProcessor_MassEntityTags")
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
pub struct UMassEnvQueryTest {
    __padding_end: [u8; 656],
}
impl UMassEnvQueryTest {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryTest")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryTest")
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
pub struct UMassEnvQueryTest_MassEntityTags {
    __padding_end: [u8; 680],
}
impl UMassEnvQueryTest_MassEntityTags {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryTest_MassEntityTags")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEnvQueryTest_MassEntityTags")
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
#[repr(transparent)]
pub struct EMassEntityTagsTestMode(pub u8);
impl EMassEntityTagsTestMode {
    pub const ANY: EMassEntityTagsTestMode = EMassEntityTagsTestMode(0);
    pub const ALL: EMassEntityTagsTestMode = EMassEntityTagsTestMode(1);
    pub const NONE: EMassEntityTagsTestMode = EMassEntityTagsTestMode(2);
}
