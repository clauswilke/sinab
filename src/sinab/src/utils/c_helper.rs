pub use std::os::raw::c_char;

use std::ffi::{CStr, CString};
use std::str::Utf8Error;
use std::ptr;

pub fn cstring_to_str(s: *const c_char) -> Result<&'static str, Utf8Error> {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };

    c_str.to_str()
}

pub fn str_to_cstring(s: &str) -> *mut c_char {
    match CString::new(s) {
        Ok(cs) => cs.into_raw(),
        Err(..) => ptr::null_mut(),
    }
}

/// Delete a string created with `str_to_c_char()`. Can be called from C on
/// a string returned from rust.
#[no_mangle]
pub extern "C" fn mdl_free_cstring(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}