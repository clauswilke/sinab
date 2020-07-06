// Modules are other .rs source files
mod renderer; // render text, rectangles, etc.
mod c_helper; // support functions to interface with C
mod style;    // css styles and parsing
mod dom;      // document object model
mod markdown; // convert markdown to HTML
mod layout;
mod test;

extern crate cssparser;

#[macro_use]
extern crate html5ever;


use crate::style::properties::parse_declaration_block;

fn simple_css_parsing() {
    let css = r#"
    color: red;
    testcolor: #0456ab;
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