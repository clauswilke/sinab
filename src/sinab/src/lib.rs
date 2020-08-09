#[macro_use]
extern crate cssparser;

#[macro_use]
extern crate html5ever;

#[macro_use]
extern crate sinab_derive;

// order of modules follows the dependency graph; earlier
// modules should not depend on later modules (not quite there yet)
#[macro_use]
pub mod utils;
mod markdown; // convert markdown to HTML
mod dom;      // document object model
mod style;
mod graphics_engine;
mod layout;
mod layout2;
mod paint;
pub mod test;

// copied from victor, not yet integrated
mod primitives;
mod text;
mod geom;
