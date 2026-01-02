#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAnimationCompressionLibraryDatabase {
    __padding_end: [u8; 552],
}
impl UAnimationCompressionLibraryDatabase {}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionCodec_ACLBase {
    __padding_end: [u8; 80],
}
impl UAnimBoneCompressionCodec_ACLBase {}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionCodec_ACL {
    __padding_end: [u8; 272],
}
impl UAnimBoneCompressionCodec_ACL {}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionCodec_ACLCustom {
    __padding_end: [u8; 280],
}
impl UAnimBoneCompressionCodec_ACLCustom {}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionCodec_ACLDatabase {
    __padding_end: [u8; 104],
}
impl UAnimBoneCompressionCodec_ACLDatabase {}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionCodec_ACLSafe {
    __padding_end: [u8; 80],
}
impl UAnimBoneCompressionCodec_ACLSafe {}
#[repr(C, align(8))]
pub struct UAnimCurveCompressionCodec_ACL {
    __padding_end: [u8; 64],
}
impl UAnimCurveCompressionCodec_ACL {}
#[repr(transparent)]
pub struct ACLVisualFidelity(pub u8);
impl ACLVisualFidelity {
    pub const HIGHEST: ACLVisualFidelity = ACLVisualFidelity(0);
    pub const MEDIUM: ACLVisualFidelity = ACLVisualFidelity(1);
    pub const LOWEST: ACLVisualFidelity = ACLVisualFidelity(2);
}
#[repr(transparent)]
pub struct ACLVisualFidelityChangeResult(pub u8);
impl ACLVisualFidelityChangeResult {
    pub const DISPATCHED: ACLVisualFidelityChangeResult = ACLVisualFidelityChangeResult(
        0,
    );
    pub const COMPLETED: ACLVisualFidelityChangeResult = ACLVisualFidelityChangeResult(
        1,
    );
    pub const FAILED: ACLVisualFidelityChangeResult = ACLVisualFidelityChangeResult(2);
}
#[repr(transparent)]
pub struct ACLCompressionLevel(pub u8);
impl ACLCompressionLevel {
    pub const ACLCL_LOWEST: ACLCompressionLevel = ACLCompressionLevel(0);
    pub const ACLCL_LOW: ACLCompressionLevel = ACLCompressionLevel(1);
    pub const ACLCL_MEDIUM: ACLCompressionLevel = ACLCompressionLevel(2);
    pub const ACLCL_HIGH: ACLCompressionLevel = ACLCompressionLevel(3);
    pub const ACLCL_HIGHEST: ACLCompressionLevel = ACLCompressionLevel(4);
    pub const ACLCL_AUTOMATIC: ACLCompressionLevel = ACLCompressionLevel(5);
}
#[repr(transparent)]
pub struct ACLPhantomTrackMode(pub u8);
impl ACLPhantomTrackMode {
    pub const IGNORE: ACLPhantomTrackMode = ACLPhantomTrackMode(0);
    pub const STRIP: ACLPhantomTrackMode = ACLPhantomTrackMode(1);
    pub const WARN: ACLPhantomTrackMode = ACLPhantomTrackMode(2);
}
#[repr(transparent)]
pub struct ACLRotationFormat(pub u8);
impl ACLRotationFormat {
    pub const ACLRF_QUAT_128: ACLRotationFormat = ACLRotationFormat(0);
    pub const ACLRF_QUAT_DROP_W_96: ACLRotationFormat = ACLRotationFormat(1);
    pub const ACLRF_QUAT_DROP_W_VARIABLE: ACLRotationFormat = ACLRotationFormat(2);
}
#[repr(transparent)]
pub struct ACLVectorFormat(pub u8);
impl ACLVectorFormat {
    pub const ACLVF_VECTOR3_96: ACLVectorFormat = ACLVectorFormat(0);
    pub const ACLVF_VECTOR3_VARIABLE: ACLVectorFormat = ACLVectorFormat(1);
}
