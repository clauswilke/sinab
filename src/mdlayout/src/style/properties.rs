use cssparser::*;

/// Type that holds a color value. Colors are kept as strings, so this is a simple
/// newtype alias for `cssparser::CowRcStr`.
#[derive(Clone,Debug)]
pub struct Color<'a>(CowRcStr<'a>);

/// Parse a color value.
impl<'i> Color<'i> {
    pub fn parse<'t>(
        input: &mut Parser<'i, 't>
    ) -> Result<Self, ParseError<'i, ()>> {
        let location = input.current_source_location();
        match *input.next()? {
            Token::Ident(ref s) => Ok(Color(s.clone())),
            Token::IDHash(ref s) => {
                let mut c = String::with_capacity(9);
                c.push('#');
                c.push_str(s);
                Ok(Self(CowRcStr::from(c)))
            },
            ref t => Err(location.new_unexpected_token_error(t.clone())),
        }
    }
}

/// Type that holds a CSS dimension (number with unit attached).
#[derive(Copy,Clone,Debug)]
#[allow(non_camel_case_types)]
pub enum Dimension {
    /// centimeters
    cm(f64),
    /// millimeters
    mm(f64),
    /// inches (1in = 96px = 2.54cm) (we write `inch` instead of `in` because `in` is  a reserved keyword)
    inch(f64),
    /// pixels (1px = 1/96th of 1in)
    px(f64),
    /// points (1pt = 1/72 of 1in)
    pt(f64),
    ///	picas (1pc = 12 pt)
    pc(f64),
    /// em (relative to the font-size of the element)
    em(f64),
    /// ex (relative to the x-height of the current font)
    ex(f64),
    /// percent (relative to the parent element, stored as fraction)
    percent(f64),
}

/// Parse a dimension value.
impl Dimension {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>
    ) -> Result<Self, ParseError<'i, ()>> {
        let location = input.current_source_location();
        match *input.next()? {
            Token::Number { .. } => Ok(Dimension::px(0.0)),
            Token::Percentage { unit_value, .. } => Ok(Dimension::percent(unit_value as f64)),
            Token::Dimension { has_sign, value, int_value, ref unit } =>
                match_ignore_ascii_case!(unit.as_ref(),
                    "cm" => Ok(Dimension::cm(value as f64)),
                    "mm" => Ok(Dimension::mm(value as f64)),
                    "in" => Ok(Dimension::inch(value as f64)),
                    "px" => Ok(Dimension::px(value as f64)),
                    "pt" => Ok(Dimension::pt(value as f64)),
                    "pc" => Ok(Dimension::pc(value as f64)),
                    "em" => Ok(Dimension::em(value as f64)),
                    "ex" => Ok(Dimension::ex(value as f64)),
                    _ => Err(location.new_unexpected_token_error(Token::Dimension{ has_sign, value, int_value, unit: unit.clone() } ))
                ),
            ref t => Err(location.new_unexpected_token_error(t.clone())),
        }
    }
}

#[derive(Clone,Debug)]
pub enum CssProperty<'i> {
    Background(Color<'i>),
    Color(Color<'i>),
    FontSize(Dimension),
    Other {
        name: CowRcStr<'i>,
        tokens: Vec<Token<'i>>,
    },
}


/// Parse a generic stream of tokens and return in a vector. Useful to parse any generic,
/// non-specified properties.
fn parse_tokens<'i, 't>(
    input: &mut Parser<'i, 't>
) -> Result<Vec<Token<'i>>, ParseError<'i, ()>> {
    let mut tokens = vec![];

    loop {
        if let Ok(token) = input.next() {
            tokens.push( token.clone());
        } else {
            break;
        }
    }

    if tokens.len() > 0 {
        Ok(tokens)
    } else {
        Err(input.new_error(BasicParseErrorKind::EndOfInput))
    }
}


/// A parser for individual declarations in a CSS declarations block, such as `color: red;`.
pub struct CssPropertyParser;

impl<'i> DeclarationParser<'i> for CssPropertyParser {
    type Declaration = CssProperty<'i>;
    type Error = ();

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, ()>> {
        Ok(match_ignore_ascii_case!(name.as_ref(),
            "background" => CssProperty::Background(Color::parse(input)?),
            "color" => CssProperty::Color(Color::parse(input)?),
            "font-size" => CssProperty::FontSize(Dimension::parse(input)?),
            _ => CssProperty::Other{name, tokens: parse_tokens(input)?},
        ))
    }
}

impl<'i> AtRuleParser<'i> for CssPropertyParser {
    type PreludeNoBlock = ();
    type PreludeBlock = ();
    type AtRule = CssProperty<'i>;
    type Error = ();
}
