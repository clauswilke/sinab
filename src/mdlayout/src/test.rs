#![allow(dead_code)]

use crate::renderer::*;
use crate::c_helper::*;
use crate::markdown::md_to_html;
use crate::layout::render_html;

use std::panic;

#[no_mangle]
pub extern "C" fn mdl_test_renderer(rdev_ptr: *mut C_RenderDevice, text: *const c_char, css: *const c_char) {
    let rdev = RenderDevice::new(rdev_ptr);
    let text_input = match cstring_to_str(text) {
        Ok(s) => md_to_html(s),
        Err(..) => "".to_string(),
    };
    let css_input = match cstring_to_str(css) {
        Ok(s) => s,
        Err(..) => "",
    };

    let result = panic::catch_unwind(move || {
        render_html(text_input.as_str(), css_input, rdev);
    });

    if result.is_err() {
        println!("Rust error: {:?}", result);
    }
}

use crate::dom::*;

fn process_nodes(node_id: NodeId, document: &Document) {
    let node = &document[node_id];
    let mut eltname = "".to_string();

    if let NodeData::Element(ElementData{ref name, ..}) = node.data {
        eltname = name.local.to_string();
        println!("<{}>", eltname);
    }

    println!("{:?}", node.data);

    if let Some(child_id) = node.first_child {
        for nid in document.node_and_following_siblings(child_id) {
            process_nodes(nid, document);
        }

    }

    if eltname != "" {
        println!("</{}>", eltname);
    }

}


pub fn test_dom() {
    let input = "<p>Hello, <em>world!</em> How is it going?</p><strong><em><span>test</span></em></strong>";

    let document = Document::parse_html(input.as_bytes());

    process_nodes(document.root_element(), &document)
}

