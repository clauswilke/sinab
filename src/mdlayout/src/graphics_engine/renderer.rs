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
use crate::style::values::{FontStyle, FontWeight};


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
pub enum Fontface {
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
pub struct GContextImpl {
    gc_ptr: *mut C_GContext,
}

impl GContextImpl {
    pub fn new() -> Self {
        Self {
            gc_ptr: unsafe { gcontext_new() }
        }
    }
    pub fn as_ptr(&self) -> *const C_GContext {
        self.gc_ptr
    }

    // setters
    pub fn set_color(&mut self, color: RGBA) {
        let ccolor = CString::new(color.to_string()).unwrap();
        unsafe { gcontext_set_color(self.gc_ptr, ccolor.as_ptr()); }
    }
    pub fn set_fill(&mut self, color: RGBA) {
        let ccolor = CString::new(color.to_string()).unwrap();
        unsafe { gcontext_set_fill(self.gc_ptr, ccolor.as_ptr()); }
    }
    pub fn set_fontfamily(&mut self, fontfamily: &str) {
        let cfontfamily = CString::new(fontfamily).unwrap();
        unsafe { gcontext_set_fontfamily(self.gc_ptr, cfontfamily.as_ptr()); }
    }
    pub fn set_fontface(&mut self, fontface: Fontface) {
        let cface:c_int = match fontface {
            Fontface::Plain => 1,
            Fontface::Bold => 2,
            Fontface::Italics => 3,
            Fontface::BoldItalics => 4,
        };
        unsafe { gcontext_set_fontface(self.gc_ptr, cface); }
    }

    pub fn set_fontstyle(&mut self, style: FontStyle) {
        match style {
            FontStyle::Italic | FontStyle::Oblique => match self.get_fontface() {
                Fontface::Plain => self.set_fontface(Fontface::Italics),
                Fontface::Bold => self.set_fontface(Fontface::BoldItalics),
                _ => {},
            },
            _ => {},
        }
    }

