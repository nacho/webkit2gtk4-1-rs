// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMCSSStyleSheet;
use crate::DOMDocument;
use crate::DOMDocumentType;
use crate::DOMHTMLDocument;
use crate::DOMObject;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WebKitDOMDOMImplementation")]
    pub struct DOMDOMImplementation(Object<ffi::WebKitDOMDOMImplementation, ffi::WebKitDOMDOMImplementationClass>) @extends DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_dom_implementation_get_type(),
    }
}

pub const NONE_DOMDOM_IMPLEMENTATION: Option<&DOMDOMImplementation> = None;

pub trait DOMDOMImplementationExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_dom_implementation_create_css_style_sheet")]
    fn create_css_style_sheet(
        &self,
        title: &str,
        media: &str,
    ) -> Result<DOMCSSStyleSheet, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_dom_implementation_create_document")]
    fn create_document(
        &self,
        namespaceURI: Option<&str>,
        qualifiedName: &str,
        doctype: Option<&impl IsA<DOMDocumentType>>,
    ) -> Result<DOMDocument, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_dom_implementation_create_document_type")]
    fn create_document_type(
        &self,
        qualifiedName: &str,
        publicId: &str,
        systemId: &str,
    ) -> Result<DOMDocumentType, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_dom_implementation_create_html_document")]
    fn create_html_document(&self, title: &str) -> Option<DOMHTMLDocument>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_dom_implementation_has_feature")]
    fn has_feature(&self, feature: &str, version: &str) -> bool;
}

impl<O: IsA<DOMDOMImplementation>> DOMDOMImplementationExt for O {
    fn create_css_style_sheet(
        &self,
        title: &str,
        media: &str,
    ) -> Result<DOMCSSStyleSheet, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_dom_implementation_create_css_style_sheet(
                self.as_ref().to_glib_none().0,
                title.to_glib_none().0,
                media.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn create_document(
        &self,
        namespaceURI: Option<&str>,
        qualifiedName: &str,
        doctype: Option<&impl IsA<DOMDocumentType>>,
    ) -> Result<DOMDocument, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_dom_implementation_create_document(
                self.as_ref().to_glib_none().0,
                namespaceURI.to_glib_none().0,
                qualifiedName.to_glib_none().0,
                doctype.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn create_document_type(
        &self,
        qualifiedName: &str,
        publicId: &str,
        systemId: &str,
    ) -> Result<DOMDocumentType, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_dom_implementation_create_document_type(
                self.as_ref().to_glib_none().0,
                qualifiedName.to_glib_none().0,
                publicId.to_glib_none().0,
                systemId.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn create_html_document(&self, title: &str) -> Option<DOMHTMLDocument> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_implementation_create_html_document(
                self.as_ref().to_glib_none().0,
                title.to_glib_none().0,
            ))
        }
    }

    fn has_feature(&self, feature: &str, version: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_dom_implementation_has_feature(
                self.as_ref().to_glib_none().0,
                feature.to_glib_none().0,
                version.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for DOMDOMImplementation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMDOMImplementation")
    }
}