use crate::dom::*;
use crate::geom::physical::{Rect, Vec2};
use crate::geom::Length;
use crate::layout::{BoxFragment, Fragment};
use crate::primitives::{CssPx, Size};
use crate::graphics_engine::renderer::RenderDevice;
use crate::style::style_for_element;

impl crate::dom::Document {
    pub fn paint_onto(&self, rdev: &mut RenderDevice, user_css: Option<&str>) {
        let page_size: Size<CssPx> = Size::new(400., 800.);
        let fragments = self.layout(page_size, user_css);
        let containing_block = Rect {
            top_left: Vec2 {
                x: Length::zero(),
                y: Length::zero(),
            },
            size: Vec2 {
                x: Length {
                    px: page_size.width,
                },
                y: Length {
                    px: page_size.height,
                },
            },
        };

        for fragment in fragments {
            fragment.paint_onto(rdev, &containing_block)
        }
    }
}

impl Fragment {
    fn paint_onto(&self, rdev: &mut RenderDevice, containing_block: &Rect<Length>) {
        //println!("{:?}", self);

        match self {
            Fragment::Box(b) => b.paint_onto(rdev, containing_block),
            Fragment::Anonymous(a) => {
                let rect = a
                    .rect
                    .to_physical(a.mode, containing_block)
                    .translate(&containing_block.top_left);
                for child in &a.children {
                    child.paint_onto(rdev, &rect)
                }
            }
            Fragment::Text(t) => {
                let mut origin = t
                    .content_rect
                    .to_physical(t.parent_style.writing_mode(), containing_block)
                    .translate(&containing_block.top_left)
                    .top_left;
                // Distance from top edge to baseline
                let ascender: Length = t.text.font.get_ascent().into();
                origin.y += ascender;

                rdev.draw_text(
                    &t.text.glyphs,
                    origin.x.into(),
                    origin.y.into(),
                    &t.text.font,
                    t.parent_style.color.color.into()
                );
                /*
                page.set_color(&t.parent_style.color.color.into());
                page.show_text(&TextRun {
                    segment: &t.text,
                    font_size: t.parent_style.font.font_size.0.into(),
                    origin: origin.into(),
                })
                .unwrap();

                 */
            }
        }
    }
}

impl BoxFragment {
    fn paint_onto(&self, rdev: &mut RenderDevice, containing_block: &Rect<Length>) {
        let background_color = self.style.to_rgba(self.style.background.background_color);
        if background_color.alpha > 0 {
            let rect = self
                .border_rect()
                .to_physical(self.style.writing_mode(), containing_block)
                .translate(&containing_block.top_left);

            rdev.draw_rect(
                rect.top_left.x.into(),
                rect.top_left.y.into(),
                rect.size.x.into(),
                rect.size.y.into(),
                background_color.into(),
            );
        }

        let content_rect = self
            .content_rect
            .to_physical(self.style.writing_mode(), containing_block)
            .translate(&containing_block.top_left);
        for child in &self.children {
            child.paint_onto(rdev, &content_rect)
        }
    }
}


pub fn render_html(text_input: &str, css_input: &str, mut rdev: RenderDevice) {
    let document = Document::parse_html(text_input.as_bytes());
    let author_styles = &document.parse_stylesheets(Some(css_input));
    let root_element = document.root_element();
    document.paint_onto(&mut rdev, Some(css_input));
}