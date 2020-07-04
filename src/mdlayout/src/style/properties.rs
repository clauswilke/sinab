use cssparser::*;
use std::ops::Deref;

/// Type that holds a color value. Colors are kept as strings, so this is a simple
/// newtype alias for `cssparser::CowRcStr`.
#[derive(Clone,Debug)]
pub struct Color<'a>(CowRcStr<'a>);

impl<'i> Deref for Color<'i> {
    type Target = CowRcStr<'i>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


/// Parse a color value.
impl<'i> Color<'i> {
    pub fn parse<'t>(
        input: &mut Parser<'i, 't>
    ) -> Result<Self, ParseError<'i, ()>> {
        let location = input.current_source_location();
        match *input.next()? {
            Token::Ident(ref s) => Ok(Color(s.clone())),
            Token::Hash(ref s) => {
                let mut c = String::with_capacity(9);
                c.push('#');
                c.push_str(s);
                Ok(Self(CowRcStr::from(c)))
            },
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
#[derive(Copy,Clone,Debug,PartialOrd,PartialEq)]
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
}

/// Parse a dimension value.
impl Dimension {
    pub fn parse<'i, 't>(
        input: &mut Parser<'i, 't>
    ) -> Result<Self, ParseError<'i, ()>> {
        let location = input.current_source_location();
        match *input.next()? {
            Token::Number { .. } => Ok(Dimension::px(0.0)),
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

/// Convenience function for working with `DeclarationListParser`: Take
/// a string input containing one or more declarations, parse them, and return
/// a vector of parsed CSS properties.
pub fn parse_declaration_block(s: &str) -> Vec<CssProperty> {
    let mut parser_input = ParserInput::new(s);
    let mut input = Parser::new(&mut parser_input);

    DeclarationListParser::new(&mut input, CssPropertyParser)
        // we discard all declarations that weren't parsed correctly
        .filter_map(|x| { x.ok() })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::style::properties::*;
    //use std::borrow::Borrow;

    #[test]
    fn ignore_css_parse_errors() {
        let css = r#"color: red green;font-size:;"#;
        let result = parse_declaration_block(css);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn parse_color() {
        let css = r#"color: red; background: #00FF00; COLOR: #ff00ff00;"#;
        let mut result = parse_declaration_block(css);
        assert_eq!(result.len(), 3);
        assert_eq!(match result.pop().unwrap() {
                CssProperty::Color(ref s) => s.0.as_ref(),
                _ => ""
        }, "#ff00ff00");
        assert_eq!(match result.pop().unwrap() {
            CssProperty::Background(ref s) => s.0.as_ref(),
            _ => ""
        }, "#00FF00");
        assert_eq!(match result.pop().unwrap() {
            CssProperty::Color(ref s) => s.0.as_ref(),
            _ => ""
        }, "red");
    }

    #[test]
    fn parse_dimension() {
        let css = r#"
           font-size: 1cm;
           font-size: 10mm;
           font-size: 0.5in;
           font-size: 12px;
           font-size: 20.5pt;
           font-size: 8pc;
           font-size: 2em;
           font-size: 5ex;
           font-size: 0;"#;
        let mut result = parse_declaration_block(css);
        assert_eq!(result.len(), 9);
        assert_eq!(match result.pop().unwrap() {
            CssProperty::FontSize(d) => d,
            _ => Dimension::px(1.0),
        }, Dimension::px(0.0));
        assert_eq!(match result.pop().unwrap() {
            CssProperty::FontSize(d) => d,
            _ => Dimension::px(0.0),
        }, Dimension::ex(5.0));
        assert_eq!(match result.pop().unwrap() {
            CssProperty::FontSize(d) => d,
            _ => Dimension::px(0.0),
        }, Dimension::em(2.0));
        assert_eq!(match result.pop().unwrap() {
            CssProperty::FontSize(d) => d,
            _ => Dimension::px(0.0),
        }, Dimension::pc(8.0));
        assert_eq!(match result.pop().unwrap() {
            CssProperty::FontSize(d) => d,
            _ => Dimension::px(0.0),
        }, Dimension::pt(20.5));
        assert_eq!(match result.pop().unwrap() {
            CssProperty::FontSize(d) => d,
            _ => Dimension::px(0.0),
        }, Dimension::px(12.0));
        assert_eq!(match result.pop().unwrap() {
            CssProperty::FontSize(d) => d,
            _ => Dimension::px(0.0),
        }, Dimension::inch(0.5));
        assert_eq!(match result.pop().unwrap() {
            CssProperty::FontSize(d) => d,
            _ => Dimension::px(0.0),
        }, Dimension::mm(10.0));
        assert_eq!(match result.pop().unwrap() {
            CssProperty::FontSize(d) => d,
            _ => Dimension::px(0.0),
        }, Dimension::cm(1.0));
    }
}
