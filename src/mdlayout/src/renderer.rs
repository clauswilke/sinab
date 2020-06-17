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
        Self { gc_ptr: unsafe { gcontext_new() } }
    }
    fn do_something(&mut self) { println!("hello there!") }
}

impl Drop for GContext {
    fn drop(&mut self) {
        unsafe { gcontext_delete(self.gc_ptr) }
    }
}


#[no_mangle]
pub extern "C" fn test_renderer() {
    let mut gc = GContext::new();
    gc.do_something();
}