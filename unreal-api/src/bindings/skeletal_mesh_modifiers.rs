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
pub static mut U_SKELETON_MODIFIER_SET_SKELETAL_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_SET_BONE_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_SET_BONES_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_RENAME_BONES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_RENAME_BONE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_REMOVE_BONES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_REMOVE_BONE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_PARENT_BONES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_PARENT_BONE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_ORIENT_BONES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_ORIENT_BONE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_MIRROR_BONES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_MIRROR_BONE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_GET_PARENT_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_GET_CHILDREN_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_GET_BONE_TRANSFORM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_GET_ALL_BONE_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_COMMIT_SKELETON_TO_SKELETAL_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_ADD_BONES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETON_MODIFIER_ADD_BONE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKIN_WEIGHT_MODIFIER_SET_VERTEX_WEIGHTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKIN_WEIGHT_MODIFIER_SET_SKELETAL_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKIN_WEIGHT_MODIFIER_PRUNE_VERTEX_WEIGHTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKIN_WEIGHT_MODIFIER_PRUNE_ALL_WEIGHTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKIN_WEIGHT_MODIFIER_NORMALIZE_VERTEX_WEIGHTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKIN_WEIGHT_MODIFIER_NORMALIZE_ALL_WEIGHTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKIN_WEIGHT_MODIFIER_GET_VERTEX_WEIGHTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKIN_WEIGHT_MODIFIER_GET_SKELETAL_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKIN_WEIGHT_MODIFIER_GET_NUM_VERTICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKIN_WEIGHT_MODIFIER_GET_ALL_BONE_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKIN_WEIGHT_MODIFIER_ENFORCE_MAX_INFLUENCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKIN_WEIGHT_MODIFIER_COMMIT_WEIGHTS_TO_SKELETAL_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USkeletonModifier::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkeletalMesh"),
            &raw mut U_SKELETON_MODIFIER_SET_SKELETAL_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoneTransform"),
            &raw mut U_SKELETON_MODIFIER_SET_BONE_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBonesTransforms"),
            &raw mut U_SKELETON_MODIFIER_SET_BONES_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameBones"),
            &raw mut U_SKELETON_MODIFIER_RENAME_BONES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameBone"),
            &raw mut U_SKELETON_MODIFIER_RENAME_BONE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveBones"),
            &raw mut U_SKELETON_MODIFIER_REMOVE_BONES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveBone"),
            &raw mut U_SKELETON_MODIFIER_REMOVE_BONE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ParentBones"),
            &raw mut U_SKELETON_MODIFIER_PARENT_BONES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ParentBone"),
            &raw mut U_SKELETON_MODIFIER_PARENT_BONE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OrientBones"),
            &raw mut U_SKELETON_MODIFIER_ORIENT_BONES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OrientBone"),
            &raw mut U_SKELETON_MODIFIER_ORIENT_BONE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MirrorBones"),
            &raw mut U_SKELETON_MODIFIER_MIRROR_BONES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MirrorBone"),
            &raw mut U_SKELETON_MODIFIER_MIRROR_BONE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParentName"),
            &raw mut U_SKELETON_MODIFIER_GET_PARENT_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChildrenNames"),
            &raw mut U_SKELETON_MODIFIER_GET_CHILDREN_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoneTransform"),
            &raw mut U_SKELETON_MODIFIER_GET_BONE_TRANSFORM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllBoneNames"),
            &raw mut U_SKELETON_MODIFIER_GET_ALL_BONE_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CommitSkeletonToSkeletalMesh"),
            &raw mut U_SKELETON_MODIFIER_COMMIT_SKELETON_TO_SKELETAL_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBones"),
            &raw mut U_SKELETON_MODIFIER_ADD_BONES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBone"),
            &raw mut U_SKELETON_MODIFIER_ADD_BONE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USkinWeightModifier::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVertexWeights"),
            &raw mut U_SKIN_WEIGHT_MODIFIER_SET_VERTEX_WEIGHTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkeletalMesh"),
            &raw mut U_SKIN_WEIGHT_MODIFIER_SET_SKELETAL_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PruneVertexWeights"),
            &raw mut U_SKIN_WEIGHT_MODIFIER_PRUNE_VERTEX_WEIGHTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PruneAllWeights"),
            &raw mut U_SKIN_WEIGHT_MODIFIER_PRUNE_ALL_WEIGHTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NormalizeVertexWeights"),
            &raw mut U_SKIN_WEIGHT_MODIFIER_NORMALIZE_VERTEX_WEIGHTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NormalizeAllWeights"),
            &raw mut U_SKIN_WEIGHT_MODIFIER_NORMALIZE_ALL_WEIGHTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVertexWeights"),
            &raw mut U_SKIN_WEIGHT_MODIFIER_GET_VERTEX_WEIGHTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkeletalMesh"),
            &raw mut U_SKIN_WEIGHT_MODIFIER_GET_SKELETAL_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumVertices"),
            &raw mut U_SKIN_WEIGHT_MODIFIER_GET_NUM_VERTICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllBoneNames"),
            &raw mut U_SKIN_WEIGHT_MODIFIER_GET_ALL_BONE_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnforceMaxInfluences"),
            &raw mut U_SKIN_WEIGHT_MODIFIER_ENFORCE_MAX_INFLUENCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CommitWeightsToSkeletalMesh"),
            &raw mut U_SKIN_WEIGHT_MODIFIER_COMMIT_WEIGHTS_TO_SKELETAL_MESH,
        );
    }
}
#[repr(C, align(8))]
pub struct FMirrorOptions {
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub b_mirror_rotation: bool,
    pub left_string: FString,
    pub right_string: FString,
    pub b_mirror_children: bool,
    __padding_end: [u8; 7],
}
impl FMirrorOptions {}
#[repr(C, align(8))]
pub struct FOrientOptions {
    pub primary: EOrientAxis,
    pub secondary: EOrientAxis,
    pub b_use_plane_as_secondary: bool,
    pub secondary_target: crate::bindings::core_u_object::FVector,
    pub b_orient_children: bool,
    __padding_end: [u8; 7],
}
impl FOrientOptions {}
#[repr(C, align(8))]
pub struct USkeletonModifier {
    __padding_end: [u8; 112],
}
impl USkeletonModifier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletonModifier")
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
pub struct USkeletalMeshMergeOptions {
    __padding_end: [u8; 56],
}
impl USkeletalMeshMergeOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshMergeOptions")
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
pub struct USkinWeightModifier {
    __padding_end: [u8; 80],
}
impl USkinWeightModifier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkinWeightModifier")
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
pub struct EOrientAxis(pub u8);
impl EOrientAxis {
    pub const NONE: EOrientAxis = EOrientAxis(0);
    pub const POSITIVE_X: EOrientAxis = EOrientAxis(1);
    pub const POSITIVE_Y: EOrientAxis = EOrientAxis(2);
    pub const POSITIVE_Z: EOrientAxis = EOrientAxis(3);
    pub const NEGATIVE_X: EOrientAxis = EOrientAxis(4);
    pub const NEGATIVE_Y: EOrientAxis = EOrientAxis(5);
    pub const NEGATIVE_Z: EOrientAxis = EOrientAxis(6);
}
#[repr(transparent)]
pub struct ESKeletalMeshMergeType(pub u8);
impl ESKeletalMeshMergeType {
    pub const NEW: ESKeletalMeshMergeType = ESKeletalMeshMergeType(0);
    pub const MERGE: ESKeletalMeshMergeType = ESKeletalMeshMergeType(1);
}
