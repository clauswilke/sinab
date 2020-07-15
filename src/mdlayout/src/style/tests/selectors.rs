// A complete test of parsing some html and css and applying the appropriate rules
// to specific elements

use crate::{
    dom::{Document, NodeId, NodeData},
    style::{StyleSet, style_for_element}
};

fn process_node<'dom>(node_id: NodeId, author_styles: &'dom StyleSet, document: &'dom Document,
) {
    let style = style_for_element(author_styles, document, node_id, None);

    let node = &document[node_id];
    match node.data {
        NodeData::Element(ref elt) => {
            println!("element: {:?}", elt.name.local);
        },
        NodeData::Text{ref contents} => {
            println!("text: {}", contents);
        },
        _ => {
            println!("other");
        },
    }

    println!("color: {:?}", &style.color.color);
    println!("background color: {:?}", &style.background.background_color);


    if let Some(child_id) = node.first_child {
        for (number, nid) in document.node_and_following_siblings(child_id).enumerate() {
            println!("nesting level: {}", number);
            process_node(nid, author_styles, document);
        }
    }
}


macro_rules! validate_element_type {
        ($node_id:expr, $element_name:tt, $document:expr) => {
            let node = &$document[$node_id];
            if let NodeData::Element(ref elt) = node.data {
                assert_eq!(elt.name.local, local_name!($element_name));
            } else {
                assert!(false);
            }
        }
    }


#[test]
fn selectors() {
    let text_input = "<body><p>Hello world</p></body>";
    let css_input = "p {color: red;}";

    let document = Document::parse_html(text_input.as_bytes());
    let author_styles = &document.parse_stylesheets(Some(css_input));
    process_node(document.root_element(), author_styles, &document);


    let node_id = document.root_element();
    validate_element_type!(node_id, "html", document);

    let node_id = document[node_id].first_child.unwrap();
    validate_element_type!(node_id, "head", document);

    let node_id = document[node_id].next_sibling.unwrap();
    validate_element_type!(node_id, "body", document);

}
