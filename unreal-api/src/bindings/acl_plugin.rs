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
    pub u_animation_compression_library_database_set_visual_fidelity: *mut crate::ffi::UFunctionOpague,
    pub u_animation_compression_library_database_get_visual_fidelity: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_animation_compression_library_database_set_visual_fidelity: std::ptr::null_mut(),
            u_animation_compression_library_database_get_visual_fidelity: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAnimationCompressionLibraryDatabase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetVisualFidelity"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_compression_library_database_set_visual_fidelity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVisualFidelity"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_compression_library_database_get_visual_fidelity,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimationCompressionLibraryDatabase {
    __padding_end: [u8; 552],
}
impl UAnimationCompressionLibraryDatabase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationCompressionLibraryDatabase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationCompressionLibraryDatabase")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_visual_fidelity(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        latent_info: crate::bindings::engine::FLatentActionInfo,
        database_asset: UPtr<UAnimationCompressionLibraryDatabase>,
        result: &mut ACLVisualFidelityChangeResult,
        visual_fidelity: ACLVisualFidelity,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::acl_plugin::__FUNCTION_PTRS
                    .u_animation_compression_library_database_set_visual_fidelity,
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
                &latent_info,
                __buffer.add(8).cast::<crate::bindings::engine::FLatentActionInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &database_asset,
                __buffer.add(40).cast::<UPtr<UAnimationCompressionLibraryDatabase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(48).cast::<ACLVisualFidelityChangeResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &visual_fidelity,
                __buffer.add(49).cast::<ACLVisualFidelity>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::acl_plugin::UAnimationCompressionLibraryDatabase::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::acl_plugin::__FUNCTION_PTRS
                    .u_animation_compression_library_database_set_visual_fidelity,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<ACLVisualFidelityChangeResult>().swap(result);
        }
    }
    pub fn get_visual_fidelity(
        database_asset: UPtr<UAnimationCompressionLibraryDatabase>,
    ) -> ACLVisualFidelity {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::acl_plugin::__FUNCTION_PTRS
                    .u_animation_compression_library_database_get_visual_fidelity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &database_asset,
                __buffer.add(0).cast::<UPtr<UAnimationCompressionLibraryDatabase>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::acl_plugin::UAnimationCompressionLibraryDatabase::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::acl_plugin::__FUNCTION_PTRS
                    .u_animation_compression_library_database_get_visual_fidelity,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<ACLVisualFidelity>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionCodec_ACLBase {
    __padding_end: [u8; 80],
}
impl UAnimBoneCompressionCodec_ACLBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBoneCompressionCodec_ACLBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBoneCompressionCodec_ACLBase")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionCodec_ACL {
    __padding_end: [u8; 272],
}
impl UAnimBoneCompressionCodec_ACL {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBoneCompressionCodec_ACL")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBoneCompressionCodec_ACL")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionCodec_ACLCustom {
    __padding_end: [u8; 280],
}
impl UAnimBoneCompressionCodec_ACLCustom {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBoneCompressionCodec_ACLCustom")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBoneCompressionCodec_ACLCustom")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionCodec_ACLDatabase {
    __padding_end: [u8; 104],
}
impl UAnimBoneCompressionCodec_ACLDatabase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBoneCompressionCodec_ACLDatabase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBoneCompressionCodec_ACLDatabase")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimBoneCompressionCodec_ACLSafe {
    __padding_end: [u8; 80],
}
impl UAnimBoneCompressionCodec_ACLSafe {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBoneCompressionCodec_ACLSafe")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBoneCompressionCodec_ACLSafe")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimCurveCompressionCodec_ACL {
    __padding_end: [u8; 64],
}
impl UAnimCurveCompressionCodec_ACL {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimCurveCompressionCodec_ACL")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimCurveCompressionCodec_ACL")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
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
