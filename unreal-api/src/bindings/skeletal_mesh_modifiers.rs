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
    pub u_skeleton_modifier_set_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_set_bone_transform: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_set_bones_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_rename_bones: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_rename_bone: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_remove_bones: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_remove_bone: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_parent_bones: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_parent_bone: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_orient_bones: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_orient_bone: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_mirror_bones: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_mirror_bone: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_get_parent_name: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_get_children_names: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_get_bone_transform: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_get_all_bone_names: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_commit_skeleton_to_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_add_bones: *mut crate::ffi::UFunctionOpague,
    pub u_skeleton_modifier_add_bone: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weight_modifier_set_vertex_weights: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weight_modifier_set_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weight_modifier_prune_vertex_weights: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weight_modifier_prune_all_weights: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weight_modifier_normalize_vertex_weights: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weight_modifier_normalize_all_weights: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weight_modifier_get_vertex_weights: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weight_modifier_get_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weight_modifier_get_num_vertices: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weight_modifier_get_all_bone_names: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weight_modifier_enforce_max_influences: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weight_modifier_commit_weights_to_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_skeleton_modifier_set_skeletal_mesh: std::ptr::null_mut(),
            u_skeleton_modifier_set_bone_transform: std::ptr::null_mut(),
            u_skeleton_modifier_set_bones_transforms: std::ptr::null_mut(),
            u_skeleton_modifier_rename_bones: std::ptr::null_mut(),
            u_skeleton_modifier_rename_bone: std::ptr::null_mut(),
            u_skeleton_modifier_remove_bones: std::ptr::null_mut(),
            u_skeleton_modifier_remove_bone: std::ptr::null_mut(),
            u_skeleton_modifier_parent_bones: std::ptr::null_mut(),
            u_skeleton_modifier_parent_bone: std::ptr::null_mut(),
            u_skeleton_modifier_orient_bones: std::ptr::null_mut(),
            u_skeleton_modifier_orient_bone: std::ptr::null_mut(),
            u_skeleton_modifier_mirror_bones: std::ptr::null_mut(),
            u_skeleton_modifier_mirror_bone: std::ptr::null_mut(),
            u_skeleton_modifier_get_parent_name: std::ptr::null_mut(),
            u_skeleton_modifier_get_children_names: std::ptr::null_mut(),
            u_skeleton_modifier_get_bone_transform: std::ptr::null_mut(),
            u_skeleton_modifier_get_all_bone_names: std::ptr::null_mut(),
            u_skeleton_modifier_commit_skeleton_to_skeletal_mesh: std::ptr::null_mut(),
            u_skeleton_modifier_add_bones: std::ptr::null_mut(),
            u_skeleton_modifier_add_bone: std::ptr::null_mut(),
            u_skin_weight_modifier_set_vertex_weights: std::ptr::null_mut(),
            u_skin_weight_modifier_set_skeletal_mesh: std::ptr::null_mut(),
            u_skin_weight_modifier_prune_vertex_weights: std::ptr::null_mut(),
            u_skin_weight_modifier_prune_all_weights: std::ptr::null_mut(),
            u_skin_weight_modifier_normalize_vertex_weights: std::ptr::null_mut(),
            u_skin_weight_modifier_normalize_all_weights: std::ptr::null_mut(),
            u_skin_weight_modifier_get_vertex_weights: std::ptr::null_mut(),
            u_skin_weight_modifier_get_skeletal_mesh: std::ptr::null_mut(),
            u_skin_weight_modifier_get_num_vertices: std::ptr::null_mut(),
            u_skin_weight_modifier_get_all_bone_names: std::ptr::null_mut(),
            u_skin_weight_modifier_enforce_max_influences: std::ptr::null_mut(),
            u_skin_weight_modifier_commit_weights_to_skeletal_mesh: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USkeletonModifier::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSkeletalMesh"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_set_skeletal_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBoneTransform"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_set_bone_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBonesTransforms"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_set_bones_transforms,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameBones"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_rename_bones,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameBone"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_rename_bone,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveBones"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_remove_bones,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveBone"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_remove_bone,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ParentBones"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_parent_bones,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ParentBone"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_parent_bone,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OrientBones"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_orient_bones,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OrientBone"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_orient_bone,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MirrorBones"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_mirror_bones,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MirrorBone"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_mirror_bone,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentName"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_get_parent_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetChildrenNames"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_get_children_names,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBoneTransform"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_get_bone_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllBoneNames"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_get_all_bone_names,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CommitSkeletonToSkeletalMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_skeleton_modifier_commit_skeleton_to_skeletal_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddBones"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_add_bones,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddBone"),
                &raw mut __FUNCTION_PTRS.u_skeleton_modifier_add_bone,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USkinWeightModifier::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetVertexWeights"),
                &raw mut __FUNCTION_PTRS.u_skin_weight_modifier_set_vertex_weights,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSkeletalMesh"),
                &raw mut __FUNCTION_PTRS.u_skin_weight_modifier_set_skeletal_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PruneVertexWeights"),
                &raw mut __FUNCTION_PTRS.u_skin_weight_modifier_prune_vertex_weights,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PruneAllWeights"),
                &raw mut __FUNCTION_PTRS.u_skin_weight_modifier_prune_all_weights,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NormalizeVertexWeights"),
                &raw mut __FUNCTION_PTRS.u_skin_weight_modifier_normalize_vertex_weights,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NormalizeAllWeights"),
                &raw mut __FUNCTION_PTRS.u_skin_weight_modifier_normalize_all_weights,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexWeights"),
                &raw mut __FUNCTION_PTRS.u_skin_weight_modifier_get_vertex_weights,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSkeletalMesh"),
                &raw mut __FUNCTION_PTRS.u_skin_weight_modifier_get_skeletal_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumVertices"),
                &raw mut __FUNCTION_PTRS.u_skin_weight_modifier_get_num_vertices,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllBoneNames"),
                &raw mut __FUNCTION_PTRS.u_skin_weight_modifier_get_all_bone_names,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnforceMaxInfluences"),
                &raw mut __FUNCTION_PTRS.u_skin_weight_modifier_enforce_max_influences,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CommitWeightsToSkeletalMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_skin_weight_modifier_commit_weights_to_skeletal_mesh,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FMirrorOptions {
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub b_mirror_rotation: bool,
    pub left_string: FString,
    pub right_string: FString,
    pub b_mirror_children: bool,
}
impl FMirrorOptions {}
#[repr(C, align(8))]
pub struct FOrientOptions {
    pub primary: EOrientAxis,
    pub secondary: EOrientAxis,
    pub b_use_plane_as_secondary: bool,
    pub secondary_target: crate::bindings::core_u_object::FVector,
    pub b_orient_children: bool,
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletonModifier")
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
    pub fn set_skeletal_mesh(
        &mut self,
        in_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_set_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_set_skeletal_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_bone_transform(
        &mut self,
        in_bone_name: FName,
        in_new_transform: &crate::bindings::core_u_object::FTransform,
        b_move_children: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<114>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_set_bone_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_move_children,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_set_bone_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(113).cast::<bool>().read() }
    }
    pub fn set_bones_transforms(
        &mut self,
        in_bone_names: &TArray<FName>,
        in_new_transforms: &TArray<crate::bindings::core_u_object::FTransform>,
        b_move_children: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_set_bones_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_bone_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_transforms,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_move_children,
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
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_set_bones_transforms,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn rename_bones(
        &mut self,
        in_old_bone_names: &TArray<FName>,
        in_new_bone_names: &TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_rename_bones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_bone_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_bone_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_rename_bones,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn rename_bone(
        &mut self,
        in_old_bone_name: FName,
        in_new_bone_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_rename_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_old_bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_bone_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_rename_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn remove_bones(
        &mut self,
        in_bone_names: &TArray<FName>,
        b_remove_children: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_remove_bones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_bone_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_children,
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
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_remove_bones,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn remove_bone(&mut self, in_bone_name: FName, b_remove_children: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_remove_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_children,
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
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_remove_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn parent_bones(
        &mut self,
        in_bone_names: &TArray<FName>,
        in_parent_names: &TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_parent_bones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_bone_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parent_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_parent_bones,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn parent_bone(&mut self, in_bone_name: FName, in_parent_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_parent_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_parent_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn orient_bones(
        &mut self,
        in_bone_names: &TArray<FName>,
        in_options: &FOrientOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_orient_bones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_bone_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_options,
                __buffer.add(16).cast::<FOrientOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_orient_bones,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn orient_bone(
        &mut self,
        in_bone_name: FName,
        in_options: &FOrientOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_orient_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_options,
                __buffer.add(16).cast::<FOrientOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_orient_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn mirror_bones(
        &mut self,
        in_bones_name: &TArray<FName>,
        in_options: &FMirrorOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_mirror_bones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_bones_name,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_options,
                __buffer.add(16).cast::<FMirrorOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_mirror_bones,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn mirror_bone(
        &mut self,
        in_bone_name: FName,
        in_options: &FMirrorOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_mirror_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_options,
                __buffer.add(16).cast::<FMirrorOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_mirror_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn get_parent_name(&self, in_bone_name: FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_get_parent_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
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
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_get_parent_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_children_names(
        &self,
        in_bone_name: FName,
        b_recursive: bool,
    ) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_get_children_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_get_children_names,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<FName>>().read() }
    }
    pub fn get_bone_transform(
        &self,
        in_bone_name: FName,
        b_global: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_get_bone_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_global, __buffer.add(12).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_get_bone_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_all_bone_names(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_get_all_bone_names,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_get_all_bone_names,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn commit_skeleton_to_skeletal_mesh(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_commit_skeleton_to_skeletal_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_commit_skeleton_to_skeletal_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn add_bones(
        &mut self,
        in_bones_name: &TArray<FName>,
        in_parents_name: &TArray<FName>,
        in_transforms: &TArray<crate::bindings::core_u_object::FTransform>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_add_bones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_bones_name,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parents_name,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_transforms,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_add_bones,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn add_bone(
        &mut self,
        in_bone_name: FName,
        in_parent_name: FName,
        in_transform: &crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<129>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_add_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_transform,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skeleton_modifier_add_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(128).cast::<bool>().read() }
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshMergeOptions")
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkinWeightModifier")
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
    pub fn set_vertex_weights(
        &mut self,
        vertex_id: i32,
        in_weights: &TMap<FName, f32>,
        b_replace_all: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<90>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_set_vertex_weights,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&vertex_id, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_weights,
                __buffer.add(8).cast::<TMap<FName, f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_replace_all,
                __buffer.add(88).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_set_vertex_weights,
                __buffer,
            )
        };
        unsafe { __buffer.add(89).cast::<bool>().read() }
    }
    pub fn set_skeletal_mesh(
        &mut self,
        in_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_set_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_set_skeletal_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn prune_vertex_weights(
        &mut self,
        vertex_id: i32,
        weight_threshold: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_prune_vertex_weights,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&vertex_id, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &weight_threshold,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_prune_vertex_weights,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn prune_all_weights(&mut self, weight_threshold: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_prune_all_weights,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &weight_threshold,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_prune_all_weights,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn normalize_vertex_weights(&mut self, vertex_id: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_normalize_vertex_weights,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&vertex_id, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_normalize_vertex_weights,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn normalize_all_weights(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_normalize_all_weights,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_normalize_all_weights,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_vertex_weights(&mut self, vertex_id: i32) -> TMap<FName, f32> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_get_vertex_weights,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&vertex_id, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_get_vertex_weights,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TMap<FName, f32>>().read() }
    }
    pub fn get_skeletal_mesh(&mut self) -> UPtr<crate::bindings::engine::USkeletalMesh> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_get_skeletal_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_get_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>().read()
        }
    }
    pub fn get_num_vertices(&mut self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_get_num_vertices,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_get_num_vertices,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_all_bone_names(&mut self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_get_all_bone_names,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_get_all_bone_names,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn enforce_max_influences(&mut self, max_influences: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_enforce_max_influences,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_influences,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_enforce_max_influences,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn commit_weights_to_skeletal_mesh(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_commit_weights_to_skeletal_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::skeletal_mesh_modifiers::__FUNCTION_PTRS
                    .u_skin_weight_modifier_commit_weights_to_skeletal_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
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
