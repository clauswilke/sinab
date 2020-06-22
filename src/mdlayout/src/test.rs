use super::renderer::*;
use std::str;

pub enum InlineBoxContent<'a> {
    Space,
    Linebreak,
    Text(&'a str),
}

struct InlineBox<'a> {
    pub content: InlineBoxContent<'a>,
    pub width: f64,
}

fn string_manip(input: &str, rdev: &mut RenderDevice) {
    let gc = GContext::new();
    let m = rdev.string_metrics(" ", &gc);
    let w_space = m.width;

    let mut inline_boxes: Vec<InlineBox> = Vec::new();

    for line in input.lines() {
        for word in line.split(" ") {
            // words of length 0 arise from repeated spaces
            if word.len() > 0 {
                // push word, then space
                let m = rdev.string_metrics(word, &gc);
                let space = m.width;
                let b = InlineBox {
                    content: InlineBoxContent::Text(word),
                    width: space,
                };
                inline_boxes.push(b);
                let b = InlineBox {
                    content: InlineBoxContent::Space,
                    width: w_space,
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
        };
        inline_boxes.push(b);
    }

    let x0 = 0.2;
    let y0 = 0.5;
    let lineheight = 0.5;
    let mut x = 0.0;
    let mut y = 0.0;
    for b in inline_boxes {
        match b.content {
            InlineBoxContent::Space => {
                x += b.width;
            },
            InlineBoxContent::Linebreak=> {
                x = 0.0;
                y += lineheight;
            },
            InlineBoxContent::Text(word) => {
                rdev.draw_text(word, x0 + x, y0 + y, &gc);
                x += b.width;
            }
        }
    }
}

/*
fn make_grobs(rdev: &mut RenderDevice) {
    let gc = GContext::new();
    let mut m = rdev.string_metrics(" ", &gc);
    let w_space = m.width;
    let mut x = 0.2;
    let y = 0.5;
    rdev.draw_text("These", x, y, &gc);
    m = rdev.string_metrics("These", &gc);
    x += w_space + m.width;
    let mut gc2 = gc.copy();
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
*/

#[no_mangle]
pub extern "C" fn test_renderer(rdev_ptr: *mut C_RenderDevice) {
    let mut rdev = RenderDevice::new(rdev_ptr);
    //make_grobs(&mut rdev);

    string_manip("This is a test.\n And some more.", &mut rdev)
}


// keep an empty test here for now as a reminder to write proper unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
