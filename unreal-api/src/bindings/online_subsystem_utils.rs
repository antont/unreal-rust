#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FBlueprintSessionResult {
    __padding_end: [u8; 288],
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
    __padding_end: [u8; 7],
}
impl FInAppPurchaseProductRequest2 {}
#[repr(C, align(8))]
pub struct UAchievementBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UAchievementBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UAchievementQueryCallbackProxy {
    __padding_end: [u8; 128],
}
impl UAchievementQueryCallbackProxy {}
#[repr(C, align(8))]
pub struct UAchievementWriteCallbackProxy {
    __padding_end: [u8; 160],
}
impl UAchievementWriteCallbackProxy {}
#[repr(C, align(8))]
pub struct UConnectionCallbackProxy {
    __padding_end: [u8; 152],
}
impl UConnectionCallbackProxy {}
#[repr(C, align(8))]
pub struct UCreateSessionCallbackProxy {
    __padding_end: [u8; 192],
}
impl UCreateSessionCallbackProxy {}
#[repr(C, align(8))]
pub struct UDestroySessionCallbackProxy {
    __padding_end: [u8; 152],
}
impl UDestroySessionCallbackProxy {}
#[repr(C, align(8))]
pub struct UEndMatchCallbackProxy {
    __padding_end: [u8; 152],
}
impl UEndMatchCallbackProxy {}
#[repr(C, align(8))]
pub struct UEndTurnCallbackProxy {
    __padding_end: [u8; 144],
}
impl UEndTurnCallbackProxy {}
#[repr(C, align(8))]
pub struct UFindSessionsCallbackProxy {
    __padding_end: [u8; 176],
}
impl UFindSessionsCallbackProxy {}
#[repr(C, align(8))]
pub struct UFindTurnBasedMatchCallbackProxy {
    __padding_end: [u8; 160],
}
impl UFindTurnBasedMatchCallbackProxy {}
#[repr(C, align(8))]
pub struct UInAppPurchaseCallbackProxy2 {
    __padding_end: [u8; 200],
}
impl UInAppPurchaseCallbackProxy2 {}
#[repr(C, align(8))]
pub struct UInAppPurchaseCheckoutCallbackProxy {
    __padding_end: [u8; 184],
}
impl UInAppPurchaseCheckoutCallbackProxy {}
#[repr(C, align(8))]
pub struct UInAppPurchaseFinalizeProxy {
    __padding_end: [u8; 48],
}
impl UInAppPurchaseFinalizeProxy {}
#[repr(C, align(8))]
pub struct UInAppPurchaseQueryCallbackProxy2 {
    __padding_end: [u8; 128],
}
impl UInAppPurchaseQueryCallbackProxy2 {}
#[repr(C, align(8))]
pub struct UInAppPurchaseReceiptsCallbackProxy {
    __padding_end: [u8; 168],
}
impl UInAppPurchaseReceiptsCallbackProxy {}
#[repr(C, align(8))]
pub struct UInAppPurchaseRestoreCallbackProxy2 {
    __padding_end: [u8; 200],
}
impl UInAppPurchaseRestoreCallbackProxy2 {}
#[repr(C, align(16))]
pub struct UIpConnection {
    __padding_end: [u8; 8208],
}
impl UIpConnection {}
#[repr(C, align(8))]
pub struct UIpNetDriver {
    __padding_end: [u8; 2840],
}
impl UIpNetDriver {}
#[repr(C, align(8))]
pub struct UJoinSessionCallbackProxy {
    __padding_end: [u8; 440],
}
impl UJoinSessionCallbackProxy {}
#[repr(C, align(8))]
pub struct ULeaderboardBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl ULeaderboardBlueprintLibrary {}
#[repr(C, align(8))]
pub struct ULeaderboardFlushCallbackProxy {
    __padding_end: [u8; 136],
}
impl ULeaderboardFlushCallbackProxy {}
#[repr(C, align(8))]
pub struct ULeaderboardQueryCallbackProxy {
    __padding_end: [u8; 192],
}
impl ULeaderboardQueryCallbackProxy {}
#[repr(C, align(8))]
pub struct ULogoutCallbackProxy {
    __padding_end: [u8; 128],
}
impl ULogoutCallbackProxy {}
#[repr(C, align(8))]
pub struct AOnlineBeacon {
    __padding_end: [u8; 1192],
}
impl AOnlineBeacon {}
#[repr(C, align(8))]
pub struct AOnlineBeaconClient {
    __padding_end: [u8; 1296],
}
impl AOnlineBeaconClient {}
#[repr(C, align(8))]
pub struct AOnlineBeaconHost {
    __padding_end: [u8; 1464],
}
impl AOnlineBeaconHost {}
#[repr(C, align(8))]
pub struct AOnlineBeaconHostObject {
    __padding_end: [u8; 1176],
}
impl AOnlineBeaconHostObject {}
#[repr(C, align(8))]
pub struct UOnlineEngineInterfaceImpl {
    __padding_end: [u8; 424],
}
impl UOnlineEngineInterfaceImpl {}
#[repr(C, align(8))]
pub struct UOnlinePIEConfig {
    __padding_end: [u8; 64],
}
impl UOnlinePIEConfig {}
#[repr(C, align(8))]
pub struct UOnlinePIESettings {
    __padding_end: [u8; 128],
}
impl UOnlinePIESettings {}
#[repr(C, align(8))]
pub struct UOnlineServicesEngineInterfaceImpl {
    __padding_end: [u8; 48],
}
impl UOnlineServicesEngineInterfaceImpl {}
#[repr(C, align(8))]
pub struct UOnlineSessionClient {
    __padding_end: [u8; 528],
}
impl UOnlineSessionClient {}
#[repr(C, align(8))]
pub struct APartyBeaconClient {
    __padding_end: [u8; 1520],
}
impl APartyBeaconClient {}
#[repr(C, align(8))]
pub struct APartyBeaconHost {
    __padding_end: [u8; 1344],
}
impl APartyBeaconHost {}
#[repr(C, align(8))]
pub struct UPartyBeaconState {
    __padding_end: [u8; 176],
}
impl UPartyBeaconState {}
#[repr(C, align(8))]
pub struct UQuitMatchCallbackProxy {
    __padding_end: [u8; 144],
}
impl UQuitMatchCallbackProxy {}
#[repr(C, align(8))]
pub struct UShowLoginUICallbackProxy {
    __padding_end: [u8; 120],
}
impl UShowLoginUICallbackProxy {}
#[repr(C, align(8))]
pub struct ASpectatorBeaconClient {
    __padding_end: [u8; 1568],
}
impl ASpectatorBeaconClient {}
#[repr(C, align(8))]
pub struct ASpectatorBeaconHost {
    __padding_end: [u8; 1344],
}
impl ASpectatorBeaconHost {}
#[repr(C, align(8))]
pub struct USpectatorBeaconState {
    __padding_end: [u8; 104],
}
impl USpectatorBeaconState {}
#[repr(C, align(8))]
pub struct ATestBeaconClient {
    __padding_end: [u8; 1296],
}
impl ATestBeaconClient {}
#[repr(C, align(8))]
pub struct ATestBeaconHost {
    __padding_end: [u8; 1176],
}
impl ATestBeaconHost {}
#[repr(C, align(8))]
pub struct AOnlineBeaconUnitTestClient {
    __padding_end: [u8; 1296],
}
impl AOnlineBeaconUnitTestClient {}
#[repr(C, align(8))]
pub struct AOnlineBeaconUnitTestHost {
    __padding_end: [u8; 1464],
}
impl AOnlineBeaconUnitTestHost {}
#[repr(C, align(8))]
pub struct AOnlineBeaconUnitTestHostObject {
    __padding_end: [u8; 1176],
}
impl AOnlineBeaconUnitTestHostObject {}
#[repr(C, align(16))]
pub struct UOnlineBeaconUnitTestNetConnection {
    __padding_end: [u8; 8208],
}
impl UOnlineBeaconUnitTestNetConnection {}
#[repr(C, align(8))]
pub struct UOnlineBeaconUnitTestNetDriver {
    __padding_end: [u8; 2840],
}
impl UOnlineBeaconUnitTestNetDriver {}
#[repr(C, align(8))]
pub struct UTurnBasedBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UTurnBasedBlueprintLibrary {}
#[repr(C, align(16))]
pub struct UVoipListenerSynthComponent {
    __padding_end: [u8; 2480],
}
impl UVoipListenerSynthComponent {}
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
