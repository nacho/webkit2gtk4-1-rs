// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitUserMessage")]
    pub struct UserMessage(Object<ffi::WebKitUserMessage, ffi::WebKitUserMessageClass>);

    match fn {
        type_ => || ffi::webkit_user_message_get_type(),
    }
}

impl UserMessage {
    pub const NONE: Option<&'static UserMessage> = None;

    #[doc(alias = "webkit_user_message_new")]
    pub fn new(name: &str, parameters: Option<&glib::Variant>) -> UserMessage {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_user_message_new(
                name.to_glib_none().0,
                parameters.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_user_message_new_with_fd_list")]
    #[doc(alias = "new_with_fd_list")]
    pub fn with_fd_list(
        name: &str,
        parameters: Option<&glib::Variant>,
        fd_list: Option<&impl IsA<gio::UnixFDList>>,
    ) -> UserMessage {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_user_message_new_with_fd_list(
                name.to_glib_none().0,
                parameters.to_glib_none().0,
                fd_list.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_user_message_error_quark")]
    pub fn error_quark() -> glib::Quark {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::webkit_user_message_error_quark()) }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::UserMessage>> Sealed for T {}
}

pub trait UserMessageExt: IsA<UserMessage> + sealed::Sealed + 'static {
    #[doc(alias = "webkit_user_message_get_fd_list")]
    #[doc(alias = "get_fd_list")]
    fn fd_list(&self) -> Option<gio::UnixFDList> {
        unsafe {
            from_glib_none(ffi::webkit_user_message_get_fd_list(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_user_message_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_user_message_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_user_message_get_parameters")]
    #[doc(alias = "get_parameters")]
    fn parameters(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::webkit_user_message_get_parameters(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_user_message_send_reply")]
    fn send_reply(&self, reply: &impl IsA<UserMessage>) {
        unsafe {
            ffi::webkit_user_message_send_reply(
                self.as_ref().to_glib_none().0,
                reply.as_ref().to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<UserMessage>> UserMessageExt for O {}

impl fmt::Display for UserMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UserMessage")
    }
}
