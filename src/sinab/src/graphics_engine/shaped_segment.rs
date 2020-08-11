use crate::graphics_engine::font::{Font, FontError};
use crate::style::values::Length;

/// A `ShapedSegment` contains a piece of shaped text in a given font. For now, though,
/// no text shaping is actually done here. We simply hand the text to the `Font` object
/// which has a function to provide the width of the shaped segment.

#[derive(Clone)]
pub(crate) struct ShapedSegment {
    pub(crate) font: Font,
    pub(crate) glyphs: String,
    // if we did actual font shaping we'd want to keep track of the width as we go,
    // but for now we just calculate it on demand; we use an Option to cache values
    // we've calculated
    pub(crate) advance_width: Option<Length>,
}

pub(crate) struct ShapedSegmentState {
    glyphs: usize,
    advance_width: Option<Length>,
}

impl std::fmt::Debug for ShapedSegment {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("ShapedSegment: ")?;
        fmt.write_str(&self.glyphs)
    }
}

impl ShapedSegment {
    pub(crate) fn shape(text: &str, font: Font) -> Result<Self, FontError> {
        let mut s = Self::new(font);
        s.append(text.chars())?;
        Ok(s)
    }

    pub(crate) fn new(font: Font) -> Self {
        Self {
            font,
            glyphs: String::new(),
            advance_width: None,
        }
    }

    pub(crate) fn append(&mut self, mut text: impl Iterator<Item = char>) -> Result<(), FontError> {
        text.try_for_each(|ch| self.append_char(ch))
    }

    pub(crate) fn append_char(&mut self, ch: char) -> Result<(), FontError> {
        // if we did actual shaping, this would look up metrics for each glyph and
        // calculate current advance width
        //let id = self.font.glyph_id(ch)?;
        //self.advance_width += self.font.glyph_width(id)?;
        self.advance_width = None; // adding a char invalidates advance_width
        self.glyphs.push(ch);
        Ok(())
    }

    pub(crate) fn save(&self) -> ShapedSegmentState {
        ShapedSegmentState {
            glyphs: self.glyphs.len(),
            advance_width: self.advance_width,
        }
    }

    pub(crate) fn restore(&mut self, state: &ShapedSegmentState) {
        self.glyphs.truncate(state.glyphs);
        self.advance_width = state.advance_width;
    }

    pub(crate) fn get_advance_width(&mut self) -> Result<Length, FontError> {
        if let Some(l) = self.advance_width {
            Ok(l)
        } else {
            let sm = self.font.string_metrics(&self.glyphs);
            let width = sm.width;
            self.advance_width = Some(width);
            Ok(width)
        }
    }

    /// Returns `true` if the current shaped segment is empty, otherwise `false`.
    pub(crate) fn empty(&self) -> bool {
        self.glyphs.len() == 0
    }
}
