// Modules are other .rs source files
pub mod renderer; // render text, rectangles, etc.
pub mod c_helper; // support functions to interface with C
pub mod dom;      // document object model
pub mod markdown; // convert markdown to HTML
pub mod layout;
pub mod test;

// copied from victor, not yet integrated
pub mod primitives;
pub mod text;
#[macro_use]
mod tagged_union_with_jump_tables;
mod geom;
mod style;

#[macro_use]
extern crate cssparser;

#[macro_use]
extern crate html5ever;

#[macro_use]
extern crate mdlayout_derive;
