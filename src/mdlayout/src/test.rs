#![allow(dead_code)]

use super::renderer::*;
use std::str;
use std::rc::Rc;
use std::ops::Deref;

pub enum InlineBoxContent<'a> {
    Space,
    Linebreak,
    Text(&'a str),
}

struct InlineBox<'a> {
    pub content: InlineBoxContent<'a>,
    pub width: f64,
    pub gc: Rc<GContext>,
}

fn string_manip(input: &str, rdev: &mut RenderDevice) {
    let gc = Rc::new(GContext::new());
    let fm = rdev.font_metrics(&gc);
    let mut inline_boxes: Vec<InlineBox> = Vec::new();

    let mut i = 0;
    for line in input.lines() {
        for word in line.split(" ") {
            i += 1;
            // words of length 0 arise from repeated spaces
            if word.len() > 0 {
                let mut gc_new = gc.clone();
                if i == 3 {
                    let mut gc_tmp = gc.deref().clone();
                    gc_tmp.set_color("red");
                    gc_tmp.set_fontsize(34.0);
                    gc_tmp.set_fontface(Fontface::Italics);
                    gc_new = Rc::new(gc_tmp);
                }

                // push word, then space
                let m = rdev.string_metrics(word, &gc_new);
                let b = InlineBox {
                    content: InlineBoxContent::Text(word),
                    width: m.width,
                    gc: gc_new,
                };
                inline_boxes.push(b);
                let b = InlineBox {
                    content: InlineBoxContent::Space,
                    width: fm.space_width,
                    gc: gc.clone(),
                };
                inline_boxes.push(b);
            }
        }
        // remove last box if it is a space before adding newline
        if let Some(b) = inline_boxes.last() {
            if let InlineBoxContent::Space = b.content {
                inline_boxes.pop();
            }
        }
        let b = InlineBox {
            content: InlineBoxContent::Linebreak,
            width: 0.0,
            gc: gc.clone(),
        };
        inline_boxes.push(b);
    }

    let x0 = 0.2;
    let y0 = 0.5;
    let linespacing = fm.linespacing;
    let mut x = 0.0;
    let mut y = 0.0;
    for b in inline_boxes {
        match b.content {
            InlineBoxContent::Space => {
                x += b.width;
            },
            InlineBoxContent::Linebreak=> {
                x = 0.0;
                y += linespacing;
            },
            InlineBoxContent::Text(word) => {
                rdev.draw_text(word, x0 + x, y0 + y, &b.gc);
                x += b.width;
            }
        }
    }
}


fn make_grobs(rdev: &mut RenderDevice) {
    let gc = GContext::new();
    let mut m = rdev.string_metrics(" ", &gc);
    let w_space = m.width;
    let mut x = 0.2;
    let y = 0.5;
    rdev.draw_text("These", x, y, &gc);
    m = rdev.string_metrics("These", &gc);
    x += w_space + m.width;
    let mut gc2 = gc.clone();
    gc2.set_color("red");
    gc2.set_fontface(Fontface::Bold);
    rdev.draw_text("grobs", x, y, &gc2);
    m = rdev.string_metrics("grobs", &gc2);
    x += w_space + m.width;
    rdev.draw_text("were made", x, y, &gc);
    m = rdev.string_metrics("were made", &gc);
    x += w_space + m.width;
    gc2.set_color("blue");
    gc2.set_fontface(Fontface::Italics);
    rdev.draw_text("in rust.", x, y, &gc2);
}


fn test_gc() {
    let mut gc = GContext::new();

    gc.set_color("blue");
    gc.set_fontfamily("Times New Roman");
    gc.set_fontface(Fontface::BoldItalics);
    gc.set_lineheight(2.10);

    println!("color: {}", gc.color());
    println!("fill: {}", gc.fill());
    println!("fontfamily: {}", gc.fontfamily());
    println!("fontface: {}", gc.fontface());
    println!("fontsize: {}", gc.fontsize());
    println!("lineheight: {}", gc.lineheight());
}

#[no_mangle]
pub extern "C" fn test_renderer(rdev_ptr: *mut C_RenderDevice) {
    let mut rdev = RenderDevice::new(rdev_ptr);
    //make_grobs(&mut rdev);
    //test_gc();

    string_manip("This is one nice test.\n And some more.", &mut rdev)
}


// keep an empty test here for now as a reminder to write proper unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}