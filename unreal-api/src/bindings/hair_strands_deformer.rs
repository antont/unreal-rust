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
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UOptimusGroomCollisionReadDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusGroomCollisionReadDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomCollisionReadDataInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomCollisionReadDataInterface")
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
pub struct UOptimusGroomCollisionReadDataProvider {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub solver_component: UPtr<
        crate::bindings::hair_strands_solver::UGroomSolverComponent,
    >,
}
impl UOptimusGroomCollisionReadDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomCollisionReadDataProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomCollisionReadDataProvider")
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
pub struct UOptimusGroomAttributeReadDataInterface {
    __padding_end: [u8; 88],
}
impl UOptimusGroomAttributeReadDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomAttributeReadDataInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomAttributeReadDataInterface")
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
pub struct UOptimusGroomAttributeReadDataProvider {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
    __padding_end: [u8; 16],
}
impl UOptimusGroomAttributeReadDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomAttributeReadDataProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomAttributeReadDataProvider")
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
pub struct UOptimusGroomExecDataInterface {
    __padding_end: [u8; 64],
}
impl UOptimusGroomExecDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomExecDataInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomExecDataInterface")
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
pub struct UOptimusGroomExecDataProvider {
    __padding_end: [u8; 64],
}
impl UOptimusGroomExecDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomExecDataProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomExecDataProvider")
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
pub struct UOptimusGroomAssetComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusGroomAssetComponentSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomAssetComponentSource")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomAssetComponentSource")
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
pub struct UOptimusGroomSolverComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusGroomSolverComponentSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomSolverComponentSource")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomSolverComponentSource")
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
pub struct UOptimusGroomCollisionComponentSource {
    __padding_end: [u8; 48],
}
impl UOptimusGroomCollisionComponentSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomCollisionComponentSource")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomCollisionComponentSource")
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
pub struct UOptimusGroomGuidesReadDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusGroomGuidesReadDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomGuidesReadDataInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomGuidesReadDataInterface")
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
pub struct UOptimusGroomGuidesReadDataProvider {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
}
impl UOptimusGroomGuidesReadDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomGuidesReadDataProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomGuidesReadDataProvider")
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
pub struct UOptimusGroomGuidesWriteDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusGroomGuidesWriteDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomGuidesWriteDataInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomGuidesWriteDataInterface")
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
pub struct UOptimusGroomGuidesWriteDataProvider {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
    __padding_end: [u8; 8],
}
impl UOptimusGroomGuidesWriteDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomGuidesWriteDataProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomGuidesWriteDataProvider")
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
pub struct UOptimusGroomMeshesReadDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusGroomMeshesReadDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomMeshesReadDataInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomMeshesReadDataInterface")
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
pub struct UOptimusGroomMeshesReadDataProvider {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
}
impl UOptimusGroomMeshesReadDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomMeshesReadDataProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomMeshesReadDataProvider")
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
pub struct UOptimusGroomSolverReadDataInterface {
    __padding_end: [u8; 64],
}
impl UOptimusGroomSolverReadDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomSolverReadDataInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomSolverReadDataInterface")
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
pub struct UOptimusGroomSolverReadDataProvider {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub solver_component: UPtr<
        crate::bindings::hair_strands_solver::UGroomSolverComponent,
    >,
    __padding_end: [u8; 24],
}
impl UOptimusGroomSolverReadDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomSolverReadDataProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomSolverReadDataProvider")
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
pub struct UOptimusGroomStrandsReadDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusGroomStrandsReadDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomStrandsReadDataInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomStrandsReadDataInterface")
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
pub struct UOptimusGroomStrandsReadDataProvider {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
}
impl UOptimusGroomStrandsReadDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomStrandsReadDataProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomStrandsReadDataProvider")
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
pub struct UOptimusGroomStrandsWriteDataInterface {
    __padding_end: [u8; 48],
}
impl UOptimusGroomStrandsWriteDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomStrandsWriteDataInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomStrandsWriteDataInterface")
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
pub struct UOptimusGroomStrandsWriteDataProvider {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub mesh_component: UPtr<crate::bindings::engine::UMeshComponent>,
    __padding_end: [u8; 8],
}
impl UOptimusGroomStrandsWriteDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomStrandsWriteDataProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOptimusGroomStrandsWriteDataProvider")
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
pub struct EOptimusGroomExecDomain(pub u8);
impl EOptimusGroomExecDomain {
    pub const NONE: EOptimusGroomExecDomain = EOptimusGroomExecDomain(0);
    pub const CONTROL_POINT: EOptimusGroomExecDomain = EOptimusGroomExecDomain(1);
    pub const CURVE: EOptimusGroomExecDomain = EOptimusGroomExecDomain(2);
    pub const STRANDS_EDGES: EOptimusGroomExecDomain = EOptimusGroomExecDomain(3);
    pub const STRANDS_OBJECTS: EOptimusGroomExecDomain = EOptimusGroomExecDomain(4);
    pub const GUIDES_POINTS: EOptimusGroomExecDomain = EOptimusGroomExecDomain(5);
    pub const GUIDES_CURVES: EOptimusGroomExecDomain = EOptimusGroomExecDomain(6);
    pub const GUIDES_EDGES: EOptimusGroomExecDomain = EOptimusGroomExecDomain(7);
    pub const GUIDES_OBJECTS: EOptimusGroomExecDomain = EOptimusGroomExecDomain(8);
}
#[repr(transparent)]
pub struct EOptimusGroomAttributeTypes(pub u8);
impl EOptimusGroomAttributeTypes {
    pub const NONE: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(0);
    pub const BOOL: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(1);
    pub const INT: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(2);
    pub const INT_VECTOR2: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(3);
    pub const INT_VECTOR3: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(4);
    pub const INT_VECTOR4: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(5);
    pub const UINT: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(6);
    pub const FLOAT: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(7);
    pub const VECTOR2: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(8);
    pub const VECTOR3: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(9);
    pub const VECTOR4: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(10);
    pub const LINEAR_COLOR: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(
        11,
    );
    pub const QUAT: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(12);
    pub const ROTATOR: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(13);
    pub const TRANSFORM: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(14);
    pub const MATRIX3X4: EOptimusGroomAttributeTypes = EOptimusGroomAttributeTypes(15);
}
