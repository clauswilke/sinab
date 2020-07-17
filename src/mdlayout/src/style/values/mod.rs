use crate::style::errors::PropertyParseError;
use crate::style::properties::ComputedValues;
use crate::style::properties::{ComputedValuesForEarlyCascade, ComputedValuesForLateCascade};
use cssparser::Parser;

mod background;
mod border;
mod box_;
mod color;
mod fonts;
mod fontfamily;
mod generic;
mod length;
mod writing_modes;
mod tests; // testing module for values

pub(super) use self::{background::*, generic::*};
pub(crate) use self::{border::*, box_::*, color::*, fonts::*, fontfamily::*, length::*, writing_modes::*};

pub(super) trait Parse: Sized {
    fn parse<'i, 't>(parser: &mut Parser<'i, 't>) -> Result<Self, PropertyParseError<'i>>;
}

pub(super) struct CascadeContext<'a> {
    pub inherited: &'a ComputedValues,
    pub this: ComputedValuesForLateCascade<'a>,
}

/// The cascade is broken into an early and a regular/late part because some properties
/// need to be calculated before others. In particular, if an element specifies a font size
/// and also some other length relative to the font size (i.e., in em), the font size for the
/// element needs to be calculated before the other length.
pub(super) struct EarlyCascadeContext<'a> {
    pub inherited: &'a ComputedValues,
    pub this: ComputedValuesForEarlyCascade<'a>,
}

pub(super) trait SpecifiedValue {
    type SpecifiedValue;
}

pub(super) trait FromSpecified: SpecifiedValue {
    fn from_specified(specified: &Self::SpecifiedValue, context: &CascadeContext) -> Self;
}

pub(super) trait EarlyFromSpecified: SpecifiedValue {
    fn early_from_specified(
        specified: &Self::SpecifiedValue,
        context: &EarlyCascadeContext,
    ) -> Self;
}

#[derive(Copy, Clone, Debug, Parse)]
pub(super) enum CssWideKeyword {
    Inherit,
    Initial,
    Unset,
}
