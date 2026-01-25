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
    pub u_procedural_foliage_editor_library_resimulate_procedural_foliage_volumes: *mut crate::ffi::UFunctionOpague,
    pub u_procedural_foliage_editor_library_resimulate_procedural_foliage_components: *mut crate::ffi::UFunctionOpague,
    pub u_procedural_foliage_editor_library_clear_procedural_foliage_volumes: *mut crate::ffi::UFunctionOpague,
    pub u_procedural_foliage_editor_library_clear_procedural_foliage_components: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_procedural_foliage_editor_library_resimulate_procedural_foliage_volumes: std::ptr::null_mut(),
            u_procedural_foliage_editor_library_resimulate_procedural_foliage_components: std::ptr::null_mut(),
            u_procedural_foliage_editor_library_clear_procedural_foliage_volumes: std::ptr::null_mut(),
            u_procedural_foliage_editor_library_clear_procedural_foliage_components: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UProceduralFoliageEditorLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResimulateProceduralFoliageVolumes"),
            &raw mut __FUNCTION_PTRS
                .u_procedural_foliage_editor_library_resimulate_procedural_foliage_volumes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResimulateProceduralFoliageComponents"),
            &raw mut __FUNCTION_PTRS
                .u_procedural_foliage_editor_library_resimulate_procedural_foliage_components,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearProceduralFoliageVolumes"),
            &raw mut __FUNCTION_PTRS
                .u_procedural_foliage_editor_library_clear_procedural_foliage_volumes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearProceduralFoliageComponents"),
            &raw mut __FUNCTION_PTRS
                .u_procedural_foliage_editor_library_clear_procedural_foliage_components,
        );
    }
}
#[repr(C, align(8))]
pub struct UActorFactoryProceduralFoliage {
    __padding_end: [u8; 144],
}
impl UActorFactoryProceduralFoliage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryProceduralFoliage")
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
pub struct UFoliageEditorSubsystem {
    __padding_end: [u8; 56],
}
impl UFoliageEditorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageEditorSubsystem")
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
pub struct UFoliageType_InstancedStaticMeshFactory {
    __padding_end: [u8; 136],
}
impl UFoliageType_InstancedStaticMeshFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageType_InstancedStaticMeshFactory")
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
pub struct UFoliageType_ActorFactory {
    __padding_end: [u8; 136],
}
impl UFoliageType_ActorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageType_ActorFactory")
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
pub struct UFoliageType_ActorThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl UFoliageType_ActorThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageType_ActorThumbnailRenderer")
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
pub struct UFoliageType_ISMThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UFoliageType_ISMThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageType_ISMThumbnailRenderer")
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
pub struct ULandscapeGrassTypeFactory {
    __padding_end: [u8; 136],
}
impl ULandscapeGrassTypeFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeGrassTypeFactory")
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
pub struct UProceduralFoliageEditorLibrary {
    __padding_end: [u8; 48],
}
impl UProceduralFoliageEditorLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralFoliageEditorLibrary")
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
    pub fn resimulate_procedural_foliage_volumes(
        procedural_foliage_volumes: &TArray<
            UPtr<crate::bindings::foliage::AProceduralFoliageVolume>,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::foliage_edit::__FUNCTION_PTRS
                    .u_procedural_foliage_editor_library_resimulate_procedural_foliage_volumes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                procedural_foliage_volumes,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::foliage::AProceduralFoliageVolume>>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::foliage_edit::UProceduralFoliageEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::foliage_edit::__FUNCTION_PTRS
                    .u_procedural_foliage_editor_library_resimulate_procedural_foliage_volumes,
                __buffer,
            )
        };
    }
    pub fn resimulate_procedural_foliage_components(
        procedural_foliage_components: &TArray<
            UPtr<crate::bindings::foliage::UProceduralFoliageComponent>,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::foliage_edit::__FUNCTION_PTRS
                    .u_procedural_foliage_editor_library_resimulate_procedural_foliage_components,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                procedural_foliage_components,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<
                            UPtr<crate::bindings::foliage::UProceduralFoliageComponent>,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::foliage_edit::UProceduralFoliageEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::foliage_edit::__FUNCTION_PTRS
                    .u_procedural_foliage_editor_library_resimulate_procedural_foliage_components,
                __buffer,
            )
        };
    }
    pub fn clear_procedural_foliage_volumes(
        procedural_foliage_volumes: &TArray<
            UPtr<crate::bindings::foliage::AProceduralFoliageVolume>,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::foliage_edit::__FUNCTION_PTRS
                    .u_procedural_foliage_editor_library_clear_procedural_foliage_volumes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                procedural_foliage_volumes,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::foliage::AProceduralFoliageVolume>>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::foliage_edit::UProceduralFoliageEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::foliage_edit::__FUNCTION_PTRS
                    .u_procedural_foliage_editor_library_clear_procedural_foliage_volumes,
                __buffer,
            )
        };
    }
    pub fn clear_procedural_foliage_components(
        procedural_foliage_components: &TArray<
            UPtr<crate::bindings::foliage::UProceduralFoliageComponent>,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::foliage_edit::__FUNCTION_PTRS
                    .u_procedural_foliage_editor_library_clear_procedural_foliage_components,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                procedural_foliage_components,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<
                            UPtr<crate::bindings::foliage::UProceduralFoliageComponent>,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::foliage_edit::UProceduralFoliageEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::foliage_edit::__FUNCTION_PTRS
                    .u_procedural_foliage_editor_library_clear_procedural_foliage_components,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UProceduralFoliageSpawnerFactory {
    __padding_end: [u8; 136],
}
impl UProceduralFoliageSpawnerFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralFoliageSpawnerFactory")
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
