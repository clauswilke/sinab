// Modules are other .rs source files
mod markdown; // convert markdown to HTML
mod renderer; // render text, rectangles, etc.
mod c_helper; // support functions to interface with C
mod test;
mod style;

use cssparser::*;
use style::properties::*;

fn simple_css_parsing() {
    let css = r#"
    color: red;
    margin: 5px;
    font-size: 5px;
    font-size: 100%;
    font-size: 0;
    font-size: 1em;
    padding-top: -10px;
    background: #ffe7e8; border: 0 solid #e66465;
    @charset
 "#;

    let mut parser_input = ParserInput::new(css);
    let mut input = Parser::new(&mut parser_input);

    let result: Vec<CssProperty> =
        DeclarationListParser::new(&mut input, CssPropertyParser)
            // we discard all declarations that weren't parsed correctly
            .filter_map(|x| { x.ok() })
            .collect();

    for rule in result.iter().enumerate() {
        println!("Declaration {}: {:?}", rule.0, rule.1);
    }
}


fn main() {
    simple_css_parsing();
}