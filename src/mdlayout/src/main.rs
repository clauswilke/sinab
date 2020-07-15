extern crate mdlayout;

/*
use mdlayout::style2::properties::parse_declaration_block;

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

 */

use mdlayout::test::test_dom;

fn main() {
    //simple_css_parsing();
    test_dom();
}