extern crate libc;

use libc::{c_char, c_double, c_int};
use std::ffi::{CString};

#[repr(C)]
pub struct CGContext { _private: [u8; 0] }

extern {
    fn gcontext_new() -> *mut CGContext;
    fn gcontext_copy(gc: *mut CGContext) -> *mut CGContext;
    fn gcontext_delete(gc: *mut CGContext);
    fn gcontext_set_color(gc: *mut CGContext, color: *const c_char);
    fn gcontext_set_fontface(gc: *mut CGContext, fontface: c_int);
}

#[repr(C)]
pub struct GContext {
    gc_ptr: *mut CGContext,
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
    pub fn as_ptr(&self) -> *const CGContext {
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
pub struct CRenderDevice { _private: [u8; 0] }

extern {
    fn rdev_draw_text(rdev_ptr: *mut CRenderDevice, label: *const c_char, x: c_double, y: c_double, gc: *const CGContext);
    fn rdev_string_metrics(rdev_ptr: *mut CRenderDevice, label: *const c_char, gc: *const CGContext, ascent: &mut c_double, descent: &mut c_double, width: &mut c_double);
}

#[repr(C)]
pub struct RenderDevice {
    rdev_ptr: *mut CRenderDevice,
}

#[allow(dead_code)]
pub struct StringMetrics {
    ascent: f64,
    descent: f64,
    width: f64
}

impl RenderDevice {
    pub fn new(rdev_ptr: *mut CRenderDevice) -> Self {
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


#[no_mangle]
pub extern "C" fn test_renderer(rdev_ptr: *mut CRenderDevice) {
    let mut rdev = RenderDevice::new(rdev_ptr);
    let gc = GContext::new();
    let mut m = rdev.string_metrics(" ", &gc);
    let w_space = m.width;
    let mut x = 0.2;
    let y = 0.5;
    rdev.draw_text("These", x, y, &gc);
    m = rdev.string_metrics("These", &gc);
    x += w_space + m.width;
    let mut gc2 = gc.copy();
    gc2.set_color("red");
    gc2.set_fontface(Fontface::Bold);
    rdev.draw_text("grobs", x, y, &gc2);
    m = rdev.string_metrics("grobs", &gc2);
    x += w_space + m.width;
    rdev.draw_text("were made", x, y, &gc);
    m = rdev.string_metrics("were made", &gc);
    x += w_space + m.width;
    gc2.set_color("blue");
    gc2.set_fontface(Fontface::Italics);
    rdev.draw_text("in rust.", x, y, &gc2);
}

