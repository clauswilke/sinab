use crate::dom::*;
use crate::geom::physical::*;
use crate::geom::Length;
use crate::layout::{BoxFragment, Fragment};
use crate::primitives::{CssPx, Size, Point, RGBA};
use crate::graphics_engine::renderer::RenderDevice;
use crate::style::style_for_element;
use crate::style::values::LineStyle;

impl crate::dom::Document {
    pub fn paint_onto(&self, rdev: &mut RenderDevice, user_css: Option<&str>, page_size: Size<CssPx>) {
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
                // record bounding box
                rdev.record_bbox(&rect);
                // draw children
                for child in &a.children {
                    child.paint_onto(rdev, &rect)
                }
            }
            Fragment::Text(t) => {
                let rect = t
                    .content_rect
                    .to_physical(t.parent_style.writing_mode(), containing_block)
                    .translate(&containing_block.top_left);
                let mut origin = rect.top_left.clone();

                // record bounding box
                rdev.record_bbox(&rect);

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
            }
        }
    }
}

impl BoxFragment {
    fn paint_onto(&self, rdev: &mut RenderDevice, containing_block: &Rect<Length>) {
        // `marging_rect` includes padding, borders, and margins
        let margin_rect = self
            .margin_rect()
            .to_physical(self.style.writing_mode(), containing_block)
            .translate(&containing_block.top_left);

        // `border_rect` includes both padding and borders
        let border_rect = self
            .border_rect()
            .to_physical(self.style.writing_mode(), containing_block)
            .translate(&containing_block.top_left);

        // `padding_rect` includes only padding but not the borders
        let padding_rect = self
            .padding_rect()
            .to_physical(self.style.writing_mode(), containing_block)
            .translate(&containing_block.top_left);

        // `contenct_rect` excludes padding and borders
        let content_rect = self
            .content_rect
            .to_physical(self.style.writing_mode(), containing_block)
            .translate(&containing_block.top_left);

        // bounding box
        rdev.record_bbox(&margin_rect);

        // background
        let background_color = self.style.to_rgba(self.style.background.background_color);
        if background_color.alpha > 0 {
            rdev.draw_rect(
                border_rect.top_left.x.into(),
                border_rect.top_left.y.into(),
                border_rect.size.x.into(),
                border_rect.size.y.into(),
                background_color.into(),
            );
        }

        // borders
        // calculate widths by subtracting border rect from padding rect
        // (Note: y goes down, so y values need to be subtracted the other way)
        let border_top_width = padding_rect.top_left.y - border_rect.top_left.y;
        let border_right_width = border_rect.top_left.x + border_rect.size.x
            - padding_rect.top_left.x - padding_rect.size.x;
        let border_bottom_width = border_rect.top_left.y + border_rect.size.y
            - padding_rect.top_left.y - padding_rect.size.y;
        let border_left_width = padding_rect.top_left.x - border_rect.top_left.x;

        if border_top_width > Length::zero() &&
            self.style.border.border_top_style != LineStyle::None {
            let mut v: Vec<Vec2<Length>> = Vec::with_capacity(2);
            let mut p1 = border_rect.top_left.clone();
            // adjust midpoint by half linewidth
            p1.y += border_top_width / 2.0;
            let mut p2 = p1.clone();
            p2.x += border_rect.size.x;
            v.push(p1);
            v.push(p2);
            let border_color = self.style.to_rgba(self.style.border.border_top_color);
            rdev.draw_line(
                &v, border_color.into(), border_top_width, self.style.border.border_top_style
            );
        }

        if border_right_width > Length::zero() &&
            self.style.border.border_right_style != LineStyle::None {
            let mut v: Vec<Vec2<Length>> = Vec::with_capacity(2);
            let mut p1 = border_rect.top_left.clone();
            // adjust midpoint by half linewidth
            p1.x += border_rect.size.x - border_right_width / 2.0;
            let mut p2 = p1.clone();
            p2.y += border_rect.size.y;
            v.push(p1);
            v.push(p2);
            let border_color = self.style.to_rgba(self.style.border.border_right_color);
            rdev.draw_line(
                &v, border_color.into(), border_right_width, self.style.border.border_right_style
            );
        }

        if border_bottom_width > Length::zero() &&
            self.style.border.border_bottom_style != LineStyle::None {
            let mut v: Vec<Vec2<Length>> = Vec::with_capacity(2);
            let mut p1 = border_rect.top_left.clone();
            // adjust midpoint by half linewidth
            p1.y += border_rect.size.y - border_bottom_width / 2.0;
            let mut p2 = p1.clone();
            p2.x += border_rect.size.x;
            v.push(p2);
            v.push(p1);
            let border_color = self.style.to_rgba(self.style.border.border_bottom_color);
            rdev.draw_line(
                &v, border_color.into(), border_bottom_width, self.style.border.border_bottom_style
            );
        }

        if border_left_width > Length::zero() &&
            self.style.border.border_left_style != LineStyle::None {
            let mut v: Vec<Vec2<Length>> = Vec::with_capacity(2);
            let mut p1 = border_rect.top_left.clone();
            // adjust midpoint by half linewidth
            p1.x += border_left_width / 2.0;
            let mut p2 = p1.clone();
            p2.y += border_rect.size.y;
            v.push(p2);
            v.push(p1);
            let border_color = self.style.to_rgba(self.style.border.border_left_color);
            rdev.draw_line(
                &v, border_color.into(), border_left_width, self.style.border.border_left_style
            );
        }

        // content
        for child in &self.children {
            child.paint_onto(rdev, &content_rect)
        }
    }
}


pub fn render_html(text_input: &str, css_input: &str, mut rdev: RenderDevice, page_size: Size<CssPx>) {
    let document = Document::parse_html(text_input.as_bytes());
    document.paint_onto(&mut rdev, Some(css_input), page_size);
}