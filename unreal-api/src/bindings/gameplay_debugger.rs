#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FGameplayDebuggerDataPackRPCParams {
    pub category_name: FName,
    pub data_pack_idx: i32,
    pub header: FGameplayDebuggerDataPackHeader,
    pub data: TArray<u8>,
}
#[repr(C, align(4))]
pub struct FGameplayDebuggerDataPackHeader {
    pub data_version: i16,
    pub sync_counter: i16,
    pub data_size: i32,
    pub data_offset: i32,
    pub flags_12: u8,
}
#[repr(C, align(8))]
pub struct FGameplayDebuggerCategoryData {
    pub category_name: FName,
    pub text_lines: TArray<FString>,
    pub shapes: TArray<FGameplayDebuggerShape>,
    pub data_packs: TArray<FGameplayDebuggerDataPackHeader>,
    pub b_is_enabled: bool,
}
#[repr(C, align(8))]
pub struct FGameplayDebuggerShape {
    pub shape_data: TArray<crate::bindings::core_u_object::FVector>,
    pub description: FString,
    pub color: crate::bindings::core_u_object::FColor,
    pub ty: EGameplayDebuggerShape,
}
#[repr(C, align(8))]
pub struct FGameplayDebuggerNetPack {
    pub owner: UPtr<AGameplayDebuggerCategoryReplicator>,
    pub saved_data: TArray<FGameplayDebuggerCategoryData>,
}
#[repr(C, align(4))]
pub struct FGameplayDebuggerDebugActor {
    pub actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub actor_name: FName,
    pub sync_counter: i16,
}
#[repr(C, align(8))]
pub struct FGameplayDebuggerVisLogSync {
    pub device_i_ds: FString,
}
#[repr(C, align(8))]
pub struct FGameplayDebuggerInputConfig {
    pub config_name: FString,
    pub key: crate::bindings::input_core::FKey,
    pub flags_48: u8,
}
#[repr(C, align(8))]
pub struct FGameplayDebuggerCategoryConfig {
    pub category_name: FString,
    pub slot_idx: i32,
    pub active_in_game: EGameplayDebuggerOverrideMode,
    pub active_in_simulate: EGameplayDebuggerOverrideMode,
    pub hidden: EGameplayDebuggerOverrideMode,
    pub flags_24: u8,
    pub input_handlers: TArray<FGameplayDebuggerInputConfig>,
}
#[repr(C, align(8))]
pub struct FGameplayDebuggerExtensionConfig {
    pub extension_name: FString,
    pub use_extension: EGameplayDebuggerOverrideMode,
    pub input_handlers: TArray<FGameplayDebuggerInputConfig>,
}
#[repr(C, align(8))]
pub struct FGameplayDebuggerPlayerData {
    pub controller: UPtr<UGameplayDebuggerLocalController>,
    pub input_component: UPtr<crate::bindings::engine::UInputComponent>,
    pub replicator: UPtr<AGameplayDebuggerCategoryReplicator>,
}
pub struct AGameplayDebuggerCategoryReplicator {
    pub owner_pc: UPtr<crate::bindings::engine::APlayerController>,
    pub b_is_enabled: bool,
    pub replicated_data: FGameplayDebuggerNetPack,
    pub debug_actor: FGameplayDebuggerDebugActor,
    pub vis_log_sync: FGameplayDebuggerVisLogSync,
    pub rendering_comp: UPtr<UGameplayDebuggerRenderingComponent>,
}
pub struct UGameplayDebuggerConfig {
    pub activation_key: crate::bindings::input_core::FKey,
    pub category_row_next_key: crate::bindings::input_core::FKey,
    pub category_row_prev_key: crate::bindings::input_core::FKey,
    pub category_slot0: crate::bindings::input_core::FKey,
    pub category_slot1: crate::bindings::input_core::FKey,
    pub category_slot2: crate::bindings::input_core::FKey,
    pub category_slot3: crate::bindings::input_core::FKey,
    pub category_slot4: crate::bindings::input_core::FKey,
    pub category_slot5: crate::bindings::input_core::FKey,
    pub category_slot6: crate::bindings::input_core::FKey,
    pub category_slot7: crate::bindings::input_core::FKey,
    pub category_slot8: crate::bindings::input_core::FKey,
    pub category_slot9: crate::bindings::input_core::FKey,
    pub debug_canvas_padding_left: f32,
    pub debug_canvas_padding_right: f32,
    pub debug_canvas_padding_top: f32,
    pub debug_canvas_padding_bottom: f32,
    pub b_debug_canvas_enable_text_shadow: bool,
    pub categories: TArray<FGameplayDebuggerCategoryConfig>,
    pub extensions: TArray<FGameplayDebuggerExtensionConfig>,
}
pub struct UGameplayDebuggerUserSettings {
    pub flags_104: u8,
    pub max_view_distance: f32,
    pub max_view_angle: f32,
    pub enabled_categories: TSet<FName>,
    pub font_size: i32,
}
pub struct UGameplayDebuggerLocalController {
    pub cached_replicator: UPtr<AGameplayDebuggerCategoryReplicator>,
    pub cached_player_manager: UPtr<AGameplayDebuggerPlayerManager>,
    pub debug_actor_candidate: UPtr<crate::bindings::engine::AActor>,
    pub hud_font: UPtr<crate::bindings::engine::UFont>,
}
pub struct AGameplayDebuggerPlayerManager {
    pub player_data: TArray<FGameplayDebuggerPlayerData>,
    pub pending_registrations: TArray<UPtr<AGameplayDebuggerCategoryReplicator>>,
    pub editor_world_data: FGameplayDebuggerPlayerData,
}
pub struct UGameplayDebuggerRenderingComponent {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGameplayDebuggerShape(pub u8);
impl EGameplayDebuggerShape {
    pub const INVALID: EGameplayDebuggerShape = EGameplayDebuggerShape(0);
    pub const POINT: EGameplayDebuggerShape = EGameplayDebuggerShape(1);
    pub const SEGMENT: EGameplayDebuggerShape = EGameplayDebuggerShape(2);
    pub const BOX: EGameplayDebuggerShape = EGameplayDebuggerShape(3);
    pub const CONE: EGameplayDebuggerShape = EGameplayDebuggerShape(4);
    pub const CYLINDER: EGameplayDebuggerShape = EGameplayDebuggerShape(5);
    pub const CIRCLE: EGameplayDebuggerShape = EGameplayDebuggerShape(6);
    pub const RECTANGLE: EGameplayDebuggerShape = EGameplayDebuggerShape(7);
    pub const CAPSULE: EGameplayDebuggerShape = EGameplayDebuggerShape(8);
    pub const POLYGON: EGameplayDebuggerShape = EGameplayDebuggerShape(9);
    pub const POLYLINE: EGameplayDebuggerShape = EGameplayDebuggerShape(10);
    pub const ARROW: EGameplayDebuggerShape = EGameplayDebuggerShape(11);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGameplayDebuggerOverrideMode(pub u8);
impl EGameplayDebuggerOverrideMode {
    pub const ENABLE: EGameplayDebuggerOverrideMode = EGameplayDebuggerOverrideMode(0);
    pub const DISABLE: EGameplayDebuggerOverrideMode = EGameplayDebuggerOverrideMode(1);
    pub const USE_DEFAULT: EGameplayDebuggerOverrideMode = EGameplayDebuggerOverrideMode(
        2,
    );
}
