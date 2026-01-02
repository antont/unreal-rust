#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
pub struct UMetaSoundDocumentInterface {}
pub struct IMetaSoundDocumentInterface {}
#[repr(C, align(8))]
pub struct UMetaSoundFrontendMemberMetadata {
    __padding_end: [u8; 64],
}
impl UMetaSoundFrontendMemberMetadata {}
#[repr(C, align(8))]
pub struct UMetaSoundBuilderDocument {
    __padding_end: [u8; 1416],
}
impl UMetaSoundBuilderDocument {}
#[repr(C, align(8))]
pub struct UMetasoundParameterPack {
    __padding_end: [u8; 72],
}
impl UMetasoundParameterPack {}
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
