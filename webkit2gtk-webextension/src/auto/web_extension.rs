// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
use crate::UserMessage;
use crate::WebPage;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};
#[cfg(feature = "v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
use std::{pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "WebKitWebExtension")]
    pub struct WebExtension(Object<ffi::WebKitWebExtension, ffi::WebKitWebExtensionClass>);

    match fn {
        type_ => || ffi::webkit_web_extension_get_type(),
    }
}

impl WebExtension {
    pub const NONE: Option<&'static WebExtension> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::WebExtension>> Sealed for T {}
}

pub trait WebExtensionExt: IsA<WebExtension> + sealed::Sealed + 'static {
    #[doc(alias = "webkit_web_extension_get_page")]
    #[doc(alias = "get_page")]
    fn page(&self, page_id: u64) -> Option<WebPage> {
        unsafe {
            from_glib_none(ffi::webkit_web_extension_get_page(
                self.as_ref().to_glib_none().0,
                page_id,
            ))
        }
    }

    #[cfg(feature = "v2_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
    #[doc(alias = "webkit_web_extension_send_message_to_context")]
    fn send_message_to_context<P: FnOnce(Result<UserMessage, glib::Error>) + 'static>(
        &self,
        message: &impl IsA<UserMessage>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn send_message_to_context_trampoline<
            P: FnOnce(Result<UserMessage, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_web_extension_send_message_to_context_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = send_message_to_context_trampoline::<P>;
        unsafe {
            ffi::webkit_web_extension_send_message_to_context(
                self.as_ref().to_glib_none().0,
                message.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "v2_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
    fn send_message_to_context_future(
        &self,
        message: &(impl IsA<UserMessage> + Clone + 'static),
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<UserMessage, glib::Error>> + 'static>>
    {
        let message = message.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.send_message_to_context(&message, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "page-created")]
    fn connect_page_created<F: Fn(&Self, &WebPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn page_created_trampoline<
            P: IsA<WebExtension>,
            F: Fn(&P, &WebPage) + 'static,
        >(
            this: *mut ffi::WebKitWebExtension,
            web_page: *mut ffi::WebKitWebPage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                WebExtension::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(web_page),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-created\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    page_created_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_28")))]
    #[doc(alias = "user-message-received")]
    fn connect_user_message_received<F: Fn(&Self, &UserMessage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn user_message_received_trampoline<
            P: IsA<WebExtension>,
            F: Fn(&P, &UserMessage) + 'static,
        >(
            this: *mut ffi::WebKitWebExtension,
            message: *mut ffi::WebKitUserMessage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                WebExtension::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(message),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"user-message-received\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    user_message_received_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<WebExtension>> WebExtensionExt for O {}

impl fmt::Display for WebExtension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebExtension")
    }
}
