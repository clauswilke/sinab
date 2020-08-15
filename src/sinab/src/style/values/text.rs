use super::{Percentage};

#[derive(Copy, Clone, Debug, Parse, SpecifiedAsComputed, PartialEq)]
pub(crate) enum WhiteSpace {
    Normal,
    Nowrap,
    Pre,
    PreWrap,
    PreLine,
    BreakSpaces,
}

#[derive(Copy, Clone, Debug, Parse, FromVariants)]
pub(in crate::style) enum SpecifiedTextAlign {
    Left,
    Right,
    Center,
    Justify,
    Percentage(Percentage),
}

#[derive(Copy, Clone, Debug, FromSpecified, FromVariants)]
pub(crate) enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
    // This is a non-standard extension that allows more fine-grained positioning
    // than Left, Right, Center. Left = 0%, Right = 100%, Center = 50%, etc.
    Percentage(Percentage),
}
