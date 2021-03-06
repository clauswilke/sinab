// this code is now obsolete and should eventually be deleted in its entirety
/*
use crate::graphics_engine::renderer::*;
use crate::graphics_engine::font::*;
use crate::style::values::*;
use crate::style::{style_for_element, StyleSet, ComputedValues};

use crate::primitives::RGBA;

// for dom
use crate::dom::*;

use std::str;
use std::cell::RefCell;
use crate::graphics_engine::shaped_segment::ShapedSegment;

pub struct LayoutContext<'a> {
    pub document: &'a Document,
    pub author_styles: &'a StyleSet,
}

pub enum InlineBoxContent {
    Space,
    Linebreak,
    Text(RefCell<String>),
}

struct InlineBox {
    pub content: InlineBoxContent,
    pub width: Length,
    pub linespacing: Length,
    pub font: Font,
    pub color: RGBA,
}


fn make_text_boxes(
    boxes: &mut Vec<InlineBox>,
    text: &str,
    font: &Font,
    color: RGBA,
) {
    // add a starting whitespace box if none exists yet
    if text.starts_with(|x: char| x.is_ascii_whitespace()) {
        maybe_add_space(boxes, font);
    }

    for word in text.split_ascii_whitespace() {
        // push word, then space

        let mut s = ShapedSegment::shape(word, font.clone()).unwrap();
        let b = InlineBox {
            content: InlineBoxContent::Text(RefCell::new(word.to_string())),
            width: s.get_advance_width().unwrap(),
            linespacing: font.calculate_linespacing(1.2),
            font: font.clone(),
            color: color,
        };
        boxes.push(b);
        add_space(boxes, font);
    }

    // remove final space if string doesn't end with whitespace
    if !text.ends_with(|x: char| x.is_ascii_whitespace()) {
        maybe_remove_space(boxes);
    }
}

/// Unconditionally add a space box
fn add_space(boxes: &mut Vec<InlineBox>, font: &Font) {
    let b = InlineBox {
        content: InlineBoxContent::Space,
        width: font.get_space_advance_width(),
        linespacing: font.calculate_linespacing(1.2),
        font: font.clone(),
        color: RGBA(0, 0, 0, 0),
    };

    boxes.push(b);
}

/// Add space only if current box list doesn't end in a space.
/// Never adds a space to an empty box list or after a linebreak.
fn maybe_add_space(boxes: &mut Vec<InlineBox>, font: &Font) {
    if let Some(b) = boxes.last() {
        match b.content {
            InlineBoxContent::Space => {},
            InlineBoxContent::Linebreak => {},
            _ => add_space(boxes, font),
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
fn add_newline(boxes: &mut Vec<InlineBox>, font: &Font) {
    maybe_remove_space(boxes);

    let b = InlineBox {
        content: InlineBoxContent::Linebreak,
        width: Length::zero(),
        linespacing: font.calculate_linespacing(1.2),
        font: font.clone(),
        color: RGBA(0, 0, 0, 0),
    };
    boxes.push(b);
}

fn apply_style_attributes(style: &ComputedValues, gc: &GContext) -> GContext {
    let mut gc_new = gc.clone();
    gc_new.set_color(style.color.color.into());
    gc_new.set_fontstyle(style.font.font_style);
    gc_new.set_fontweight(style.font.font_weight);
    gc_new.set_fontsize(style.font.font_size.0.into());
    let family = match &style.font.font_family {
        FontFamily::GenericSans => "sans",
        FontFamily::GenericSerif => "serif",
        FontFamily::GenericMonospace => "mono",
        FontFamily::FamilyName(ref s) => s.as_str(),
        _ => "sans", // use sans for Fantasy and Cursive
    };
    gc_new.set_fontfamily(family);
    gc_new
}

fn retrieve_font(style: &ComputedValues) -> Font {
    let family = match &style.font.font_family {
        FontFamily::GenericSans => "sans",
        FontFamily::GenericSerif => "serif",
        FontFamily::GenericMonospace => "mono",
        FontFamily::FamilyName(ref s) => s.as_str(),
        _ => "sans", // use sans for Fantasy and Cursive
    };

    Font::new(
        family,
        style.font.font_style,
        style.font.font_weight,
        style.font.font_size.0.into()
    )
}

fn process_node<'dom>(
    boxes: &mut Vec<InlineBox>,
    node_id: NodeId,
    parent_element_style: &ComputedValues,
    context: &'dom LayoutContext
) {
    let style = style_for_element(
        context.author_styles,
        context.document,
        node_id,
        Some(parent_element_style),
    );

    let font = retrieve_font(&style);

    let node = &context.document[node_id];
    match node.data {
        NodeData::Element(ref elt) => {
            match &elt.name.local {
                &local_name!("br") => add_newline(boxes, &font),
                _ => {},
            }
        },
        NodeData::Text{ref contents} => {
            make_text_boxes(boxes, contents, &font, style.color.color.into());
        },
        _ => {},
    }

    if let Some(child_id) = node.first_child {
        for nid in context.document.node_and_following_siblings(child_id) {
            process_node(
                boxes,
                nid,
                &style,
                context
            );
        }
    }
}

fn render_inline_boxes(inline_boxes: &Vec<InlineBox>, rdev: &mut RenderDevice) {
    let x0 = Length{ px: 0.2 * 96.0 };
    let y0 = Length{ px: 2.0 * 96.0 };
    let mut x = Length::zero();
    let mut y = Length::zero();
    for b in inline_boxes {
        match &b.content {
            InlineBoxContent::Space => {
                x += b.width;
            },
            InlineBoxContent::Linebreak=> {
                x = Length::zero();
                y += b.linespacing;
            },
            InlineBoxContent::Text(word) => {
                rdev.draw_text(&word.borrow(), x0 + x, y0 + y, &b.font, b.color);
                x += b.width;
            }
        }
    }
}

pub fn render_html(text_input: &str, css_input: &str, mut rdev: RenderDevice) {
    let mut inline_boxes: Vec<InlineBox> = Vec::new();
    let document = Document::parse_html(text_input.as_bytes());
    let author_styles = &document.parse_stylesheets(Some(css_input));
    let context = LayoutContext {
        document: &document,
        author_styles,
    };
    let root_element = document.root_element();
    let style = style_for_element(context.author_styles, context.document, root_element, None);

    // new layouting pipeline
    document.paint_onto(&mut rdev, Some(css_input));

    // old layouting pipeline
    process_node(&mut inline_boxes, document.root_element(), &style, &context);
    render_inline_boxes(&inline_boxes, &mut rdev);
}
*/