#[derive(Copy, Clone, Debug, PartialEq, Parse)]
pub(crate) enum Direction {
    Ltr,
    Rtl,
}

#[derive(Copy, Clone, Debug, PartialEq, Parse)]
pub(crate) enum WritingMode {
    HorizontalTb,
    VerticalRl,
    VerticalLr,
    SidewaysRl,
    SidewaysLr,
}
