// Modules are other .rs source files
mod markdown; // convert markdown to HTML
mod renderer; // render text, rectangles, etc.
mod c_helper; // support functions to interface with C
mod style;    // css styles and parsing
mod test;

use crate::style::properties::*;

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

    let result = parse_declaration_block(css);

    for rule in result.iter().enumerate() {
        println!("Declaration {}: {:?}", rule.0, rule.1);
    }
}


fn main() {
    simple_css_parsing();
}