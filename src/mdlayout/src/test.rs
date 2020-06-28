extern crate kuchiki;

use super::renderer::*;
use super::c_helper::*;
use super::markdown::md_to_html;

use kuchiki::traits::*;
use kuchiki::NodeData::*;
use kuchiki::NodeRef;

use std::str;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

pub enum InlineBoxContent {
    Space,
    Linebreak,
    Text(RefCell<String>),
}

struct InlineBox {
    pub content: InlineBoxContent,
    pub width: f64,
    pub linespacing: f64,
    pub gc: Rc<GContext>,
}


fn make_text_boxes(
    boxes: &mut Vec<InlineBox>,
    text: &RefCell<String>,
    gc: &Rc<GContext>,
    rdev: &mut RenderDevice
) {
    let fm = rdev.font_metrics(&gc);

    // add a starting whitespace box if none exists yet
    let s = text.borrow();
    if s.starts_with(|x: char| x.is_ascii_whitespace()) {
        maybe_add_space(boxes, &fm, gc);
    }

    for word in s.split_ascii_whitespace() {
        // push word, then space
        let m = rdev.string_metrics(word, gc.deref());
        let b = InlineBox {
            content: InlineBoxContent::Text(RefCell::new(word.to_string())),
            width: m.width,
            linespacing: fm.linespacing,
            gc: gc.clone(),
        };
        boxes.push(b);
        add_space(boxes, &fm, &gc);
    }

    // remove final space if string doesn't end with whitespace
    if !s.ends_with(|x: char| x.is_ascii_whitespace()) {
        maybe_remove_space(boxes);
    }
}

/// Unconditionally add a space box
fn add_space(boxes: &mut Vec<InlineBox>, fm: &FontMetrics, gc: &Rc<GContext>) {
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
fn maybe_add_space(boxes: &mut Vec<InlineBox>, fm: &FontMetrics, gc: &Rc<GContext>) {
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
fn add_newline(boxes: &mut Vec<InlineBox>, gc: &Rc<GContext>, rdev: &mut RenderDevice) {
    let fm = rdev.font_metrics(&gc);

    maybe_remove_space(boxes);

    let b = InlineBox {
        content: InlineBoxContent::Linebreak,
        width: 0.0,
        linespacing: fm.linespacing,
        gc: gc.clone(),
    };
    boxes.push(b);
}


fn process_node(
    boxes: &mut Vec<InlineBox>,
    node: &NodeRef,
    gc: &Rc<GContext>,
    rdev: &mut RenderDevice
) {
    let mut gc_new = gc.clone();

    match node.data() {
        Element(elt) => {
            let name = &elt.name.local;
            match name.as_ref() {
                "em" => {
                    let mut tmp = gc.deref().clone();
                    tmp.modify_fontface(Fontface::Italics);
                    gc_new = Rc::new(tmp);
                },
                "strong" => {
                    let mut tmp = gc.deref().clone();
                    tmp.modify_fontface(Fontface::Bold);
                    gc_new = Rc::new(tmp);
                },
                "br" => add_newline(boxes, &gc_new, rdev),
                _ => {},
            }
        },
        Text(text) => {
            make_text_boxes(boxes, text, &gc_new, rdev);
        },
        _ => {},
    }

    for child in node.children() {
        process_node(boxes, &child, &gc_new, rdev);
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
    let gc = Rc::new(GContext::new());
    let document = kuchiki::parse_html().one(input);

    process_node(&mut inline_boxes, &document, &gc, rdev);
    render_inline_boxes(&inline_boxes, rdev);
}

#[no_mangle]
pub extern "C" fn mdl_test_renderer(rdev_ptr: *mut C_RenderDevice, text: *const c_char) {
    let mut rdev = RenderDevice::new(rdev_ptr);
    let input = match cstring_to_str(text) {
        Ok(s) => md_to_html(s),
        Err(..) => "".to_string(),
    };

    render_html(input.as_str(), &mut rdev);
}


// keep an empty test here for now as a reminder to write proper unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
