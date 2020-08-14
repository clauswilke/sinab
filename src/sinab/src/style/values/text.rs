#[derive(Copy, Clone, Debug, Parse, SpecifiedAsComputed, PartialEq)]
pub(crate) enum WhiteSpace {
    Normal,
    Nowrap,
    Pre,
    PreWrap,
    PreLine,
    BreakSpaces,
}

#[derive(Copy, Clone, Debug, Parse, SpecifiedAsComputed, PartialEq)]
pub(crate) enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}