    pub fn set_fontweight(&mut self, weight: FontWeight) {
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
    pub fn set_fontsize(&mut self, size: Length<CssPx>) {
        let csize = (size.get() * 72.0 / 96.0) as c_double;
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
    pub fn get_fontfamily(&self) -> &str {
        let c_str = unsafe {
            CStr::from_ptr(gcontext_fontfamily(self.gc_ptr))
        };
        c_str.to_str().unwrap()
    }
    pub fn get_fontface(&self) -> Fontface {
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
    pub fn get_fontsize(&self) -> Length<CssPx> {
        let csize:c_double = unsafe {
            gcontext_fontsize(self.gc_ptr)
        };

        Length::<CssPx>::new((csize as f32) * 96.0 / 72.0)
    }
    pub fn get_lineheight(&self) -> f64 {
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
pub struct GContext(Rc<GContextImpl>);

impl GContext {
    pub fn new() -> Self {
        Self(Rc::new(GContextImpl::new()))
    }

    pub fn new_from(&self) -> Self {
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
    fn rdev_string_metrics(rdev_ptr: *const C_RenderDevice, label: *const c_char, gc: *const C_GContext, ascent: &mut c_double, descent: &mut c_double, width: &mut c_double);
}

pub struct RenderDevice {
    rdev_ptr: *mut C_RenderDevice,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct StringMetrics {
    pub ascent: Length<CssPx>,
    pub descent: Length<CssPx>,
    pub width: Length<CssPx>,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct FontMetrics {
    pub fontsize: Length<CssPx>,    // fontsize, in px
    pub lineheight: f64,  // height of line in multiples of fontsize; this is a unitless scalar
    pub linespacing: Length<CssPx>, // distance from baseline to baseline for the current font
    pub lineascent: Length<CssPx>,  // height from baseline for the current font
    pub linedescent: Length<CssPx>, // depth below baseline for the current font
    pub space_width: Length<CssPx>, // width of a space
}

impl RenderDevice {
    pub fn new(rdev_ptr: *mut C_RenderDevice) -> Self {
        Self {
            rdev_ptr
        }
    }

    pub(crate) fn draw_text(&mut self, label: &str, x: Length<CssPx>, y: Length<CssPx>, font: &Font, color: RGBA) {
        let clabel = CString::new(label).unwrap();
        // divide by 96.0 to convert px to in
        let cx = (x.get() as c_double) / 96.0;
        let cy = (y.get() as c_double) / 96.0;

        let mut gc = font.graphics_context();
        gc.set_color(color);

        unsafe {
            rdev_draw_text(self.rdev_ptr, clabel.as_ptr(), cx, cy, gc.as_ptr());
        }
    }

    pub(crate) fn new_font_manager(&self) -> FontManager {
        FontManager{ rdev_ptr: self.rdev_ptr }
    }
}

// Mark as UnwindSafe so we can catch errors with panic::catch_unwind()
impl UnwindSafe for RenderDevice {}


#[derive(Copy, Clone)]
pub(crate) struct FontManager {
    rdev_ptr: *const C_RenderDevice,
}


impl FontManager {
    pub(crate) fn new_font(&self, name: &str, style: FontStyle, weight: FontWeight, size: Length<CssPx>) -> Font {
        let mut gc = GContext::new();
        gc.set_fontfamily(name);
        gc.set_fontstyle(style);
        gc.set_fontweight(weight);
        gc.set_fontsize(size);
        Font(Rc::new(FontImpl{ rdev_ptr: self.rdev_ptr, name: name.to_string(), style, weight, size, gc }))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FontImpl {
    rdev_ptr: *const C_RenderDevice,
    name: String,
    style: FontStyle,
    weight: FontWeight,
    size: Length<CssPx>,
    gc: GContext,
}

impl FontImpl {
    pub(crate) fn string_metrics(&self, label: &str) -> StringMetrics {
        let clabel = CString::new(label).unwrap();
        let mut cascent: c_double = 0.0;
        let mut cdescent: c_double = 0.0;
        let mut cwidth: c_double = 0.0;

        unsafe {
            rdev_string_metrics(
                self.rdev_ptr,
                clabel.as_ptr(),
                self.gc.as_ptr(),
                &mut cascent,
                &mut cdescent,
                &mut cwidth
            );
        }

        StringMetrics {
            // multiply with 96.0 to convert in to px
            ascent: Length::<CssPx>::new(96.0 * cascent as f32),
            descent: Length::<CssPx>::new(96.0 * cdescent as f32),
            width: Length::<CssPx>::new(96.0 * cwidth as f32),
        }
    }

    pub(crate) fn font_metrics(&self) -> FontMetrics {
        let m1 = self.string_metrics("gjpqyQ");
        let m2 = self.string_metrics(" ");

        let fontsize = self.gc.get_fontsize();
        let lineheight = self.gc.get_lineheight();
        let linespacing = Length::<CssPx>::new(fontsize.get() * lineheight as f32);

        FontMetrics {
            fontsize: fontsize,
            lineheight: lineheight,
            linespacing: linespacing,
            lineascent: linespacing - m1.descent,
            linedescent: m1.descent,
            space_width: m2.width,
        }
    }

    pub(crate) fn graphics_context(&self) -> GContext {
        self.gc.clone()
    }
}

#[derive(Debug)]
pub(crate) struct Font(Rc<FontImpl>);

impl Clone for Font {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Deref for Font {
    type Target = FontImpl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Font {
    fn deref_mut(&mut self) -> &mut FontImpl {
        Rc::make_mut(&mut self.0)
    }
}

/// Enum to signal problems with fonts. Since we're not doing proper font handling
/// at this time, doesn't do much.
#[derive(Debug)]
pub enum FontError {
    /// Something's wrong.
    GeneralError,
}