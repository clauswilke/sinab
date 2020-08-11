#[derive(Copy, Clone, Debug, Parse, SpecifiedAsComputed, PartialEq)]
pub(crate) enum WhiteSpace {
    Normal,
    Nowrap,
    Pre,
    PreWrap,
    PreLine,
    BreakSpaces,
}
