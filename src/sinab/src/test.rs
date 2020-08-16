#![allow(dead_code)]

use crate::graphics_engine::renderer::*;
use crate::utils::c_helper::*;
use crate::markdown::md_to_html;
use crate::paint::render_html;
use crate::primitives::*;

use std::panic;

#[no_mangle]
pub extern "C" fn sinab_test_renderer(
    rdev_ptr: *mut C_RenderDevice,
    text: *const c_char,
    css: *const c_char,
    width_px: c_double,
    height_px: c_double ) {
    let rdev = RenderDevice::new(rdev_ptr);
    let text_input = match cstring_to_str(text) {
        Ok(s) => md_to_html(s),
        Err(..) => "".to_string(),
    };
    let css_input = match cstring_to_str(css) {
        Ok(s) => s,
        Err(..) => "",
    };

    let page_size: Size<CssPx> = Size::new(width_px as f32, height_px as f32);

    let result = panic::catch_unwind(move || {
        render_html(text_input.as_str(), css_input, rdev, page_size);
    });

    if result.is_err() {
        println!("Rust error: {:?}", result);
    }
}
