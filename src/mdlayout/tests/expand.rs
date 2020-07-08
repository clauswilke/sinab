// A mock integration test to make it easy to look at expanded macro code for specific macros.
// Run with: cargo expand --test expand --theme GitHub

/* Not sure if this is needed; needs more investigation
#[macro_use]
extern crate mdlayout_derive;

use cssparser::*;
*/

//#[derive(Copy, Clone, Eq, Parse, PartialEq, SpecifiedAsComputed)]
#[derive(Debug, PartialEq)]
enum Float {
    None,
    Left,
    Right,
}

#[test]
fn expand_test() {
    let f = Float::Left;

    println!("Hello world! {:?}", f);
}