// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use std::fmt;

bitflags! {
    #[doc(alias = "WebKitHitTestResultContext")]
    pub struct HitTestResultContext: u32 {
        #[doc(alias = "WEBKIT_HIT_TEST_RESULT_CONTEXT_DOCUMENT")]
        const DOCUMENT = ffi::WEBKIT_HIT_TEST_RESULT_CONTEXT_DOCUMENT as u32;
        #[doc(alias = "WEBKIT_HIT_TEST_RESULT_CONTEXT_LINK")]
        const LINK = ffi::WEBKIT_HIT_TEST_RESULT_CONTEXT_LINK as u32;
        #[doc(alias = "WEBKIT_HIT_TEST_RESULT_CONTEXT_IMAGE")]
        const IMAGE = ffi::WEBKIT_HIT_TEST_RESULT_CONTEXT_IMAGE as u32;
        #[doc(alias = "WEBKIT_HIT_TEST_RESULT_CONTEXT_MEDIA")]
        const MEDIA = ffi::WEBKIT_HIT_TEST_RESULT_CONTEXT_MEDIA as u32;
        #[doc(alias = "WEBKIT_HIT_TEST_RESULT_CONTEXT_EDITABLE")]
        const EDITABLE = ffi::WEBKIT_HIT_TEST_RESULT_CONTEXT_EDITABLE as u32;
        #[doc(alias = "WEBKIT_HIT_TEST_RESULT_CONTEXT_SCROLLBAR")]
        const SCROLLBAR = ffi::WEBKIT_HIT_TEST_RESULT_CONTEXT_SCROLLBAR as u32;
        #[doc(alias = "WEBKIT_HIT_TEST_RESULT_CONTEXT_SELECTION")]
        const SELECTION = ffi::WEBKIT_HIT_TEST_RESULT_CONTEXT_SELECTION as u32;
    }
}

impl fmt::Display for HitTestResultContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for HitTestResultContext {
    type GlibType = ffi::WebKitHitTestResultContext;

    fn into_glib(self) -> ffi::WebKitHitTestResultContext {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WebKitHitTestResultContext> for HitTestResultContext {
    unsafe fn from_glib(value: ffi::WebKitHitTestResultContext) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}