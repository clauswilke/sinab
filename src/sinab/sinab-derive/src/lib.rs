#![recursion_limit = "128"]

/// Macros that enable `#[derive(...)]` features in the main crate. Originally written by Simon Sapin.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

include!("style.rs");
include!("generic.rs"); // TODO: Should this be integrated into "style.rs"? It's also used in style.

// The remainder of this file is a test of proc_macro_derive().
// It implements a trait StringFormat that converts a given type
// into a string "hello type: <Type>", where <Type> is the type
// for which the trait is implemented.

#[proc_macro_derive(StringFormat)]
pub fn string_format_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    let expanded = quote! {
        impl StringFormat for #name {
            fn string_format(&self) -> String {
                "hello type: ".to_string() + stringify!(#name)
            }
        }
    };

    TokenStream::from(expanded)
}