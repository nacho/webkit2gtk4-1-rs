#![doc = include_str!("../../README.md")]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use gio;
pub use glib;

#[macro_use]
mod rt;

#[allow(unused_imports)]
#[allow(unused_mut)]
#[allow(clippy::clone_on_copy)]
mod auto;
mod credential;
mod javascript_result;
mod web_context;
mod web_view;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
mod website_data_manager;

pub use crate::auto::*;
pub use credential::Credential;
pub use javascript_result::JavascriptResult;

pub mod prelude {
    pub use super::auto::traits::*;
    pub use super::web_context::WebContextExtManual;
    pub use super::web_view::WebViewExtManual;
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    pub use super::website_data_manager::WebsiteDataManagerExtManual;
}

pub mod builders {
    pub use super::auto::builders::*;
}
