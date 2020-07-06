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
