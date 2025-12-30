#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FCryptoEncryptionKey {
    pub guid: FGuid,
    pub name: FString,
    pub key: FString,
}
pub struct UCryptoKeysCommandlet {}
pub struct UCryptoKeysSettings {
    pub encryption_key: FString,
    pub secondary_encryption_keys: TArray<FCryptoEncryptionKey>,
    pub b_encrypt_pak_ini_files: bool,
    pub b_encrypt_pak_index: bool,
    pub b_encrypt_u_asset_files: bool,
    pub b_encrypt_all_asset_files: bool,
    pub signing_public_exponent: FString,
    pub signing_modulus: FString,
    pub signing_private_exponent: FString,
    pub b_enable_pak_signing: bool,
}
