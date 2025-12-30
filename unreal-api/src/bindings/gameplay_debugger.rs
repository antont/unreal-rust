#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
    pub shape_data: TArray<FVector>,
    pub description: FString,
    pub color: FColor,
    pub ty: EGameplayDebuggerShape,
}
#[repr(C, align(8))]
pub struct FGameplayDebuggerNetPack {
    pub owner: UPtr<AGameplayDebuggerCategoryReplicator>,
    pub saved_data: TArray<FGameplayDebuggerCategoryData>,
}
#[repr(C, align(4))]
pub struct FGameplayDebuggerDebugActor {
    pub actor: TWeakObjectPtr<AActor>,
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
    pub key: FKey,
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
    pub input_component: UPtr<UInputComponent>,
    pub replicator: UPtr<AGameplayDebuggerCategoryReplicator>,
}
pub struct AGameplayDebuggerCategoryReplicator {
    pub owner_pc: UPtr<APlayerController>,
    pub b_is_enabled: bool,
    pub replicated_data: FGameplayDebuggerNetPack,
    pub debug_actor: FGameplayDebuggerDebugActor,
    pub vis_log_sync: FGameplayDebuggerVisLogSync,
    pub rendering_comp: UPtr<UGameplayDebuggerRenderingComponent>,
}
pub struct UGameplayDebuggerConfig {
    pub activation_key: FKey,
    pub category_row_next_key: FKey,
    pub category_row_prev_key: FKey,
    pub category_slot0: FKey,
    pub category_slot1: FKey,
    pub category_slot2: FKey,
    pub category_slot3: FKey,
    pub category_slot4: FKey,
    pub category_slot5: FKey,
    pub category_slot6: FKey,
    pub category_slot7: FKey,
    pub category_slot8: FKey,
    pub category_slot9: FKey,
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
    pub debug_actor_candidate: UPtr<AActor>,
    pub hud_font: UPtr<UFont>,
}
pub struct AGameplayDebuggerPlayerManager {
    pub player_data: TArray<FGameplayDebuggerPlayerData>,
    pub pending_registrations: TArray<UPtr<AGameplayDebuggerCategoryReplicator>>,
    pub editor_world_data: FGameplayDebuggerPlayerData,
}
pub struct UGameplayDebuggerRenderingComponent {}
