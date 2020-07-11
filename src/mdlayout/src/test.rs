#![allow(dead_code)]

use crate::renderer::*;
use crate::c_helper::*;
use crate::markdown::md_to_html;
use crate::layout::render_html;
use crate::layout::Context;
use crate::style::style_for_element;

use std::panic;

#[no_mangle]
pub extern "C" fn mdl_test_renderer(rdev_ptr: *mut C_RenderDevice, text: *const c_char) {
    let mut rdev = RenderDevice::new(rdev_ptr);
    let input = match cstring_to_str(text) {
        Ok(s) => md_to_html(s),
        Err(..) => "".to_string(),
    };

    let result = panic::catch_unwind(move || {
        render_html(input.as_str(), rdev);
    });

    if result.is_err() {
        println!("Rust error: {:?}", result);
    }
}

use crate::dom::*;
use std::ops::Index;

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

/*
pub fn test_dom() {
    let input = "<p>Hello, <em>world!</em> How is it going?</p><strong><em><span>test</span></em></strong>";

    let document = Document::parse_html(input.as_bytes());

    process_nodes(document.root_element(), &document)
}
*/

pub fn test_dom() {
    let input = "<p>The <strong>quick <span style='color:brown'>brown</span>
        <em>fox</em></strong> jumps over the <span style='color:#0000ff80'>lazy dog.</span><br>
        The quick brown fox.</p>";

    let document = Document::parse_html(input.as_bytes());
    let author_styles = &document.parse_stylesheets();
    let context = Context {
        document: &document,
        author_styles,
    };
    let root_element = document.root_element();
    println!("Hello world");
    // this next line currently crashes
    let style = style_for_element(context.author_styles, context.document, root_element, None);
}