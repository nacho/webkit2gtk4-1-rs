// // Copyright 2013-2017, The Gtk-rs Project Developers.
// // See the COPYRIGHT file at the top-level directory of this distribution.
// // Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use ffi;
pub use glib;

#[allow(unused_imports)]
mod auto;
mod global_context_ref;
mod string_ref;
mod value_ref;

pub use auto::*;
pub use global_context_ref::*;
pub use string_ref::*;
pub use value_ref::*;

pub mod prelude {
    pub use super::auto::traits::*;
}

pub mod builders {
    pub use super::auto::builders::*;
}
