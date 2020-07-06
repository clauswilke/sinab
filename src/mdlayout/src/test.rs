#![allow(dead_code)]

use crate::renderer::*;
use crate::c_helper::*;
use crate::markdown::md_to_html;
use crate::layout::render_html;

#[no_mangle]
pub extern "C" fn mdl_test_renderer(rdev_ptr: *mut C_RenderDevice, text: *const c_char) {
    let mut rdev = RenderDevice::new(rdev_ptr);
    let input = match cstring_to_str(text) {
        Ok(s) => md_to_html(s),
        Err(..) => "".to_string(),
    };

    render_html(input.as_str(), &mut rdev);
}

use crate::dom::*;
use std::ops::Index;

pub fn test_dom() {
    let input = "<p>Hello, <em>world!</em></p>";

    let document = Document::parse_html(input.as_bytes());

    for n in document.nodes() {
        let node = document.index(n);
        println!("{:?}", node.data);
    }

    println!("testing the dom");
}
