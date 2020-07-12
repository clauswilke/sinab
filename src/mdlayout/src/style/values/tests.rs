#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::{Parser, ParserInput, Color, RGBA};

    /// Parse css expressions corresponding to values of the given type:
    /// `parse_value!(css, Type)`
    macro_rules! parse_value {
        ($css:expr, $value_type:ty) => {
            {
                let mut parser_input = ParserInput::new($css);
                let mut input = Parser::new(&mut parser_input);
                <$value_type>::parse(&mut input).unwrap()
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
}