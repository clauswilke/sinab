use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn string_from_rust() -> *mut c_char {
    let s = CString::new("Hello ピカチュウ !").unwrap();
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn free_rust_cstring(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}