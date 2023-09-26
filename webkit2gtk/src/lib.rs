#![cfg_attr(docsrs, feature(doc_cfg))]

pub use cairo;
pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use gtk;
pub use javascriptcore;

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
#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
mod website_data_manager;

pub use crate::auto::*;
pub use credential::Credential;
pub use javascript_result::JavascriptResult;

pub mod prelude {
    pub use super::auto::traits::*;
    pub use super::web_context::WebContextExtManual;
    pub use super::web_view::WebViewExtManual;
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    pub use super::website_data_manager::WebsiteDataManagerExtManual;
    pub use javascriptcore::prelude::*;
}

pub mod builders {
    pub use super::auto::builders::*;
}
