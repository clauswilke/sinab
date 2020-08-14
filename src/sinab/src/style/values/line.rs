#[derive(Copy, Clone, Debug, Parse, SpecifiedAsComputed, PartialEq)]
pub(crate) enum VerticalAlign {
    Baseline,
    Sub,
    Super,
    Top,
    TextTop,
    Middle,
    Bottom,
    TextBottom,
    // TODO: Percentage and Length are missing
    // https://drafts.csswg.org/css2/#propdef-vertical-align
}
