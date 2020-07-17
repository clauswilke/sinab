use crate::style::values::{Parse};
use crate::style::errors::{PropertyParseError, PropertyParseErrorKind};
use cssparser::{Parser, Token};

// TODO: CSS allows for multiple fonts separated by comma

#[derive(Clone, Debug, SpecifiedAsComputed, PartialEq)]
pub enum FontFamily {
    FamilyName(String),
    GenericSerif,
    GenericSans,
    GenericCursive,
    GenericFantasy,
    GenericMonospace,
}

impl Parse for FontFamily {
    fn parse<'i, 't>(parser: &mut Parser<'i, 't>) -> Result<Self, PropertyParseError<'i>> {
        match parser.next()? {
            Token::Ident(ref s) => match_ignore_ascii_case!(s,
                "sans-serif" => Ok(FontFamily::GenericSans),
                "serif" => Ok(FontFamily::GenericSerif),
                "cursive" => Ok(FontFamily::GenericCursive),
                "fantasy" => Ok(FontFamily::GenericFantasy),
                "monospace" => Ok(FontFamily::GenericMonospace),
                _ => Ok(FontFamily::FamilyName(s.to_string())),
            ),
            Token::QuotedString(ref s) => {
                Ok(FontFamily::FamilyName(s.to_string()))
            },
            token => {
                let t = token.clone();
                Err(parser.new_unexpected_token_error(t))
            }
        }
    }
}
