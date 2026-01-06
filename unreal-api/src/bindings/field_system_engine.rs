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
pub static mut U_FIELD_SYSTEM_COMPONENT_RESET_FIELD_SYSTEM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_COMPONENT_REMOVE_PERSISTENT_FIELDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_COMPONENT_APPLY_UNIFORM_VECTOR_FALLOFF_FORCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_COMPONENT_APPLY_STRAIN_FIELD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_COMPONENT_APPLY_STAY_DYNAMIC_FIELD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_COMPONENT_APPLY_RADIAL_VECTOR_FALLOFF_FORCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_COMPONENT_APPLY_RADIAL_FORCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_COMPONENT_APPLY_PHYSICS_FIELD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_COMPONENT_APPLY_LINEAR_FORCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_COMPONENT_ADD_PERSISTENT_FIELD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_COMPONENT_ADD_FIELD_COMMAND: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_META_DATA_ITERATION_SET_META_DATA_ITERATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_META_DATA_PROCESSING_RESOLUTION_SET_META_DATAA_PROCESSING_RESOLUTION_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FIELD_SYSTEM_META_DATA_FILTER_SET_META_DATA_FILTER_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_UNIFORM_INTEGER_SET_UNIFORM_INTEGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_INT_MASK_SET_RADIAL_INT_MASK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_UNIFORM_SCALAR_SET_UNIFORM_SCALAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WAVE_SCALAR_SET_WAVE_SCALAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_FALLOFF_SET_RADIAL_FALLOFF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PLANE_FALLOFF_SET_PLANE_FALLOFF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BOX_FALLOFF_SET_BOX_FALLOFF: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_NOISE_FIELD_SET_NOISE_FIELD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_UNIFORM_VECTOR_SET_UNIFORM_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RADIAL_VECTOR_SET_RADIAL_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RANDOM_VECTOR_SET_RANDOM_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_OPERATOR_FIELD_SET_OPERATOR_FIELD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TO_INTEGER_FIELD_SET_TO_INTEGER_FIELD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_TO_FLOAT_FIELD_SET_TO_FLOAT_FIELD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CULLING_FIELD_SET_CULLING_FIELD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RETURN_RESULTS_TERMINAL_SET_RETURN_RESULTS_TERMINAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFieldSystemComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetFieldSystem"),
            &raw mut U_FIELD_SYSTEM_COMPONENT_RESET_FIELD_SYSTEM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemovePersistentFields"),
            &raw mut U_FIELD_SYSTEM_COMPONENT_REMOVE_PERSISTENT_FIELDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyUniformVectorFalloffForce"),
            &raw mut U_FIELD_SYSTEM_COMPONENT_APPLY_UNIFORM_VECTOR_FALLOFF_FORCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyStrainField"),
            &raw mut U_FIELD_SYSTEM_COMPONENT_APPLY_STRAIN_FIELD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyStayDynamicField"),
            &raw mut U_FIELD_SYSTEM_COMPONENT_APPLY_STAY_DYNAMIC_FIELD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyRadialVectorFalloffForce"),
            &raw mut U_FIELD_SYSTEM_COMPONENT_APPLY_RADIAL_VECTOR_FALLOFF_FORCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyRadialForce"),
            &raw mut U_FIELD_SYSTEM_COMPONENT_APPLY_RADIAL_FORCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyPhysicsField"),
            &raw mut U_FIELD_SYSTEM_COMPONENT_APPLY_PHYSICS_FIELD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyLinearForce"),
            &raw mut U_FIELD_SYSTEM_COMPONENT_APPLY_LINEAR_FORCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddPersistentField"),
            &raw mut U_FIELD_SYSTEM_COMPONENT_ADD_PERSISTENT_FIELD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFieldCommand"),
            &raw mut U_FIELD_SYSTEM_COMPONENT_ADD_FIELD_COMMAND,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFieldSystemMetaDataIteration::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMetaDataIteration"),
            &raw mut U_FIELD_SYSTEM_META_DATA_ITERATION_SET_META_DATA_ITERATION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFieldSystemMetaDataProcessingResolution::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMetaDataaProcessingResolutionType"),
            &raw mut U_FIELD_SYSTEM_META_DATA_PROCESSING_RESOLUTION_SET_META_DATAA_PROCESSING_RESOLUTION_TYPE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFieldSystemMetaDataFilter::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMetaDataFilterType"),
            &raw mut U_FIELD_SYSTEM_META_DATA_FILTER_SET_META_DATA_FILTER_TYPE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUniformInteger::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUniformInteger"),
            &raw mut U_UNIFORM_INTEGER_SET_UNIFORM_INTEGER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URadialIntMask::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRadialIntMask"),
            &raw mut U_RADIAL_INT_MASK_SET_RADIAL_INT_MASK,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUniformScalar::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUniformScalar"),
            &raw mut U_UNIFORM_SCALAR_SET_UNIFORM_SCALAR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWaveScalar::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWaveScalar"),
            &raw mut U_WAVE_SCALAR_SET_WAVE_SCALAR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URadialFalloff::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRadialFalloff"),
            &raw mut U_RADIAL_FALLOFF_SET_RADIAL_FALLOFF,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPlaneFalloff::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaneFalloff"),
            &raw mut U_PLANE_FALLOFF_SET_PLANE_FALLOFF,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBoxFalloff::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoxFalloff"),
            &raw mut U_BOX_FALLOFF_SET_BOX_FALLOFF,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNoiseField::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNoiseField"),
            &raw mut U_NOISE_FIELD_SET_NOISE_FIELD,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUniformVector::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUniformVector"),
            &raw mut U_UNIFORM_VECTOR_SET_UNIFORM_VECTOR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URadialVector::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRadialVector"),
            &raw mut U_RADIAL_VECTOR_SET_RADIAL_VECTOR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URandomVector::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRandomVector"),
            &raw mut U_RANDOM_VECTOR_SET_RANDOM_VECTOR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UOperatorField::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOperatorField"),
            &raw mut U_OPERATOR_FIELD_SET_OPERATOR_FIELD,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UToIntegerField::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetToIntegerField"),
            &raw mut U_TO_INTEGER_FIELD_SET_TO_INTEGER_FIELD,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UToFloatField::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetToFloatField"),
            &raw mut U_TO_FLOAT_FIELD_SET_TO_FLOAT_FIELD,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCullingField::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCullingField"),
            &raw mut U_CULLING_FIELD_SET_CULLING_FIELD,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UReturnResultsTerminal::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReturnResultsTerminal"),
            &raw mut U_RETURN_RESULTS_TERMINAL_SET_RETURN_RESULTS_TERMINAL,
        );
    }
}
#[repr(C, align(8))]
pub struct FFieldObjectCommands {
    pub target_names: TArray<FName>,
    pub root_nodes: TArray<UPtr<UFieldNodeBase>>,
    pub meta_datas: TArray<UPtr<UFieldSystemMetaData>>,
}
impl FFieldObjectCommands {}
#[repr(C, align(8))]
pub struct AFieldSystemActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub field_system_component: UPtr<UFieldSystemComponent>,
}
impl AFieldSystemActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AFieldSystemActor")
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
pub struct UFieldSystem {
    __padding_end: [u8; 64],
}
impl UFieldSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldSystem")
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
#[repr(C, align(16))]
pub struct UFieldSystemComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub field_system: UPtr<UFieldSystem>,
    __padding_end: [u8; 200],
}
impl UFieldSystemComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldSystemComponent")
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
pub struct UFieldSystemMetaData {
    __padding_end: [u8; 240],
}
impl UFieldSystemMetaData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldSystemMetaData")
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
pub struct UFieldSystemMetaDataIteration {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub iterations: i32,
    __padding_end: [u8; 4],
}
impl UFieldSystemMetaDataIteration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldSystemMetaDataIteration")
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
pub struct UFieldSystemMetaDataProcessingResolution {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub resolution_type: crate::bindings::chaos::EFieldResolutionType,
    __padding_end: [u8; 7],
}
impl UFieldSystemMetaDataProcessingResolution {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldSystemMetaDataProcessingResolution")
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
pub struct UFieldSystemMetaDataFilter {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub filter_type: crate::bindings::chaos::EFieldFilterType,
    pub object_type: crate::bindings::chaos::EFieldObjectType,
    pub position_type: crate::bindings::chaos::EFieldPositionType,
    __padding_end: [u8; 5],
}
impl UFieldSystemMetaDataFilter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldSystemMetaDataFilter")
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
pub struct UFieldNodeBase {
    __padding_end: [u8; 240],
}
impl UFieldNodeBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldNodeBase")
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
pub struct UFieldNodeInt {
    __padding_end: [u8; 240],
}
impl UFieldNodeInt {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldNodeInt")
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
pub struct UFieldNodeFloat {
    __padding_end: [u8; 240],
}
impl UFieldNodeFloat {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldNodeFloat")
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
pub struct UFieldNodeVector {
    __padding_end: [u8; 240],
}
impl UFieldNodeVector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldNodeVector")
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
pub struct UUniformInteger {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: i32,
    __padding_end: [u8; 4],
}
impl UUniformInteger {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformInteger")
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
pub struct URadialIntMask {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub radius: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub interior_value: i32,
    pub exterior_value: i32,
    pub set_mask_condition: crate::bindings::chaos::ESetMaskConditionType,
    __padding_end: [u8; 7],
}
impl URadialIntMask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URadialIntMask")
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
pub struct UUniformScalar {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    __padding_end: [u8; 4],
}
impl UUniformScalar {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformScalar")
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
pub struct UWaveScalar {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub wavelength: f32,
    pub period: f32,
    pub function: crate::bindings::chaos::EWaveFunctionType,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
    __padding_end: [u8; 6],
}
impl UWaveScalar {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaveScalar")
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
pub struct URadialFalloff {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub radius: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
    __padding_end: [u8; 7],
}
impl URadialFalloff {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URadialFalloff")
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
pub struct UPlaneFalloff {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub distance: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
    __padding_end: [u8; 7],
}
impl UPlaneFalloff {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneFalloff")
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
#[repr(C, align(16))]
pub struct UBoxFalloff {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
    __padding_end: [u8; 15],
}
impl UBoxFalloff {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBoxFalloff")
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
#[repr(C, align(16))]
pub struct UNoiseField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub min_range: f32,
    pub max_range: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl UNoiseField {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNoiseField")
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
pub struct UUniformVector {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub direction: crate::bindings::core_u_object::FVector,
}
impl UUniformVector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformVector")
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
pub struct URadialVector {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub position: crate::bindings::core_u_object::FVector,
}
impl URadialVector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URadialVector")
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
pub struct URandomVector {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    __padding_end: [u8; 4],
}
impl URandomVector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URandomVector")
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
pub struct UOperatorField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub right_field: UPtr<UFieldNodeBase>,
    pub left_field: UPtr<UFieldNodeBase>,
    pub operation: crate::bindings::chaos::EFieldOperationType,
    __padding_end: [u8; 7],
}
impl UOperatorField {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOperatorField")
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
pub struct UToIntegerField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub float_field: UPtr<UFieldNodeFloat>,
}
impl UToIntegerField {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToIntegerField")
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
pub struct UToFloatField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub int_field: UPtr<UFieldNodeInt>,
}
impl UToFloatField {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToFloatField")
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
pub struct UCullingField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub culling: UPtr<UFieldNodeBase>,
    pub field: UPtr<UFieldNodeBase>,
    pub operation: crate::bindings::chaos::EFieldCullingOperationType,
    __padding_end: [u8; 7],
}
impl UCullingField {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCullingField")
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
pub struct UReturnResultsTerminal {
    __padding_end: [u8; 240],
}
impl UReturnResultsTerminal {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReturnResultsTerminal")
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
