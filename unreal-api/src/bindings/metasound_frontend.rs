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
pub static mut U_METASOUND_PARAMETER_PACK_SET_TRIGGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_SET_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_SET_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_SET_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_SET_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_MAKE_METASOUND_PARAMETER_PACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_HAS_TRIGGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_HAS_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_HAS_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_HAS_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_HAS_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_GET_TRIGGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_GET_STRING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_GET_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_GET_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_METASOUND_PARAMETER_PACK_GET_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetasoundParameterPack::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTrigger"),
            &raw mut U_METASOUND_PARAMETER_PACK_SET_TRIGGER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetString"),
            &raw mut U_METASOUND_PARAMETER_PACK_SET_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInt"),
            &raw mut U_METASOUND_PARAMETER_PACK_SET_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFloat"),
            &raw mut U_METASOUND_PARAMETER_PACK_SET_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBool"),
            &raw mut U_METASOUND_PARAMETER_PACK_SET_BOOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeMetasoundParameterPack"),
            &raw mut U_METASOUND_PARAMETER_PACK_MAKE_METASOUND_PARAMETER_PACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasTrigger"),
            &raw mut U_METASOUND_PARAMETER_PACK_HAS_TRIGGER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasString"),
            &raw mut U_METASOUND_PARAMETER_PACK_HAS_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasInt"),
            &raw mut U_METASOUND_PARAMETER_PACK_HAS_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasFloat"),
            &raw mut U_METASOUND_PARAMETER_PACK_HAS_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasBool"),
            &raw mut U_METASOUND_PARAMETER_PACK_HAS_BOOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTrigger"),
            &raw mut U_METASOUND_PARAMETER_PACK_GET_TRIGGER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetString"),
            &raw mut U_METASOUND_PARAMETER_PACK_GET_STRING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInt"),
            &raw mut U_METASOUND_PARAMETER_PACK_GET_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFloat"),
            &raw mut U_METASOUND_PARAMETER_PACK_GET_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBool"),
            &raw mut U_METASOUND_PARAMETER_PACK_GET_BOOL,
        );
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
    __padding_end: [u8; 4],
}
impl FMetasoundFrontendInterfaceUClassOptions {}
#[repr(C, align(8))]
pub struct FMetaSoundDocumentInfo {
    pub document_version: FMetasoundFrontendVersionNumber,
    #[doc(hidden)]
    __padding_24: [u8; 16],
    pub flags_24: u8,
    __padding_end: [u8; 7],
}
impl FMetaSoundDocumentInfo {}
#[repr(C, align(4))]
pub struct FMetasoundCommentNodeIntVector {
    __padding_end: [u8; 8],
}
impl FMetasoundCommentNodeIntVector {}
#[repr(C, align(8))]
pub struct FMetasoundFrontendLiteral {
    __padding_end: [u8; 88],
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
