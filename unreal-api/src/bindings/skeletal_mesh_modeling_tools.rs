#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct ISkeletalMeshBackedDynamicMeshComponentProvider {}
#[repr(C, align(8))]
pub struct USkeletalMeshBackedDynamicMeshComponentProvider {
    __padding_end: [u8; 48],
}
impl USkeletalMeshBackedDynamicMeshComponentProvider {}
#[repr(C, align(16))]
pub struct USkeletalMeshBackedDynamicMeshComponent {
    __padding_end: [u8; 3168],
}
impl USkeletalMeshBackedDynamicMeshComponent {}
#[repr(C, align(16))]
pub struct USkeletalMeshEditingCache {
    __padding_end: [u8; 1104],
}
impl USkeletalMeshEditingCache {}
#[repr(C, align(8))]
pub struct USkeletalMeshEditorContextObject {
    __padding_end: [u8; 280],
}
impl USkeletalMeshEditorContextObject {}
#[repr(C, align(8))]
pub struct USkeletalMeshGizmoWrapper {
    __padding_end: [u8; 72],
}
impl USkeletalMeshGizmoWrapper {}
#[repr(C, align(8))]
pub struct USkeletalMeshGizmoContextObject {
    __padding_end: [u8; 56],
}
impl USkeletalMeshGizmoContextObject {}
#[repr(C, align(16))]
pub struct USkeletalMeshModelingToolsEditorMode {
    __padding_end: [u8; 992],
}
impl USkeletalMeshModelingToolsEditorMode {}
#[repr(C, align(8))]
pub struct USkeletonFromStaticMeshFactory {
    __padding_end: [u8; 176],
}
impl USkeletonFromStaticMeshFactory {}
#[repr(C, align(8))]
pub struct USkeletalMeshFromStaticMeshFactory {
    __padding_end: [u8; 448],
}
impl USkeletalMeshFromStaticMeshFactory {}
#[repr(C, align(8))]
pub struct UStaticMeshToSkeletalMeshConvertOptions {
    __padding_end: [u8; 264],
}
impl UStaticMeshToSkeletalMeshConvertOptions {}
#[repr(C, align(8))]
pub struct USkeletalMeshBackedDynamicMeshComponentToolTarget {
    __padding_end: [u8; 176],
}
impl USkeletalMeshBackedDynamicMeshComponentToolTarget {}
#[repr(C, align(8))]
pub struct USkeletalMeshBackedDynamicMeshComponentToolTargetFactory {
    __padding_end: [u8; 64],
}
impl USkeletalMeshBackedDynamicMeshComponentToolTargetFactory {}
#[repr(transparent)]
pub struct ERootBonePositionReference(pub i32);
impl ERootBonePositionReference {
    pub const RELATIVE: ERootBonePositionReference = ERootBonePositionReference(0);
    pub const ABSOLUTE: ERootBonePositionReference = ERootBonePositionReference(1);
}
#[repr(transparent)]
pub struct EReferenceSkeletonImportOption(pub i32);
impl EReferenceSkeletonImportOption {
    pub const CREATE_NEW: EReferenceSkeletonImportOption = EReferenceSkeletonImportOption(
        0,
    );
    pub const USE_EXISTING_SKELETON: EReferenceSkeletonImportOption = EReferenceSkeletonImportOption(
        1,
    );
    pub const USE_EXISTING_SKELETAL_MESH: EReferenceSkeletonImportOption = EReferenceSkeletonImportOption(
        2,
    );
}
#[repr(transparent)]
pub struct ERootBonePlacementOptions(pub i32);
impl ERootBonePlacementOptions {
    pub const BOTTOM_CENTER: ERootBonePlacementOptions = ERootBonePlacementOptions(0);
    pub const CENTER: ERootBonePlacementOptions = ERootBonePlacementOptions(1);
    pub const ORIGIN: ERootBonePlacementOptions = ERootBonePlacementOptions(2);
}
