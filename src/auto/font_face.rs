// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use pango_sys;
use std::fmt;
use std::mem;
use std::ptr;
use FontDescription;

glib_wrapper! {
    pub struct FontFace(Object<pango_sys::PangoFontFace, pango_sys::PangoFontFaceClass, FontFaceClass>);

    match fn {
        get_type => || pango_sys::pango_font_face_get_type(),
    }
}

pub const NONE_FONT_FACE: Option<&FontFace> = None;

pub trait FontFaceExt: 'static {
    fn describe(&self) -> Option<FontDescription>;

    fn get_face_name(&self) -> Option<GString>;

    fn is_synthesized(&self) -> bool;

    fn list_sizes(&self) -> Vec<i32>;
}

impl<O: IsA<FontFace>> FontFaceExt for O {
    fn describe(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_full(pango_sys::pango_font_face_describe(self.as_ref().to_glib_none().0))
        }
    }

    fn get_face_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(pango_sys::pango_font_face_get_face_name(self.as_ref().to_glib_none().0))
        }
    }

    fn is_synthesized(&self) -> bool {
        unsafe {
            from_glib(pango_sys::pango_font_face_is_synthesized(self.as_ref().to_glib_none().0))
        }
    }

    fn list_sizes(&self) -> Vec<i32> {
        unsafe {
            let mut sizes = ptr::null_mut();
            let mut n_sizes = mem::MaybeUninit::uninit();
            pango_sys::pango_font_face_list_sizes(self.as_ref().to_glib_none().0, &mut sizes, n_sizes.as_mut_ptr());
            FromGlibContainer::from_glib_full_num(sizes, n_sizes.assume_init() as usize)
        }
    }
}

impl fmt::Display for FontFace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontFace")
    }
}
