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
    pub u_zone_shape_component_set_tags: *mut crate::ffi::UFunctionOpague,
    pub u_zone_shape_component_set_shape_type: *mut crate::ffi::UFunctionOpague,
    pub u_zone_shape_component_set_reverse_lane_profile: *mut crate::ffi::UFunctionOpague,
    pub u_zone_shape_component_set_polygon_routing_type: *mut crate::ffi::UFunctionOpague,
    pub u_zone_shape_component_is_lane_profile_reversed: *mut crate::ffi::UFunctionOpague,
    pub u_zone_shape_component_get_tags: *mut crate::ffi::UFunctionOpague,
    pub u_zone_shape_component_get_shape_type: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_zone_shape_component_set_tags: std::ptr::null_mut(),
            u_zone_shape_component_set_shape_type: std::ptr::null_mut(),
            u_zone_shape_component_set_reverse_lane_profile: std::ptr::null_mut(),
            u_zone_shape_component_set_polygon_routing_type: std::ptr::null_mut(),
            u_zone_shape_component_is_lane_profile_reversed: std::ptr::null_mut(),
            u_zone_shape_component_get_tags: std::ptr::null_mut(),
            u_zone_shape_component_get_shape_type: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UZoneShapeComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTags"),
                &raw mut __FUNCTION_PTRS.u_zone_shape_component_set_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetShapeType"),
                &raw mut __FUNCTION_PTRS.u_zone_shape_component_set_shape_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetReverseLaneProfile"),
                &raw mut __FUNCTION_PTRS.u_zone_shape_component_set_reverse_lane_profile,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPolygonRoutingType"),
                &raw mut __FUNCTION_PTRS.u_zone_shape_component_set_polygon_routing_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLaneProfileReversed"),
                &raw mut __FUNCTION_PTRS.u_zone_shape_component_is_lane_profile_reversed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTags"),
                &raw mut __FUNCTION_PTRS.u_zone_shape_component_get_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetShapeType"),
                &raw mut __FUNCTION_PTRS.u_zone_shape_component_get_shape_type,
            );
        }
    }
}
#[repr(C, align(1))]
pub struct FZoneGraphTag {
    pub bit: u8,
}
impl FZoneGraphTag {}
#[repr(C, align(4))]
pub struct FZoneGraphTagMask {
    pub(crate) __padding_end: [u8; 4],
}
impl FZoneGraphTagMask {}
#[repr(C, align(4))]
pub struct FZoneLaneDesc {
    pub width: f32,
    pub direction: EZoneLaneDirection,
    pub tags: FZoneGraphTagMask,
}
impl FZoneLaneDesc {}
#[repr(C, align(4))]
pub struct FZoneLaneProfileRef {
    pub name: FName,
    pub id: crate::bindings::core_u_object::FGuid,
}
impl FZoneLaneProfileRef {}
#[repr(C, align(8))]
pub struct FZoneShapePoint {
    pub position: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 48],
    pub tangent_length: f32,
    pub inner_turn_radius: f32,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub ty: FZoneShapePointType,
    pub lane_profile: u8,
    pub b_reverse_lane_profile: bool,
    pub lane_connection_restrictions: i32,
}
impl FZoneShapePoint {}
#[repr(C, align(8))]
pub struct AZoneGraphData {
    __padding_end: [u8; 1440],
}
impl AZoneGraphData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AZoneGraphData")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AZoneGraphData")
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
#[repr(C, align(16))]
pub struct UZoneGraphRenderingComponent {
    __padding_end: [u8; 1536],
}
impl UZoneGraphRenderingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZoneGraphRenderingComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZoneGraphRenderingComponent")
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
pub struct UZoneGraphSettings {
    __padding_end: [u8; 872],
}
impl UZoneGraphSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZoneGraphSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZoneGraphSettings")
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
pub struct UZoneGraphSubsystem {
    __padding_end: [u8; 608],
}
impl UZoneGraphSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZoneGraphSubsystem")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZoneGraphSubsystem")
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
pub struct AZoneShape {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub shape_component: UPtr<UZoneShapeComponent>,
}
impl AZoneShape {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AZoneShape")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AZoneShape")
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
#[repr(C, align(16))]
pub struct UZoneShapeComponent {
    #[doc(hidden)]
    pub(crate) __padding_1504: [u8; 1504],
    pub lane_profile: FZoneLaneProfileRef,
    pub b_reverse_lane_profile: bool,
    #[doc(hidden)]
    pub(crate) __padding_1552: [u8; 16],
    pub points: TArray<FZoneShapePoint>,
    pub shape_type: FZoneShapeType,
    pub polygon_routing_type: EZoneShapePolygonRoutingType,
    pub tags: FZoneGraphTagMask,
    __padding_end: [u8; 72],
}
impl UZoneShapeComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZoneShapeComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UZoneShapeComponent")
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
    pub fn set_tags(&mut self, new_tags: FZoneGraphTagMask) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_set_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_tags,
                __buffer.add(0).cast::<FZoneGraphTagMask>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_set_tags,
                __buffer,
            )
        };
    }
    pub fn set_shape_type(&mut self, ty: FZoneShapeType) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_set_shape_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ty,
                __buffer.add(0).cast::<FZoneShapeType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_set_shape_type,
                __buffer,
            )
        };
    }
    pub fn set_reverse_lane_profile(&mut self, b_reverse: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_set_reverse_lane_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_reverse, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_set_reverse_lane_profile,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_polygon_routing_type(&mut self, new_type: EZoneShapePolygonRoutingType) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_set_polygon_routing_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_type,
                __buffer.add(0).cast::<EZoneShapePolygonRoutingType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_set_polygon_routing_type,
                __buffer,
            )
        };
    }
    pub fn is_lane_profile_reversed(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_is_lane_profile_reversed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_is_lane_profile_reversed,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_tags(&self) -> FZoneGraphTagMask {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_get_tags,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_get_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FZoneGraphTagMask>().read() }
    }
    pub fn get_shape_type(&self) -> FZoneShapeType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_get_shape_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::zone_graph::__FUNCTION_PTRS
                    .u_zone_shape_component_get_shape_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FZoneShapeType>().read() }
    }
}
#[repr(transparent)]
pub struct EZoneLaneDirection(pub u8);
impl EZoneLaneDirection {
    pub const NONE: EZoneLaneDirection = EZoneLaneDirection(0);
    pub const FORWARD: EZoneLaneDirection = EZoneLaneDirection(1);
    pub const BACKWARD: EZoneLaneDirection = EZoneLaneDirection(2);
}
#[repr(transparent)]
pub struct EZoneLaneLinkType(pub u8);
impl EZoneLaneLinkType {
    pub const NONE: EZoneLaneLinkType = EZoneLaneLinkType(0);
    pub const ALL: EZoneLaneLinkType = EZoneLaneLinkType(255);
    pub const OUTGOING: EZoneLaneLinkType = EZoneLaneLinkType(1);
    pub const INCOMING: EZoneLaneLinkType = EZoneLaneLinkType(2);
    pub const ADJACENT: EZoneLaneLinkType = EZoneLaneLinkType(4);
}
#[repr(transparent)]
pub struct FZoneShapePointType(pub u8);
impl FZoneShapePointType {
    pub const SHARP: FZoneShapePointType = FZoneShapePointType(0);
    pub const BEZIER: FZoneShapePointType = FZoneShapePointType(1);
    pub const AUTO_BEZIER: FZoneShapePointType = FZoneShapePointType(2);
    pub const LANE_PROFILE: FZoneShapePointType = FZoneShapePointType(3);
}
#[repr(transparent)]
pub struct FZoneShapeType(pub u8);
impl FZoneShapeType {
    pub const SPLINE: FZoneShapeType = FZoneShapeType(0);
    pub const POLYGON: FZoneShapeType = FZoneShapeType(1);
}
#[repr(transparent)]
pub struct EZoneGraphLaneRoutingCountRule(pub u8);
impl EZoneGraphLaneRoutingCountRule {
    pub const ANY: EZoneGraphLaneRoutingCountRule = EZoneGraphLaneRoutingCountRule(0);
    pub const ONE: EZoneGraphLaneRoutingCountRule = EZoneGraphLaneRoutingCountRule(1);
    pub const MANY: EZoneGraphLaneRoutingCountRule = EZoneGraphLaneRoutingCountRule(2);
}
#[repr(transparent)]
pub struct EZoneShapePolygonRoutingType(pub u8);
impl EZoneShapePolygonRoutingType {
    pub const BEZIER: EZoneShapePolygonRoutingType = EZoneShapePolygonRoutingType(0);
    pub const ARCS: EZoneShapePolygonRoutingType = EZoneShapePolygonRoutingType(1);
}
