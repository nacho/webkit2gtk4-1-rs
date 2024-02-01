#[cfg(feature = "v2_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
use crate::{AuthenticationRequest, Credential};
use glib::prelude::*;
#[cfg(feature = "v2_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
use glib::translate::*;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AuthenticationRequest>> Sealed for T {}
}

pub trait AuthenticationRequestExtManual:
    IsA<AuthenticationRequest> + sealed::Sealed + 'static
{
    #[cfg(feature = "v2_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_authentication_request_authenticate")]
    fn authenticate(&self, mut credential: Option<&mut Credential>) {
        unsafe {
            ffi::webkit_authentication_request_authenticate(
                self.as_ref().to_glib_none().0,
                credential.to_glib_none_mut().0,
            );
        }
    }
}

impl<O: IsA<AuthenticationRequest>> AuthenticationRequestExtManual for O {}
