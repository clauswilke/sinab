#![allow(dead_code)]
extern crate libc;

use libc::{c_char, c_double, c_int};
use std::ffi::{CString, CStr};
use std::fmt;

// for Rc implementation of GContext
use std::rc::Rc;
use std::ops::{Deref, DerefMut};



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
    pub fn set_color(&mut self, color: &str) {
        let ccolor = CString::new(color).unwrap();
        unsafe { gcontext_set_color(self.gc_ptr, ccolor.as_ptr()); }
    }
    pub fn set_fill(&mut self, color: &str) {
        let ccolor = CString::new(color).unwrap();
        unsafe { gcontext_set_fill(self.gc_ptr, ccolor.as_ptr()); }
    }
    pub fn set_fontfamily(&mut self, fontfamily: &str) {
        let cfontfamily = CString::new(fontfamily).unwrap();
        unsafe { gcontext_set_fontfamily(self.gc_ptr, cfontfamily.as_ptr()); }
    }
    pub fn set_fontface(&mut self, fontface: Fontface) {
        let cface:c_int = match fontface {
            Fontface::Plain => 1,
            Fontface::Bold=> 2,
            Fontface::Italics=> 3,
            Fontface::BoldItalics => 4,
        };
        unsafe { gcontext_set_fontface(self.gc_ptr, cface); }
    }
    /// Sets the fontface taking into consideration the current value, i.e., if
    /// it is already Italics and is set to Bold, the result is BoldItalics. Never
    /// removes an attribute, only adds.
    pub fn modify_fontface(&mut self, fontface: Fontface) {
        match fontface {
            Fontface::Plain => {},
            Fontface::Bold => {
                match self.fontface() {
                    Fontface::Plain => self.set_fontface(Fontface::Bold),
                    Fontface::Italics => self.set_fontface(Fontface::BoldItalics),
                    _ => {},
                };
            }
            Fontface::Italics => {
                match self.fontface() {
                    Fontface::Plain => self.set_fontface(Fontface::Italics),
                    Fontface::Bold => self.set_fontface(Fontface::BoldItalics),
                    _ => {},
                };
            }
            Fontface::BoldItalics => self.set_fontface(Fontface::BoldItalics),
        }
    }
    pub fn set_fontsize(&mut self, fontsize: f64) {
        unsafe { gcontext_set_fontsize(self.gc_ptr, fontsize as c_double); }
    }
    pub fn set_lineheight(&mut self, lineheight: f64) {
        unsafe { gcontext_set_lineheight(self.gc_ptr, lineheight as c_double); }
    }

    // getters
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
    pub fn fontsize(&self) -> f64 {
        let csize:c_double = unsafe {
            gcontext_fontsize(self.gc_ptr)
        };

        csize as f64
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
    fn rdev_string_metrics(rdev_ptr: *mut C_RenderDevice, label: *const c_char, gc: *const C_GContext, ascent: &mut c_double, descent: &mut c_double, width: &mut c_double);
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
    pub fontsize: f64,    // fontsize, in pt
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

    pub fn draw_text(&mut self, label: &str, x: f64, y: f64, gc: &GContext) {
        let clabel = CString::new(label).unwrap();
        let cx = x as c_double;
        let cy = y as c_double;

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
        let linespacing = fontsize * lineheight / 72.0; // divide by 72 to convert to in

        FontMetrics {
            fontsize: fontsize,
            lineheight: lineheight,
            linespacing: linespacing,
            lineascent: linespacing - m1.descent,
            linedescent: m1.descent,
            space_width: m2.width,
        }
    }
}

