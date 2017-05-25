// This file was generated by gir (9bc1647) from gir-files (71d73f0)
// DO NOT EDIT

use Direction;
use FontDescription;
use FontMap;
use Gravity;
use GravityHint;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Context(Object<ffi::PangoContext>);

    match fn {
        get_type => || ffi::pango_context_get_type(),
    }
}

impl Context {
    pub fn new() -> Context {
        unsafe {
            from_glib_full(ffi::pango_context_new())
        }
    }
}

pub trait ContextExt {
    #[cfg(feature = "v1_32_4")]
    fn changed(&self);

    fn get_base_dir(&self) -> Direction;

    fn get_base_gravity(&self) -> Gravity;

    fn get_font_description(&self) -> Option<FontDescription>;

    fn get_font_map(&self) -> Option<FontMap>;

    fn get_gravity(&self) -> Gravity;

    fn get_gravity_hint(&self) -> GravityHint;

    //fn get_language(&self) -> /*Ignored*/Option<Language>;

    //fn get_matrix(&self) -> /*Ignored*/Option<Matrix>;

    //fn get_metrics<'a, 'b, P: Into<Option<&'a FontDescription>>, Q: Into<Option<&'b /*Ignored*/Language>>>(&self, desc: P, language: Q) -> /*Ignored*/Option<FontMetrics>;

    #[cfg(feature = "v1_32_4")]
    fn get_serial(&self) -> u32;

    //fn list_families(&self, families: /*Unimplemented*/Vec<FontFamily>) -> i32;

    //fn load_font(&self, desc: &FontDescription) -> /*Ignored*/Option<Font>;

    //fn load_fontset(&self, desc: &FontDescription, language: /*Ignored*/&mut Language) -> /*Ignored*/Option<Fontset>;

    fn set_base_dir(&self, direction: Direction);

    fn set_base_gravity(&self, gravity: Gravity);

    fn set_font_description(&self, desc: &FontDescription);

    fn set_font_map(&self, font_map: &FontMap);

    fn set_gravity_hint(&self, hint: GravityHint);

    //fn set_language(&self, language: /*Ignored*/&mut Language);

    //fn set_matrix<'a, P: Into<Option<&'a /*Ignored*/Matrix>>>(&self, matrix: P);
}

impl<O: IsA<Context>> ContextExt for O {
    #[cfg(feature = "v1_32_4")]
    fn changed(&self) {
        unsafe {
            ffi::pango_context_changed(self.to_glib_none().0);
        }
    }

    fn get_base_dir(&self) -> Direction {
        unsafe {
            from_glib(ffi::pango_context_get_base_dir(self.to_glib_none().0))
        }
    }

    fn get_base_gravity(&self) -> Gravity {
        unsafe {
            from_glib(ffi::pango_context_get_base_gravity(self.to_glib_none().0))
        }
    }

    fn get_font_description(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_none(ffi::pango_context_get_font_description(self.to_glib_none().0))
        }
    }

    fn get_font_map(&self) -> Option<FontMap> {
        unsafe {
            from_glib_none(ffi::pango_context_get_font_map(self.to_glib_none().0))
        }
    }

    fn get_gravity(&self) -> Gravity {
        unsafe {
            from_glib(ffi::pango_context_get_gravity(self.to_glib_none().0))
        }
    }

    fn get_gravity_hint(&self) -> GravityHint {
        unsafe {
            from_glib(ffi::pango_context_get_gravity_hint(self.to_glib_none().0))
        }
    }

    //fn get_language(&self) -> /*Ignored*/Option<Language> {
    //    unsafe { TODO: call ffi::pango_context_get_language() }
    //}

    //fn get_matrix(&self) -> /*Ignored*/Option<Matrix> {
    //    unsafe { TODO: call ffi::pango_context_get_matrix() }
    //}

    //fn get_metrics<'a, 'b, P: Into<Option<&'a FontDescription>>, Q: Into<Option<&'b /*Ignored*/Language>>>(&self, desc: P, language: Q) -> /*Ignored*/Option<FontMetrics> {
    //    unsafe { TODO: call ffi::pango_context_get_metrics() }
    //}

    #[cfg(feature = "v1_32_4")]
    fn get_serial(&self) -> u32 {
        unsafe {
            ffi::pango_context_get_serial(self.to_glib_none().0)
        }
    }

    //fn list_families(&self, families: /*Unimplemented*/Vec<FontFamily>) -> i32 {
    //    unsafe { TODO: call ffi::pango_context_list_families() }
    //}

    //fn load_font(&self, desc: &FontDescription) -> /*Ignored*/Option<Font> {
    //    unsafe { TODO: call ffi::pango_context_load_font() }
    //}

    //fn load_fontset(&self, desc: &FontDescription, language: /*Ignored*/&mut Language) -> /*Ignored*/Option<Fontset> {
    //    unsafe { TODO: call ffi::pango_context_load_fontset() }
    //}

    fn set_base_dir(&self, direction: Direction) {
        unsafe {
            ffi::pango_context_set_base_dir(self.to_glib_none().0, direction.to_glib());
        }
    }

    fn set_base_gravity(&self, gravity: Gravity) {
        unsafe {
            ffi::pango_context_set_base_gravity(self.to_glib_none().0, gravity.to_glib());
        }
    }

    fn set_font_description(&self, desc: &FontDescription) {
        unsafe {
            ffi::pango_context_set_font_description(self.to_glib_none().0, desc.to_glib_none().0);
        }
    }

    fn set_font_map(&self, font_map: &FontMap) {
        unsafe {
            ffi::pango_context_set_font_map(self.to_glib_none().0, font_map.to_glib_none().0);
        }
    }

    fn set_gravity_hint(&self, hint: GravityHint) {
        unsafe {
            ffi::pango_context_set_gravity_hint(self.to_glib_none().0, hint.to_glib());
        }
    }

    //fn set_language(&self, language: /*Ignored*/&mut Language) {
    //    unsafe { TODO: call ffi::pango_context_set_language() }
    //}

    //fn set_matrix<'a, P: Into<Option<&'a /*Ignored*/Matrix>>>(&self, matrix: P) {
    //    unsafe { TODO: call ffi::pango_context_set_matrix() }
    //}
}
