#![allow(dead_code)]

use libc::c_double;
use std::ffi::CString;

// for Rc implementation of GContext
use std::rc::Rc;
use std::ops::{Deref, DerefMut};

use crate::primitives::*;
use crate::graphics_engine::renderer::*;
use crate::style::values::{FontStyle, FontWeight};

#[derive(Copy, Clone)]
pub struct StringMetrics {
    pub ascent: Length<CssPx>,
    pub descent: Length<CssPx>,
    pub width: Length<CssPx>,
}


#[derive(Clone, Debug)]
pub(crate) struct FontImpl {
    name: String,
    style: FontStyle,
    weight: FontWeight,
    size: Length<CssPx>,
    ascent: Length<CssPx>,
    descent: Length<CssPx>,
    space_advance_width: Length<CssPx>,
    gc: GContext,
}

impl FontImpl {
    fn string_metrics_internal(label: &str, gc: &GContext) -> StringMetrics {
        let clabel = CString::new(label).unwrap();
        let mut cascent: c_double = 0.0;
        let mut cdescent: c_double = 0.0;
        let mut cwidth: c_double = 0.0;

        unsafe {
            rdev_string_metrics(
                clabel.as_ptr(),
                gc.as_ptr(),
                &mut cascent,
                &mut cdescent,
                &mut cwidth
            );
        }

        StringMetrics {
            // multiply with 96.0 to convert in to px
            ascent: Length::<CssPx>::new(96.0 * cascent as f32),
            descent: Length::<CssPx>::new(96.0 * cdescent as f32),
            width: Length::<CssPx>::new(96.0 * cwidth as f32),
        }
    }

    fn new(name: &str, style: FontStyle, weight: FontWeight, size: Length<CssPx>) -> FontImpl {
        let mut gc = GContext::new();
        gc.set_fontfamily(name);
        gc.set_fontstyle(style);
        gc.set_fontweight(weight);
        gc.set_fontsize(size);
        let m1 = FontImpl::string_metrics_internal("gjpqyQ", &gc);
        let m2 = FontImpl::string_metrics_internal(" ", &gc);

        FontImpl{
            name: name.to_string(),
            style,
            weight,
            size,
            ascent: m1.ascent,
            descent: m1.descent,
            space_advance_width: m2.width,
            gc
        }
    }


    pub(crate) fn string_metrics(&self, label: &str) -> StringMetrics {
        FontImpl::string_metrics_internal(label, &self.gc)
    }

    pub(crate) fn get_ascent(&self) -> Length<CssPx> {
        self.ascent
    }

    pub(crate) fn get_descent(&self) -> Length<CssPx> {
        self.descent
    }

    pub(crate) fn get_space_advance_width(&self) -> Length<CssPx> {
        self.space_advance_width
    }

    /// multiplies the font size with a scalar (e.g., 1.2) to calculate the line spacing
    pub(crate) fn calculate_linespacing(&self, lineheight: f32) -> Length<CssPx> {
        Length::<CssPx>::new(self.size.get() * lineheight)
    }

    pub(crate) fn graphics_context(&self) -> GContext {
        self.gc.clone()
    }
}

#[derive(Debug)]
pub(crate) struct Font(Rc<FontImpl>);

impl Font {
    pub(crate) fn new(name: &str, style: FontStyle, weight: FontWeight, size: Length<CssPx>) -> Font {
        Font(Rc::new(FontImpl::new(name, style, weight, size)))
    }
}

impl Clone for Font {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Deref for Font {
    type Target = FontImpl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Font {
    fn deref_mut(&mut self) -> &mut FontImpl {
        Rc::make_mut(&mut self.0)
    }
}

/// Enum to signal problems with fonts. Since we're not doing proper font handling
/// at this time, doesn't do much.
#[derive(Debug)]
pub enum FontError {
    /// Something's wrong.
    GeneralError,
}