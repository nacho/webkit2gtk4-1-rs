// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitDOMXPathNSResolver")]
    pub struct DOMXPathNSResolver(Interface<ffi::WebKitDOMXPathNSResolver, ffi::WebKitDOMXPathNSResolverIface>);

    match fn {
        type_ => || ffi::webkit_dom_xpath_ns_resolver_get_type(),
    }
}

impl DOMXPathNSResolver {
    pub const NONE: Option<&'static DOMXPathNSResolver> = None;
}

pub trait DOMXPathNSResolverExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_xpath_ns_resolver_lookup_namespace_uri")]
    fn lookup_namespace_uri(&self, prefix: &str) -> Option<glib::GString>;
}

impl<O: IsA<DOMXPathNSResolver>> DOMXPathNSResolverExt for O {
    fn lookup_namespace_uri(&self, prefix: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_xpath_ns_resolver_lookup_namespace_uri(
                self.as_ref().to_glib_none().0,
                prefix.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for DOMXPathNSResolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMXPathNSResolver")
    }
}