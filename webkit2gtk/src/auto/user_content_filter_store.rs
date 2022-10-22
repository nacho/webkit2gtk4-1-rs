// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::UserContentFilter;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WebKitUserContentFilterStore")]
    pub struct UserContentFilterStore(Object<ffi::WebKitUserContentFilterStore, ffi::WebKitUserContentFilterStoreClass>);

    match fn {
        type_ => || ffi::webkit_user_content_filter_store_get_type(),
    }
}

impl UserContentFilterStore {
    pub const NONE: Option<&'static UserContentFilterStore> = None;

    #[doc(alias = "webkit_user_content_filter_store_new")]
    pub fn new(storage_path: &str) -> UserContentFilterStore {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_user_content_filter_store_new(
                storage_path.to_glib_none().0,
            ))
        }
    }
}

pub trait UserContentFilterStoreExt: 'static {
    #[doc(alias = "webkit_user_content_filter_store_get_path")]
    #[doc(alias = "get_path")]
    fn path(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_user_content_filter_store_load")]
    fn load<P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static>(
        &self,
        identifier: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    );

    fn load_future(
        &self,
        identifier: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<UserContentFilter, glib::Error>> + 'static>>;

    #[doc(alias = "webkit_user_content_filter_store_remove")]
    fn remove<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        identifier: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    );

    fn remove_future(
        &self,
        identifier: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "webkit_user_content_filter_store_save")]
    fn save<P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static>(
        &self,
        identifier: &str,
        source: &glib::Bytes,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    );

    fn save_future(
        &self,
        identifier: &str,
        source: &glib::Bytes,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<UserContentFilter, glib::Error>> + 'static>>;

    #[doc(alias = "webkit_user_content_filter_store_save_from_file")]
    fn save_from_file<P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static>(
        &self,
        identifier: &str,
        file: &impl IsA<gio::File>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    );

    fn save_from_file_future(
        &self,
        identifier: &str,
        file: &(impl IsA<gio::File> + Clone + 'static),
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<UserContentFilter, glib::Error>> + 'static>>;
}

impl<O: IsA<UserContentFilterStore>> UserContentFilterStoreExt for O {
    fn path(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_user_content_filter_store_get_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn load<P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static>(
        &self,
        identifier: &str,
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
        unsafe extern "C" fn load_trampoline<
            P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_user_content_filter_store_load_finish(
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
        let callback = load_trampoline::<P>;
        unsafe {
            ffi::webkit_user_content_filter_store_load(
                self.as_ref().to_glib_none().0,
                identifier.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn load_future(
        &self,
        identifier: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<UserContentFilter, glib::Error>> + 'static>>
    {
        let identifier = String::from(identifier);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.load(&identifier, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    fn remove<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        identifier: &str,
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
        unsafe extern "C" fn remove_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_user_content_filter_store_remove_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = remove_trampoline::<P>;
        unsafe {
            ffi::webkit_user_content_filter_store_remove(
                self.as_ref().to_glib_none().0,
                identifier.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn remove_future(
        &self,
        identifier: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let identifier = String::from(identifier);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.remove(&identifier, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    fn save<P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static>(
        &self,
        identifier: &str,
        source: &glib::Bytes,
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
        unsafe extern "C" fn save_trampoline<
            P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_user_content_filter_store_save_finish(
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
        let callback = save_trampoline::<P>;
        unsafe {
            ffi::webkit_user_content_filter_store_save(
                self.as_ref().to_glib_none().0,
                identifier.to_glib_none().0,
                source.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn save_future(
        &self,
        identifier: &str,
        source: &glib::Bytes,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<UserContentFilter, glib::Error>> + 'static>>
    {
        let identifier = String::from(identifier);
        let source = source.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.save(&identifier, &source, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    fn save_from_file<P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static>(
        &self,
        identifier: &str,
        file: &impl IsA<gio::File>,
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
        unsafe extern "C" fn save_from_file_trampoline<
            P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_user_content_filter_store_save_from_file_finish(
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
        let callback = save_from_file_trampoline::<P>;
        unsafe {
            ffi::webkit_user_content_filter_store_save_from_file(
                self.as_ref().to_glib_none().0,
                identifier.to_glib_none().0,
                file.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn save_from_file_future(
        &self,
        identifier: &str,
        file: &(impl IsA<gio::File> + Clone + 'static),
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<UserContentFilter, glib::Error>> + 'static>>
    {
        let identifier = String::from(identifier);
        let file = file.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.save_from_file(&identifier, &file, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }
}

impl fmt::Display for UserContentFilterStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UserContentFilterStore")
    }
}