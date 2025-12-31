#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FBlueprintSessionResult {}
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
#[repr(C, align(8))]
pub struct FInAppPurchaseRestoreInfo2 {
    pub item_name: FString,
    pub item_id: FString,
    pub validation_info: FString,
}
#[repr(C, align(8))]
pub struct FInAppPurchaseReceiptInfo2 {
    pub item_name: FString,
    pub item_id: FString,
    pub validation_info: FString,
    pub transaction_identifier: FString,
}
#[repr(C, align(8))]
pub struct FOnlineAccountStoredCredentials {
    pub id: FString,
    pub token: FString,
    pub ty: FString,
    pub token_bytes: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FPlayerReservation {
    pub unique_id: crate::bindings::engine::FUniqueNetIdRepl,
    pub validation_str: FString,
    pub platform: FString,
    pub b_allow_crossplay: bool,
    pub elapsed_time: f32,
}
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
#[repr(C, align(8))]
pub struct FInAppPurchaseProductRequest2 {
    pub product_identifier: FString,
    pub b_is_consumable: bool,
}
#[repr(C, align(8))]
pub struct FPartyReservation {
    pub team_num: i32,
    pub party_leader: crate::bindings::engine::FUniqueNetIdRepl,
    pub party_members: TArray<FPlayerReservation>,
    pub removed_party_members: TArray<FPlayerReservation>,
}
#[repr(C, align(8))]
pub struct FPartyBeaconCrossplayPlatformMapping {
    pub platform_name: FString,
    pub platform_type: FString,
}
#[repr(C, align(8))]
pub struct FSpectatorReservation {
    pub spectator_id: crate::bindings::engine::FUniqueNetIdRepl,
    pub spectator: FPlayerReservation,
}
pub struct UAchievementBlueprintLibrary {}
pub struct UAchievementQueryCallbackProxy {
    pub on_success: FAchievementQueryCallbackProxy_OnSuccess,
    pub on_failure: FAchievementQueryCallbackProxy_OnFailure,
}
pub struct UAchievementWriteCallbackProxy {
    pub on_write_success: FAchievementWriteCallbackProxy_OnWriteSuccess,
    pub on_write_failure: FAchievementWriteCallbackProxy_OnWriteFailure,
}
pub struct UConnectionCallbackProxy {
    pub on_success: FConnectionCallbackProxy_OnSuccess,
    pub on_failure: FConnectionCallbackProxy_OnFailure,
}
pub struct UCreateSessionCallbackProxy {
    pub on_success: FCreateSessionCallbackProxy_OnSuccess,
    pub on_failure: FCreateSessionCallbackProxy_OnFailure,
}
pub struct UDestroySessionCallbackProxy {
    pub on_success: FDestroySessionCallbackProxy_OnSuccess,
    pub on_failure: FDestroySessionCallbackProxy_OnFailure,
}
pub struct UEndMatchCallbackProxy {
    pub on_success: FEndMatchCallbackProxy_OnSuccess,
    pub on_failure: FEndMatchCallbackProxy_OnFailure,
}
pub struct UEndTurnCallbackProxy {
    pub on_success: FEndTurnCallbackProxy_OnSuccess,
    pub on_failure: FEndTurnCallbackProxy_OnFailure,
}
pub struct UFindSessionsCallbackProxy {
    pub on_success: FFindSessionsCallbackProxy_OnSuccess,
    pub on_failure: FFindSessionsCallbackProxy_OnFailure,
}
pub struct UFindTurnBasedMatchCallbackProxy {
    pub on_success: FFindTurnBasedMatchCallbackProxy_OnSuccess,
    pub on_failure: FFindTurnBasedMatchCallbackProxy_OnFailure,
}
pub struct UInAppPurchaseCallbackProxy2 {
    pub on_success: FInAppPurchaseCallbackProxy2_OnSuccess,
    pub on_failure: FInAppPurchaseCallbackProxy2_OnFailure,
}
pub struct UInAppPurchaseCheckoutCallbackProxy {
    pub on_success: FInAppPurchaseCheckoutCallbackProxy_OnSuccess,
    pub on_failure: FInAppPurchaseCheckoutCallbackProxy_OnFailure,
}
pub struct UInAppPurchaseFinalizeProxy {}
pub struct UInAppPurchaseQueryCallbackProxy2 {
    pub on_success: FInAppPurchaseQueryCallbackProxy2_OnSuccess,
    pub on_failure: FInAppPurchaseQueryCallbackProxy2_OnFailure,
}
pub struct UInAppPurchaseReceiptsCallbackProxy {
    pub on_success: FInAppPurchaseReceiptsCallbackProxy_OnSuccess,
    pub on_failure: FInAppPurchaseReceiptsCallbackProxy_OnFailure,
}
pub struct UInAppPurchaseRestoreCallbackProxy2 {
    pub on_success: FInAppPurchaseRestoreCallbackProxy2_OnSuccess,
    pub on_failure: FInAppPurchaseRestoreCallbackProxy2_OnFailure,
}
pub struct UIpConnection {
    pub socket_error_disconnect_delay: f32,
}
pub struct UIpNetDriver {
    pub flags_2664: u8,
    pub max_port_count_to_try: u32,
    pub server_desired_socket_receive_buffer_bytes: u32,
    pub server_desired_socket_send_buffer_bytes: u32,
    pub client_desired_socket_receive_buffer_bytes: u32,
    pub client_desired_socket_send_buffer_bytes: u32,
    pub max_seconds_in_receive: f64,
    pub nb_packets_between_receive_time_test: i32,
    pub resolution_connection_timeout: f32,
}
pub struct UJoinSessionCallbackProxy {
    pub on_success: FJoinSessionCallbackProxy_OnSuccess,
    pub on_failure: FJoinSessionCallbackProxy_OnFailure,
}
pub struct ULeaderboardBlueprintLibrary {}
pub struct ULeaderboardFlushCallbackProxy {
    pub on_success: FLeaderboardFlushCallbackProxy_OnSuccess,
    pub on_failure: FLeaderboardFlushCallbackProxy_OnFailure,
}
pub struct ULeaderboardQueryCallbackProxy {
    pub on_success: FLeaderboardQueryCallbackProxy_OnSuccess,
    pub on_failure: FLeaderboardQueryCallbackProxy_OnFailure,
}
pub struct ULogoutCallbackProxy {
    pub on_success: FLogoutCallbackProxy_OnSuccess,
    pub on_failure: FLogoutCallbackProxy_OnFailure,
}
pub struct AOnlineBeacon {
    pub beacon_connection_initial_timeout: f32,
    pub beacon_connection_timeout: f32,
    pub net_driver: UPtr<crate::bindings::engine::UNetDriver>,
}
pub struct AOnlineBeaconClient {
    pub beacon_owner: UPtr<AOnlineBeaconHostObject>,
    pub beacon_connection: UPtr<crate::bindings::engine::UNetConnection>,
    pub connection_state: EBeaconConnectionState,
}
pub struct AOnlineBeaconHost {
    pub listen_port: i32,
    pub b_reuse_address_and_port: bool,
    pub b_auth_required: bool,
    pub max_auth_token_size: u32,
    pub client_actors: TArray<UPtr<AOnlineBeaconClient>>,
}
pub struct AOnlineBeaconHostObject {
    pub beacon_type_name: FString,
    pub client_beacon_actor_class: TSubclassOf<AOnlineBeaconClient>,
    pub client_actors: TArray<UPtr<AOnlineBeaconClient>>,
}
pub struct UOnlineEngineInterfaceImpl {
    pub mapped_unique_net_id_types: TMap<FName, FName>,
    pub compatible_unique_net_id_types: TArray<FName>,
    pub voice_subsystem_name_override: FName,
    pub b_online_services_compatibility_enabled: bool,
    pub online_services_compatibility_interface: UPtr<
        crate::bindings::engine::UOnlineEngineInterface,
    >,
}
pub struct UOnlinePIEConfig {
    pub login_types_allowing_duplicates: TArray<FString>,
}
pub struct UOnlinePIESettings {
    pub b_online_pie_enabled: bool,
    pub logins: TArray<FOnlineAccountStoredCredentials>,
}
pub struct UOnlineServicesEngineInterfaceImpl {}
pub struct UOnlineSessionClient {
    pub b_is_from_invite: bool,
    pub b_handling_disconnect: bool,
}
pub struct APartyBeaconClient {
    pub dest_session_id: FString,
    pub pending_reservation: FPartyReservation,
    pub request_type: EClientRequestType,
    pub b_pending_reservation_sent: bool,
    pub b_cancel_reservation: bool,
}
pub struct APartyBeaconHost {
    pub state: UPtr<UPartyBeaconState>,
    pub b_logout_on_session_timeout: bool,
    pub b_is_validation_str_required: bool,
    pub session_timeout_secs: f32,
    pub travel_session_timeout_secs: f32,
}
pub struct UPartyBeaconState {
    pub session_name: FName,
    pub num_consumed_reservations: i32,
    pub max_reservations: i32,
    pub num_teams: i32,
    pub num_players_per_team: i32,
    pub team_assignment_method: FName,
    pub reserved_host_team_num: i32,
    pub force_team_num: i32,
    pub b_restrict_cross_console: bool,
    pub platform_crossplay_restrictions: TArray<FString>,
    pub platform_type_mapping: TArray<FPartyBeaconCrossplayPlatformMapping>,
    pub b_enable_removal_requests: bool,
    pub b_respect_competitive_integrity: bool,
    pub reservations: TArray<FPartyReservation>,
}
pub struct UQuitMatchCallbackProxy {
    pub on_success: FQuitMatchCallbackProxy_OnSuccess,
    pub on_failure: FQuitMatchCallbackProxy_OnFailure,
}
pub struct UShowLoginUICallbackProxy {
    pub on_success: FShowLoginUICallbackProxy_OnSuccess,
    pub on_failure: FShowLoginUICallbackProxy_OnFailure,
}
pub struct ASpectatorBeaconClient {
    pub dest_session_id: FString,
    pub pending_reservation: FSpectatorReservation,
    pub request_type: ESpectatorClientRequestType,
    pub b_pending_reservation_sent: bool,
    pub b_cancel_reservation: bool,
}
pub struct ASpectatorBeaconHost {
    pub state: UPtr<USpectatorBeaconState>,
    pub b_logout_on_session_timeout: bool,
    pub b_is_validation_str_required: bool,
    pub session_timeout_secs: f32,
    pub travel_session_timeout_secs: f32,
}
pub struct USpectatorBeaconState {
    pub session_name: FName,
    pub num_consumed_reservations: i32,
    pub max_reservations: i32,
    pub b_restrict_cross_console: bool,
    pub reservations: TArray<FSpectatorReservation>,
}
pub struct ATestBeaconClient {}
pub struct ATestBeaconHost {}
pub struct AOnlineBeaconUnitTestClient {}
pub struct AOnlineBeaconUnitTestHost {}
pub struct AOnlineBeaconUnitTestHostObject {}
pub struct UOnlineBeaconUnitTestNetConnection {}
pub struct UOnlineBeaconUnitTestNetDriver {}
pub struct UTurnBasedBlueprintLibrary {}
pub struct UVoipListenerSynthComponent {}
pub struct FAchievementQueryCallbackProxy_OnSuccess;
pub struct FAchievementQueryCallbackProxy_OnFailure;
pub struct FAchievementWriteCallbackProxy_OnWriteSuccess;
pub struct FAchievementWriteCallbackProxy_OnWriteFailure;
pub struct FConnectionCallbackProxy_OnSuccess;
pub struct FConnectionCallbackProxy_OnFailure;
pub struct FCreateSessionCallbackProxy_OnSuccess;
pub struct FCreateSessionCallbackProxy_OnFailure;
pub struct FDestroySessionCallbackProxy_OnSuccess;
pub struct FDestroySessionCallbackProxy_OnFailure;
pub struct FEndMatchCallbackProxy_OnSuccess;
pub struct FEndMatchCallbackProxy_OnFailure;
pub struct FEndTurnCallbackProxy_OnSuccess;
pub struct FEndTurnCallbackProxy_OnFailure;
pub struct FFindSessionsCallbackProxy_OnSuccess;
pub struct FFindSessionsCallbackProxy_OnFailure;
pub struct FFindTurnBasedMatchCallbackProxy_OnSuccess;
pub struct FFindTurnBasedMatchCallbackProxy_OnFailure;
pub struct FInAppPurchaseCallbackProxy2_OnSuccess;
pub struct FInAppPurchaseCallbackProxy2_OnFailure;
pub struct FInAppPurchaseCheckoutCallbackProxy_OnSuccess;
pub struct FInAppPurchaseCheckoutCallbackProxy_OnFailure;
pub struct FInAppPurchaseQueryCallbackProxy2_OnSuccess;
pub struct FInAppPurchaseQueryCallbackProxy2_OnFailure;
pub struct FInAppPurchaseReceiptsCallbackProxy_OnSuccess;
pub struct FInAppPurchaseReceiptsCallbackProxy_OnFailure;
pub struct FInAppPurchaseRestoreCallbackProxy2_OnSuccess;
pub struct FInAppPurchaseRestoreCallbackProxy2_OnFailure;
pub struct FJoinSessionCallbackProxy_OnSuccess;
pub struct FJoinSessionCallbackProxy_OnFailure;
pub struct FLeaderboardFlushCallbackProxy_OnSuccess;
pub struct FLeaderboardFlushCallbackProxy_OnFailure;
pub struct FLeaderboardQueryCallbackProxy_OnSuccess;
pub struct FLeaderboardQueryCallbackProxy_OnFailure;
pub struct FLogoutCallbackProxy_OnSuccess;
pub struct FLogoutCallbackProxy_OnFailure;
pub struct FQuitMatchCallbackProxy_OnSuccess;
pub struct FQuitMatchCallbackProxy_OnFailure;
pub struct FShowLoginUICallbackProxy_OnSuccess;
pub struct FShowLoginUICallbackProxy_OnFailure;
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBeaconConnectionState(pub u8);
impl EBeaconConnectionState {
    pub const INVALID: EBeaconConnectionState = EBeaconConnectionState(0);
    pub const CLOSED: EBeaconConnectionState = EBeaconConnectionState(1);
    pub const PENDING: EBeaconConnectionState = EBeaconConnectionState(2);
    pub const OPEN: EBeaconConnectionState = EBeaconConnectionState(3);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
