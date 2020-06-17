extern crate libc;

use libc::{c_char, c_double};
use std::ffi::{CString};

#[repr(C)]
struct CGContext { _private: [u8; 0] }

extern {
    fn gcontext_new() -> *mut CGContext;
    fn gcontext_delete(gc: *mut CGContext);
}

#[repr(C)]
struct GContext {
    gc_ptr: *mut CGContext,
}

impl GContext {
    fn new() -> Self {
        Self {
            gc_ptr: unsafe { gcontext_new() }
        }
    }
    fn as_ptr(&self) -> *const CGContext {
        self.gc_ptr
    }
}

impl Drop for GContext {
    fn drop(&mut self) {
        unsafe { gcontext_delete(self.gc_ptr) }
    }
}

#[repr(C)]
pub struct CGRObject { _private: [u8; 0] }

extern {
    fn gr_draw_text(
        gro_ptr: *mut CGRObject,
        label: *const c_char,
        x: c_double,
        y: c_double,
        gc: *const CGContext
    );
}

#[repr(C)]
struct GRObject {
    gro_ptr: *mut CGRObject,
}

impl GRObject {
    fn new(gro_ptr: *mut CGRObject) -> Self {
        Self {
            gro_ptr
        }
    }

    fn draw_text(&mut self, label: &str, x: f64, y: f64, gc: &GContext) {
        let clabel = CString::new(label).unwrap();
        let cx = x as c_double;
        let cy = y as c_double;

        unsafe {
            gr_draw_text(self.gro_ptr, clabel.as_ptr(), cx, cy, gc.as_ptr());
        }
    }
}


#[no_mangle]
pub extern "C" fn test_renderer(gro_ptr: *mut CGRObject) {
    let gc = GContext::new();
    let mut gro = GRObject::new(gro_ptr);
    gro.draw_text("test", 0.2, 2.0, &gc);
}