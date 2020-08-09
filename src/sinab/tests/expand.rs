// A mock integration test to make it easy to look at expanded macro code for specific macros.
// Run with: cargo expand --test expand --theme GitHub


#[macro_use]
extern crate sinab_derive;


trait StringFormat {
    // Static method signature; `Self` refers to the implementor type.
    fn string_format(&self) -> String;
}


//#[derive(Copy, Clone, Eq, Parse, PartialEq, SpecifiedAsComputed)]
#[derive(Debug, PartialEq, StringFormat)]
enum Float {
    None,
    Left,
    Right,
}

#[test]
fn expand_test() {
    let f = Float::Left;

    println!("Macro expansion test.\nOutput from #[derive]ed code: {:?}", f.string_format());
}


