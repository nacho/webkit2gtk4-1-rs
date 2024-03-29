// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::DOMObject;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
use std::ptr;
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitDOMDOMTokenList")]
    pub struct DOMDOMTokenList(Object<ffi::WebKitDOMDOMTokenList, ffi::WebKitDOMDOMTokenListClass>) @extends DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_dom_token_list_get_type(),
    }
}

impl DOMDOMTokenList {
    pub const NONE: Option<&'static DOMDOMTokenList> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DOMDOMTokenList>> Sealed for T {}
}

pub trait DOMDOMTokenListExt: IsA<DOMDOMTokenList> + sealed::Sealed + 'static {
    //#[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    //#[cfg(feature = "v2_16")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    //#[allow(deprecated)]
    //#[doc(alias = "webkit_dom_dom_token_list_add")]
    //fn add(&self, error: &mut glib::Error, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:webkit_dom_dom_token_list_add() }
    //}

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_token_list_contains")]
    fn contains(&self, token: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_dom_token_list_contains(
                self.as_ref().to_glib_none().0,
                token.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_token_list_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_dom_token_list_get_length(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_token_list_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_dom_token_list_get_value(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_token_list_item")]
    fn item(&self, index: libc::c_ulong) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_dom_token_list_item(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    //#[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    //#[cfg(feature = "v2_16")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    //#[allow(deprecated)]
    //#[doc(alias = "webkit_dom_dom_token_list_remove")]
    //fn remove(&self, error: &mut glib::Error, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:webkit_dom_dom_token_list_remove() }
    //}

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_token_list_replace")]
    fn replace(&self, token: &str, newToken: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_dom_token_list_replace(
                self.as_ref().to_glib_none().0,
                token.to_glib_none().0,
                newToken.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_token_list_set_value")]
    fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_dom_token_list_set_value(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_token_list_toggle")]
    fn toggle(&self, token: &str, force: bool) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::webkit_dom_dom_token_list_toggle(
                self.as_ref().to_glib_none().0,
                token.to_glib_none().0,
                force.into_glib(),
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_property_length(&self) -> libc::c_ulong {
        ObjectExt::property(self.as_ref(), "length")
    }

    fn get_property_value(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "value")
    }

    fn set_property_value(&self, value: Option<&str>) {
        ObjectExt::set_property(self.as_ref(), "value", value)
    }

    #[doc(alias = "length")]
    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<
            P: IsA<DOMDOMTokenList>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMTokenList,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMTokenList::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "value")]
    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<
            P: IsA<DOMDOMTokenList>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMTokenList,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMTokenList::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DOMDOMTokenList>> DOMDOMTokenListExt for O {}

impl fmt::Display for DOMDOMTokenList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMDOMTokenList")
    }
}
