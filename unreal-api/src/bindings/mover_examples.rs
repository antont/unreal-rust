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
    pub u_mover_example_ability_inputs_library_get_mover_example_ability_inputs: *mut crate::ffi::UFunctionOpague,
    pub a_mover_examples_character_request_move_by_velocity: *mut crate::ffi::UFunctionOpague,
    pub a_mover_examples_character_request_move_by_intent: *mut crate::ffi::UFunctionOpague,
    pub a_mover_examples_character_on_produce_input_in_blueprint: *mut crate::ffi::UFunctionOpague,
    pub a_mover_examples_character_get_mover_component: *mut crate::ffi::UFunctionOpague,
    pub u_zipline_get_start_component: *mut crate::ffi::UFunctionOpague,
    pub u_zipline_get_end_component: *mut crate::ffi::UFunctionOpague,
    pub u_follow_spline_mode_set_control_spline: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_mover_example_ability_inputs_library_get_mover_example_ability_inputs: std::ptr::null_mut(),
            a_mover_examples_character_request_move_by_velocity: std::ptr::null_mut(),
            a_mover_examples_character_request_move_by_intent: std::ptr::null_mut(),
            a_mover_examples_character_on_produce_input_in_blueprint: std::ptr::null_mut(),
            a_mover_examples_character_get_mover_component: std::ptr::null_mut(),
            u_zipline_get_start_component: std::ptr::null_mut(),
            u_zipline_get_end_component: std::ptr::null_mut(),
            u_follow_spline_mode_set_control_spline: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMoverExampleAbilityInputsLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMoverExampleAbilityInputs"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_example_ability_inputs_library_get_mover_example_ability_inputs,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AMoverExamplesCharacter::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestMoveByVelocity"),
                &raw mut __FUNCTION_PTRS
                    .a_mover_examples_character_request_move_by_velocity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestMoveByIntent"),
                &raw mut __FUNCTION_PTRS
                    .a_mover_examples_character_request_move_by_intent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnProduceInputInBlueprint"),
                &raw mut __FUNCTION_PTRS
                    .a_mover_examples_character_on_produce_input_in_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMoverComponent"),
                &raw mut __FUNCTION_PTRS.a_mover_examples_character_get_mover_component,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UZipline::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStartComponent"),
                &raw mut __FUNCTION_PTRS.u_zipline_get_start_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEndComponent"),
                &raw mut __FUNCTION_PTRS.u_zipline_get_end_component,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UFollowSplineMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlSpline"),
                &raw mut __FUNCTION_PTRS.u_follow_spline_mode_set_control_spline,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FMoverExampleAbilityInputs {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub b_is_dash_just_pressed: bool,
    pub b_is_aim_pressed: bool,
    pub b_is_vault_just_pressed: bool,
    pub b_wants_to_start_ziplining: bool,
    pub b_wants_to_be_crouched: bool,
}
impl FMoverExampleAbilityInputs {}
#[repr(C, align(8))]
pub struct FZipliningState {
    pub(crate) __padding_end: [u8; 24],
}
impl FZipliningState {}
#[repr(C, align(8))]
pub struct FFollowPathState {
    pub(crate) __padding_end: [u8; 40],
}
impl FFollowPathState {}
#[repr(C, align(4))]
pub struct FSplineOffsetRangeInput {
    pub value: f32,
    pub offset_unit: ESplineOffsetUnit,
}
impl FSplineOffsetRangeInput {}
#[repr(C, align(8))]
pub struct FFollowSplineState {
    pub(crate) __padding_end: [u8; 16],
}
impl FFollowSplineState {}
#[repr(C, align(8))]
pub struct UMoverExampleAbilityInputsLibrary {
    __padding_end: [u8; 48],
}
impl UMoverExampleAbilityInputsLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverExampleAbilityInputsLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverExampleAbilityInputsLibrary")
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
    pub fn get_mover_example_ability_inputs(
        from_collection: &crate::bindings::mover::FMoverDataCollection,
    ) -> FMoverExampleAbilityInputs {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .u_mover_example_ability_inputs_library_get_mover_example_ability_inputs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                from_collection,
                __buffer.add(0).cast::<crate::bindings::mover::FMoverDataCollection>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover_examples::UMoverExampleAbilityInputsLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .u_mover_example_ability_inputs_library_get_mover_example_ability_inputs,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FMoverExampleAbilityInputs>().read() }
    }
}
#[repr(C, align(8))]
pub struct AMoverExamplesCharacter {
    #[doc(hidden)]
    pub(crate) __padding_1272: [u8; 1272],
    pub move_input_action: UPtr<crate::bindings::enhanced_input::UInputAction>,
    pub look_input_action: UPtr<crate::bindings::enhanced_input::UInputAction>,
    pub jump_input_action: UPtr<crate::bindings::enhanced_input::UInputAction>,
    pub fly_input_action: UPtr<crate::bindings::enhanced_input::UInputAction>,
    pub b_use_base_relative_movement: bool,
    pub b_orient_rotation_to_movement: bool,
    pub b_maintain_last_input_orientation: bool,
    pub character_motion_component: UPtr<
        crate::bindings::mover::UCharacterMoverComponent,
    >,
    pub nav_mover_component: UPtr<crate::bindings::mover::UNavMoverComponent>,
    __padding_end: [u8; 128],
}
impl AMoverExamplesCharacter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AMoverExamplesCharacter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AMoverExamplesCharacter")
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
    pub fn request_move_by_velocity(
        &mut self,
        desired_velocity: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .a_mover_examples_character_request_move_by_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                desired_velocity,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .a_mover_examples_character_request_move_by_velocity,
                __buffer,
            )
        };
    }
    pub fn request_move_by_intent(
        &mut self,
        desired_intent: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .a_mover_examples_character_request_move_by_intent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                desired_intent,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .a_mover_examples_character_request_move_by_intent,
                __buffer,
            )
        };
    }
    pub fn on_produce_input(
        &mut self,
        delta_ms: f32,
        input_cmd: crate::bindings::mover::FMoverInputCmdContext,
    ) -> crate::bindings::mover::FMoverInputCmdContext {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .a_mover_examples_character_on_produce_input_in_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&delta_ms, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_cmd,
                __buffer.add(8).cast::<crate::bindings::mover::FMoverInputCmdContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .a_mover_examples_character_on_produce_input_in_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::mover::FMoverInputCmdContext>()
                .read()
        }
    }
    pub fn get_mover_component(
        &self,
    ) -> UPtr<crate::bindings::mover::UCharacterMoverComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .a_mover_examples_character_get_mover_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .a_mover_examples_character_get_mover_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::mover::UCharacterMoverComponent>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct AMoverExamplesGameMode {
    __padding_end: [u8; 1376],
}
impl AMoverExamplesGameMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AMoverExamplesGameMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AMoverExamplesGameMode")
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
pub struct AMoverExamplesGameState {
    __padding_end: [u8; 1272],
}
impl AMoverExamplesGameState {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AMoverExamplesGameState")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AMoverExamplesGameState")
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
#[repr(C, align(16))]
pub struct UMoverExamplesPhysicsCharacterMoverComponent {
    __padding_end: [u8; 1936],
}
impl UMoverExamplesPhysicsCharacterMoverComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverExamplesPhysicsCharacterMoverComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverExamplesPhysicsCharacterMoverComponent")
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
pub struct IZipline {}
#[repr(C, align(8))]
pub struct UZipline {
    __padding_end: [u8; 48],
}
impl UZipline {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZipline")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UZipline").copied()
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
    pub fn get_start_component(
        &mut self,
    ) -> UPtr<crate::bindings::engine::USceneComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .u_zipline_get_start_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .u_zipline_get_start_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::USceneComponent>>()
                .read()
        }
    }
    pub fn get_end_component(
        &mut self,
    ) -> UPtr<crate::bindings::engine::USceneComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .u_zipline_get_end_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .u_zipline_get_end_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::USceneComponent>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UZiplineStartTransition {
    __padding_end: [u8; 88],
}
impl UZiplineStartTransition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZiplineStartTransition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZiplineStartTransition")
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
pub struct UZiplineEndTransition {
    __padding_end: [u8; 88],
}
impl UZiplineEndTransition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZiplineEndTransition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZiplineEndTransition")
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
pub struct UZipliningMode {
    __padding_end: [u8; 128],
}
impl UZipliningMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZipliningMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZipliningMode")
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
pub struct UFollowPathMode {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub control_points: TArray<crate::bindings::engine::FInterpControlPoint>,
    pub behaviour_type: crate::bindings::engine::EInterpToBehaviourType,
    pub rotation_type: EFollowPathRotationType,
    pub duration: f32,
    __padding_end: [u8; 24],
}
impl UFollowPathMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFollowPathMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFollowPathMode")
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
pub struct UFollowSplineMode {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub behaviour_type: crate::bindings::engine::EInterpToBehaviourType,
    pub rotation_type: EFollowSplineRotationType,
    pub b_orient_mover_to_movement: bool,
    pub b_constant_follow_velocity: bool,
    pub start_reveresed: bool,
    pub start_offset: FSplineOffsetRangeInput,
    pub end_offset: FSplineOffsetRangeInput,
    pub custom_duration_seconds_override: f32,
    pub interpolation_curve: UPtr<crate::bindings::engine::UCurveFloat>,
    pub control_spline: UPtr<crate::bindings::engine::USplineComponent>,
    __padding_end: [u8; 32],
}
impl UFollowSplineMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFollowSplineMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFollowSplineMode")
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
    pub fn set_control_spline(
        &mut self,
        spline_provider_actor: UPtr<crate::bindings::engine::AActor>,
        offset: FSplineOffsetRangeInput,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .u_follow_spline_mode_set_control_spline,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spline_provider_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset,
                __buffer.add(8).cast::<FSplineOffsetRangeInput>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover_examples::__FUNCTION_PTRS
                    .u_follow_spline_mode_set_control_spline,
                __buffer,
            )
        };
    }
}
#[repr(transparent)]
pub struct ESplineOffsetUnit(pub u8);
impl ESplineOffsetUnit {
    pub const PERCENTAGE: ESplineOffsetUnit = ESplineOffsetUnit(0);
    pub const DURATION_ABSOLUTE_SECONDS: ESplineOffsetUnit = ESplineOffsetUnit(1);
    pub const DISTANCE_ABSOLUTE: ESplineOffsetUnit = ESplineOffsetUnit(2);
}
#[repr(transparent)]
pub struct EFollowPathRotationType(pub u8);
impl EFollowPathRotationType {
    pub const FIXED: EFollowPathRotationType = EFollowPathRotationType(0);
    pub const ALIGN_WITH_PATH_TANGENTS: EFollowPathRotationType = EFollowPathRotationType(
        1,
    );
    pub const ALIGN_WITH_PATH: EFollowPathRotationType = EFollowPathRotationType(2);
}
#[repr(transparent)]
pub struct EFollowSplineRotationType(pub u8);
impl EFollowSplineRotationType {
    pub const FOLLOW_SPLINE_TANGENT: EFollowSplineRotationType = EFollowSplineRotationType(
        0,
    );
    pub const NO_ROTATION: EFollowSplineRotationType = EFollowSplineRotationType(1);
}
