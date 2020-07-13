use crate::style::errors::PropertyParseError;
use crate::style::values::Parse;
use cssparser::Parser;

#[derive(Debug)]
pub(in crate::style) struct FourSides<T> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl<T> Parse for FourSides<T>
where
    T: Parse + Clone,
{
    fn parse<'i, 't>(parser: &mut Parser<'i, 't>) -> Result<Self, PropertyParseError<'i>> {
        let top = T::parse(parser)?;

        let right = if let Ok(right) = parser.r#try(T::parse) {
            right
        } else {
            return Ok(FourSides {
                top: top.clone(),
                right: top.clone(),
                bottom: top.clone(),
                left: top,
            });
        };

        let bottom = if let Ok(bottom) = parser.r#try(T::parse) {
            bottom
        } else {
            return Ok(FourSides {
                top: top.clone(),
                right: right.clone(),
                bottom: top,
                left: right,
            });
        };

        let left = if let Ok(left) = parser.r#try(T::parse) {
            left
        } else {
            return Ok(FourSides {
                top: top,
                left: right.clone(),
                bottom: bottom,
                right: right,
            });
        };

        Ok(FourSides {
            top,
            right,
            bottom,
            left,
        })
    }
}
