use std::fmt::{Display, Formatter, Result};

/// Origin at top-left corner, unit `1px`
pub struct CssPx;

pub use euclid::point2 as point;
pub use euclid::rect;
pub type Length<U> = euclid::Length<f32, U>;
pub type Point<U> = euclid::Point2D<f32, U>;
pub type Size<U> = euclid::Size2D<f32, U>;
pub type Rect<U> = euclid::Rect<f32, U>;
pub type SideOffsets<U> = euclid::SideOffsets2D<f32, U>;
pub type Scale<Src, Dest> = euclid::Scale<f32, Src, Dest>;

#[derive(Copy, Clone, PartialEq)]
pub struct RGBA(pub u8, pub u8, pub u8, pub u8);

impl From<cssparser::RGBA> for RGBA {
    fn from(c: cssparser::RGBA) -> Self {
        RGBA(c.red, c.green, c.blue, c.alpha)
    }
}

impl Display for RGBA {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.3 == 255 { // without alpha component
            write!(f, "#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
        } else { // with alpha component
            write!(f, "#{:02x}{:02x}{:02x}{:02x}", self.0, self.1, self.2, self.3)
        }
    }
}


/*
// This should probably live elsewhere, to create a better dependency graph without cycles

use crate::text;

pub struct TextRun<'a> {
    pub segment: &'a text::ShapedSegment,
    pub font_size: Length<CssPx>,
    pub origin: Point<CssPx>,
}
*/
