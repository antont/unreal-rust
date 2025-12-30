#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(16))]
pub struct FXRHMDData {
    pub b_valid: bool,
    pub device_name: FName,
    pub application_instance_id: FGuid,
    pub tracking_status: ETrackingStatus,
    pub position: FVector,
    pub rotation: FQuat,
}
#[repr(C, align(16))]
pub struct FXRMotionControllerState {
    pub b_valid: bool,
    pub device_name: FName,
    pub application_instance_id: FGuid,
    pub xr_space_type: EXRSpaceType,
    pub hand: EControllerHand,
    pub tracking_status: ETrackingStatus,
    pub xr_controller_pose_type: EXRControllerPoseType,
    pub controller_location: FVector,
    pub controller_rotation: FQuat,
    pub grip_unreal_space_location: FVector,
    pub grip_unreal_space_rotation: FQuat,
}
#[repr(C, align(8))]
pub struct FXRHandTrackingState {
    pub b_valid: bool,
    pub device_name: FName,
    pub application_instance_id: FGuid,
    pub xr_space_type: EXRSpaceType,
    pub hand: EControllerHand,
    pub tracking_status: ETrackingStatus,
    pub hand_key_locations: TArray<FVector>,
    pub hand_key_rotations: TArray<FQuat>,
    pub hand_key_radii: TArray<f32>,
}
#[repr(C, align(4))]
pub struct FXRDeviceId {
    pub system_name: FName,
    pub device_id: i32,
}
pub struct UHandKeypointConversion {}
pub struct UMotionControllerComponent {
    pub player_index: i32,
    pub motion_source: FName,
    pub flags_1520: u8,
    pub current_tracking_status: ETrackingStatus,
}
