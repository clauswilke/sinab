// Modules are other .rs source files
pub mod renderer; // render text, rectangles, etc.
pub mod c_helper; // support functions to interface with C
pub mod style;    // css styles and parsing
pub mod dom;      // document object model
pub mod markdown; // convert markdown to HTML
pub mod layout;
mod test;

extern crate cssparser;

#[macro_use]
extern crate html5ever;
