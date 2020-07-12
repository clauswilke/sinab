#[cfg(test)]
mod tests {
    use crate::style::values::*;
    use cssparser::{Parser, ParserInput, Color, RGBA};

    /// Parse css expressions corresponding to values of the given type:
    /// `parse_value!(css, Type)`
    macro_rules! parse_value {
        ($css:expr, $value_type:ty) => {
            {
                let mut parser_input = ParserInput::new($css);
                let mut input = Parser::new(&mut parser_input);
                match <$value_type>::parse(&mut input) {
                    Ok(value) => value,
                    Err(_) => panic!("Error parsing css value of type {}", stringify!($value_type)),
                }
            }
        };
    }

    #[test]
    fn test_color() {
        if let Color::RGBA(rgba) = parse_value!("green", Color) {
            assert_eq!(rgba, RGBA::new(0, 128, 0, 255));
        } else {
            assert!(false);
        }

        if let Color::RGBA(rgba) = parse_value!("#808080", Color) {
            assert_eq!(rgba, RGBA::new(128, 128, 128, 255));
        } else {
            assert!(false);
        }

        if let Color::RGBA(rgba) = parse_value!("#ff000080", Color) {
            assert_eq!(rgba, RGBA::new(255, 0, 0, 128));
        } else {
            assert!(false);
        }

        assert_eq!(parse_value!("currentcolor", Color), Color::CurrentColor);
    }

    #[test]
    fn test_specified_length() {
        if let SpecifiedLength::Em(value) = parse_value!("2em", SpecifiedLength) {
            assert_eq!(value, 2.0);
        } else {
            assert!(false);
        }

        let one_inch = parse_value!("96px", SpecifiedLength);

        if let SpecifiedLength::Absolute(Length{ px: value }) = one_inch {
            assert_eq!(value, 96.0);
        } else {
            assert!(false);
        }

        assert_eq!(parse_value!("72pt", SpecifiedLength), one_inch);
        assert_eq!(parse_value!("1in", SpecifiedLength), one_inch);
        assert_eq!(parse_value!("2.54cm", SpecifiedLength), one_inch);
        assert_eq!(parse_value!("25.4mm", SpecifiedLength), one_inch);
        assert_eq!(parse_value!("6pc", SpecifiedLength), one_inch);
    }
}