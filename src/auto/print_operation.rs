// This file was generated by gir (d9591be+) from gir-files (???)
// DO NOT EDIT

use Error;
use PrintOperationResponse;
use WebView;
use ffi;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gtk;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct PrintOperation(Object<ffi::WebKitPrintOperation>);

    match fn {
        get_type => || ffi::webkit_print_operation_get_type(),
    }
}

impl PrintOperation {
    pub fn new(web_view: &WebView) -> PrintOperation {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::webkit_print_operation_new(web_view.to_glib_none().0))
        }
    }

    pub fn get_page_setup(&self) -> Option<gtk::PageSetup> {
        unsafe {
            from_glib_none(ffi::webkit_print_operation_get_page_setup(self.to_glib_none().0))
        }
    }

    pub fn get_print_settings(&self) -> Option<gtk::PrintSettings> {
        unsafe {
            from_glib_none(ffi::webkit_print_operation_get_print_settings(self.to_glib_none().0))
        }
    }

    pub fn print(&self) {
        unsafe {
            ffi::webkit_print_operation_print(self.to_glib_none().0);
        }
    }

    pub fn run_dialog<T: IsA<gtk::Window>>(&self, parent: Option<&T>) -> PrintOperationResponse {
        unsafe {
            from_glib(ffi::webkit_print_operation_run_dialog(self.to_glib_none().0, parent.to_glib_none().0))
        }
    }

    pub fn set_page_setup(&self, page_setup: &gtk::PageSetup) {
        unsafe {
            ffi::webkit_print_operation_set_page_setup(self.to_glib_none().0, page_setup.to_glib_none().0);
        }
    }

    pub fn set_print_settings(&self, print_settings: &gtk::PrintSettings) {
        unsafe {
            ffi::webkit_print_operation_set_print_settings(self.to_glib_none().0, print_settings.to_glib_none().0);
        }
    }

    pub fn connect_failed<F: Fn(&PrintOperation, &Error) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&PrintOperation, &Error) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "failed",
                transmute(failed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_finished<F: Fn(&PrintOperation) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&PrintOperation) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "finished",
                transmute(finished_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn failed_trampoline(this: *mut ffi::WebKitPrintOperation, error: *mut glib_ffi::GError, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&PrintOperation, &Error) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(error))
}

unsafe extern "C" fn finished_trampoline(this: *mut ffi::WebKitPrintOperation, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&PrintOperation) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
