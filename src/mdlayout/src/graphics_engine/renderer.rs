#![allow(dead_code)]
extern crate libc;

use libc::{c_char, c_double, c_int};
use std::ffi::{CString, CStr};
use std::fmt;
use std::panic::UnwindSafe;

// for Rc implementation of GContext
use std::rc::Rc;
use std::ops::{Deref, DerefMut};

use crate::primitives::*;
use crate::style::values::{Length, FontStyle, FontWeight};
use crate::graphics_engine::font::Font;


#[repr(C)]
pub struct C_GContext { _private: [u8; 0] }

extern {
    // construction and deletion
    fn gcontext_new() -> *mut C_GContext;
    fn gcontext_clone(gc: *mut C_GContext) -> *mut C_GContext;
    fn gcontext_delete(gc: *mut C_GContext);

    // setters
    fn gcontext_set_color(gc: *mut C_GContext, color: *const c_char);
    fn gcontext_set_fill(gc: *mut C_GContext, color: *const c_char);
    fn gcontext_set_fontfamily(gc: *mut C_GContext, color: *const c_char);
    fn gcontext_set_fontface(gc: *mut C_GContext, fontface: c_int);
    fn gcontext_set_fontsize(gc: *mut C_GContext, fontsize: c_double);
    fn gcontext_set_lineheight(gc: *mut C_GContext, lineheight: c_double);

    // getters
    fn gcontext_color(gc: *mut C_GContext) -> *const c_char;
    fn gcontext_fill(gc: *mut C_GContext) -> *const c_char;
    fn gcontext_fontfamily(gc: *mut C_GContext) -> *const c_char;
    fn gcontext_fontface(gc: *mut C_GContext) -> c_int;
    fn gcontext_fontsize(gc: *mut C_GContext) -> c_double;
    fn gcontext_lineheight(gc: *mut C_GContext) -> c_double;
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Fontface {
    Plain,
    Bold,
    Italics,
    BoldItalics,
}

impl fmt::Display for Fontface {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Fontface::Plain => f.write_str("Plain"),
            Fontface::Bold => f.write_str("Bold"),
            Fontface::Italics => f.write_str("Italics"),
            Fontface::BoldItalics => f.write_str("BoldItalics"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct GContextImpl {
    gc_ptr: *mut C_GContext,
}

impl GContextImpl {
    pub(crate) fn new() -> Self {
        Self {
            gc_ptr: unsafe { gcontext_new() }
        }
    }
    pub(crate) fn as_ptr(&self) -> *const C_GContext {
        self.gc_ptr
    }

    // setters
    pub(crate) fn set_color(&mut self, color: RGBA) {
        let ccolor = CString::new(color.to_string()).unwrap();
        unsafe { gcontext_set_color(self.gc_ptr, ccolor.as_ptr()); }
    }
    pub(crate) fn set_fill(&mut self, color: RGBA) {
        let ccolor = CString::new(color.to_string()).unwrap();
        unsafe { gcontext_set_fill(self.gc_ptr, ccolor.as_ptr()); }
    }
    pub(crate) fn set_fontfamily(&mut self, fontfamily: &str) {
        let cfontfamily = CString::new(fontfamily).unwrap();
        unsafe { gcontext_set_fontfamily(self.gc_ptr, cfontfamily.as_ptr()); }
    }
    pub(crate) fn set_fontface(&mut self, fontface: Fontface) {
        let cface:c_int = match fontface {
            Fontface::Plain => 1,
            Fontface::Bold => 2,
            Fontface::Italics => 3,
            Fontface::BoldItalics => 4,
        };
        unsafe { gcontext_set_fontface(self.gc_ptr, cface); }
    }

    pub(crate) fn set_fontstyle(&mut self, style: FontStyle) {
        match style {
            FontStyle::Italic | FontStyle::Oblique => match self.get_fontface() {
                Fontface::Plain => self.set_fontface(Fontface::Italics),
                Fontface::Bold => self.set_fontface(Fontface::BoldItalics),
                _ => {},
            },
            _ => {},
        }
    }

    pub(crate) fn set_fontweight(&mut self, weight: FontWeight) {
        match weight {
            FontWeight::Bold => match self.get_fontface() {
                Fontface::Plain => self.set_fontface(Fontface::Bold),
                Fontface::Italics => self.set_fontface(Fontface::BoldItalics),
                _ => {},
            },
            _ => {},
        }
    }

    /// Sets the fontsize, in px
    pub(crate) fn set_fontsize(&mut self, size: Length) {
        let csize = (size.px * 72.0 / 96.0) as c_double;
        unsafe { gcontext_set_fontsize(self.gc_ptr, csize); }
    }

    // getters
    /* // need to update to work with RGBA; however, may also not be needed at all
    pub fn get_color(&self) -> &str {
        let c_str = unsafe {
            CStr::from_ptr(gcontext_color(self.gc_ptr))
        };
        c_str.to_str().unwrap()
    }
    pub fn get_fill(&self) -> &str {
        let c_str = unsafe {
            CStr::from_ptr(gcontext_fill(self.gc_ptr))
        };
        c_str.to_str().unwrap()
    }
    */
    pub(crate) fn get_fontfamily(&self) -> &str {
        let c_str = unsafe {
            CStr::from_ptr(gcontext_fontfamily(self.gc_ptr))
        };
        c_str.to_str().unwrap()
    }
    pub(crate) fn get_fontface(&self) -> Fontface {
        let cface:c_int = unsafe {
            gcontext_fontface(self.gc_ptr)
        };

        match cface {
            1 => Fontface::Plain,
            2 => Fontface::Bold,
            3 => Fontface::Italics,
            4 => Fontface::BoldItalics,
            _ => Fontface::Plain, // interpret unknown fontfaces as Plain
        }
    }
    /// Returns the current fontsize, in px
    pub(crate) fn get_fontsize(&self) -> Length {
        let csize:c_double = unsafe {
            gcontext_fontsize(self.gc_ptr)
        };

        Length{ px: (csize as f32) * 96.0 / 72.0 }
    }
    pub(crate) fn get_lineheight(&self) -> f64 {
        let cheight:c_double = unsafe {
            gcontext_lineheight(self.gc_ptr)
        };
        cheight as f64
    }
}

impl Clone for GContextImpl {
    fn clone(&self) -> Self {
        Self {
            gc_ptr: unsafe { gcontext_clone(self.gc_ptr) }
        }
    }
}

impl Drop for GContextImpl {
    fn drop(&mut self) {
        unsafe { gcontext_delete(self.gc_ptr); }
    }
}

#[derive(Debug)]
pub(crate) struct GContext(Rc<GContextImpl>);

impl GContext {
    pub(crate) fn new() -> Self {
        Self(Rc::new(GContextImpl::new()))
    }

    pub(crate) fn new_from(&self) -> Self {
        Self(Rc::new(self.deref().clone()))
    }
}

impl Clone for GContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Deref for GContext {
    type Target = GContextImpl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GContext {
    fn deref_mut(&mut self) -> &mut GContextImpl {
        Rc::make_mut(&mut self.0)
    }
}

#[repr(C)]
pub struct C_RenderDevice { _private: [u8; 0] }

extern {
    fn rdev_draw_text(rdev_ptr: *mut C_RenderDevice, label: *const c_char, x: c_double, y: c_double, gc: *const C_GContext);
    fn rdev_draw_rect(rdev_ptr: *mut C_RenderDevice, x: c_double, y: c_double, width: c_double, height: c_double, gc: *const C_GContext);
    pub(super) fn rdev_string_metrics(label: *const c_char, gc: *const C_GContext, ascent: &mut c_double, descent: &mut c_double, width: &mut c_double);
}

pub struct RenderDevice {
    rdev_ptr: *mut C_RenderDevice,
}

impl RenderDevice {
    pub(crate) fn new(rdev_ptr: *mut C_RenderDevice) -> Self {
        Self {
            rdev_ptr
        }
    }

    pub(crate) fn draw_text(&mut self, label: &str, x: Length, y: Length, font: &Font, color: RGBA) {
        let clabel = CString::new(label).unwrap();
        // divide by 96.0 to convert px to in
        let cx = (x.px as c_double) / 96.0;
        let cy = (y.px as c_double) / 96.0;

        let mut gc = font.graphics_context();
        gc.set_color(color);

        unsafe {
            rdev_draw_text(self.rdev_ptr, clabel.as_ptr(), cx, cy, gc.as_ptr());
        }
    }

    pub(crate) fn draw_rect(&mut self, x: Length, y: Length, width: Length, height: Length, fill: RGBA) {
        // divide by 96.0 to convert px to in
        let cx = (x.px as c_double) / 96.0;
        let cy = (y.px as c_double) / 96.0;
        let cwidth = (width.px as c_double) / 96.0;
        let cheight = (height.px as c_double) / 96.0;

        let mut gc = GContext::new();
        gc.set_color(RGBA(0, 0, 0, 0));
        gc.set_fill(fill);

        unsafe {
            rdev_draw_rect(self.rdev_ptr, cx, cy, cwidth, cheight,gc.as_ptr());
        }
    }
}

// Mark as UnwindSafe so we can catch errors with panic::catch_unwind()
impl UnwindSafe for RenderDevice {}
