#[cfg(test)]
mod selectors;

#[cfg(test)]
mod tests {
    use crate::style::values::*;
    use crate::style::properties::LonghandDeclaration;
    use crate::style::declaration_block::DeclarationBlock;
    use cssparser::{Parser, ParserInput, Color, RGBA};

    /// A simple macro to help with iterating through declarations,
    /// unstructuring them, and testing the inner variable for some value.
    macro_rules! validate_next_declaration {
        ($decl_iter:expr, $pattern:pat, $assert_expr:expr) => {
            match $decl_iter.next().unwrap() {
                $pattern => { $assert_expr; },
                _ => { assert!(false); },
            }
        }
    }


    #[test]
    fn test_declaration_block() {
        let css = "
            color: green;
            background: red;
            margin: 1in;
            padding: 10px 5px 0 15px;
            padding-top: 20px;";
        let mut parser_input = ParserInput::new(css);
        let mut input = Parser::new(&mut parser_input);
        let mut declarations = DeclarationBlock::parse(&mut input);

        /*
        for d in declarations.get_declarations() {
            println!("{:?}", d);
        }
        */

        assert_eq!(declarations.get_declarations().len(), 11);
        let mut decl_iter = declarations.get_declarations().into_iter();

        validate_next_declaration!(
            decl_iter,
            LonghandDeclaration::color(Color::RGBA(ref rgba)),
            assert_eq!(*rgba, RGBA::new(0, 128, 0, 255))
        );

        validate_next_declaration!(
            decl_iter,
            LonghandDeclaration::background_color(Color::RGBA(ref rgba)),
            assert_eq!(*rgba, RGBA::new(255, 0, 0, 255))
        );

        validate_next_declaration!(
            decl_iter,
            LonghandDeclaration::margin_top(
                SpecifiedLengthOrPercentageOrAuto::Length(
                    SpecifiedLength::Absolute(Length{ px: value }))),
            assert_eq!(*value, 96.0)
        );

        validate_next_declaration!(
            decl_iter,
            LonghandDeclaration::margin_left(
                SpecifiedLengthOrPercentageOrAuto::Length(
                    SpecifiedLength::Absolute(Length{ px: value }))),
            assert_eq!(*value, 96.0)
        );

        validate_next_declaration!(
            decl_iter,
            LonghandDeclaration::margin_bottom(
                SpecifiedLengthOrPercentageOrAuto::Length(
                    SpecifiedLength::Absolute(Length{ px: value }))),
            assert_eq!(*value, 96.0)
        );

        validate_next_declaration!(
            decl_iter,
            LonghandDeclaration::margin_right(
                SpecifiedLengthOrPercentageOrAuto::Length(
                    SpecifiedLength::Absolute(Length{ px: value }))),
            assert_eq!(*value, 96.0)
        );

        validate_next_declaration!(
            decl_iter,
            LonghandDeclaration::padding_top(
                SpecifiedLengthOrPercentage::Length(
                    SpecifiedLength::Absolute(Length{ px: value }))),
            assert_eq!(*value, 10.0)
        );

        validate_next_declaration!(
            decl_iter,
            LonghandDeclaration::padding_left(
                SpecifiedLengthOrPercentage::Length(
                    SpecifiedLength::Absolute(Length{ px: value }))),
            assert_eq!(*value, 15.0)
        );


        validate_next_declaration!(
            decl_iter,
            LonghandDeclaration::padding_bottom(
                SpecifiedLengthOrPercentage::Length(
                    SpecifiedLength::Absolute(Length{ px: value }))),
            assert_eq!(*value, 0.0)
        );


        validate_next_declaration!(
            decl_iter,
            LonghandDeclaration::padding_right(
                SpecifiedLengthOrPercentage::Length(
                    SpecifiedLength::Absolute(Length{ px: value }))),
            assert_eq!(*value, 5.0)
        );

        validate_next_declaration!(
            decl_iter,
            LonghandDeclaration::padding_top(
                SpecifiedLengthOrPercentage::Length(
                    SpecifiedLength::Absolute(Length{ px: value }))),
            assert_eq!(*value, 20.0)
        );
    }
}