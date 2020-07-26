use crate::geom::physical::{Rect, Vec2};
use crate::geom::Length;
use crate::layout::{BoxFragment, Fragment};
use crate::primitives::{CssPx, Size};
use crate::graphics_engine::renderer::RenderDevice;

impl crate::dom::Document {
    pub fn paint_onto(&self, mut rdev: &RenderDevice) {
        let font_manager = rdev.new_font_manager();
        let page_size: Size<CssPx> = Size::new(600., 800.);
        let fragments = self.layout(page_size, &font_manager);
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
    fn paint_onto(&self, mut rdev: &RenderDevice, containing_block: &Rect<Length>) {
        println!("{:?}", self);
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
                /*
                let ascender = t.parent_style.font.font_size * t.text.font.ascender();
                origin.y += ascender;

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
    fn paint_onto(&self, mut rdev: &RenderDevice, containing_block: &Rect<Length>) {
        /*
        let background_color = self.style.to_rgba(self.style.background.background_color);
        if background_color.alpha > 0 {
            page.set_color(&background_color.into());
            let rect = self
                .border_rect()
                .to_physical(self.style.writing_mode(), containing_block)
                .translate(&containing_block.top_left)
                .into();
            page.paint_rectangle(&rect);
        }

         */
        let content_rect = self
            .content_rect
            .to_physical(self.style.writing_mode(), containing_block)
            .translate(&containing_block.top_left);
        for child in &self.children {
            child.paint_onto(rdev, &content_rect)
        }
    }
}
