#![allow(dead_code)]

extern crate kuchiki;

use crate::renderer::*;
use crate::c_helper::*;
use crate::markdown::md_to_html;
use crate::style::properties::*;

use kuchiki::traits::*;
use kuchiki::NodeData::*;
use kuchiki::NodeRef;
use kuchiki::ElementData;

use std::str;
use std::cell::RefCell;
use std::option::Option;

pub enum InlineBoxContent {
    Space,
    Linebreak,
    Text(RefCell<String>),
}

struct InlineBox<'a> {
    pub content: InlineBoxContent,
    pub width: f64,
    pub linespacing: f64,
    pub gc: GContext<'a>,
}


fn make_text_boxes<'a>(
    boxes: &mut Vec<InlineBox<'a>>,
    text: &RefCell<String>,
    gc: &GContext<'a>,
    rdev: &mut RenderDevice
) {
    let fm = rdev.font_metrics(gc);

    // add a starting whitespace box if none exists yet
    let s = text.borrow();
    if s.starts_with(|x: char| x.is_ascii_whitespace()) {
        maybe_add_space(boxes, &fm, gc);
    }

    for word in s.split_ascii_whitespace() {
        // push word, then space
        let m = rdev.string_metrics(word, gc);
        let b = InlineBox {
            content: InlineBoxContent::Text(RefCell::new(word.to_string())),
            width: m.width,
            linespacing: fm.linespacing,
            gc: gc.clone(),
        };
        boxes.push(b);
        add_space(boxes, &fm, gc);
    }

    // remove final space if string doesn't end with whitespace
    if !s.ends_with(|x: char| x.is_ascii_whitespace()) {
        maybe_remove_space(boxes);
    }
}

/// Unconditionally add a space box
fn add_space<'a>(boxes: &mut Vec<InlineBox<'a>>, fm: &FontMetrics, gc: &GContext<'a>) {
    let b = InlineBox {
        content: InlineBoxContent::Space,
        width: fm.space_width,
        linespacing: fm.linespacing,
        gc: gc.clone(),
    };

    boxes.push(b);
}

/// Add space only if current box list doesn't end in a space.
/// Never adds a space to an empty box list or after a linebreak.
fn maybe_add_space<'a>(boxes: &mut Vec<InlineBox<'a>>, fm: &FontMetrics, gc: &GContext<'a>) {
    if let Some(b) = boxes.last() {
        match b.content {
            InlineBoxContent::Space => {},
            InlineBoxContent::Linebreak => {},
            _ => add_space(boxes, fm, gc),
        }
    }
}

/// Remove last box if it is a space box
fn maybe_remove_space(boxes: &mut Vec<InlineBox>) {
    if let Some(b) = boxes.last() {
        if let InlineBoxContent::Space = b.content {
            boxes.pop();
        }
    }
}

/// Add a newline box. First removes a last space if it exists.
fn add_newline<'a>(boxes: &mut Vec<InlineBox<'a>>, gc: &GContext<'a>, rdev: &mut RenderDevice) {
    let fm = rdev.font_metrics(gc);

    maybe_remove_space(boxes);

    let b = InlineBox {
        content: InlineBoxContent::Linebreak,
        width: 0.0,
        linespacing: fm.linespacing,
        gc: gc.clone(),
    };
    boxes.push(b);
}

fn apply_style_attribute<'a>(elt: &ElementData, gc: &GContext<'a>) -> Option<GContext<'a>> {
    if let Some(css) = elt.attributes.borrow().get("style") {
        let result = parse_declaration_block(css);
        if result.len() > 0 {
            let mut gc_new = gc.clone();
            for decl in result.iter() {
                match decl {
                    CssProperty::Color(ref s) => {
                        gc_new.set_color(s.as_ref());
                    }
                    _ => {}
                }
            }
            return Some(gc_new);
        }
    }
    None
}

fn process_node<'a>(
    boxes: &mut Vec<InlineBox<'a>>,
    node: &NodeRef,
    gc: &GContext<'a>,
    rdev: &mut RenderDevice
) {
    let mut gc_opt:Option<GContext> = Option::None;

    match node.data() {
        Element(elt) => {
            let name = &elt.name.local;
            match name.as_ref() {
                "em" => {
                    let mut gc_new = gc.clone();
                    gc_new.modify_fontface(Fontface::Italics);
                    gc_opt = Some(gc_new.clone());
                    if let Some(g) = apply_style_attribute(elt, &gc_new) {
                        gc_opt = Some(g);
                    }
                },
                "span" => {
                    if let Some(g) = apply_style_attribute(elt, gc) {
                        gc_opt = Some(g);
                    }
                },
                "strong" => {
                    let mut gc_new = gc.clone();
                    gc_new.modify_fontface(Fontface::Bold);
                    gc_opt = Some(gc_new.clone());
                    if let Some(g) = apply_style_attribute(elt, &gc_new) {
                        gc_opt = Some(g);
                    }
                },
                "br" => add_newline(boxes, gc, rdev),
                _ => {},
            }
        },
        Text(text) => {
            make_text_boxes(boxes, text, gc, rdev);
        },
        _ => {},
    }

    if let Some(g) = gc_opt {
        for child in node.children() {
            process_node(boxes, &child, &g, rdev);
        }
    } else {
        for child in node.children() {
            process_node(boxes, &child, gc, rdev);
        }
    }
}

fn render_inline_boxes(inline_boxes: &Vec<InlineBox>, rdev: &mut RenderDevice) {
    let x0 = 0.2;
    let y0 = 0.5;
    let mut x = 0.0;
    let mut y = 0.0;
    for b in inline_boxes {
        match &b.content {
            InlineBoxContent::Space => {
                x += b.width;
            },
            InlineBoxContent::Linebreak=> {
                x = 0.0;
                y += b.linespacing;
            },
            InlineBoxContent::Text(word) => {
                rdev.draw_text(&word.borrow(), x0 + x, y0 + y, &b.gc);
                x += b.width;
            }
        }
    }
}

fn render_html(input: &str, rdev: &mut RenderDevice) {
    let mut inline_boxes: Vec<InlineBox> = Vec::new();
    let gc = GContext::new();
    let document = kuchiki::parse_html().one(input);

    process_node(&mut inline_boxes, &document, &gc, rdev);
    render_inline_boxes(&inline_boxes, rdev);
}

fn test() {
    /*
    println!("1");
    let mut gc = GContext::new();
    println!("2 {}", gc.color());
    gc.set_color("red");
    println!("3 {}", gc.color());
    let mut gc2 = gc.clone();
    println!("4 {}", gc2.color());
    gc2.set_color("green");
    gc2.set_fontsize(15.0);
    println!("5 {} {}", gc2.color(), gc2.fontsize());
    gc2.set_color("blue");
    let gc3 = gc2.clone();
    println!("6 {} {}", gc2.color(), gc3.color());
*/

    let mut gc_opt1:Option<GContext> = Option::None;
    let mut gc_opt2:Option<GContext> = Option::None;
    let gc1 = GContext::new();
    gc_opt1 = Some(gc1.clone());
    gc_opt2 = Some(gc1.clone());
    println!("{}", gc_opt1.unwrap().color());
    println!("{}", gc_opt2.unwrap().color());
}

#[no_mangle]
pub extern "C" fn mdl_test_renderer(rdev_ptr: *mut C_RenderDevice, text: *const c_char) {
    let mut rdev = RenderDevice::new(rdev_ptr);
    let input = match cstring_to_str(text) {
        Ok(s) => md_to_html(s),
        Err(..) => "".to_string(),
    };

    //render_html(input.as_str(), &mut rdev);

    test();
}
