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
    pub u_achievement_blueprint_library_get_cached_achievement_progress: *mut crate::ffi::UFunctionOpague,
    pub u_achievement_blueprint_library_get_cached_achievement_description: *mut crate::ffi::UFunctionOpague,
    pub u_achievement_query_callback_proxy_cache_achievements: *mut crate::ffi::UFunctionOpague,
    pub u_achievement_query_callback_proxy_cache_achievement_descriptions: *mut crate::ffi::UFunctionOpague,
    pub u_achievement_write_callback_proxy_write_progress: *mut crate::ffi::UFunctionOpague,
    pub u_connection_callback_proxy_connect_to_service: *mut crate::ffi::UFunctionOpague,
    pub u_create_session_callback_proxy_create_session: *mut crate::ffi::UFunctionOpague,
    pub u_destroy_session_callback_proxy_destroy_session: *mut crate::ffi::UFunctionOpague,
    pub u_end_match_callback_proxy_end_match: *mut crate::ffi::UFunctionOpague,
    pub u_end_turn_callback_proxy_end_turn: *mut crate::ffi::UFunctionOpague,
    pub u_find_sessions_callback_proxy_get_server_name: *mut crate::ffi::UFunctionOpague,
    pub u_find_sessions_callback_proxy_get_ping_in_ms: *mut crate::ffi::UFunctionOpague,
    pub u_find_sessions_callback_proxy_get_max_players: *mut crate::ffi::UFunctionOpague,
    pub u_find_sessions_callback_proxy_get_current_players: *mut crate::ffi::UFunctionOpague,
    pub u_find_sessions_callback_proxy_find_sessions: *mut crate::ffi::UFunctionOpague,
    pub u_find_turn_based_match_callback_proxy_find_turn_based_match: *mut crate::ffi::UFunctionOpague,
    pub u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase_unprocessed_purchases: *mut crate::ffi::UFunctionOpague,
    pub u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase_query_owned: *mut crate::ffi::UFunctionOpague,
    pub u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase: *mut crate::ffi::UFunctionOpague,
    pub u_in_app_purchase_checkout_callback_proxy_create_proxy_object_for_in_app_purchase_checkout: *mut crate::ffi::UFunctionOpague,
    pub u_in_app_purchase_finalize_proxy_create_proxy_object_for_in_app_purchase_finalize: *mut crate::ffi::UFunctionOpague,
    pub u_in_app_purchase_query_callback_proxy2_create_proxy_object_for_in_app_purchase_query: *mut crate::ffi::UFunctionOpague,
    pub u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_restore_owned_products: *mut crate::ffi::UFunctionOpague,
    pub u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_query_owned_products: *mut crate::ffi::UFunctionOpague,
    pub u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_get_known_receipts: *mut crate::ffi::UFunctionOpague,
    pub u_in_app_purchase_restore_callback_proxy2_create_proxy_object_for_in_app_purchase_restore: *mut crate::ffi::UFunctionOpague,
    pub u_join_session_callback_proxy_join_session: *mut crate::ffi::UFunctionOpague,
    pub u_leaderboard_blueprint_library_write_leaderboard_integer: *mut crate::ffi::UFunctionOpague,
    pub u_leaderboard_flush_callback_proxy_create_proxy_object_for_flush: *mut crate::ffi::UFunctionOpague,
    pub u_leaderboard_query_callback_proxy_create_proxy_object_for_int_query: *mut crate::ffi::UFunctionOpague,
    pub u_logout_callback_proxy_logout: *mut crate::ffi::UFunctionOpague,
    pub a_online_beacon_client_client_on_connected: *mut crate::ffi::UFunctionOpague,
    pub a_party_beacon_client_server_update_reservation_request: *mut crate::ffi::UFunctionOpague,
    pub a_party_beacon_client_server_reservation_request: *mut crate::ffi::UFunctionOpague,
    pub a_party_beacon_client_server_remove_member_from_reservation_request: *mut crate::ffi::UFunctionOpague,
    pub a_party_beacon_client_server_cancel_reservation_request: *mut crate::ffi::UFunctionOpague,
    pub a_party_beacon_client_server_add_or_update_reservation_request: *mut crate::ffi::UFunctionOpague,
    pub a_party_beacon_client_client_send_reservation_updates: *mut crate::ffi::UFunctionOpague,
    pub a_party_beacon_client_client_send_reservation_full: *mut crate::ffi::UFunctionOpague,
    pub a_party_beacon_client_client_reservation_response: *mut crate::ffi::UFunctionOpague,
    pub a_party_beacon_client_client_cancel_reservation_response: *mut crate::ffi::UFunctionOpague,
    pub u_quit_match_callback_proxy_quit_match: *mut crate::ffi::UFunctionOpague,
    pub u_show_login_ui_callback_proxy_show_external_login_ui: *mut crate::ffi::UFunctionOpague,
    pub a_spectator_beacon_client_server_reservation_request: *mut crate::ffi::UFunctionOpague,
    pub a_spectator_beacon_client_server_cancel_reservation_request: *mut crate::ffi::UFunctionOpague,
    pub a_spectator_beacon_client_client_send_reservation_updates: *mut crate::ffi::UFunctionOpague,
    pub a_spectator_beacon_client_client_send_reservation_full: *mut crate::ffi::UFunctionOpague,
    pub a_spectator_beacon_client_client_reservation_response: *mut crate::ffi::UFunctionOpague,
    pub a_spectator_beacon_client_client_cancel_reservation_response: *mut crate::ffi::UFunctionOpague,
    pub a_test_beacon_client_server_pong: *mut crate::ffi::UFunctionOpague,
    pub a_test_beacon_client_client_ping: *mut crate::ffi::UFunctionOpague,
    pub u_turn_based_blueprint_library_register_turn_based_match_interface_object: *mut crate::ffi::UFunctionOpague,
    pub u_turn_based_blueprint_library_get_player_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_turn_based_blueprint_library_get_my_player_index: *mut crate::ffi::UFunctionOpague,
    pub u_turn_based_blueprint_library_get_is_my_turn: *mut crate::ffi::UFunctionOpague,
    pub u_voip_listener_synth_component_is_idling: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_achievement_blueprint_library_get_cached_achievement_progress: std::ptr::null_mut(),
            u_achievement_blueprint_library_get_cached_achievement_description: std::ptr::null_mut(),
            u_achievement_query_callback_proxy_cache_achievements: std::ptr::null_mut(),
            u_achievement_query_callback_proxy_cache_achievement_descriptions: std::ptr::null_mut(),
            u_achievement_write_callback_proxy_write_progress: std::ptr::null_mut(),
            u_connection_callback_proxy_connect_to_service: std::ptr::null_mut(),
            u_create_session_callback_proxy_create_session: std::ptr::null_mut(),
            u_destroy_session_callback_proxy_destroy_session: std::ptr::null_mut(),
            u_end_match_callback_proxy_end_match: std::ptr::null_mut(),
            u_end_turn_callback_proxy_end_turn: std::ptr::null_mut(),
            u_find_sessions_callback_proxy_get_server_name: std::ptr::null_mut(),
            u_find_sessions_callback_proxy_get_ping_in_ms: std::ptr::null_mut(),
            u_find_sessions_callback_proxy_get_max_players: std::ptr::null_mut(),
            u_find_sessions_callback_proxy_get_current_players: std::ptr::null_mut(),
            u_find_sessions_callback_proxy_find_sessions: std::ptr::null_mut(),
            u_find_turn_based_match_callback_proxy_find_turn_based_match: std::ptr::null_mut(),
            u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase_unprocessed_purchases: std::ptr::null_mut(),
            u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase_query_owned: std::ptr::null_mut(),
            u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase: std::ptr::null_mut(),
            u_in_app_purchase_checkout_callback_proxy_create_proxy_object_for_in_app_purchase_checkout: std::ptr::null_mut(),
            u_in_app_purchase_finalize_proxy_create_proxy_object_for_in_app_purchase_finalize: std::ptr::null_mut(),
            u_in_app_purchase_query_callback_proxy2_create_proxy_object_for_in_app_purchase_query: std::ptr::null_mut(),
            u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_restore_owned_products: std::ptr::null_mut(),
            u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_query_owned_products: std::ptr::null_mut(),
            u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_get_known_receipts: std::ptr::null_mut(),
            u_in_app_purchase_restore_callback_proxy2_create_proxy_object_for_in_app_purchase_restore: std::ptr::null_mut(),
            u_join_session_callback_proxy_join_session: std::ptr::null_mut(),
            u_leaderboard_blueprint_library_write_leaderboard_integer: std::ptr::null_mut(),
            u_leaderboard_flush_callback_proxy_create_proxy_object_for_flush: std::ptr::null_mut(),
            u_leaderboard_query_callback_proxy_create_proxy_object_for_int_query: std::ptr::null_mut(),
            u_logout_callback_proxy_logout: std::ptr::null_mut(),
            a_online_beacon_client_client_on_connected: std::ptr::null_mut(),
            a_party_beacon_client_server_update_reservation_request: std::ptr::null_mut(),
            a_party_beacon_client_server_reservation_request: std::ptr::null_mut(),
            a_party_beacon_client_server_remove_member_from_reservation_request: std::ptr::null_mut(),
            a_party_beacon_client_server_cancel_reservation_request: std::ptr::null_mut(),
            a_party_beacon_client_server_add_or_update_reservation_request: std::ptr::null_mut(),
            a_party_beacon_client_client_send_reservation_updates: std::ptr::null_mut(),
            a_party_beacon_client_client_send_reservation_full: std::ptr::null_mut(),
            a_party_beacon_client_client_reservation_response: std::ptr::null_mut(),
            a_party_beacon_client_client_cancel_reservation_response: std::ptr::null_mut(),
            u_quit_match_callback_proxy_quit_match: std::ptr::null_mut(),
            u_show_login_ui_callback_proxy_show_external_login_ui: std::ptr::null_mut(),
            a_spectator_beacon_client_server_reservation_request: std::ptr::null_mut(),
            a_spectator_beacon_client_server_cancel_reservation_request: std::ptr::null_mut(),
            a_spectator_beacon_client_client_send_reservation_updates: std::ptr::null_mut(),
            a_spectator_beacon_client_client_send_reservation_full: std::ptr::null_mut(),
            a_spectator_beacon_client_client_reservation_response: std::ptr::null_mut(),
            a_spectator_beacon_client_client_cancel_reservation_response: std::ptr::null_mut(),
            a_test_beacon_client_server_pong: std::ptr::null_mut(),
            a_test_beacon_client_client_ping: std::ptr::null_mut(),
            u_turn_based_blueprint_library_register_turn_based_match_interface_object: std::ptr::null_mut(),
            u_turn_based_blueprint_library_get_player_display_name: std::ptr::null_mut(),
            u_turn_based_blueprint_library_get_my_player_index: std::ptr::null_mut(),
            u_turn_based_blueprint_library_get_is_my_turn: std::ptr::null_mut(),
            u_voip_listener_synth_component_is_idling: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAchievementBlueprintLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCachedAchievementProgress"),
                &raw mut __FUNCTION_PTRS
                    .u_achievement_blueprint_library_get_cached_achievement_progress,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCachedAchievementDescription"),
                &raw mut __FUNCTION_PTRS
                    .u_achievement_blueprint_library_get_cached_achievement_description,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAchievementQueryCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CacheAchievements"),
                &raw mut __FUNCTION_PTRS
                    .u_achievement_query_callback_proxy_cache_achievements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CacheAchievementDescriptions"),
                &raw mut __FUNCTION_PTRS
                    .u_achievement_query_callback_proxy_cache_achievement_descriptions,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAchievementWriteCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WriteProgress"),
                &raw mut __FUNCTION_PTRS
                    .u_achievement_write_callback_proxy_write_progress,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UConnectionCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConnectToService"),
                &raw mut __FUNCTION_PTRS.u_connection_callback_proxy_connect_to_service,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UCreateSessionCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateSession"),
                &raw mut __FUNCTION_PTRS.u_create_session_callback_proxy_create_session,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDestroySessionCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DestroySession"),
                &raw mut __FUNCTION_PTRS.u_destroy_session_callback_proxy_destroy_session,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEndMatchCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EndMatch"),
                &raw mut __FUNCTION_PTRS.u_end_match_callback_proxy_end_match,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEndTurnCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EndTurn"),
                &raw mut __FUNCTION_PTRS.u_end_turn_callback_proxy_end_turn,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UFindSessionsCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetServerName"),
                &raw mut __FUNCTION_PTRS.u_find_sessions_callback_proxy_get_server_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPingInMs"),
                &raw mut __FUNCTION_PTRS.u_find_sessions_callback_proxy_get_ping_in_ms,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMaxPlayers"),
                &raw mut __FUNCTION_PTRS.u_find_sessions_callback_proxy_get_max_players,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentPlayers"),
                &raw mut __FUNCTION_PTRS
                    .u_find_sessions_callback_proxy_get_current_players,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindSessions"),
                &raw mut __FUNCTION_PTRS.u_find_sessions_callback_proxy_find_sessions,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UFindTurnBasedMatchCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindTurnBasedMatch"),
                &raw mut __FUNCTION_PTRS
                    .u_find_turn_based_match_callback_proxy_find_turn_based_match,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInAppPurchaseCallbackProxy2::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "CreateProxyObjectForInAppPurchaseUnprocessedPurchases",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase_unprocessed_purchases,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateProxyObjectForInAppPurchaseQueryOwned"),
                &raw mut __FUNCTION_PTRS
                    .u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase_query_owned,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateProxyObjectForInAppPurchase"),
                &raw mut __FUNCTION_PTRS
                    .u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInAppPurchaseCheckoutCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateProxyObjectForInAppPurchaseCheckout"),
                &raw mut __FUNCTION_PTRS
                    .u_in_app_purchase_checkout_callback_proxy_create_proxy_object_for_in_app_purchase_checkout,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInAppPurchaseFinalizeProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateProxyObjectForInAppPurchaseFinalize"),
                &raw mut __FUNCTION_PTRS
                    .u_in_app_purchase_finalize_proxy_create_proxy_object_for_in_app_purchase_finalize,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInAppPurchaseQueryCallbackProxy2::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateProxyObjectForInAppPurchaseQuery"),
                &raw mut __FUNCTION_PTRS
                    .u_in_app_purchase_query_callback_proxy2_create_proxy_object_for_in_app_purchase_query,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInAppPurchaseReceiptsCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "CreateProxyObjectForInAppPurchaseRestoreOwnedProducts",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_restore_owned_products,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "CreateProxyObjectForInAppPurchaseQueryOwnedProducts",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_query_owned_products,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "CreateProxyObjectForInAppPurchaseGetKnownReceipts",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_get_known_receipts,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInAppPurchaseRestoreCallbackProxy2::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateProxyObjectForInAppPurchaseRestore"),
                &raw mut __FUNCTION_PTRS
                    .u_in_app_purchase_restore_callback_proxy2_create_proxy_object_for_in_app_purchase_restore,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UJoinSessionCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("JoinSession"),
                &raw mut __FUNCTION_PTRS.u_join_session_callback_proxy_join_session,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULeaderboardBlueprintLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WriteLeaderboardInteger"),
                &raw mut __FUNCTION_PTRS
                    .u_leaderboard_blueprint_library_write_leaderboard_integer,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULeaderboardFlushCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateProxyObjectForFlush"),
                &raw mut __FUNCTION_PTRS
                    .u_leaderboard_flush_callback_proxy_create_proxy_object_for_flush,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULeaderboardQueryCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateProxyObjectForIntQuery"),
                &raw mut __FUNCTION_PTRS
                    .u_leaderboard_query_callback_proxy_create_proxy_object_for_int_query,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULogoutCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Logout"),
                &raw mut __FUNCTION_PTRS.u_logout_callback_proxy_logout,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AOnlineBeaconClient::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientOnConnected"),
                &raw mut __FUNCTION_PTRS.a_online_beacon_client_client_on_connected,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = APartyBeaconClient::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerUpdateReservationRequest"),
                &raw mut __FUNCTION_PTRS
                    .a_party_beacon_client_server_update_reservation_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerReservationRequest"),
                &raw mut __FUNCTION_PTRS.a_party_beacon_client_server_reservation_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerRemoveMemberFromReservationRequest"),
                &raw mut __FUNCTION_PTRS
                    .a_party_beacon_client_server_remove_member_from_reservation_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerCancelReservationRequest"),
                &raw mut __FUNCTION_PTRS
                    .a_party_beacon_client_server_cancel_reservation_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerAddOrUpdateReservationRequest"),
                &raw mut __FUNCTION_PTRS
                    .a_party_beacon_client_server_add_or_update_reservation_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientSendReservationUpdates"),
                &raw mut __FUNCTION_PTRS
                    .a_party_beacon_client_client_send_reservation_updates,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientSendReservationFull"),
                &raw mut __FUNCTION_PTRS
                    .a_party_beacon_client_client_send_reservation_full,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientReservationResponse"),
                &raw mut __FUNCTION_PTRS
                    .a_party_beacon_client_client_reservation_response,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientCancelReservationResponse"),
                &raw mut __FUNCTION_PTRS
                    .a_party_beacon_client_client_cancel_reservation_response,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UQuitMatchCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("QuitMatch"),
                &raw mut __FUNCTION_PTRS.u_quit_match_callback_proxy_quit_match,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UShowLoginUICallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShowExternalLoginUI"),
                &raw mut __FUNCTION_PTRS
                    .u_show_login_ui_callback_proxy_show_external_login_ui,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ASpectatorBeaconClient::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerReservationRequest"),
                &raw mut __FUNCTION_PTRS
                    .a_spectator_beacon_client_server_reservation_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerCancelReservationRequest"),
                &raw mut __FUNCTION_PTRS
                    .a_spectator_beacon_client_server_cancel_reservation_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientSendReservationUpdates"),
                &raw mut __FUNCTION_PTRS
                    .a_spectator_beacon_client_client_send_reservation_updates,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientSendReservationFull"),
                &raw mut __FUNCTION_PTRS
                    .a_spectator_beacon_client_client_send_reservation_full,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientReservationResponse"),
                &raw mut __FUNCTION_PTRS
                    .a_spectator_beacon_client_client_reservation_response,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientCancelReservationResponse"),
                &raw mut __FUNCTION_PTRS
                    .a_spectator_beacon_client_client_cancel_reservation_response,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ATestBeaconClient::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerPong"),
                &raw mut __FUNCTION_PTRS.a_test_beacon_client_server_pong,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientPing"),
                &raw mut __FUNCTION_PTRS.a_test_beacon_client_client_ping,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTurnBasedBlueprintLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterTurnBasedMatchInterfaceObject"),
                &raw mut __FUNCTION_PTRS
                    .u_turn_based_blueprint_library_register_turn_based_match_interface_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlayerDisplayName"),
                &raw mut __FUNCTION_PTRS
                    .u_turn_based_blueprint_library_get_player_display_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMyPlayerIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_turn_based_blueprint_library_get_my_player_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetIsMyTurn"),
                &raw mut __FUNCTION_PTRS.u_turn_based_blueprint_library_get_is_my_turn,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UVoipListenerSynthComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsIdling"),
                &raw mut __FUNCTION_PTRS.u_voip_listener_synth_component_is_idling,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FBlueprintSessionResult {
    pub(crate) __padding_end: [u8; 288],
}
impl FBlueprintSessionResult {}
#[repr(C, align(8))]
pub struct FOnlineProxyStoreOffer {
    pub offer_id: FString,
    pub title: FText,
    pub description: FText,
    pub long_description: FText,
    pub regular_price_text: FText,
    pub regular_price: i32,
    pub price_text: FText,
    pub numeric_price: i32,
    pub currency_code: FString,
    pub release_date: crate::bindings::core_u_object::FDateTime,
    pub expiration_date: crate::bindings::core_u_object::FDateTime,
    pub discount_type: EOnlineProxyStoreOfferDiscountType,
    pub dynamic_fields: TMap<FString, FString>,
}
impl FOnlineProxyStoreOffer {}
#[repr(C, align(8))]
pub struct FInAppPurchaseRestoreInfo2 {
    pub item_name: FString,
    pub item_id: FString,
    pub validation_info: FString,
}
impl FInAppPurchaseRestoreInfo2 {}
#[repr(C, align(8))]
pub struct FInAppPurchaseReceiptInfo2 {
    pub item_name: FString,
    pub item_id: FString,
    pub validation_info: FString,
    pub transaction_identifier: FString,
}
impl FInAppPurchaseReceiptInfo2 {}
#[repr(C, align(8))]
pub struct FInAppPurchaseProductInfo2 {
    pub identifier: FString,
    pub transaction_identifier: FString,
    pub display_name: FString,
    pub display_description: FString,
    pub display_price: FString,
    pub raw_price: f32,
    pub currency_code: FString,
    pub currency_symbol: FString,
    pub decimal_separator: FString,
    pub grouping_separator: FString,
    pub receipt_data: FString,
    pub dynamic_fields: TMap<FString, FString>,
}
impl FInAppPurchaseProductInfo2 {}
#[repr(C, align(8))]
pub struct FInAppPurchaseProductRequest2 {
    pub product_identifier: FString,
    pub b_is_consumable: bool,
}
impl FInAppPurchaseProductRequest2 {}
#[repr(C, align(8))]
pub struct UAchievementBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UAchievementBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAchievementBlueprintLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAchievementBlueprintLibrary")
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
    pub fn get_cached_achievement_progress(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        achievement_id: FName,
        b_found_id: &mut bool,
        progress: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_achievement_blueprint_library_get_cached_achievement_progress,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &achievement_id,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_found_id,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(progress, __buffer.add(32).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UAchievementBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_achievement_blueprint_library_get_cached_achievement_progress,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(28).cast::<bool>().swap(b_found_id);
        }
        unsafe {
            __buffer.add(32).cast::<f32>().swap(progress);
        }
    }
    pub fn get_cached_achievement_description(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        achievement_id: FName,
        b_found_id: &mut bool,
        title: &mut FText,
        locked_description: &mut FText,
        unlocked_description: &mut FText,
        b_hidden: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_achievement_blueprint_library_get_cached_achievement_description,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &achievement_id,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_found_id,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(title, __buffer.add(32).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                locked_description,
                __buffer.add(48).cast::<FText>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                unlocked_description,
                __buffer.add(64).cast::<FText>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(b_hidden, __buffer.add(80).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UAchievementBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_achievement_blueprint_library_get_cached_achievement_description,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(28).cast::<bool>().swap(b_found_id);
        }
        unsafe {
            __buffer.add(32).cast::<FText>().swap(title);
        }
        unsafe {
            __buffer.add(48).cast::<FText>().swap(locked_description);
        }
        unsafe {
            __buffer.add(64).cast::<FText>().swap(unlocked_description);
        }
        unsafe {
            __buffer.add(80).cast::<bool>().swap(b_hidden);
        }
    }
}
#[repr(C, align(8))]
pub struct UAchievementQueryCallbackProxy {
    __padding_end: [u8; 128],
}
impl UAchievementQueryCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAchievementQueryCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAchievementQueryCallbackProxy")
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
    pub fn cache_achievements(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UAchievementQueryCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_achievement_query_callback_proxy_cache_achievements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UAchievementQueryCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_achievement_query_callback_proxy_cache_achievements,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UAchievementQueryCallbackProxy>>().read() }
    }
    pub fn cache_achievement_descriptions(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UAchievementQueryCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_achievement_query_callback_proxy_cache_achievement_descriptions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UAchievementQueryCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_achievement_query_callback_proxy_cache_achievement_descriptions,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UAchievementQueryCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAchievementWriteCallbackProxy {
    __padding_end: [u8; 160],
}
impl UAchievementWriteCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAchievementWriteCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAchievementWriteCallbackProxy")
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
    pub fn write_progress(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        achievement_name: FString,
        progress: f32,
        user_tag: i32,
    ) -> UPtr<UAchievementWriteCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_achievement_write_callback_proxy_write_progress,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &achievement_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&progress, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&user_tag, __buffer.add(36).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UAchievementWriteCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_achievement_write_callback_proxy_write_progress,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<UAchievementWriteCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UConnectionCallbackProxy {
    __padding_end: [u8; 152],
}
impl UConnectionCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConnectionCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConnectionCallbackProxy")
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
    pub fn connect_to_service(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UConnectionCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_connection_callback_proxy_connect_to_service,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UConnectionCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_connection_callback_proxy_connect_to_service,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UConnectionCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UCreateSessionCallbackProxy {
    __padding_end: [u8; 192],
}
impl UCreateSessionCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCreateSessionCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCreateSessionCallbackProxy")
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
    pub fn create_session(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        public_connections: i32,
        b_use_lan: bool,
        b_use_lobbies_if_available: bool,
    ) -> UPtr<UCreateSessionCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_create_session_callback_proxy_create_session,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &public_connections,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_lan,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_lobbies_if_available,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UCreateSessionCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_create_session_callback_proxy_create_session,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UCreateSessionCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UDestroySessionCallbackProxy {
    __padding_end: [u8; 152],
}
impl UDestroySessionCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDestroySessionCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDestroySessionCallbackProxy")
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
    pub fn destroy_session(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UDestroySessionCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_destroy_session_callback_proxy_destroy_session,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UDestroySessionCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_destroy_session_callback_proxy_destroy_session,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UDestroySessionCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEndMatchCallbackProxy {
    __padding_end: [u8; 152],
}
impl UEndMatchCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEndMatchCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEndMatchCallbackProxy")
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
    pub fn end_match(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        match_actor: TScriptInterface<
            crate::bindings::online_subsystem::UTurnBasedMatchInterface,
        >,
        match_id: FString,
        local_player_outcome: crate::bindings::online_subsystem::EMPMatchOutcome,
        other_players_outcome: crate::bindings::online_subsystem::EMPMatchOutcome,
    ) -> UPtr<UEndMatchCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_end_match_callback_proxy_end_match,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &match_actor,
                __buffer
                    .add(16)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::online_subsystem::UTurnBasedMatchInterface,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &match_id,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &local_player_outcome,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::online_subsystem::EMPMatchOutcome>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &other_players_outcome,
                __buffer
                    .add(49)
                    .cast::<crate::bindings::online_subsystem::EMPMatchOutcome>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UEndMatchCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_end_match_callback_proxy_end_match,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<UEndMatchCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEndTurnCallbackProxy {
    __padding_end: [u8; 144],
}
impl UEndTurnCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEndTurnCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEndTurnCallbackProxy")
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
    pub fn end_turn(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        match_id: FString,
        turn_based_match_interface: TScriptInterface<
            crate::bindings::online_subsystem::UTurnBasedMatchInterface,
        >,
    ) -> UPtr<UEndTurnCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_end_turn_callback_proxy_end_turn,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &match_id,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &turn_based_match_interface,
                __buffer
                    .add(32)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::online_subsystem::UTurnBasedMatchInterface,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UEndTurnCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_end_turn_callback_proxy_end_turn,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<UEndTurnCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UFindSessionsCallbackProxy {
    __padding_end: [u8; 176],
}
impl UFindSessionsCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFindSessionsCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFindSessionsCallbackProxy")
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
    pub fn get_server_name(result: &FBlueprintSessionResult) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_find_sessions_callback_proxy_get_server_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(0).cast::<FBlueprintSessionResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UFindSessionsCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_find_sessions_callback_proxy_get_server_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(288).cast::<FString>().read() }
    }
    pub fn get_ping_in_ms(result: &FBlueprintSessionResult) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<292>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_find_sessions_callback_proxy_get_ping_in_ms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(0).cast::<FBlueprintSessionResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UFindSessionsCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_find_sessions_callback_proxy_get_ping_in_ms,
                __buffer,
            )
        };
        unsafe { __buffer.add(288).cast::<i32>().read() }
    }
    pub fn get_max_players(result: &FBlueprintSessionResult) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<292>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_find_sessions_callback_proxy_get_max_players,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(0).cast::<FBlueprintSessionResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UFindSessionsCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_find_sessions_callback_proxy_get_max_players,
                __buffer,
            )
        };
        unsafe { __buffer.add(288).cast::<i32>().read() }
    }
    pub fn get_current_players(result: &FBlueprintSessionResult) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<292>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_find_sessions_callback_proxy_get_current_players,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(0).cast::<FBlueprintSessionResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UFindSessionsCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_find_sessions_callback_proxy_get_current_players,
                __buffer,
            )
        };
        unsafe { __buffer.add(288).cast::<i32>().read() }
    }
    pub fn find_sessions(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        max_results: i32,
        b_use_lan: bool,
        b_use_lobbies: bool,
    ) -> UPtr<UFindSessionsCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_find_sessions_callback_proxy_find_sessions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_results,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_lan,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_lobbies,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UFindSessionsCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_find_sessions_callback_proxy_find_sessions,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UFindSessionsCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UFindTurnBasedMatchCallbackProxy {
    __padding_end: [u8; 160],
}
impl UFindTurnBasedMatchCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFindTurnBasedMatchCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFindTurnBasedMatchCallbackProxy")
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
    pub fn find_turn_based_match(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        match_actor: TScriptInterface<
            crate::bindings::online_subsystem::UTurnBasedMatchInterface,
        >,
        min_players: i32,
        max_players: i32,
        player_group: i32,
        show_existing_matches: bool,
    ) -> UPtr<UFindTurnBasedMatchCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_find_turn_based_match_callback_proxy_find_turn_based_match,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &match_actor,
                __buffer
                    .add(16)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::online_subsystem::UTurnBasedMatchInterface,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &min_players,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_players,
                __buffer.add(36).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_group,
                __buffer.add(40).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &show_existing_matches,
                __buffer.add(44).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UFindTurnBasedMatchCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_find_turn_based_match_callback_proxy_find_turn_based_match,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<UPtr<UFindTurnBasedMatchCallbackProxy>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UInAppPurchaseCallbackProxy2 {
    __padding_end: [u8; 200],
}
impl UInAppPurchaseCallbackProxy2 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInAppPurchaseCallbackProxy2")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInAppPurchaseCallbackProxy2")
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
    pub fn create_proxy_object_for_in_app_purchase_unprocessed_purchases(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UInAppPurchaseCallbackProxy2> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase_unprocessed_purchases,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UInAppPurchaseCallbackProxy2::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase_unprocessed_purchases,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UInAppPurchaseCallbackProxy2>>().read() }
    }
    pub fn create_proxy_object_for_in_app_purchase_query_owned(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UInAppPurchaseCallbackProxy2> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase_query_owned,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UInAppPurchaseCallbackProxy2::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase_query_owned,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UInAppPurchaseCallbackProxy2>>().read() }
    }
    pub fn create_proxy_object_for_in_app_purchase(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        product_request: &FInAppPurchaseProductRequest2,
    ) -> UPtr<UInAppPurchaseCallbackProxy2> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                product_request,
                __buffer.add(8).cast::<FInAppPurchaseProductRequest2>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UInAppPurchaseCallbackProxy2::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_callback_proxy2_create_proxy_object_for_in_app_purchase,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UInAppPurchaseCallbackProxy2>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInAppPurchaseCheckoutCallbackProxy {
    __padding_end: [u8; 184],
}
impl UInAppPurchaseCheckoutCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInAppPurchaseCheckoutCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInAppPurchaseCheckoutCallbackProxy")
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
    pub fn create_proxy_object_for_in_app_purchase_checkout(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        product_request: &FInAppPurchaseProductRequest2,
    ) -> UPtr<UInAppPurchaseCheckoutCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_checkout_callback_proxy_create_proxy_object_for_in_app_purchase_checkout,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                product_request,
                __buffer.add(8).cast::<FInAppPurchaseProductRequest2>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UInAppPurchaseCheckoutCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_checkout_callback_proxy_create_proxy_object_for_in_app_purchase_checkout,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<UPtr<UInAppPurchaseCheckoutCallbackProxy>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UInAppPurchaseFinalizeProxy {
    __padding_end: [u8; 48],
}
impl UInAppPurchaseFinalizeProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInAppPurchaseFinalizeProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInAppPurchaseFinalizeProxy")
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
    pub fn create_proxy_object_for_in_app_purchase_finalize(
        in_app_purchase_receipt: &FInAppPurchaseReceiptInfo2,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UInAppPurchaseFinalizeProxy> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_finalize_proxy_create_proxy_object_for_in_app_purchase_finalize,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_app_purchase_receipt,
                __buffer.add(0).cast::<FInAppPurchaseReceiptInfo2>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(64)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UInAppPurchaseFinalizeProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_finalize_proxy_create_proxy_object_for_in_app_purchase_finalize,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<UPtr<UInAppPurchaseFinalizeProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInAppPurchaseQueryCallbackProxy2 {
    __padding_end: [u8; 128],
}
impl UInAppPurchaseQueryCallbackProxy2 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInAppPurchaseQueryCallbackProxy2")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInAppPurchaseQueryCallbackProxy2")
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
    pub fn create_proxy_object_for_in_app_purchase_query(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        product_identifiers: &TArray<FString>,
    ) -> UPtr<UInAppPurchaseQueryCallbackProxy2> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_query_callback_proxy2_create_proxy_object_for_in_app_purchase_query,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                product_identifiers,
                __buffer.add(8).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UInAppPurchaseQueryCallbackProxy2::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_query_callback_proxy2_create_proxy_object_for_in_app_purchase_query,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<UPtr<UInAppPurchaseQueryCallbackProxy2>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UInAppPurchaseReceiptsCallbackProxy {
    __padding_end: [u8; 168],
}
impl UInAppPurchaseReceiptsCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInAppPurchaseReceiptsCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInAppPurchaseReceiptsCallbackProxy")
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
    pub fn create_proxy_object_for_in_app_purchase_restore_owned_products(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UInAppPurchaseReceiptsCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_restore_owned_products,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UInAppPurchaseReceiptsCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_restore_owned_products,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<UInAppPurchaseReceiptsCallbackProxy>>().read()
        }
    }
    pub fn create_proxy_object_for_in_app_purchase_query_owned_products(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UInAppPurchaseReceiptsCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_query_owned_products,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UInAppPurchaseReceiptsCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_query_owned_products,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<UInAppPurchaseReceiptsCallbackProxy>>().read()
        }
    }
    pub fn create_proxy_object_for_in_app_purchase_get_known_receipts(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UInAppPurchaseReceiptsCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_get_known_receipts,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UInAppPurchaseReceiptsCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_receipts_callback_proxy_create_proxy_object_for_in_app_purchase_get_known_receipts,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<UInAppPurchaseReceiptsCallbackProxy>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UInAppPurchaseRestoreCallbackProxy2 {
    __padding_end: [u8; 200],
}
impl UInAppPurchaseRestoreCallbackProxy2 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInAppPurchaseRestoreCallbackProxy2")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInAppPurchaseRestoreCallbackProxy2")
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
    pub fn create_proxy_object_for_in_app_purchase_restore(
        consumable_product_flags: &TArray<FInAppPurchaseProductRequest2>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UInAppPurchaseRestoreCallbackProxy2> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_restore_callback_proxy2_create_proxy_object_for_in_app_purchase_restore,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                consumable_product_flags,
                __buffer.add(0).cast::<TArray<FInAppPurchaseProductRequest2>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UInAppPurchaseRestoreCallbackProxy2::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_in_app_purchase_restore_callback_proxy2_create_proxy_object_for_in_app_purchase_restore,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<UPtr<UInAppPurchaseRestoreCallbackProxy2>>().read()
        }
    }
}
#[repr(C, align(16))]
pub struct UIpConnection {
    __padding_end: [u8; 8208],
}
impl UIpConnection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIpConnection")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIpConnection")
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
pub struct UIpNetDriver {
    __padding_end: [u8; 2840],
}
impl UIpNetDriver {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIpNetDriver")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIpNetDriver")
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
pub struct UJoinSessionCallbackProxy {
    __padding_end: [u8; 440],
}
impl UJoinSessionCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UJoinSessionCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UJoinSessionCallbackProxy")
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
    pub fn join_session(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        search_result: &FBlueprintSessionResult,
    ) -> UPtr<UJoinSessionCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<312>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_join_session_callback_proxy_join_session,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                search_result,
                __buffer.add(16).cast::<FBlueprintSessionResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UJoinSessionCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_join_session_callback_proxy_join_session,
                __buffer,
            )
        };
        unsafe { __buffer.add(304).cast::<UPtr<UJoinSessionCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULeaderboardBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl ULeaderboardBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULeaderboardBlueprintLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULeaderboardBlueprintLibrary")
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
    pub fn write_leaderboard_integer(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        stat_name: FName,
        stat_value: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_leaderboard_blueprint_library_write_leaderboard_integer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stat_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stat_value,
                __buffer.add(20).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::ULeaderboardBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_leaderboard_blueprint_library_write_leaderboard_integer,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULeaderboardFlushCallbackProxy {
    __padding_end: [u8; 136],
}
impl ULeaderboardFlushCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULeaderboardFlushCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULeaderboardFlushCallbackProxy")
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
    pub fn create_proxy_object_for_flush(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        session_name: FName,
    ) -> UPtr<ULeaderboardFlushCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_leaderboard_flush_callback_proxy_create_proxy_object_for_flush,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &session_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::ULeaderboardFlushCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_leaderboard_flush_callback_proxy_create_proxy_object_for_flush,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<ULeaderboardFlushCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULeaderboardQueryCallbackProxy {
    __padding_end: [u8; 192],
}
impl ULeaderboardQueryCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULeaderboardQueryCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULeaderboardQueryCallbackProxy")
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
    pub fn create_proxy_object_for_int_query(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        stat_name: FString,
    ) -> UPtr<ULeaderboardQueryCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_leaderboard_query_callback_proxy_create_proxy_object_for_int_query,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stat_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::ULeaderboardQueryCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_leaderboard_query_callback_proxy_create_proxy_object_for_int_query,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<ULeaderboardQueryCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULogoutCallbackProxy {
    __padding_end: [u8; 128],
}
impl ULogoutCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULogoutCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULogoutCallbackProxy")
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
    pub fn logout(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<ULogoutCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_logout_callback_proxy_logout,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::ULogoutCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_logout_callback_proxy_logout,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<ULogoutCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct AOnlineBeacon {
    __padding_end: [u8; 1192],
}
impl AOnlineBeacon {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeacon")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeacon")
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
pub struct AOnlineBeaconClient {
    __padding_end: [u8; 1296],
}
impl AOnlineBeaconClient {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeaconClient")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeaconClient")
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
pub struct AOnlineBeaconHost {
    __padding_end: [u8; 1464],
}
impl AOnlineBeaconHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeaconHost")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeaconHost")
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
pub struct AOnlineBeaconHostObject {
    __padding_end: [u8; 1176],
}
impl AOnlineBeaconHostObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeaconHostObject")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeaconHostObject")
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
pub struct UOnlineEngineInterfaceImpl {
    __padding_end: [u8; 424],
}
impl UOnlineEngineInterfaceImpl {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlineEngineInterfaceImpl")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlineEngineInterfaceImpl")
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
pub struct UOnlinePIEConfig {
    __padding_end: [u8; 64],
}
impl UOnlinePIEConfig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlinePIEConfig")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlinePIEConfig")
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
pub struct UOnlinePIESettings {
    __padding_end: [u8; 128],
}
impl UOnlinePIESettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlinePIESettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlinePIESettings")
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
pub struct UOnlineServicesEngineInterfaceImpl {
    __padding_end: [u8; 48],
}
impl UOnlineServicesEngineInterfaceImpl {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlineServicesEngineInterfaceImpl")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlineServicesEngineInterfaceImpl")
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
pub struct UOnlineSessionClient {
    __padding_end: [u8; 528],
}
impl UOnlineSessionClient {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlineSessionClient")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlineSessionClient")
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
pub struct APartyBeaconClient {
    __padding_end: [u8; 1520],
}
impl APartyBeaconClient {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APartyBeaconClient")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APartyBeaconClient")
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
pub struct APartyBeaconHost {
    __padding_end: [u8; 1344],
}
impl APartyBeaconHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APartyBeaconHost")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APartyBeaconHost")
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
pub struct UPartyBeaconState {
    __padding_end: [u8; 176],
}
impl UPartyBeaconState {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPartyBeaconState")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPartyBeaconState")
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
pub struct UQuitMatchCallbackProxy {
    __padding_end: [u8; 144],
}
impl UQuitMatchCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UQuitMatchCallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UQuitMatchCallbackProxy")
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
    pub fn quit_match(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        match_id: FString,
        outcome: crate::bindings::online_subsystem::EMPMatchOutcome,
        turn_timeout_in_seconds: i32,
    ) -> UPtr<UQuitMatchCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_quit_match_callback_proxy_quit_match,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &match_id,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &outcome,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::online_subsystem::EMPMatchOutcome>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &turn_timeout_in_seconds,
                __buffer.add(36).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UQuitMatchCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_quit_match_callback_proxy_quit_match,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<UQuitMatchCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UShowLoginUICallbackProxy {
    __padding_end: [u8; 120],
}
impl UShowLoginUICallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UShowLoginUICallbackProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UShowLoginUICallbackProxy")
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
    pub fn show_external_login_ui(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> UPtr<UShowLoginUICallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_show_login_ui_callback_proxy_show_external_login_ui,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UShowLoginUICallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_show_login_ui_callback_proxy_show_external_login_ui,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UShowLoginUICallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct ASpectatorBeaconClient {
    __padding_end: [u8; 1568],
}
impl ASpectatorBeaconClient {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASpectatorBeaconClient")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASpectatorBeaconClient")
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
pub struct ASpectatorBeaconHost {
    __padding_end: [u8; 1344],
}
impl ASpectatorBeaconHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASpectatorBeaconHost")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASpectatorBeaconHost")
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
pub struct USpectatorBeaconState {
    __padding_end: [u8; 104],
}
impl USpectatorBeaconState {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpectatorBeaconState")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpectatorBeaconState")
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
pub struct ATestBeaconClient {
    __padding_end: [u8; 1296],
}
impl ATestBeaconClient {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ATestBeaconClient")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ATestBeaconClient")
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
pub struct ATestBeaconHost {
    __padding_end: [u8; 1176],
}
impl ATestBeaconHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ATestBeaconHost")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ATestBeaconHost")
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
pub struct AOnlineBeaconUnitTestClient {
    __padding_end: [u8; 1296],
}
impl AOnlineBeaconUnitTestClient {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeaconUnitTestClient")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeaconUnitTestClient")
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
pub struct AOnlineBeaconUnitTestHost {
    __padding_end: [u8; 1464],
}
impl AOnlineBeaconUnitTestHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeaconUnitTestHost")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeaconUnitTestHost")
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
pub struct AOnlineBeaconUnitTestHostObject {
    __padding_end: [u8; 1176],
}
impl AOnlineBeaconUnitTestHostObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeaconUnitTestHostObject")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AOnlineBeaconUnitTestHostObject")
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
pub struct UOnlineBeaconUnitTestNetConnection {
    __padding_end: [u8; 8208],
}
impl UOnlineBeaconUnitTestNetConnection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlineBeaconUnitTestNetConnection")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlineBeaconUnitTestNetConnection")
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
pub struct UOnlineBeaconUnitTestNetDriver {
    __padding_end: [u8; 2840],
}
impl UOnlineBeaconUnitTestNetDriver {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlineBeaconUnitTestNetDriver")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnlineBeaconUnitTestNetDriver")
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
pub struct UTurnBasedBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UTurnBasedBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTurnBasedBlueprintLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTurnBasedBlueprintLibrary")
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
    pub fn register_turn_based_match_interface_object(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_turn_based_blueprint_library_register_turn_based_match_interface_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(16).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UTurnBasedBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_turn_based_blueprint_library_register_turn_based_match_interface_object,
                __buffer,
            )
        };
    }
    pub fn get_player_display_name(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        match_id: FString,
        player_index: i32,
        player_display_name: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_turn_based_blueprint_library_get_player_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &match_id,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_index,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                player_display_name,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UTurnBasedBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_turn_based_blueprint_library_get_player_display_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<FString>().swap(player_display_name);
        }
    }
    pub fn get_my_player_index(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        match_id: FString,
        player_index: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_turn_based_blueprint_library_get_my_player_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &match_id,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                player_index,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UTurnBasedBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_turn_based_blueprint_library_get_my_player_index,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<i32>().swap(player_index);
        }
    }
    pub fn get_is_my_turn(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        match_id: FString,
        b_is_my_turn: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_turn_based_blueprint_library_get_is_my_turn,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &match_id,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_my_turn,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::online_subsystem_utils::UTurnBasedBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_turn_based_blueprint_library_get_is_my_turn,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<bool>().swap(b_is_my_turn);
        }
    }
}
#[repr(C, align(16))]
pub struct UVoipListenerSynthComponent {
    __padding_end: [u8; 2480],
}
impl UVoipListenerSynthComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoipListenerSynthComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoipListenerSynthComponent")
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
    pub fn is_idling(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_voip_listener_synth_component_is_idling,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::online_subsystem_utils::__FUNCTION_PTRS
                    .u_voip_listener_synth_component_is_idling,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct FAchievementQueryCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAchievementQueryCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAchievementWriteCallbackProxy_OnWriteSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAchievementWriteCallbackProxy_OnWriteFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FConnectionCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FConnectionCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FCreateSessionCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FCreateSessionCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FDestroySessionCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FDestroySessionCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEndMatchCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEndMatchCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEndTurnCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEndTurnCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFindSessionsCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFindSessionsCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFindTurnBasedMatchCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFindTurnBasedMatchCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInAppPurchaseCallbackProxy2_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInAppPurchaseCallbackProxy2_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInAppPurchaseCheckoutCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInAppPurchaseCheckoutCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInAppPurchaseQueryCallbackProxy2_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInAppPurchaseQueryCallbackProxy2_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInAppPurchaseReceiptsCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInAppPurchaseReceiptsCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInAppPurchaseRestoreCallbackProxy2_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInAppPurchaseRestoreCallbackProxy2_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FJoinSessionCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FJoinSessionCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLeaderboardFlushCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLeaderboardFlushCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLeaderboardQueryCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLeaderboardQueryCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLogoutCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLogoutCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FQuitMatchCallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FQuitMatchCallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FShowLoginUICallbackProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FShowLoginUICallbackProxy_OnFailure {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EOnlineProxyStoreOfferDiscountType(pub u8);
impl EOnlineProxyStoreOfferDiscountType {
    pub const NOT_ON_SALE: EOnlineProxyStoreOfferDiscountType = EOnlineProxyStoreOfferDiscountType(
        0,
    );
    pub const PERCENTAGE: EOnlineProxyStoreOfferDiscountType = EOnlineProxyStoreOfferDiscountType(
        1,
    );
    pub const DISCOUNT_AMOUNT: EOnlineProxyStoreOfferDiscountType = EOnlineProxyStoreOfferDiscountType(
        2,
    );
    pub const PAY_AMOUNT: EOnlineProxyStoreOfferDiscountType = EOnlineProxyStoreOfferDiscountType(
        3,
    );
}
#[repr(transparent)]
pub struct EPartyReservationResult(pub u8);
impl EPartyReservationResult {
    pub const NO_RESULT: EPartyReservationResult = EPartyReservationResult(0);
    pub const REQUEST_PENDING: EPartyReservationResult = EPartyReservationResult(1);
    pub const GENERAL_ERROR: EPartyReservationResult = EPartyReservationResult(2);
    pub const PARTY_LIMIT_REACHED: EPartyReservationResult = EPartyReservationResult(3);
    pub const INCORRECT_PLAYER_COUNT: EPartyReservationResult = EPartyReservationResult(
        4,
    );
    pub const REQUEST_TIMED_OUT: EPartyReservationResult = EPartyReservationResult(5);
    pub const RESERVATION_DUPLICATE: EPartyReservationResult = EPartyReservationResult(
        6,
    );
    pub const RESERVATION_NOT_FOUND: EPartyReservationResult = EPartyReservationResult(
        7,
    );
    pub const RESERVATION_ACCEPTED: EPartyReservationResult = EPartyReservationResult(8);
    pub const RESERVATION_DENIED: EPartyReservationResult = EPartyReservationResult(9);
    pub const RESERVATION_DENIED_CROSS_PLAY_RESTRICTION: EPartyReservationResult = EPartyReservationResult(
        10,
    );
    pub const RESERVATION_DENIED_BANNED: EPartyReservationResult = EPartyReservationResult(
        11,
    );
    pub const RESERVATION_REQUEST_CANCELED: EPartyReservationResult = EPartyReservationResult(
        12,
    );
    pub const RESERVATION_INVALID: EPartyReservationResult = EPartyReservationResult(13);
    pub const BAD_SESSION_ID: EPartyReservationResult = EPartyReservationResult(14);
    pub const RESERVATION_DENIED_CONTAINS_EXISTING_PLAYERS: EPartyReservationResult = EPartyReservationResult(
        15,
    );
    pub const RESERVATION_DENIED_VALIDATION_FAILED: EPartyReservationResult = EPartyReservationResult(
        16,
    );
}
#[repr(transparent)]
pub struct ESpectatorReservationResult(pub u8);
impl ESpectatorReservationResult {
    pub const NO_RESULT: ESpectatorReservationResult = ESpectatorReservationResult(0);
    pub const REQUEST_PENDING: ESpectatorReservationResult = ESpectatorReservationResult(
        1,
    );
    pub const GENERAL_ERROR: ESpectatorReservationResult = ESpectatorReservationResult(
        2,
    );
    pub const SPECTATOR_LIMIT_REACHED: ESpectatorReservationResult = ESpectatorReservationResult(
        3,
    );
    pub const INCORRECT_PLAYER_COUNT: ESpectatorReservationResult = ESpectatorReservationResult(
        4,
    );
    pub const REQUEST_TIMED_OUT: ESpectatorReservationResult = ESpectatorReservationResult(
        5,
    );
    pub const RESERVATION_DUPLICATE: ESpectatorReservationResult = ESpectatorReservationResult(
        6,
    );
    pub const RESERVATION_NOT_FOUND: ESpectatorReservationResult = ESpectatorReservationResult(
        7,
    );
    pub const RESERVATION_ACCEPTED: ESpectatorReservationResult = ESpectatorReservationResult(
        8,
    );
    pub const RESERVATION_DENIED: ESpectatorReservationResult = ESpectatorReservationResult(
        9,
    );
    pub const RESERVATION_DENIED_CROSS_PLAY_RESTRICTION: ESpectatorReservationResult = ESpectatorReservationResult(
        10,
    );
    pub const RESERVATION_DENIED_BANNED: ESpectatorReservationResult = ESpectatorReservationResult(
        11,
    );
    pub const RESERVATION_REQUEST_CANCELED: ESpectatorReservationResult = ESpectatorReservationResult(
        12,
    );
    pub const RESERVATION_INVALID: ESpectatorReservationResult = ESpectatorReservationResult(
        13,
    );
    pub const BAD_SESSION_ID: ESpectatorReservationResult = ESpectatorReservationResult(
        14,
    );
    pub const RESERVATION_DENIED_CONTAINS_EXISTING_PLAYERS: ESpectatorReservationResult = ESpectatorReservationResult(
        15,
    );
}
#[repr(transparent)]
pub struct EInAppPurchaseStatus(pub u8);
impl EInAppPurchaseStatus {
    pub const INVALID: EInAppPurchaseStatus = EInAppPurchaseStatus(0);
    pub const FAILED: EInAppPurchaseStatus = EInAppPurchaseStatus(1);
    pub const DEFERRED: EInAppPurchaseStatus = EInAppPurchaseStatus(2);
    pub const CANCELED: EInAppPurchaseStatus = EInAppPurchaseStatus(3);
    pub const PURCHASED: EInAppPurchaseStatus = EInAppPurchaseStatus(4);
    pub const RESTORED: EInAppPurchaseStatus = EInAppPurchaseStatus(5);
}
#[repr(transparent)]
pub struct EBeaconConnectionState(pub u8);
impl EBeaconConnectionState {
    pub const INVALID: EBeaconConnectionState = EBeaconConnectionState(0);
    pub const CLOSED: EBeaconConnectionState = EBeaconConnectionState(1);
    pub const PENDING: EBeaconConnectionState = EBeaconConnectionState(2);
    pub const OPEN: EBeaconConnectionState = EBeaconConnectionState(3);
}
#[repr(transparent)]
pub struct EClientRequestType(pub u8);
impl EClientRequestType {
    pub const NONE_PENDING: EClientRequestType = EClientRequestType(0);
    pub const EXISTING_SESSION_RESERVATION: EClientRequestType = EClientRequestType(1);
    pub const RESERVATION_UPDATE: EClientRequestType = EClientRequestType(2);
    pub const EMPTY_SERVER_RESERVATION: EClientRequestType = EClientRequestType(3);
    pub const RECONNECT: EClientRequestType = EClientRequestType(4);
    pub const ABANDON: EClientRequestType = EClientRequestType(5);
    pub const RESERVATION_REMOVE_MEMBERS: EClientRequestType = EClientRequestType(6);
    pub const ADD_OR_UPDATE_RESERVATION: EClientRequestType = EClientRequestType(7);
}
#[repr(transparent)]
pub struct ESpectatorClientRequestType(pub u8);
impl ESpectatorClientRequestType {
    pub const NONE_PENDING: ESpectatorClientRequestType = ESpectatorClientRequestType(0);
    pub const EXISTING_SESSION_RESERVATION: ESpectatorClientRequestType = ESpectatorClientRequestType(
        1,
    );
    pub const RESERVATION_UPDATE: ESpectatorClientRequestType = ESpectatorClientRequestType(
        2,
    );
    pub const EMPTY_SERVER_RESERVATION: ESpectatorClientRequestType = ESpectatorClientRequestType(
        3,
    );
    pub const RECONNECT: ESpectatorClientRequestType = ESpectatorClientRequestType(4);
    pub const ABANDON: ESpectatorClientRequestType = ESpectatorClientRequestType(5);
}
