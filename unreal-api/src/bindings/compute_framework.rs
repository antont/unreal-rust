#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UComputeDataInterface {
    __padding_end: [u8; 48],
}
impl UComputeDataInterface {}
#[repr(C, align(8))]
pub struct UComputeKernelSource {
    __padding_end: [u8; 160],
}
impl UComputeKernelSource {}
#[repr(C, align(8))]
pub struct UComputeKernelSourceWithText {
    __padding_end: [u8; 176],
}
impl UComputeKernelSourceWithText {}
#[repr(C, align(8))]
pub struct UComputeSource {
    __padding_end: [u8; 64],
}
impl UComputeSource {}
#[repr(C, align(8))]
pub struct UComputeDataInterfaceBuffer {
    __padding_end: [u8; 64],
}
impl UComputeDataInterfaceBuffer {}
#[repr(C, align(8))]
pub struct UComputeDataProvider {
    __padding_end: [u8; 48],
}
impl UComputeDataProvider {}
#[repr(C, align(8))]
pub struct UBufferDataProvider {
    __padding_end: [u8; 64],
}
impl UBufferDataProvider {}
#[repr(C, align(8))]
pub struct UComputeDataInterfaceDispatch {
    __padding_end: [u8; 64],
}
impl UComputeDataInterfaceDispatch {}
#[repr(C, align(8))]
pub struct UDispatchDataProvider {
    __padding_end: [u8; 64],
}
impl UDispatchDataProvider {}
#[repr(C, align(8))]
pub struct UComputeGraph {
    __padding_end: [u8; 232],
}
impl UComputeGraph {}
#[repr(C, align(8))]
pub struct UComputeGraphComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub compute_graph: UPtr<UComputeGraph>,
    __padding_end: [u8; 24],
}
impl UComputeGraphComponent {}
#[repr(C, align(8))]
pub struct UComputeGraphFromText {
    __padding_end: [u8; 248],
}
impl UComputeGraphFromText {}
#[repr(C, align(8))]
pub struct UComputeKernel {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub kernel_flags: i32,
    __padding_end: [u8; 4],
}
impl UComputeKernel {}
#[repr(transparent)]
pub struct EShaderFundamentalType(pub u8);
impl EShaderFundamentalType {
    pub const BOOL: EShaderFundamentalType = EShaderFundamentalType(0);
    pub const INT: EShaderFundamentalType = EShaderFundamentalType(1);
    pub const UINT: EShaderFundamentalType = EShaderFundamentalType(2);
    pub const FLOAT: EShaderFundamentalType = EShaderFundamentalType(3);
    pub const STRUCT: EShaderFundamentalType = EShaderFundamentalType(4);
    pub const NONE: EShaderFundamentalType = EShaderFundamentalType(255);
}
#[repr(transparent)]
pub struct EShaderFundamentalDimensionType(pub u8);
impl EShaderFundamentalDimensionType {
    pub const SCALAR: EShaderFundamentalDimensionType = EShaderFundamentalDimensionType(
        0,
    );
    pub const VECTOR: EShaderFundamentalDimensionType = EShaderFundamentalDimensionType(
        1,
    );
    pub const MATRIX: EShaderFundamentalDimensionType = EShaderFundamentalDimensionType(
        2,
    );
}
#[repr(transparent)]
pub struct EShaderParamBindingType(pub u8);
impl EShaderParamBindingType {
    pub const NONE: EShaderParamBindingType = EShaderParamBindingType(0);
    pub const CONSTANT_PARAMETER: EShaderParamBindingType = EShaderParamBindingType(1);
    pub const READ_ONLY_RESOURCE: EShaderParamBindingType = EShaderParamBindingType(2);
    pub const READ_WRITE_RESOURCE: EShaderParamBindingType = EShaderParamBindingType(3);
}
#[repr(transparent)]
pub struct EShaderResourceType(pub u8);
impl EShaderResourceType {
    pub const NONE: EShaderResourceType = EShaderResourceType(0);
    pub const TEXTURE1_D: EShaderResourceType = EShaderResourceType(1);
    pub const TEXTURE2_D: EShaderResourceType = EShaderResourceType(2);
    pub const TEXTURE3_D: EShaderResourceType = EShaderResourceType(3);
    pub const TEXTURE_CUBE: EShaderResourceType = EShaderResourceType(4);
    pub const BUFFER: EShaderResourceType = EShaderResourceType(5);
    pub const STRUCTURED_BUFFER: EShaderResourceType = EShaderResourceType(6);
    pub const BYTE_ADDRESS_BUFFER: EShaderResourceType = EShaderResourceType(7);
}
#[repr(transparent)]
pub struct EShaderParamModifier(pub u8);
impl EShaderParamModifier {
    pub const NONE: EShaderParamModifier = EShaderParamModifier(0);
    pub const IN: EShaderParamModifier = EShaderParamModifier(1);
    pub const OUT: EShaderParamModifier = EShaderParamModifier(2);
    pub const IN_OUT: EShaderParamModifier = EShaderParamModifier(3);
}
