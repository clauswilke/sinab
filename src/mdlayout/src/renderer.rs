#![allow(dead_code)]
extern crate libc;

use libc::{c_char, c_double, c_int};
use std::ffi::{CString};

#[repr(C)]
pub struct C_GContext { _private: [u8; 0] }

extern {
    fn gcontext_new() -> *mut C_GContext;
    fn gcontext_copy(gc: *mut C_GContext) -> *mut C_GContext;
    fn gcontext_delete(gc: *mut C_GContext);
    fn gcontext_set_color(gc: *mut C_GContext, color: *const c_char);
    fn gcontext_set_fontface(gc: *mut C_GContext, fontface: c_int);
}

#[repr(C)]
pub struct GContext {
    gc_ptr: *mut C_GContext,
}

#[allow(dead_code)]
pub enum Fontface {
    Plain,
    Bold,
    Italics,
    BoldItalics,
}

impl GContext {
    pub fn new() -> Self {
        Self {
            gc_ptr: unsafe { gcontext_new() }
        }
    }
    pub fn copy(&self) -> GContext {
        Self {
            gc_ptr: unsafe { gcontext_copy(self.gc_ptr) }
        }
    }
    pub fn as_ptr(&self) -> *const C_GContext {
        self.gc_ptr
    }
    pub fn set_color(&mut self, color: &str) {
        let ccolor = CString::new(color).unwrap();
        unsafe { gcontext_set_color(self.gc_ptr, ccolor.as_ptr()); }
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

}

impl Drop for GContext {
    fn drop(&mut self) {
        unsafe { gcontext_delete(self.gc_ptr); }
    }
}

#[repr(C)]
pub struct C_RenderDevice { _private: [u8; 0] }

extern {
    fn rdev_draw_text(rdev_ptr: *mut C_RenderDevice, label: *const c_char, x: c_double, y: c_double, gc: *const C_GContext);
    fn rdev_string_metrics(rdev_ptr: *mut C_RenderDevice, label: *const c_char, gc: *const C_GContext, ascent: &mut c_double, descent: &mut c_double, width: &mut c_double);
}

#[repr(C)]
pub struct RenderDevice {
    rdev_ptr: *mut C_RenderDevice,
}

#[allow(dead_code)]
pub struct StringMetrics {
    pub ascent: f64,
    pub descent: f64,
    pub width: f64,
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
}

