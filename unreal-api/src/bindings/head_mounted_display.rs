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
pub static mut U_HAND_KEYPOINT_CONVERSION_CONV_HAND_KEYPOINT_TO_INT32: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOTION_CONTROLLER_COMPONENT_SET_TRACKING_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOTION_CONTROLLER_COMPONENT_SET_TRACKING_MOTION_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOTION_CONTROLLER_COMPONENT_SET_ASSOCIATED_PLAYER_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOTION_CONTROLLER_COMPONENT_ON_MOTION_CONTROLLER_UPDATED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOTION_CONTROLLER_COMPONENT_IS_TRACKED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOTION_CONTROLLER_COMPONENT_GET_TRACKING_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOTION_CONTROLLER_COMPONENT_GET_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOTION_CONTROLLER_COMPONENT_GET_LINEAR_VELOCITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOTION_CONTROLLER_COMPONENT_GET_LINEAR_ACCELERATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOTION_CONTROLLER_COMPONENT_GET_HAND_JOINT_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOTION_CONTROLLER_COMPONENT_GET_ANGULAR_VELOCITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UHandKeypointConversion::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_HandKeypointToInt32"),
            &raw mut U_HAND_KEYPOINT_CONVERSION_CONV_HAND_KEYPOINT_TO_INT32,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMotionControllerComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTrackingSource"),
            &raw mut U_MOTION_CONTROLLER_COMPONENT_SET_TRACKING_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTrackingMotionSource"),
            &raw mut U_MOTION_CONTROLLER_COMPONENT_SET_TRACKING_MOTION_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAssociatedPlayerIndex"),
            &raw mut U_MOTION_CONTROLLER_COMPONENT_SET_ASSOCIATED_PLAYER_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMotionControllerUpdated"),
            &raw mut U_MOTION_CONTROLLER_COMPONENT_ON_MOTION_CONTROLLER_UPDATED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTracked"),
            &raw mut U_MOTION_CONTROLLER_COMPONENT_IS_TRACKED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTrackingSource"),
            &raw mut U_MOTION_CONTROLLER_COMPONENT_GET_TRACKING_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParameterValue"),
            &raw mut U_MOTION_CONTROLLER_COMPONENT_GET_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinearVelocity"),
            &raw mut U_MOTION_CONTROLLER_COMPONENT_GET_LINEAR_VELOCITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinearAcceleration"),
            &raw mut U_MOTION_CONTROLLER_COMPONENT_GET_LINEAR_ACCELERATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHandJointPosition"),
            &raw mut U_MOTION_CONTROLLER_COMPONENT_GET_HAND_JOINT_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAngularVelocity"),
            &raw mut U_MOTION_CONTROLLER_COMPONENT_GET_ANGULAR_VELOCITY,
        );
    }
}
#[repr(C, align(16))]
pub struct FXRHMDData {
    pub b_valid: bool,
    pub device_name: FName,
    pub application_instance_id: crate::bindings::core_u_object::FGuid,
    pub tracking_status: ETrackingStatus,
    pub position: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FQuat,
}
impl FXRHMDData {}
#[repr(C, align(16))]
pub struct FXRMotionControllerState {
    pub b_valid: bool,
    pub device_name: FName,
    pub application_instance_id: crate::bindings::core_u_object::FGuid,
    pub xr_space_type: EXRSpaceType,
    pub hand: crate::bindings::input_core::EControllerHand,
    pub tracking_status: ETrackingStatus,
    pub xr_controller_pose_type: EXRControllerPoseType,
    pub controller_location: crate::bindings::core_u_object::FVector,
    pub controller_rotation: crate::bindings::core_u_object::FQuat,
    __padding_end: [u8; 64],
}
impl FXRMotionControllerState {}
#[repr(C, align(8))]
pub struct FXRHandTrackingState {
    pub b_valid: bool,
    pub device_name: FName,
    pub application_instance_id: crate::bindings::core_u_object::FGuid,
    pub xr_space_type: EXRSpaceType,
    pub hand: crate::bindings::input_core::EControllerHand,
    pub tracking_status: ETrackingStatus,
    pub hand_key_locations: TArray<crate::bindings::core_u_object::FVector>,
    pub hand_key_rotations: TArray<crate::bindings::core_u_object::FQuat>,
    pub hand_key_radii: TArray<f32>,
}
impl FXRHandTrackingState {}
#[repr(C, align(4))]
pub struct FXRDeviceId {
    pub system_name: FName,
    pub device_id: i32,
}
impl FXRDeviceId {}
#[repr(C, align(8))]
pub struct UHandKeypointConversion {
    __padding_end: [u8; 48],
}
impl UHandKeypointConversion {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHandKeypointConversion")
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
    pub fn conv_hand_keypoint_to_int32(input: EHandKeypoint) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::head_mounted_display::U_HAND_KEYPOINT_CONVERSION_CONV_HAND_KEYPOINT_TO_INT32,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input,
                __buffer.add(0).cast::<EHandKeypoint>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::head_mounted_display::UHandKeypointConversion::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::head_mounted_display::U_HAND_KEYPOINT_CONVERSION_CONV_HAND_KEYPOINT_TO_INT32,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
}
#[repr(C, align(16))]
pub struct UMotionControllerComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub player_index: i32,
    pub motion_source: FName,
    pub flags_1520: u8,
    #[doc(hidden)]
    __padding_1524: [u8; 3],
    pub current_tracking_status: ETrackingStatus,
    __padding_end: [u8; 299],
}
impl UMotionControllerComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionControllerComponent")
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
    pub fn set_tracking_source(
        &mut self,
        new_source: crate::bindings::input_core::EControllerHand,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_SET_TRACKING_SOURCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_source,
                __buffer.add(0).cast::<crate::bindings::input_core::EControllerHand>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_SET_TRACKING_SOURCE,
                __buffer,
            )
        };
    }
    pub fn set_tracking_motion_source(&mut self, new_source: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_SET_TRACKING_MOTION_SOURCE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_source,
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
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_SET_TRACKING_MOTION_SOURCE,
                __buffer,
            )
        };
    }
    pub fn set_associated_player_index(&mut self, new_player: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_SET_ASSOCIATED_PLAYER_INDEX,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&new_player, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_SET_ASSOCIATED_PLAYER_INDEX,
                __buffer,
            )
        };
    }
    pub fn is_tracked(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_IS_TRACKED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_IS_TRACKED,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_tracking_source(&self) -> crate::bindings::input_core::EControllerHand {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_GET_TRACKING_SOURCE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_GET_TRACKING_SOURCE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::input_core::EControllerHand>().read()
        }
    }
    pub fn get_parameter_value(
        &mut self,
        in_name: FName,
        b_value_found: &mut bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_GET_PARAMETER_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_value_found,
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
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_GET_PARAMETER_VALUE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(b_value_found);
        }
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_linear_velocity(
        &self,
        out_linear_velocity: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_GET_LINEAR_VELOCITY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_linear_velocity,
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
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_GET_LINEAR_VELOCITY,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_linear_velocity);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_linear_acceleration(
        &self,
        out_linear_acceleration: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_GET_LINEAR_ACCELERATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_linear_acceleration,
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
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_GET_LINEAR_ACCELERATION,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_linear_acceleration);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_hand_joint_position(
        &mut self,
        joint_index: i32,
        b_value_found: &mut bool,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_GET_HAND_JOINT_POSITION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &joint_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_value_found,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_GET_HAND_JOINT_POSITION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<bool>().swap(b_value_found);
        }
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_angular_velocity(
        &self,
        out_angular_velocity: &mut crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_GET_ANGULAR_VELOCITY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_angular_velocity,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::head_mounted_display::U_MOTION_CONTROLLER_COMPONENT_GET_ANGULAR_VELOCITY,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FRotator>()
                .swap(out_angular_velocity);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
}
#[repr(transparent)]
pub struct ETrackingStatus(pub u8);
impl ETrackingStatus {
    pub const NOT_TRACKED: ETrackingStatus = ETrackingStatus(0);
    pub const INERTIAL_ONLY: ETrackingStatus = ETrackingStatus(1);
    pub const TRACKED: ETrackingStatus = ETrackingStatus(2);
}
#[repr(transparent)]
pub struct EXRSpaceType(pub u8);
impl EXRSpaceType {
    pub const UNREAL_WORLD_SPACE: EXRSpaceType = EXRSpaceType(0);
    pub const XR_TRACKING_SPACE: EXRSpaceType = EXRSpaceType(1);
}
#[repr(transparent)]
pub struct EXRControllerPoseType(pub u8);
impl EXRControllerPoseType {
    pub const AIM: EXRControllerPoseType = EXRControllerPoseType(0);
    pub const GRIP: EXRControllerPoseType = EXRControllerPoseType(1);
    pub const PALM: EXRControllerPoseType = EXRControllerPoseType(2);
}
#[repr(transparent)]
pub struct EHandKeypoint(pub u8);
impl EHandKeypoint {
    pub const PALM: EHandKeypoint = EHandKeypoint(0);
    pub const WRIST: EHandKeypoint = EHandKeypoint(1);
    pub const THUMB_METACARPAL: EHandKeypoint = EHandKeypoint(2);
    pub const THUMB_PROXIMAL: EHandKeypoint = EHandKeypoint(3);
    pub const THUMB_DISTAL: EHandKeypoint = EHandKeypoint(4);
    pub const THUMB_TIP: EHandKeypoint = EHandKeypoint(5);
    pub const INDEX_METACARPAL: EHandKeypoint = EHandKeypoint(6);
    pub const INDEX_PROXIMAL: EHandKeypoint = EHandKeypoint(7);
    pub const INDEX_INTERMEDIATE: EHandKeypoint = EHandKeypoint(8);
    pub const INDEX_DISTAL: EHandKeypoint = EHandKeypoint(9);
    pub const INDEX_TIP: EHandKeypoint = EHandKeypoint(10);
    pub const MIDDLE_METACARPAL: EHandKeypoint = EHandKeypoint(11);
    pub const MIDDLE_PROXIMAL: EHandKeypoint = EHandKeypoint(12);
    pub const MIDDLE_INTERMEDIATE: EHandKeypoint = EHandKeypoint(13);
    pub const MIDDLE_DISTAL: EHandKeypoint = EHandKeypoint(14);
    pub const MIDDLE_TIP: EHandKeypoint = EHandKeypoint(15);
    pub const RING_METACARPAL: EHandKeypoint = EHandKeypoint(16);
    pub const RING_PROXIMAL: EHandKeypoint = EHandKeypoint(17);
    pub const RING_INTERMEDIATE: EHandKeypoint = EHandKeypoint(18);
    pub const RING_DISTAL: EHandKeypoint = EHandKeypoint(19);
    pub const RING_TIP: EHandKeypoint = EHandKeypoint(20);
    pub const LITTLE_METACARPAL: EHandKeypoint = EHandKeypoint(21);
    pub const LITTLE_PROXIMAL: EHandKeypoint = EHandKeypoint(22);
    pub const LITTLE_INTERMEDIATE: EHandKeypoint = EHandKeypoint(23);
    pub const LITTLE_DISTAL: EHandKeypoint = EHandKeypoint(24);
    pub const LITTLE_TIP: EHandKeypoint = EHandKeypoint(25);
}
