#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FDocumentationRedirect {
    pub from: FString,
    pub to_url: FDocumentationUrl,
}
#[repr(C, align(8))]
pub struct FDocumentationUrl {
    pub link: FString,
    pub base_url_id: FString,
}
#[repr(C, align(8))]
pub struct FDocumentationBaseUrl {
    pub id: FString,
    pub url: FString,
}
pub struct UDocumentationSettings {
    pub documentation_base_urls: TArray<FDocumentationBaseUrl>,
    pub documentation_redirects: TArray<FDocumentationRedirect>,
}
