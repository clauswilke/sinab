use crate::renderer::*;
use crate::style2::properties::*;

// for dom
use crate::dom::*;

use std::str;
use std::cell::RefCell;
use std::option::Option;

pub enum InlineBoxContent {
    Space,
    Linebreak,
    Text(RefCell<String>),
}

struct InlineBox {
    pub content: InlineBoxContent,
    pub width: f64,
    pub linespacing: f64,
    pub gc: GContext,
}


fn make_text_boxes(
    boxes: &mut Vec<InlineBox>,
    text: &str,
    gc: &GContext,
    rdev: &mut RenderDevice
) {
    let fm = rdev.font_metrics(gc);

    // add a starting whitespace box if none exists yet
    if text.starts_with(|x: char| x.is_ascii_whitespace()) {
        maybe_add_space(boxes, &fm, gc);
    }

    for word in text.split_ascii_whitespace() {
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
    if !text.ends_with(|x: char| x.is_ascii_whitespace()) {
        maybe_remove_space(boxes);
    }
}

/// Unconditionally add a space box
fn add_space(boxes: &mut Vec<InlineBox>, fm: &FontMetrics, gc: &GContext) {
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
fn maybe_add_space(boxes: &mut Vec<InlineBox>, fm: &FontMetrics, gc: &GContext) {
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
fn add_newline(boxes: &mut Vec<InlineBox>, gc: &GContext, rdev: &mut RenderDevice) {
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

fn apply_style_attribute(elt: &ElementData, gc: &GContext) -> Option<GContext> {
    if let Some(css) = elt.get_attr(&local_name!("style")) {
        let result = parse_declaration_block(css);
        if result.len() > 0 {
            let mut gc_new = gc.clone();
            for decl in result.iter() {
                match decl {
                    CssProperty::Color(ref color) => {
                        gc_new.set_color(color);
                    }
                    _ => {}
                }
            }
            return Some(gc_new);
        }
    }
    None
}

fn process_node(
    boxes: &mut Vec<InlineBox>,
    node_id: NodeId,
    document: &Document,
    gc: &GContext,
    rdev: &mut RenderDevice
) {
    let mut gc_opt:Option<GContext> = Option::None;

    let node = &document[node_id];
    match node.data {
        NodeData::Element(ref elt) => {
            match &elt.name.local {
                &local_name!("em") => {
                    let mut gc_new = gc.clone();
                    gc_new.modify_fontface(Fontface::Italics);
                    gc_opt = Some(gc_new.clone());
                    if let Some(g) = apply_style_attribute(elt, &gc_new) {
                        gc_opt = Some(g);
                    }
                },
                &local_name!("span") => {
                    if let Some(g) = apply_style_attribute(elt, gc) {
                        gc_opt = Some(g);
                    }
                },
                &local_name!("strong") => {
                    let mut gc_new = gc.clone();
                    gc_new.modify_fontface(Fontface::Bold);
                    gc_opt = Some(gc_new.clone());
                    if let Some(g) = apply_style_attribute(elt, &gc_new) {
                        gc_opt = Some(g);
                    }
                },
                &local_name!("br") => add_newline(boxes, gc, rdev),
                _ => {},
            }
        },
        NodeData::Text{ref contents} => {
            make_text_boxes(boxes, contents, gc, rdev);
        },
        _ => {},
    }

    let gc_final = match gc_opt {
        Some(g) => g,
        None => gc.clone(),
    };

    if let Some(child_id) = node.first_child {
        for nid in document.node_and_following_siblings(child_id) {
            process_node(boxes, nid, document, &gc_final, rdev);
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

pub fn render_html(input: &str, rdev: &mut RenderDevice) {
    let mut inline_boxes: Vec<InlineBox> = Vec::new();
    let gc = GContext::new();
    let document = Document::parse_html(input.as_bytes());

    process_node(&mut inline_boxes, document.root_element(), &document, &gc, rdev);
    render_inline_boxes(&inline_boxes, rdev);
}
