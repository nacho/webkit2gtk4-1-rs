#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use crate::WebsiteDataTypes;
use crate::WebsiteDataManager;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use std::boxed::Box as Box_;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;
use std::ptr;

pub trait WebsiteDataManagerExtManual: 'static {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_website_data_manager_clear")]
    fn clear<P: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        types: WebsiteDataTypes,
        timespan: glib::TimeSpan,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    );
}

impl<O: IsA<WebsiteDataManager>> WebsiteDataManagerExtManual for O {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn clear<P: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        types: WebsiteDataTypes,
        timespan: glib::TimeSpan,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let user_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn clear_trampoline<
            P: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_website_data_manager_clear_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = clear_trampoline::<P>;
        unsafe {
            ffi::webkit_website_data_manager_clear(
                self.as_ref().to_glib_none().0,
                types.into_glib(),
                timespan.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }
}
