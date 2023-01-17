// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
use crate::OptionMenuItem;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
use glib::translate::*;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitOptionMenu")]
    pub struct OptionMenu(Object<ffi::WebKitOptionMenu, ffi::WebKitOptionMenuClass>);

    match fn {
        type_ => || ffi::webkit_option_menu_get_type(),
    }
}

impl OptionMenu {
    pub const NONE: Option<&'static OptionMenu> = None;
}

pub trait OptionMenuExt: 'static {
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_option_menu_activate_item")]
    fn activate_item(&self, index: u32);

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_option_menu_close")]
    fn close(&self);

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_option_menu_get_item")]
    #[doc(alias = "get_item")]
    fn item(&self, index: u32) -> Option<OptionMenuItem>;

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_option_menu_get_n_items")]
    #[doc(alias = "get_n_items")]
    fn n_items(&self) -> u32;

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_option_menu_select_item")]
    fn select_item(&self, index: u32);

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "close")]
    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<OptionMenu>> OptionMenuExt for O {
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn activate_item(&self, index: u32) {
        unsafe {
            ffi::webkit_option_menu_activate_item(self.as_ref().to_glib_none().0, index);
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn close(&self) {
        unsafe {
            ffi::webkit_option_menu_close(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn item(&self, index: u32) -> Option<OptionMenuItem> {
        unsafe {
            from_glib_none(ffi::webkit_option_menu_get_item(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn n_items(&self) -> u32 {
        unsafe { ffi::webkit_option_menu_get_n_items(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn select_item(&self, index: u32) {
        unsafe {
            ffi::webkit_option_menu_select_item(self.as_ref().to_glib_none().0, index);
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_trampoline<P: IsA<OptionMenu>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitOptionMenu,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(OptionMenu::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    close_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for OptionMenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("OptionMenu")
    }
}
