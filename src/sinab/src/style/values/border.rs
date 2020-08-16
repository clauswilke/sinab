use super::length::*;
use crate::style::errors::PropertyParseError;
use cssparser::{Color, Parser};

/// https://drafts.csswg.org/css-backgrounds/#typedef-line-style
#[derive(Copy, Clone, Debug, Parse, SpecifiedAsComputed, PartialEq)]
pub(crate) enum LineStyle {
    None,
    Solid,
    Dotted,
    Dashed,
}

#[derive(Copy, Clone, Debug, Parse)]
enum ParsedLineWidth {
    Thin,
    Medium,
    Thick,
    Other(SpecifiedLengthOrPercentage),
}

#[derive(Copy, Clone, Debug)]
pub(in crate::style) struct SpecifiedLineWidth(pub SpecifiedLengthOrPercentage);

#[derive(Copy, Clone, Debug, FromSpecified)]
pub(crate) struct LineWidth(pub LengthOrPercentage);

impl LineWidth {
    pub(in crate::style) const MEDIUM: Self =
        LineWidth(LengthOrPercentage::Length(Length { px: 3. }));

    pub(in crate::style) fn fixup(&mut self, style: LineStyle) {
        if let LineStyle::None = style {
            self.0 = LengthOrPercentage::Length(Length::zero())
        }
    }
}

impl super::Parse for SpecifiedLineWidth {
    fn parse<'i, 't>(parser: &mut Parser<'i, 't>) -> Result<Self, PropertyParseError<'i>> {
        let px = match ParsedLineWidth::parse(parser)? {
            ParsedLineWidth::Thin => 1.0,
            ParsedLineWidth::Medium => 3.0,
            ParsedLineWidth::Thick => 5.0,
            ParsedLineWidth::Other(value) => return Ok(SpecifiedLineWidth(value)),
        };
        Ok(SpecifiedLineWidth(
            SpecifiedLength::Absolute(Length { px }).into(),
        ))
    }
}

macro_rules! parse_one_or_more {
    ($type: ty { $( $field: ident, )+ }) => {
        impl crate::style::values::Parse for $type {
            fn parse<'i, 't>(parser: &mut Parser<'i, 't>)
                -> Result<Self, PropertyParseError<'i>>
            {
                let mut values = Self::default();
                let mut any = false;
                loop {
                    $(
                        if values.$field.is_none() {
                            if let Ok(value) = parser.r#try(crate::style::values::Parse::parse) {
                                values.$field = Some(value);
                                any = true;
                                continue
                            }
                        }
                    )+
                    break
                }
                if any {
                    Ok(values)
                } else {
                    Err(parser.new_error_for_next_token())
                }
            }
        }
    };
}

parse_one_or_more!(BorderSide {
    style,
    color,
    width,
});

#[derive(Debug, Default)]
pub(in crate::style) struct BorderSide {
    pub style: Option<LineStyle>,
    pub color: Option<Color>,
    pub width: Option<SpecifiedLineWidth>,
}

#[derive(Debug, Default)]
pub(in crate::style) struct BorderFourSides {
    pub style_top: Option<LineStyle>,
    pub style_right: Option<LineStyle>,
    pub style_bottom: Option<LineStyle>,
    pub style_left: Option<LineStyle>,
    pub color_top: Option<Color>,
    pub color_right: Option<Color>,
    pub color_bottom: Option<Color>,
    pub color_left: Option<Color>,
    pub width_top: Option<SpecifiedLineWidth>,
    pub width_right: Option<SpecifiedLineWidth>,
    pub width_bottom: Option<SpecifiedLineWidth>,
    pub width_left: Option<SpecifiedLineWidth>,
}


impl super::Parse for BorderFourSides {
    fn parse<'i, 't>(parser: &mut Parser<'i, 't>) -> Result<Self, PropertyParseError<'i>> {
        let border = BorderSide::parse(parser)?;
        Ok(BorderFourSides {
            style_top: border.style.clone(),
            style_right: border.style.clone(),
            style_bottom: border.style.clone(),
            style_left: border.style.clone(),
            color_top: border.color.clone(),
            color_right: border.color.clone(),
            color_bottom: border.color.clone(),
            color_left: border.color.clone(),
            width_top: border.width.clone(),
            width_right: border.width.clone(),
            width_bottom: border.width.clone(),
            width_left: border.width.clone(),
        })
    }
}
