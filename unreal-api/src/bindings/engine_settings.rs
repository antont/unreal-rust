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
pub static mut U_GAME_MAPS_SETTINGS_SET_SKIP_ASSIGNING_GAMEPAD_TO_PLAYER1: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAME_MAPS_SETTINGS_GET_SKIP_ASSIGNING_GAMEPAD_TO_PLAYER1: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GAME_MAPS_SETTINGS_GET_GAME_MAPS_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGameMapsSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkipAssigningGamepadToPlayer1"),
            &raw mut U_GAME_MAPS_SETTINGS_SET_SKIP_ASSIGNING_GAMEPAD_TO_PLAYER1,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkipAssigningGamepadToPlayer1"),
            &raw mut U_GAME_MAPS_SETTINGS_GET_SKIP_ASSIGNING_GAMEPAD_TO_PLAYER1,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGameMapsSettings"),
            &raw mut U_GAME_MAPS_SETTINGS_GET_GAME_MAPS_SETTINGS,
        );
    }
}
#[repr(C, align(8))]
pub struct UGameMapsSettings {
    __padding_end: [u8; 400],
}
impl UGameMapsSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameMapsSettings")
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
    pub fn set_skip_assigning_gamepad_to_player1(&mut self, b_skip_first_player: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_settings::U_GAME_MAPS_SETTINGS_SET_SKIP_ASSIGNING_GAMEPAD_TO_PLAYER1,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_skip_first_player,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_settings::U_GAME_MAPS_SETTINGS_SET_SKIP_ASSIGNING_GAMEPAD_TO_PLAYER1,
                __buffer,
            )
        };
    }
    pub fn get_skip_assigning_gamepad_to_player1(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_settings::U_GAME_MAPS_SETTINGS_GET_SKIP_ASSIGNING_GAMEPAD_TO_PLAYER1,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_settings::U_GAME_MAPS_SETTINGS_GET_SKIP_ASSIGNING_GAMEPAD_TO_PLAYER1,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_game_maps_settings() -> UPtr<UGameMapsSettings> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::engine_settings::U_GAME_MAPS_SETTINGS_GET_GAME_MAPS_SETTINGS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::engine_settings::UGameMapsSettings::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::engine_settings::U_GAME_MAPS_SETTINGS_GET_GAME_MAPS_SETTINGS,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UGameMapsSettings>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UGameNetworkManagerSettings {
    __padding_end: [u8; 96],
}
impl UGameNetworkManagerSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameNetworkManagerSettings")
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
pub struct UGameSessionSettings {
    __padding_end: [u8; 64],
}
impl UGameSessionSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameSessionSettings")
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
pub struct UGeneralEngineSettings {
    __padding_end: [u8; 48],
}
impl UGeneralEngineSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeneralEngineSettings")
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
pub struct UGeneralProjectSettings {
    __padding_end: [u8; 280],
}
impl UGeneralProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeneralProjectSettings")
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
pub struct UHudSettings {
    __padding_end: [u8; 72],
}
impl UHudSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHudSettings")
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
pub struct UConsoleSettings {
    __padding_end: [u8; 120],
}
impl UConsoleSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConsoleSettings")
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
pub struct ETwoPlayerSplitScreenType(pub u8);
impl ETwoPlayerSplitScreenType {
    pub const HORIZONTAL: ETwoPlayerSplitScreenType = ETwoPlayerSplitScreenType(0);
    pub const VERTICAL: ETwoPlayerSplitScreenType = ETwoPlayerSplitScreenType(1);
}
#[repr(transparent)]
pub struct EThreePlayerSplitScreenType(pub u8);
impl EThreePlayerSplitScreenType {
    pub const FAVOR_TOP: EThreePlayerSplitScreenType = EThreePlayerSplitScreenType(0);
    pub const FAVOR_BOTTOM: EThreePlayerSplitScreenType = EThreePlayerSplitScreenType(1);
    pub const VERTICAL: EThreePlayerSplitScreenType = EThreePlayerSplitScreenType(2);
    pub const HORIZONTAL: EThreePlayerSplitScreenType = EThreePlayerSplitScreenType(3);
}
#[repr(transparent)]
pub struct EFourPlayerSplitScreenType(pub u8);
impl EFourPlayerSplitScreenType {
    pub const GRID: EFourPlayerSplitScreenType = EFourPlayerSplitScreenType(0);
    pub const VERTICAL: EFourPlayerSplitScreenType = EFourPlayerSplitScreenType(1);
    pub const HORIZONTAL: EFourPlayerSplitScreenType = EFourPlayerSplitScreenType(2);
}
