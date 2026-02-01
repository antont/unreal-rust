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
    pub u_metasound_parameter_pack_set_trigger: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_set_string: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_set_int: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_set_float: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_set_bool: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_make_metasound_parameter_pack: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_has_trigger: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_has_string: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_has_int: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_has_float: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_has_bool: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_get_trigger: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_get_string: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_get_int: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_get_float: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_parameter_pack_get_bool: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_metasound_parameter_pack_set_trigger: std::ptr::null_mut(),
            u_metasound_parameter_pack_set_string: std::ptr::null_mut(),
            u_metasound_parameter_pack_set_int: std::ptr::null_mut(),
            u_metasound_parameter_pack_set_float: std::ptr::null_mut(),
            u_metasound_parameter_pack_set_bool: std::ptr::null_mut(),
            u_metasound_parameter_pack_make_metasound_parameter_pack: std::ptr::null_mut(),
            u_metasound_parameter_pack_has_trigger: std::ptr::null_mut(),
            u_metasound_parameter_pack_has_string: std::ptr::null_mut(),
            u_metasound_parameter_pack_has_int: std::ptr::null_mut(),
            u_metasound_parameter_pack_has_float: std::ptr::null_mut(),
            u_metasound_parameter_pack_has_bool: std::ptr::null_mut(),
            u_metasound_parameter_pack_get_trigger: std::ptr::null_mut(),
            u_metasound_parameter_pack_get_string: std::ptr::null_mut(),
            u_metasound_parameter_pack_get_int: std::ptr::null_mut(),
            u_metasound_parameter_pack_get_float: std::ptr::null_mut(),
            u_metasound_parameter_pack_get_bool: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMetasoundParameterPack::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTrigger"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_set_trigger,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetString"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_set_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetInt"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_set_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFloat"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_set_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBool"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_set_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeMetasoundParameterPack"),
                &raw mut __FUNCTION_PTRS
                    .u_metasound_parameter_pack_make_metasound_parameter_pack,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasTrigger"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_has_trigger,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasString"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_has_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasInt"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_has_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasFloat"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_has_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasBool"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_has_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTrigger"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_get_trigger,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetString"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_get_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInt"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_get_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFloat"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_get_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBool"),
                &raw mut __FUNCTION_PTRS.u_metasound_parameter_pack_get_bool,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FMetasoundFrontendVersionNumber {
    pub major: i32,
    pub minor: i32,
}
impl FMetasoundFrontendVersionNumber {}
#[repr(C, align(4))]
pub struct FMetasoundFrontendClassName {
    pub namespace: FName,
    pub name: FName,
    pub variant: FName,
}
impl FMetasoundFrontendClassName {}
#[repr(C, align(4))]
pub struct FMetasoundFrontendVersion {
    pub name: FName,
    pub number: FMetasoundFrontendVersionNumber,
}
impl FMetasoundFrontendVersion {}
#[repr(C, align(8))]
pub struct FMetasoundFrontendInterfaceMetadata {
    pub version: FMetasoundFrontendVersion,
    pub u_class_options: TArray<FMetasoundFrontendInterfaceUClassOptions>,
}
impl FMetasoundFrontendInterfaceMetadata {}
#[repr(C, align(4))]
pub struct FMetasoundFrontendInterfaceUClassOptions {
    pub class_path: crate::bindings::core_u_object::FTopLevelAssetPath,
    pub(crate) __padding_end: [u8; 4],
}
impl FMetasoundFrontendInterfaceUClassOptions {}
#[repr(C, align(8))]
pub struct FMetaSoundDocumentInfo {
    pub document_version: FMetasoundFrontendVersionNumber,
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 16],
    pub flags_24: u8,
}
impl FMetaSoundDocumentInfo {}
#[repr(C, align(4))]
pub struct FMetasoundCommentNodeIntVector {
    pub(crate) __padding_end: [u8; 8],
}
impl FMetasoundCommentNodeIntVector {}
#[repr(C, align(8))]
pub struct FMetasoundFrontendLiteral {
    pub(crate) __padding_end: [u8; 88],
}
impl FMetasoundFrontendLiteral {}
#[repr(C, align(4))]
pub struct FMetaSoundClassVertexInfo {
    pub name: FName,
    pub type_name: FName,
    pub access_type: EMetasoundFrontendVertexAccessType,
}
impl FMetaSoundClassVertexInfo {}
#[repr(C, align(8))]
pub struct FMetaSoundClassSearchInfo {
    pub class_display_name: FText,
    pub class_description: FText,
    pub hierarchy: TArray<FText>,
    pub keywords: TArray<FText>,
}
impl FMetaSoundClassSearchInfo {}
#[repr(C, align(8))]
pub struct FMetaSoundClassVertexCollectionInfo {
    pub class_vertex_info: TArray<FMetaSoundClassVertexInfo>,
}
impl FMetaSoundClassVertexCollectionInfo {}
#[repr(C, align(8))]
pub struct FMetaSoundClassInterfaceInfo {
    pub defined_interfaces: TArray<FMetasoundFrontendInterfaceMetadata>,
    pub search_info: FMetaSoundClassSearchInfo,
    pub inputs: TArray<FMetaSoundClassVertexInfo>,
    pub outputs: TArray<FMetaSoundClassVertexInfo>,
    pub inherited_interfaces: TArray<FMetasoundFrontendVersion>,
}
impl FMetaSoundClassInterfaceInfo {}
pub struct IMetaSoundDocumentInterface {}
#[repr(C, align(8))]
pub struct UMetaSoundDocumentInterface {
    __padding_end: [u8; 48],
}
impl UMetaSoundDocumentInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundDocumentInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundDocumentInterface")
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
pub struct UMetaSoundFrontendMemberMetadata {
    __padding_end: [u8; 64],
}
impl UMetaSoundFrontendMemberMetadata {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundFrontendMemberMetadata")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundFrontendMemberMetadata")
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
pub struct UMetaSoundBuilderDocument {
    __padding_end: [u8; 1416],
}
impl UMetaSoundBuilderDocument {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundBuilderDocument")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundBuilderDocument")
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
pub struct UMetasoundParameterPack {
    __padding_end: [u8; 72],
}
impl UMetasoundParameterPack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundParameterPack")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundParameterPack")
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
    pub fn set_trigger(
        &mut self,
        parameter_name: FName,
        only_if_exists: bool,
    ) -> ESetParamResult {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_set_trigger,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_if_exists,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_set_trigger,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<ESetParamResult>().read() }
    }
    pub fn set_string(
        &mut self,
        parameter_name: FName,
        in_value: FString,
        only_if_exists: bool,
    ) -> ESetParamResult {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_set_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_if_exists,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_set_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<ESetParamResult>().read() }
    }
    pub fn set_int(
        &mut self,
        parameter_name: FName,
        in_value: i32,
        only_if_exists: bool,
    ) -> ESetParamResult {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_set_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_if_exists,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_set_int,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<ESetParamResult>().read() }
    }
    pub fn set_float(
        &mut self,
        parameter_name: FName,
        in_value: f32,
        only_if_exists: bool,
    ) -> ESetParamResult {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_set_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_if_exists,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_set_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<ESetParamResult>().read() }
    }
    pub fn set_bool(
        &mut self,
        parameter_name: FName,
        in_value: bool,
        only_if_exists: bool,
    ) -> ESetParamResult {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_set_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_if_exists,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_set_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<ESetParamResult>().read() }
    }
    pub fn make_metasound_parameter_pack() -> UPtr<UMetasoundParameterPack> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_make_metasound_parameter_pack,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::metasound_frontend::UMetasoundParameterPack::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_make_metasound_parameter_pack,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMetasoundParameterPack>>().read() }
    }
    pub fn has_trigger(&self, parameter_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_has_trigger,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_has_trigger,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn has_string(&self, parameter_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_has_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_has_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn has_int(&self, parameter_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_has_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_has_int,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn has_float(&self, parameter_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_has_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_has_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn has_bool(&self, parameter_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_has_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_has_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_trigger(
        &self,
        parameter_name: FName,
        result: &mut ESetParamResult,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_get_trigger,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(12).cast::<ESetParamResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_get_trigger,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<ESetParamResult>().swap(result);
        }
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn get_string(
        &self,
        parameter_name: FName,
        result: &mut ESetParamResult,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_get_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(12).cast::<ESetParamResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_get_string,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<ESetParamResult>().swap(result);
        }
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_int(&self, parameter_name: FName, result: &mut ESetParamResult) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_get_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(12).cast::<ESetParamResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_get_int,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<ESetParamResult>().swap(result);
        }
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_float(&self, parameter_name: FName, result: &mut ESetParamResult) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_get_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(12).cast::<ESetParamResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_get_float,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<ESetParamResult>().swap(result);
        }
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_bool(&self, parameter_name: FName, result: &mut ESetParamResult) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_get_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(12).cast::<ESetParamResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_frontend::__FUNCTION_PTRS
                    .u_metasound_parameter_pack_get_bool,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<ESetParamResult>().swap(result);
        }
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
}
#[repr(transparent)]
pub struct EMetaSoundFrontendGraphCommentMoveMode(pub u8);
impl EMetaSoundFrontendGraphCommentMoveMode {
    pub const GROUP_MOVEMENT: EMetaSoundFrontendGraphCommentMoveMode = EMetaSoundFrontendGraphCommentMoveMode(
        0,
    );
    pub const NO_GROUP_MOVEMENT: EMetaSoundFrontendGraphCommentMoveMode = EMetaSoundFrontendGraphCommentMoveMode(
        1,
    );
}
#[repr(transparent)]
pub struct EMetasoundFrontendLiteralType(pub u8);
impl EMetasoundFrontendLiteralType {
    pub const NONE: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(0);
    pub const BOOLEAN: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(1);
    pub const INTEGER: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(2);
    pub const FLOAT: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(3);
    pub const STRING: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(4);
    pub const U_OBJECT: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(5);
    pub const NONE_ARRAY: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(
        6,
    );
    pub const BOOLEAN_ARRAY: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(
        7,
    );
    pub const INTEGER_ARRAY: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(
        8,
    );
    pub const FLOAT_ARRAY: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(
        9,
    );
    pub const STRING_ARRAY: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(
        10,
    );
    pub const U_OBJECT_ARRAY: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(
        11,
    );
    pub const INVALID: EMetasoundFrontendLiteralType = EMetasoundFrontendLiteralType(12);
}
#[repr(transparent)]
pub struct EMetasoundFrontendNodeStyleDisplayVisibility(pub u8);
impl EMetasoundFrontendNodeStyleDisplayVisibility {
    pub const VISIBLE: EMetasoundFrontendNodeStyleDisplayVisibility = EMetasoundFrontendNodeStyleDisplayVisibility(
        0,
    );
    pub const HIDDEN: EMetasoundFrontendNodeStyleDisplayVisibility = EMetasoundFrontendNodeStyleDisplayVisibility(
        1,
    );
}
#[repr(transparent)]
pub struct EMetasoundFrontendVertexAccessType(pub i32);
impl EMetasoundFrontendVertexAccessType {
    pub const REFERENCE: EMetasoundFrontendVertexAccessType = EMetasoundFrontendVertexAccessType(
        0,
    );
    pub const VALUE: EMetasoundFrontendVertexAccessType = EMetasoundFrontendVertexAccessType(
        1,
    );
    pub const UNSET: EMetasoundFrontendVertexAccessType = EMetasoundFrontendVertexAccessType(
        2,
    );
}
#[repr(transparent)]
pub struct EMetasoundFrontendClassType(pub u8);
impl EMetasoundFrontendClassType {
    pub const EXTERNAL: EMetasoundFrontendClassType = EMetasoundFrontendClassType(0);
    pub const GRAPH: EMetasoundFrontendClassType = EMetasoundFrontendClassType(1);
    pub const INPUT: EMetasoundFrontendClassType = EMetasoundFrontendClassType(2);
    pub const OUTPUT: EMetasoundFrontendClassType = EMetasoundFrontendClassType(3);
    pub const LITERAL: EMetasoundFrontendClassType = EMetasoundFrontendClassType(4);
    pub const VARIABLE: EMetasoundFrontendClassType = EMetasoundFrontendClassType(5);
    pub const VARIABLE_DEFERRED_ACCESSOR: EMetasoundFrontendClassType = EMetasoundFrontendClassType(
        6,
    );
    pub const VARIABLE_ACCESSOR: EMetasoundFrontendClassType = EMetasoundFrontendClassType(
        7,
    );
    pub const VARIABLE_MUTATOR: EMetasoundFrontendClassType = EMetasoundFrontendClassType(
        8,
    );
    pub const TEMPLATE: EMetasoundFrontendClassType = EMetasoundFrontendClassType(9);
    pub const INVALID: EMetasoundFrontendClassType = EMetasoundFrontendClassType(10);
}
#[repr(transparent)]
pub struct ESetParamResult(pub u8);
impl ESetParamResult {
    pub const SUCCEEDED: ESetParamResult = ESetParamResult(0);
    pub const FAILED: ESetParamResult = ESetParamResult(1);
}
