extern crate pulldown_cmark;

use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use pulldown_cmark::{html, Options, Parser};

#[no_mangle]
pub extern "C" fn md_to_html(text: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!text.is_null());
        CStr::from_ptr(text)
    };

    let markdown_input = c_str.to_str().unwrap();

    // set up markdown parser
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(markdown_input, options);

    // write parsed input as html
    let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    let s = CString::new(html_output).unwrap();
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