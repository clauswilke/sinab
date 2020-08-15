use super::{Length, SpecifiedLength, Percentage, SpecifiedValue};

#[derive(Clone, Debug, Parse, FromVariants)]
pub(in crate::style) enum SpecifiedVerticalAlign {
    Baseline,
    Sub,
    Super,
    Top,
    TextTop,
    Middle,
    Bottom,
    TextBottom,
    Length(SpecifiedLength),
    Percentage(Percentage),
}

#[derive(Clone, Debug, FromSpecified, FromVariants)]
pub(crate) enum VerticalAlign {
    Baseline,
    Sub,
    Super,
    Top,
    TextTop,
    Middle,
    Bottom,
    TextBottom,
    Length(Length),
    Percentage(Percentage),
}
