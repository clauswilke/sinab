#![allow(dead_code)]
extern crate libc;

use libc::{c_char, c_double, c_int};
use std::ffi::{CString, CStr};
use std::fmt;
use std::panic::UnwindSafe;

// for Rc implementation of GContext
use std::rc::Rc;
use std::ops::{Deref, DerefMut};

use cssparser::{RGBA};
use crate::style::values::*;


/// helper function to convert cssparser::RGBA to a String
fn rgba_to_string(color: RGBA) -> String {
    //match color {
    //    Color::RGBA(RGBA{ red, green, blue, alpha }) => {
    if color.alpha == 255 { // without alpha component
        format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)
    } else { // with alpha component
        format!("#{:02x}{:02x}{:02x}{:02x}", color.red, color.green, color.blue, color.alpha)
    }
}



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
        let ccolor = CString::new(rgba_to_string(color)).unwrap();
        unsafe { gcontext_set_color(self.gc_ptr, ccolor.as_ptr()); }
    }
    pub fn set_fill(&mut self, color: RGBA) {
        let ccolor = CString::new(rgba_to_string(color)).unwrap();
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
            FontStyle::Italic | FontStyle::Oblique => match self.fontface() {
                Fontface::Plain => self.set_fontface(Fontface::Italics),
                Fontface::Bold => self.set_fontface(Fontface::BoldItalics),
                _ => {},
            },
            _ => {},
        }
    }

    pub fn set_fontweight(&mut self, weight: FontWeight) {
        match weight {
            FontWeight::Bold => match self.fontface() {
                Fontface::Plain => self.set_fontface(Fontface::Bold),
                Fontface::Italics => self.set_fontface(Fontface::BoldItalics),
                _ => {},
            },
            _ => {},
        }
    }

    /// Sets the fontsize, in px
    pub fn set_fontsize(&mut self, px: f64) {
        let csize = (px * 72.0 / 96.0) as c_double;
        unsafe { gcontext_set_fontsize(self.gc_ptr, csize); }
    }

    // getters
    /* // need to update to work with cssparser::RGBA
    pub fn color(&self) -> &str {
        let c_str = unsafe {
            CStr::from_ptr(gcontext_color(self.gc_ptr))
        };
        c_str.to_str().unwrap()
    }
    pub fn fill(&self) -> &str {
        let c_str = unsafe {
            CStr::from_ptr(gcontext_fill(self.gc_ptr))
        };
        c_str.to_str().unwrap()
    }
    */
    pub fn fontfamily(&self) -> &str {
        let c_str = unsafe {
            CStr::from_ptr(gcontext_fontfamily(self.gc_ptr))
        };
        c_str.to_str().unwrap()
    }
    pub fn fontface(&self) -> Fontface {
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
    pub fn fontsize(&self) -> f64 {
        let csize:c_double = unsafe {
            gcontext_fontsize(self.gc_ptr)
        };

        (csize as f64) * 96.0 / 72.0
    }
    pub fn lineheight(&self) -> f64 {
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
    pub ascent: f64,
    pub descent: f64,
    pub width: f64,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct FontMetrics {
    pub fontsize: f64,    // fontsize, in px
    pub lineheight: f64,  // height of line in multiples of fontsize
    pub linespacing: f64, // distance from baseline to baseline for the current font
    pub lineascent: f64,  // height from baseline for the current font
    pub linedescent: f64, // depth below baseline for the current font
    pub space_width: f64, // width of a space
}

impl RenderDevice {
    pub fn new(rdev_ptr: *mut C_RenderDevice) -> Self {
        Self {
            rdev_ptr
        }
    }

    pub(crate) fn draw_text(&mut self, label: &str, x: f64, y: f64, font: &Font, color: RGBA) {
        let clabel = CString::new(label).unwrap();
        let cx = x as c_double;
        let cy = y as c_double;

        let mut gc = font.graphics_context();
        gc.set_color(color);

        unsafe {
            rdev_draw_text(self.rdev_ptr, clabel.as_ptr(), cx, cy, gc.as_ptr());
        }
    }

    pub fn string_metrics(&mut self, label: &str, gc: &GContext) -> StringMetrics {
        let clabel = CString::new(label).unwrap();
        let mut cascent: c_double = 0.0;
        let mut cdescent: c_double = 0.0;
        let mut cwidth: c_double = 0.0;

        unsafe {
            rdev_string_metrics(self.rdev_ptr, clabel.as_ptr(), gc.as_ptr(), &mut cascent, &mut cdescent, &mut cwidth);
        }

        StringMetrics {
            ascent: cascent as f64,
            descent: cdescent as f64,
            width: cwidth as f64
        }
    }

    pub fn font_metrics(&mut self, gc: &GContext) -> FontMetrics {
        let m1 = self.string_metrics("gjpqyQ", gc);
        let m2 = self.string_metrics(" ", gc);

        let fontsize = gc.fontsize();
        let lineheight = gc.lineheight();
        let linespacing = fontsize * lineheight / 72.0; // divide by 96 to convert to in

        FontMetrics {
            fontsize: fontsize,
            lineheight: lineheight,
            linespacing: linespacing,
            lineascent: linespacing - m1.descent,
            linedescent: m1.descent,
            space_width: m2.width,
        }
    }

    pub(crate) fn new_font_manager(&self) -> FontManager {
        FontManager{ rdev_ptr: self.rdev_ptr }
    }
}

// Mark as UnwindSafe so we can catch errors with panic::catch_unwind()
impl UnwindSafe for RenderDevice {}



pub(crate) struct FontManager {
    rdev_ptr: *const C_RenderDevice,
}


impl FontManager {
    pub(crate) fn new_font(&self, name: &str, style: FontStyle, weight: FontWeight, size: Length) -> Font {
        let mut gc = GContext::new();
        gc.set_fontfamily(name);
        gc.set_fontstyle(style);
        gc.set_fontweight(weight);
        gc.set_fontsize(size.px as f64);
        Font{ rdev_ptr: self.rdev_ptr, name: name.to_string(), style, weight, size, gc }
    }
}


// TODO: this is currently extremely inefficient, since the font data gets copied on every .clone() call.

#[derive(Clone)]
pub(crate) struct Font {
    rdev_ptr: *const C_RenderDevice,
    name: String,
    style: FontStyle,
    weight: FontWeight,
    size: Length,
    gc: GContext,
}

impl Font {
    pub fn string_metrics(&self, label: &str) -> StringMetrics {
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
            ascent: cascent as f64,
            descent: cdescent as f64,
            width: cwidth as f64
        }
    }

    pub fn font_metrics(&self) -> FontMetrics {
        let m1 = self.string_metrics("gjpqyQ");
        let m2 = self.string_metrics(" ");

        let fontsize = self.gc.fontsize();
        let lineheight = self.gc.lineheight();
        let linespacing = fontsize * lineheight / 72.0; // divide by 96 to convert to in

        FontMetrics {
            fontsize: fontsize,
            lineheight: lineheight,
            linespacing: linespacing,
            lineascent: linespacing - m1.descent,
            linedescent: m1.descent,
            space_width: m2.width,
        }
    }

    pub fn graphics_context(&self) -> GContext {
        self.gc.clone()
    }
}