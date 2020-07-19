#[macro_use]
pub mod utils;
mod graphics_engine;
mod dom;      // document object model
mod style;
mod markdown; // convert markdown to HTML
mod layout;
pub mod test;

// copied from victor, not yet integrated
mod primitives;
mod text;
mod geom;

#[macro_use]
extern crate cssparser;

#[macro_use]
extern crate html5ever;

#[macro_use]
extern crate mdlayout_derive;
