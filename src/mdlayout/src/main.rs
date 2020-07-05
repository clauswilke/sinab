// Modules are other .rs source files
mod markdown; // convert markdown to HTML
mod renderer; // render text, rectangles, etc.
mod c_helper; // support functions to interface with C
mod style;    // css styles and parsing
mod layout;
mod test;

use crate::style::properties::parse_declaration_block;

use cssparser::Color;
use cssparser::RGBA;

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

fn color_to_string(color: &Color) -> String {
    match color {
        Color::RGBA(RGBA{ red, green, blue, alpha }) => {
            if *alpha == 255 { // without alpha component
                format!("#{:x}{:x}{:x}", *red, *green, *blue)
            } else { // with alpha component
                format!("#{:x}{:x}{:x}{:x}", *red, *green, *blue, *alpha)
            }
        }
        _ => {
            String::from("#000000")
        }
    }
}

fn main() {
    simple_css_parsing();

    let c: Color = Color::RGBA(RGBA::new(254, 40, 80, 128));
    println!("{}", color_to_string(&c));
}